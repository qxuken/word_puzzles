use crate::templates::spelling_bee::{input_hinted, input_simple, solution};
use maud::{html, Markup};

use super::ContainerGameMode;

fn switch() -> Markup {
    html!(
      div hx-target="#spelling-bee-form" hx-swap="outerHTML" hx-include="#spelling-bee-form" hx-params="*" {
        button hx-get="/games/spelling_bee/input_simple" hx-push-url="/?mode=simple" { "Only letters" };
        button hx-get="/games/spelling_bee/input_hinted" hx-push-url="/?mode=hinted" { "Advanced hints" };
      }
    )
}

pub fn container(mode: ContainerGameMode) -> Markup {
    let input = match mode {
        ContainerGameMode::Simple => input_simple("", ""),
        ContainerGameMode::Hinted => input_hinted("", "", "", ""),
    };
    html!(
      div {
        h2 { "Spelling bee" };
        (switch())
        (input)
        (solution(vec![]))
      }
    )
}
