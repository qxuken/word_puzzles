use axum::{
    http::{header, HeaderValue},
    response::{IntoResponse, Response},
};
use maud::Markup;

pub struct HtmlTemplate(pub Markup);

impl IntoResponse for HtmlTemplate {
    fn into_response(self) -> Response {
        (
            [
                (
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/html; charset=utf-8"),
                ),
                (
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("max-age:300, private"),
                ),
            ],
            self.0.into_string(),
        )
            .into_response()
    }
}
