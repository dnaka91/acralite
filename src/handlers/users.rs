use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;

use super::AppError;
use crate::{
    db::{
        models::NewUser,
        repositories::{self, UserRepository, UserSaveError},
        DbConnPool,
    },
    templates,
};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("failed saving user")]
    Save(#[from] UserSaveError),
}

pub async fn list(Extension(db): Extension<DbConnPool>) -> impl IntoResponse {
    let user_repo = repositories::user_repo(db);
    let users = user_repo.list().await.unwrap();

    templates::users::List { users }
}

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

pub async fn create_post(
    Form(data): Form<NewUserForm>,
    Extension(db): Extension<DbConnPool>,
) -> Result<impl IntoResponse, AppError> {
    repositories::user_repo(db)
        .save(data.into())
        .await
        .map_err(UserError::Save)?;
    Ok(Redirect::to("/users".parse().unwrap()))
}
