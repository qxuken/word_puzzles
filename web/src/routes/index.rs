use axum::extract::Query;
use maud::{html, Markup};
use serde::Deserialize;

use crate::templates::{
    layout::layout,
    spelling_bee::{self, ContainerGameMode},
};

#[derive(Deserialize)]
pub struct IndexPageQuery {
    pub mode: Option<String>,
}

pub async fn index_route(Query(query): Query<IndexPageQuery>) -> Markup {
    let mode = if query.mode.is_some_and(|m| m == "hinted") {
        ContainerGameMode::Hinted
    } else {
        ContainerGameMode::Simple
    };
    let template = html!(
        h1 { "Puzzle solver" };
        (spelling_bee::container(mode))
    );
    layout(template, None)
}
