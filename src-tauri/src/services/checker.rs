use crate::services::is_local_address;
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;

pub struct PageMetadata {
    pub is_alive: bool,
    pub description: Option<String>,
    pub keywords: Option<String>,
}

/**
 * Get page metadata
 *
 * @param url URL to check
 * @param skip_local Whether to skip local addresses
 * @returns Page metadata
 */
pub async fn get_page_metadata(url: &str, skip_local: bool) -> PageMetadata {
    if skip_local && is_local_address(url) {
        return PageMetadata {
            is_alive: true,
            description: None,
            keywords: None,
        };
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36")
        .build().unwrap();

    let resp = match client.get(url).send().await {
        Ok(r) if r.status().is_success() => r,
        _ => {
            return PageMetadata {
                is_alive: false,
                description: None,
                keywords: None,
            }
        }
    };

    let html_content = resp.text().await.unwrap_or_default();
    let document = Html::parse_document(&html_content);

    let desc_selector = Selector::parse("meta[name='description']").unwrap();
    let key_selector = Selector::parse("meta[name='keywords']").unwrap();

    let description = document
        .select(&desc_selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(|s| s.to_string());

    let keywords = document
        .select(&key_selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(|s| s.to_string());

    PageMetadata {
        is_alive: true,
        description,
        keywords,
    }
}

/**
 * Check URL availability(Deprecated)
 *
 * @param url URL to check
 * @param skip_local Whether to skip local addresses
 * @returns True if URL is alive, False otherwise
 */
pub async fn check_url(url: &str, skip_local: bool) -> Result<bool, String> {
    if skip_local && url.starts_with(url) {
        return Ok(true);
    };

    let client = match Client::builder()
        .timeout(Duration::from_secs(8))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36")
        .build() {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };

    match client.head(url).send().await {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => match client.get(url).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Err("Failed to send GET request".to_string()),
        },
    }
}
