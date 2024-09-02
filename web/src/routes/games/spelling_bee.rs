use crate::{
    app_state::SharedAppState,
    templates::spelling_bee::{input_hinted, input_simple, solution},
};
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use maud::html;
use serde::Deserialize;
use words::spelling_bee::{SpellingBee, SpellingBeeHintedParams, SpellingBeeSimpleParams};

pub async fn input_simple_route() -> impl IntoResponse {
    html!(
        ul id="spelling-bee-solution" hx-swap-oob="true" {}
        (input_simple("", ""))
    )
}

pub async fn input_hinted_route() -> impl IntoResponse {
    html!(
        ul id="spelling-bee-solution" hx-swap-oob="true" {}
        (input_hinted("", "", "", ""))
    )
}

#[derive(Deserialize)]
pub struct SimpleSolutionForm {
    pub letters: Option<String>,
}

pub async fn solve_simple_route(
    State(app_state): State<SharedAppState>,
    Form(data): Form<SimpleSolutionForm>,
) -> impl IntoResponse {
    if let Some(letters) = data.letters {
        match SpellingBeeSimpleParams::new(&letters) {
            Ok(game) => {
                let words = game.scan_dict(&app_state.words_dict, &app_state.words_shortcuts);
                html!(
                    div.errors id="letters-error" hx-swap-oob="true" {}
                    (solution(words))
                )
            }
            Err(err) => html!(
                div.errors id="letters-error" hx-swap-oob="true" {
                    (err.to_string())
                }
                (solution(vec![]))
            ),
        }
    } else {
        html!(
            div.errors id="letters-error" hx-swap-oob="true" {}
            (solution(vec![]))
        )
    }
}

#[derive(Deserialize)]
pub struct HintedSolutionForm {
    pub letters: Option<String>,
    pub letter_matrix: Option<String>,
    pub letter_list: Option<String>,
}

pub async fn solve_hinted_route(
    State(app_state): State<SharedAppState>,
    Form(data): Form<HintedSolutionForm>,
) -> impl IntoResponse {
    if let Some(letters) = data.letters.map(|l| {
        l.to_lowercase()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
    }) {
        let allowed_bytes = letters.as_bytes();
        let letters_len = data
            .letter_matrix
            .map(|ll| ll.to_lowercase())
            .map(|ll| {
                let mut out: Vec<(u8, Vec<usize>)> = vec![];
                for entry in ll.split('\n') {
                    let bytes = entry.as_bytes();
                    if let Some(first_byte) = bytes.first() {
                        if !allowed_bytes.contains(first_byte) {
                            continue;
                        }
                        let mut avail_lens = vec![];
                        let parts = entry.split_whitespace().collect::<Vec<&str>>();
                        for i in 1..6 {
                            if parts.get(i).is_some_and(|&e| e != "-") {
                                avail_lens.push(i + 3);
                            }
                        }

                        out.push((*first_byte, avail_lens));
                    }
                }
                out
            })
            .unwrap_or_default();
        let letter_list = data
            .letter_list
            .map(|ll| ll.to_lowercase().as_bytes().to_vec())
            .filter(|bytes| bytes.len() > 1)
            .map(|bytes| {
                let mut out = vec![];
                let mut i = 0;
                while i < bytes.len() - 1 {
                    if allowed_bytes.contains(&bytes[i]) && allowed_bytes.contains(&bytes[i + 1]) {
                        out.push([bytes[i], bytes[i + 1]]);
                        i += 2;
                        continue;
                    }

                    i += 1;
                }
                out
            })
            .unwrap_or_default();
        match SpellingBeeHintedParams::new(&letters, letters_len, letter_list) {
            Ok(game) => {
                let words = game.scan_dict(&app_state.words_dict, &app_state.words_shortcuts);
                html!(
                    div.errors id="letters-error" hx-swap-oob="true" {}
                    (solution(words))
                )
            }
            Err(err) => html!(
                div.errors id="letters-error" hx-swap-oob="true" {
                    (err.to_string())
                }
                (solution(vec![]))
            ),
        }
    } else {
        html!(
            div.errors id="letters-error" hx-swap-oob="true" {}
            (solution(vec![]))
        )
    }
}

pub fn create_router() -> Router<SharedAppState> {
    Router::new()
        .route("/input_simple", get(input_simple_route))
        .route("/input_hinted", get(input_hinted_route))
        .route("/solve_simple", post(solve_simple_route))
        .route("/solve_hinted", post(solve_hinted_route))
}
