use crate::app_state::SharedAppState;
use axum::Router;

mod spelling_bee;

pub fn create_router() -> Router<SharedAppState> {
    Router::new().nest("/spelling_bee", spelling_bee::create_router())
}
