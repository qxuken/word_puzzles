use maud::{html, Markup};
use words::spelling_bee::LETTERS_COUNT;

const LETTER_INPUT_SIZE: usize = LETTERS_COUNT * 2 + 4;

pub fn input_letters(letters: &str, error: &str) -> Markup {
    html!(
      fieldset {
        label for="letters" { "Letters" };
        input id="letters"
              size={(LETTER_INPUT_SIZE)}
              required
              name="letters"
              value={(letters)};
        div.errors id="letters-error" {
          (error)
        }
      }
    )
}

pub fn input_hinted(
    letters: &str,
    letters_error: &str,
    letter_matrix: &str,
    letter_list: &str,
) -> Markup {
    html!(
      form id="spelling-bee-form"
            hx-post="/games/spelling_bee/solve_hinted"
            hx-trigger="keyup change check delay:250ms"
            hx-swap="outerHTML"
            hx-target="#spelling-bee-solution"{
        (input_letters(letters, letters_error))
        fieldset {
          label for="letter_matrix" { "Start matrix" };
          textarea id="letter_matrix"
                name="letter_matrix"
                rows="10"
                cols="60"
                value={(letter_matrix)}  {};
        }
        fieldset {
          label for="letter_list" { "Two letter list" };
          textarea id="letter_list"
                name="letter_list"
                rows="10"
                cols="25"
                value={(letter_list)} {};
        }
      }
    )
}

pub fn input_simple(letters: &str, letters_error: &str) -> Markup {
    html!(
      form id="spelling-bee-form"
            hx-post="/games/spelling_bee/solve_simple"
            hx-trigger="keyup change check delay:250ms"
            hx-swap="outerHTML"
            hx-target="#spelling-bee-solution" {
        (input_letters(letters, letters_error))
      }
    )
}
