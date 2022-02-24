#![allow(clippy::unused_async)]

use anyhow::{Context, Result};
use axum::{
    body::BoxBody,
    extract::{ContentLengthLimit, Extension, Json, Path},
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
};
use serde_json::Value;
use tokio::fs;
use tracing::{error, info, warn};

pub mod apps;
pub mod error;
pub mod users;

use self::users::UserError;
use crate::{
    db::{
        models::{NewReport, NewVersion},
        repositories::{self, AppRepository, ReportRepository, UserSaveError, VersionRepository},
        DbConnPool,
    },
    extractors::User,
    report::Report,
    retrace,
    templates::{self, ErrorPage},
};

#[derive(derive_more::From)]
pub enum AppError {
    User(UserError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<BoxBody> {
        let (status, message) = match self {
            Self::User(err) => match err {
                UserError::Save(err) => match err {
                    UserSaveError::AlreadyExists(name) => (
                        StatusCode::CONFLICT,
                        format!("The user `{}` already exists", name),
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("An internal error happened: {:?}", err),
                    ),
                },
            },
        };

        ErrorPage { status, message }.into_response()
    }
}

pub fn index() -> impl IntoResponse {
    Redirect::temporary("/apps".parse().unwrap())
}

pub async fn versions_list(
    Path((id,)): Path<(i64,)>,
    Extension(db): Extension<DbConnPool>,
) -> impl IntoResponse {
    let version_repo = repositories::version_repo(db.clone());
    let app_repo = repositories::app_repo(db);

    let app = app_repo.get(id).await.unwrap();
    let versions = version_repo.list_by_app(id).await.unwrap();

    templates::apps::Details { app, versions }
}

pub async fn report_save(
    user: User,
    ContentLengthLimit(Json(raw)): ContentLengthLimit<Json<Value>, { 1024 * 512 }>,
    Extension(db): Extension<DbConnPool>,
) -> impl IntoResponse {
    if let Err(e) = save_raw(&raw).await {
        error!("failed saving report to file: {}", e);
    }

    let report = match serde_json::from_value::<Report>(raw) {
        Ok(r) => r,
        Err(e) => {
            warn!("invalid report: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    let app_repo = repositories::app_repo(db.clone());
    let version_repo = repositories::version_repo(db.clone());
    let report_repo = repositories::report_repo(db);

    let app = app_repo
        .get_by_username(user.username().to_owned())
        .await
        .unwrap();
    let version_id = version_repo
        .get_or_create(NewVersion {
            app_id: app.id,
            name: report.app_version_name.clone(),
            code: i64::from(report.app_version_code),
        })
        .await
        .unwrap();
    report_repo
        .save(NewReport {
            version_id,
            report_id: report.report_id.clone(),
            crash_date: report.user_crash_date.clone(),
        })
        .await
        .unwrap();

    tokio::spawn(async move {
        match retrace::retrace(&report.stack_trace).await {
            Ok(st) => info!("Stacktrace: {}", st),
            Err(e) => warn!("failed retracing: {}", e),
        }
    });

    StatusCode::OK
}

#[cfg(debug_assertions)]
const REPORTS_DIR: &str = "reports";
#[cfg(not(debug_assertions))]
const REPORTS_DIR: &str = concat!("/var/lib/", env!("CARGO_PKG_NAME"), "/reports");

async fn save_raw(raw: &Value) -> Result<()> {
    let report_id = raw
        .as_object()
        .and_then(|r| r.get("REPORT_ID"))
        .and_then(Value::as_str)
        .context("report id is missing")?;

    fs::create_dir_all(REPORTS_DIR).await?;

    fs::write(
        format!("{}/{}.json", REPORTS_DIR, report_id),
        serde_json::to_vec(raw)?,
    )
    .await
    .map_err(Into::into)
}
