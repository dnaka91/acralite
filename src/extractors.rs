use std::sync::Arc;

use axum::{
    body::Body,
    extract::{FromRef, FromRequestParts},
    http::{
        StatusCode,
        header::{AUTHORIZATION, WWW_AUTHENTICATE},
        request::Parts,
    },
    response::{IntoResponse, Response},
};
use axum_extra::{TypedHeader, typed_header::TypedHeaderRejection};
use headers::{Authorization, authorization::Basic};

use crate::settings::Auth;

pub struct User(Basic);

impl User {
    pub fn username(&self) -> &str {
        self.0.username()
    }
}

impl<S> FromRequestParts<S> for User
where
    Arc<Auth>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if !parts.headers.contains_key(AUTHORIZATION) {
            return Err(Self::Rejection::InvalidCredentials);
        }

        let settings = Arc::<Auth>::from_ref(state);
        let TypedHeader(Authorization(header)) =
            TypedHeader::<Authorization<Basic>>::from_request_parts(parts, state).await?;

        if header.username() != settings.username || header.password() != settings.password {
            return Err(Self::Rejection::InvalidCredentials);
        }

        Ok(Self(header))
    }
}

#[derive(Debug)]
pub enum AuthRejection {
    TypedHeaderRejection(TypedHeaderRejection),
    InvalidCredentials,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        match self {
            Self::TypedHeaderRejection(r) => r.into_response(),
            Self::InvalidCredentials => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(WWW_AUTHENTICATE, "Basic")
                .body(Body::empty())
                .unwrap(),
        }
    }
}

impl From<TypedHeaderRejection> for AuthRejection {
    fn from(value: TypedHeaderRejection) -> Self {
        Self::TypedHeaderRejection(value)
    }
}
