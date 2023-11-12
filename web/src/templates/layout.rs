use crate::assets::ASSETS_PREFIX;
use maud::{html, Markup, DOCTYPE};

pub fn layout(content: Markup) -> Markup {
    html!(
      (DOCTYPE)
      html lang="en" {
        head {
          meta charset="utf-8";
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          meta name="theme-color" content="#000000";

          title { "Puzzle solver" }

          link rel="preload" href={(ASSETS_PREFIX) "/fonts/inter/inter-roman.var.woff2?v=3.15"} as="font" type="font/woff2" crossorigin;
          link rel="preload" href={(ASSETS_PREFIX) "/fonts/inter.css?v=3.15"} as="style";
          link rel="stylesheet" href={(ASSETS_PREFIX) "/fonts/inter.css?v=3.15"};

          link rel="preload" href={(ASSETS_PREFIX) "/styles.css"} as="style";
          link rel="stylesheet" href={(ASSETS_PREFIX) "/styles.css"};

          script async src={(ASSETS_PREFIX) "/htmx.min.js?v=1.9.8"} {};
          script async src={(ASSETS_PREFIX) "/script.js"} type="module" {};
        }
        body {
          (content)
        }
      }
    )
}
