use anyhow::Result;
use axum::http::Uri;
use url::Url;

pub fn modify_query(new_queries: Vec<(String, String)>, url: Option<String>) -> Result<Uri> {
    if let Some(url) = url {
        let mut url = Url::parse(&url)?;
        let new_query_names: Vec<&String> = new_queries.iter().map(|(name, _)| name).collect();
        let old_queries: Vec<(String, String)> = url
            .query_pairs()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .filter(|(name, value)| !value.is_empty() && !new_query_names.contains(&name))
            .collect();
        url.query_pairs_mut()
            .clear()
            .extend_pairs(&old_queries)
            .extend_pairs(&new_queries);
        Ok(url.as_str().to_string().parse()?)
    } else {
        let url = format!(
            "/?{}",
            new_queries
                .iter()
                .map(|p| format!("{}={}", p.0, p.1))
                .fold(String::new(), |acc, q| format!("{}&{}", acc, q))
        )
        .replace("?&", "?");
        log::info!("{}", url);
        Ok(url.parse::<Uri>()?)
    }
}
