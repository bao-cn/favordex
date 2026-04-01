use std::fmt::format;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::models::{ClassificationTask, ClassifyOptions};
use crate::services::browser::edge::open_sync_edge_page;
use crate::services::browser::chrome::open_sync_chrome_page;
use crate::services::checker::get_page_metadata;
use crate::commands::smart_classify_v3;


#[tokio::test]
pub async fn test_get_page_metadata() {
    let url = "https://www.bilibili.com/";
    let metadata = get_page_metadata(url, false).await;
    assert!(metadata.is_alive);
    assert!(metadata.description.is_some());
    assert!(metadata.keywords.is_some());
    println!("{} {}", metadata.description.unwrap(), metadata.keywords.unwrap());
   }


#[test]
fn test_open_sync_page() {
    open_sync_edge_page();
}
#[test]
fn test_open_sync_chrome_page() {
    open_sync_chrome_page();
}