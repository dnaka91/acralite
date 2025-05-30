use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use tracing::instrument;

use super::AppError;
use crate::{
    db::{
        DbConnPool,
        models::NewUser,
        repositories::{self, UserRepository, UserSaveError},
    },
    templates,
};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("failed saving user")]
    Save(#[from] UserSaveError),
}

#[instrument(skip_all)]
pub async fn list(State(db): State<DbConnPool>) -> impl IntoResponse {
    let user_repo = repositories::user_repo(db);
    let users = user_repo.list().await.unwrap();

    templates::users::List { users }
}

#[instrument(skip_all)]
pub async fn create() -> impl IntoResponse {
    templates::users::Create {}
}

#[derive(Deserialize)]
pub struct NewUserForm {
    username: String,
    password: String,
}

impl From<NewUserForm> for NewUser {
    fn from(value: NewUserForm) -> Self {
        Self {
            username: value.username,
            password: value.password,
        }
    }
}

#[instrument(skip_all)]
pub async fn create_post(
    State(db): State<DbConnPool>,
    Form(data): Form<NewUserForm>,
) -> Result<impl IntoResponse, AppError> {
    repositories::user_repo(db)
        .save(data.into())
        .await
        .map_err(UserError::Save)?;
    Ok(Redirect::to("/users"))
}
