use askama::Template;
use axum::response::{self, IntoResponse};
use hyper::{Body, Response, StatusCode};

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response<Body> {
        match self.0.render() {
            Ok(html) => response::Html(html).into_response(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap(),
        }
    }
}
