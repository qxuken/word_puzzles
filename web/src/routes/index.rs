use maud::html;

use crate::templates::{html_template::HtmlTemplate, layout::layout};

pub async fn index_route() -> HtmlTemplate {
    let template = layout(html!(h1 { "Puzzle solver" }));
    HtmlTemplate(template)
}
