use crate::ai_supplier;
use crate::models::{Bookmark, BookmarkFolder, ClassificationTask, ClassifyOptions};
use crate::services::{ai, browser, checker};
use futures::StreamExt;
use serde::Serialize;
use std::collections::HashMap;
use tauri::Emitter;

use crate::AppConfig;

#[tauri::command]
pub fn get_bookmark_folders(browser: String) -> Result<Vec<BookmarkFolder>, String> {
    browser::get_bookmark_folders(&browser)
}

#[tauri::command]
pub fn get_bookmarks_by_folder(browser: String, folder_id: usize) -> Result<Vec<Bookmark>, String> {
    browser::get_bookmarks_by_folder(&browser, folder_id)
}

#[tauri::command]
pub fn get_all_bookmarks(browser: String) -> Result<Vec<Bookmark>, String> {
    browser::get_all_bookmarks(&browser)
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
    let provider: ai_supplier::AiProvider = provider
        .parse()
        .map_err(|e| format!("无效的 provider: {}", e))?;
    let client = reqwest::Client::new();
    let models = ai_supplier::get_models(&client, &provider, &api_url, api_key).await?;
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
pub fn check_backup(browser: String) -> Result<bool, String> {
    Ok(browser::check_backup(&browser))
}

#[derive(Clone, Serialize)]
struct ProgressPayload {
    current: usize,
    total: usize,
    message: String,
}

#[tauri::command]
pub async fn organize_bookmarks(
    app: tauri::AppHandle,
    browser: String,
    task: ClassificationTask,
    options: ClassifyOptions,
    config: tauri::State<'_, AppConfig>,
) -> Result<Vec<BookmarkFolder>, String> {
    // 1. 初始化数据
    let bookmarks = browser::get_all_bookmarks(&browser).map_err(|e| e.to_string())?;
    let total = bookmarks.len();
    if total == 0 {
        return Err("书签列表为空".to_string());
    }

    let max_concurrent = options.max_tasks.unwrap_or(3) as usize;
    let mut folder_buffer: HashMap<usize, Vec<BookmarkFolder>> = HashMap::new();

    let mut stream = futures::stream::iter(bookmarks.into_iter().enumerate())
        .map(|(index, bookmark)| {
            let task_inner = ClassificationTask {
                title: bookmark.name.clone(),
                url: bookmark.url.clone(),
                description: None,
                keywords: None,
                taxonomy: task.taxonomy.clone(),
            };
            let opt_inner = options.clone();
            let cfg_inner = config.inner().clone();
            let app_inner = app.clone();

            async move {
                let result = smart_classify_v3(task_inner, opt_inner, &cfg_inner).await;
                (bookmark, result)
            }
        })
        .buffer_unordered(max_concurrent);

    let mut completed = 0;
    // 4. 收集结果
    while let Some((bookmark, result)) = stream.next().await {
        completed += 1;
        println!("emit 进度: {}/{} - {}", completed, total, bookmark.name);
        let _ = app.emit_to(
            "main",
            "organize-progress",
            ProgressPayload {
                current: completed,
                total,
                message: format!("已完成: {}", bookmark.name),
            },
        );

        if let Ok(target_folder) = result {
            let bookmark_node = BookmarkFolder {
                id: 0,
                name: bookmark.name,
                guid: Some(uuid::Uuid::new_v4().to_string()),
                url: Some(bookmark.url.clone()),
                type_: Some("url".to_string()),
                children: Vec::new(),
            };

            folder_buffer
                .entry(target_folder.id)
                .or_insert_with(Vec::new)
                .push(bookmark_node);
        }

    }

    // 5. 递归合并
    let mut tree = task.taxonomy.clone();
    merge_buffer_into_forest(&mut tree, &mut folder_buffer);
    Ok(tree)
}

pub async fn smart_classify_v3(
    task: ClassificationTask,
    options: ClassifyOptions,
    config: &AppConfig,
) -> Result<BookmarkFolder, String> {
    let _permit = config.permit.acquire().await.map_err(|e| e.to_string())?;

    if options.auto_sort_local && crate::services::is_local_address(&task.url) {
        return Ok(BookmarkFolder {
            id: 997,
            name: "本地链接".to_string(),
            guid: None,
            url: None,
            type_: None,
            children: Vec::new(),
        });
    }

    let metadata = checker::get_page_metadata(&task.url, true).await;

    if !metadata.is_alive {
        if options.auto_delete {
            return Ok(BookmarkFolder {
                id: 998,
                name: "已标记删除".to_string(),
                guid: None,
                url: None,
                type_: None,
                children: Vec::new(),
            });
        }
        if options.auto_sort_dead {
            return Ok(BookmarkFolder {
                id: 999,
                name: "失效链接".to_string(),
                guid: None,
                url: None,
                type_: None,
                children: Vec::new(),
            });
        }
    }

    let mut final_task = task;
    final_task.description = metadata.description;
    final_task.keywords = metadata.keywords;

    ai::get_category(final_task, &options).await
}

fn merge_buffer_into_tree(
    node: &mut BookmarkFolder,
    buffer: &mut HashMap<usize, Vec<BookmarkFolder>>,
) {
    if let Some(mut items) = buffer.remove(&node.id) {
        node.children.append(&mut items);
    }
    for child in &mut node.children {
        merge_buffer_into_tree(child, buffer);
    }
}

fn merge_buffer_into_forest(
    nodes: &mut Vec<BookmarkFolder>,
    buffer: &mut HashMap<usize, Vec<BookmarkFolder>>,
) {
    for node in nodes {
        merge_buffer_into_tree(node, buffer);
    }
}
