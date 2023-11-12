mod assets;
mod index;

use axum::{routing::get, Router};

use crate::{app_state::SharedAppState, assets::ASSETS_PATH};

use self::{
    assets::{assets_route, favicon_route},
    index::index_route,
};

pub fn create_router() -> Router<SharedAppState> {
    Router::new()
        .route("/", get(index_route))
        .route("/favicon.svg", get(favicon_route))
        .route(ASSETS_PATH, get(assets_route))
}
