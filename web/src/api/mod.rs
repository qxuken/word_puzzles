use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
// use simple_server_timing_header::Timer;

use crate::app_state::SharedAppState;

pub fn create_router() -> Router<SharedAppState> {
    Router::new().route("/search", get(search_route))
}

#[derive(Deserialize)]
struct SearchQuery {
    pub q: Option<String>,
}

async fn search_route(
    State(app_state): State<SharedAppState>,
    Query(query): Query<SearchQuery>,
) -> Response {
    // let mut timer = Timer::new();

    let search: String = query.q.map(|s| s.to_lowercase()).unwrap_or_default();
    let search_bytes = search.as_bytes();
    // timer.add("parsed_query");

    let Some(range) = app_state.words_shortcuts.search_range(search_bytes) else {
        return (StatusCode::NOT_FOUND, "No words found").into_response();
    };
    let words = if search.len() > 2 {
        app_state.words_dict.search_range(range, search_bytes)
    } else {
        app_state
            .words_dict
            .iter_range(range)
            .map(|b| String::from_utf8_lossy(b).to_string())
            .collect()
    };
    // timer.add("search");

    // let mut res = Json(words).into_response();
    // timer.add("response_prep");
    // res.headers_mut()
    //     .append("server-timing", timer.header_value().parse().unwrap());
    // res
    Json(words).into_response()
}
