use crate::models::{Bookmark, BookmarkFolder, ClassificationTask};
use crate::services::{ai, browser, checker};
use crate::AppConfig;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tauri::command]
pub fn get_bookmark_folders(browser: String) -> Result<Vec<BookmarkFolder>, String> {
    browser::get_bookmark_folders(&browser)
}

#[tauri::command]
pub fn get_bookmarks_by_folder(
    browser: String,
    folder_id: String,
) -> Result<Vec<Bookmark>, String> {
    browser::get_bookmarks_by_folder(&browser, &folder_id)
}

#[tauri::command]
pub fn get_bookmarks_num(browser: String) -> Result<usize, String> {
    browser::get_bookmarks_num(&browser)
}

#[tauri::command]
pub async fn get_ai_models(
    provider: String,
    api_url: String,
    api_key: Option<String>,
) -> Result<Vec<String>, String> {
    let provider: ai::AiProvider = provider
        .parse()
        .map_err(|e| format!("无效的 provider: {}", e))?;
    let client = reqwest::Client::new();
    let models = ai::get_models(&client, &provider, &api_url, api_key).await?;
    Ok(models)
}

#[tauri::command]
pub fn check_browsers() -> Result<browser::BrowserStatus, ()> {
    Ok(browser::check_browsers())
}

#[tauri::command]
pub fn backup_bookmarks(browser: String) -> Result<String, String> {
    Ok(browser::backup_bookmarks(&browser)?)
}

#[tauri::command]
pub async fn check_link_status(url: String, skip_local: bool) -> Result<bool, String> {
    checker::check_url(&url, skip_local).await
}

#[tauri::command]
pub async fn bulk_ai_classify(
    tasks: Vec<ClassificationTask>,
    provider: String,
    api_key: Option<String>,
) -> Result<Vec<String>, String> {
    let semaphore = Arc::new(Semaphore::new(3)); // 限制最多 3 个并发 AI 请求，保护 CPU
    let mut handles = Vec::new();
    let provider = Arc::new(provider);
    let api_key = Arc::new(api_key);

    for task in tasks {
        let sem = semaphore.clone();
        let p = provider.clone();
        let k = api_key.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            ai::get_category(task, &p, (*k).clone()).await
        }));
    }

    let mut results = Vec::new();
    for h in handles {
        match h.await.unwrap() {
            Ok(res) => results.push(res),
            Err(e) => results.push(format!("错误: {}", e)),
        }
    }
    Ok(results)
}

#[tauri::command]
pub async fn smart_classify_v3(
    task: ClassificationTask,
    options: crate::models::ClassifyOptions,
    config: tauri::State<'_, AppConfig>,
) -> Result<String, String> {
    let _permit = config.permit.acquire().await.map_err(|e| e.to_string())?;

    if options.auto_sort_local && crate::services::is_local_address(&task.url) {
        return Ok("系统 / 管理 > 本地链接".to_string());
    }

    let metadata = checker::get_page_metadata(&task.url, true).await;

    if options.auto_sort_dead && !metadata.is_alive {
        return Ok("系统 / 管理 > 失效链接".to_string());
    }

    let mut final_task = task;
    final_task.description = metadata.description;
    final_task.keywords = metadata.keywords;

    ai::get_category(final_task, &options.provider, options.api_key).await
}
