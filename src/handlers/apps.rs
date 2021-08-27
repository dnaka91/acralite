use anyhow::anyhow;
use axum::{extract::Extension, response::IntoResponse};

use crate::{
    db::{
        repositories::{self, AppRepository},
        DbConnPool,
    },
    responses::HtmlTemplate,
    templates,
};

pub async fn list(Extension(db): Extension<DbConnPool>) -> impl IntoResponse {
    let app_repo = repositories::app_repo(db);
    let apps = app_repo.list().await.unwrap();

    HtmlTemplate(templates::apps::Index { apps })
}

pub async fn create() -> impl IntoResponse {
    HtmlTemplate(templates::apps::Create {})
}

pub fn create_post() -> impl IntoResponse {
    HtmlTemplate(templates::apps::CreateResult {
        result: Err(anyhow!("not implemented yet!")),
    })
}
