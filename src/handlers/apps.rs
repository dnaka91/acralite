use anyhow::anyhow;
use axum::{extract::State, response::IntoResponse};
use tracing::instrument;

use crate::{
    db::{
        DbConnPool,
        repositories::{self, AppRepository},
    },
    templates,
};

#[instrument(skip_all)]
pub async fn list(State(db): State<DbConnPool>) -> impl IntoResponse {
    let app_repo = repositories::app_repo(db);
    let apps = app_repo.list().await.unwrap();

    templates::apps::Index { apps }
}

#[instrument(skip_all)]
pub async fn create() -> impl IntoResponse {
    templates::apps::Create {}
}

#[instrument(skip_all)]
pub fn create_post() -> impl IntoResponse {
    templates::apps::CreateResult {
        result: Err(anyhow!("not implemented yet!")),
    }
}
