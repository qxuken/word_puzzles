use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
// use simple_server_timing_header::Timer;
use words::search_string;

pub fn create_router() -> Router {
    Router::new().route("/search", get(search_route))
}

#[derive(Deserialize)]
struct SearchQuery {
    pub q: Option<String>,
}

async fn search_route(Query(query): Query<SearchQuery>) -> Response {
    // let mut timer = Timer::new();
    let search: String = query.q.map(|s| s.to_lowercase()).unwrap_or_default();
    // timer.add("parsed_query");
    let words = search_string(search.bytes());
    // timer.add("search");

    // let mut res = Json(words).into_response();
    // timer.add("response_prep");
    // res.headers_mut()
    //     .append("server-timing", timer.header_value().parse().unwrap());
    Json(words).into_response()
}
