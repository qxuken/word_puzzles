use crate::assets::ASSETS_PREFIX;
use maud::{html, Markup, DOCTYPE};

fn head(title: &str) -> Markup {
    html!(
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta name="theme-color" content="#000000";

        title { (title) }
        link rel="icon" href="/favicon.svg" type="image/svg+xml";

        link rel="preload" href={(ASSETS_PREFIX) "/fonts/inter/inter-roman.var.woff2?v=3.15"} as="font" type="font/woff2" crossorigin;
        link rel="preload" href={(ASSETS_PREFIX) "/fonts/inter.css?v=3.15"} as="style";
        link rel="stylesheet" href={(ASSETS_PREFIX) "/fonts/inter.css?v=3.15"};

        link rel="preload" href={(ASSETS_PREFIX) "/styles.css"} as="style";
        link rel="stylesheet" href={(ASSETS_PREFIX) "/styles.css"};

        script src={(ASSETS_PREFIX) "/libs/htmx.min.js?v=1.9.8"} {};
        // script src={(ASSETS_PREFIX) "/libs/_hyperscript.min.js?v=0.9.12"} {};
        script async src={(ASSETS_PREFIX) "/script.js"} {};
      }
    )
}

pub fn layout(content: Markup, title: Option<&str>) -> Markup {
    html!(
      (DOCTYPE)
      html lang="en" {
        (head(title.unwrap_or("Puzzle solver")))
        body hx-boost="true" {
          (content)
        }
      }
    )
}
