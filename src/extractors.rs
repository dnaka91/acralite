use std::sync::Arc;

use axum::{
    body::{self, BoxBody, Empty},
    extract::{
        rejection::{ExtensionRejection, TypedHeaderRejection},
        Extension, FromRequest, RequestParts, TypedHeader,
    },
    http::{
        header::{AUTHORIZATION, WWW_AUTHENTICATE},
        Response, StatusCode,
    },
    response::IntoResponse,
};
use headers::{authorization::Basic, Authorization};

use crate::settings::Auth;

pub struct User(Basic);

impl User {
    pub fn username(&self) -> &str {
        self.0.username()
    }
}

#[axum::async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = AuthRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if !req
            .headers()
            .map(|headers| headers.contains_key(AUTHORIZATION))
            .unwrap_or_default()
        {
            return Err(Self::Rejection::InvalidCredentials);
        }

        let Extension(settings) = Extension::<Arc<Auth>>::from_request(req).await?;
        let TypedHeader(Authorization(header)) =
            TypedHeader::<Authorization<Basic>>::from_request(req).await?;

        if header.username() != settings.username || header.password() != settings.password {
            return Err(Self::Rejection::InvalidCredentials);
        }

        Ok(Self(header))
    }
}

#[derive(Debug)]
pub enum AuthRejection {
    ExtensionRejection(ExtensionRejection),
    TypedHeaderRejection(TypedHeaderRejection),
    InvalidCredentials,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response<BoxBody> {
        match self {
            Self::ExtensionRejection(r) => r.into_response(),
            Self::TypedHeaderRejection(r) => r.into_response(),
            Self::InvalidCredentials => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(WWW_AUTHENTICATE, "Basic")
                .body(body::boxed(Empty::new()))
                .unwrap(),
        }
    }
}

impl From<ExtensionRejection> for AuthRejection {
    fn from(value: ExtensionRejection) -> Self {
        Self::ExtensionRejection(value)
    }
}

impl From<TypedHeaderRejection> for AuthRejection {
    fn from(value: TypedHeaderRejection) -> Self {
        Self::TypedHeaderRejection(value)
    }
}
