use maud::{html, Markup};

pub fn solution(words: Vec<String>) -> Markup {
    html!(
      ul id="spelling-bee-solution" {
        @for word in &words {
          li { (word) }
        }
      }
    )
}
