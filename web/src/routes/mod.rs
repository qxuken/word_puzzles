mod assets;
mod games;
mod index;

use self::{
    assets::{assets_route, favicon_route},
    index::index_route,
};
use crate::{app_state::SharedAppState, assets::ASSETS_PATH};
use axum::{routing::get, Router};

pub fn create_router() -> Router<SharedAppState> {
    Router::new()
        .route("/", get(index_route))
        .nest("/games", games::create_router())
        .route("/favicon.svg", get(favicon_route))
        .route(ASSETS_PATH, get(assets_route))
}
