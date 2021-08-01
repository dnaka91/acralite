#![allow(clippy::unused_async)]

use anyhow::{anyhow, Context, Result};
use axum::{
    extract::{ContentLengthLimit, Extension, Json, UrlParams},
    prelude::*,
    response::IntoResponse,
};
use hyper::{header::LOCATION, Response, StatusCode};
use serde_json::Value;
use tokio::fs;
use tracing::{error, info, warn};

use crate::{
    db::{
        models::{NewReport, NewVersion},
        repositories::{self, AppRepository, ReportRepository, VersionRepository},
        DbConnPool,
    },
    extractors::User,
    report::Report,
    responses::HtmlTemplate,
    retrace, templates,
};

pub fn index() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(LOCATION, "/apps")
        .body(Body::empty())
        .unwrap()
}

pub async fn apps_list(Extension(db): Extension<DbConnPool>) -> impl IntoResponse {
    let app_repo = repositories::app_repo(db);
    let apps = app_repo.list().await.unwrap();

    HtmlTemplate(templates::apps::Index { apps })
}

pub async fn apps_create() -> impl IntoResponse {
    HtmlTemplate(templates::apps::Create {})
}

pub fn apps_create_post() -> impl IntoResponse {
    HtmlTemplate(templates::apps::CreateResult {
        result: Err(anyhow!("not implemented yet!")),
    })
}

pub async fn versions_list(
    UrlParams((id,)): UrlParams<(i64,)>,
    Extension(db): Extension<DbConnPool>,
) -> impl IntoResponse {
    let version_repo = repositories::version_repo(db.clone());
    let app_repo = repositories::app_repo(db);

    let app = app_repo.get(id).await.unwrap();
    let versions = version_repo.list_by_app(id).await.unwrap();

    HtmlTemplate(templates::apps::Details { app, versions })
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
const REPORTS_DIR: &str = concat!("/var/lib/", env!("CARGO_CRATE_NAME"), "/reports");

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
