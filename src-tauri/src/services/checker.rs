use crate::services::is_local_address;
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;

#[derive(Debug)]
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
 * @param timeout_ms Timeout in milliseconds
 * @param sys_proxy Whether to use system proxy
 * @returns Page metadata
 */
pub async fn get_page_metadata(url: &str, skip_local: bool, timeout_ms: u64, sys_proxy: bool) -> PageMetadata {
    if skip_local && is_local_address(url) {
        return PageMetadata {
            is_alive: true,
            description: None,
            keywords: None,
        };
    }

    let timeout_duration = Duration::from_millis(timeout_ms);

    let mut client_builder = Client::builder()
        .timeout(timeout_duration)
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36");

    if !sys_proxy {
        client_builder = client_builder.no_proxy();
    } 
    let client = match client_builder.build() {
        Ok(c) => c,
        Err(_) => return PageMetadata {
            is_alive: false,
            description: None,
            keywords: None,
        },
    };

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