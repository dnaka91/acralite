#![allow(clippy::needless_pass_by_value)]

use axum::BoxError;
use hyper::StatusCode;
use tower::timeout::error::Elapsed;

pub fn timeout(err: BoxError) -> StatusCode {
    if err.is::<Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
