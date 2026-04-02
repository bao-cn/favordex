use crate::models::{Bookmark, BookmarkFolder};
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::Local;
use uuid::Uuid;

pub mod chrome;
pub mod edge;

/**
 * Get bookmark folders from browser file
 *
 * @param browser Browser name
 * @returns BookmarkFolder list
 */
pub fn get_bookmark_folders(browser: &str) -> Result<Vec<BookmarkFolder>, String> {
    let path = get_bookmarks_path(browser)?;
    let content = fs::read_to_string(path).map_err(|e| format!("读取书签文件失败: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let mut roots = Vec::new();

    if let Some(root_node_1) = get_root_node_id_1(&json) {
        if let Some(folder) = extract_folder_tree(root_node_1) {
            roots.push(folder);
        }
    }

    Ok(roots)
}

/**
 * Get root node with ID 1 from JSON object
 *
 * @param json JSON object
 * @returns Root node with ID 1
 */
fn get_root_node_id_1(json: &serde_json::Value) -> Option<&serde_json::Value> {
    if let Some(roots_obj) = json["roots"].as_object() {
        for (_, root_node) in roots_obj {
            if get_id_as_usize(root_node) == Some(1) {
                return Some(root_node);
            }
        }
    }
    None
}
/**
 * Collect bookmarks from JSON node
 *
 * @param node JSON node
 * @param bookmarks Bookmark list
 */
fn collect_bookmarks(node: &serde_json::Value, bookmarks: &mut Vec<Bookmark>) {
    if node["type"].as_str() == Some("url") {
        if let (Some(name), Some(url)) = (node["name"].as_str(), node["url"].as_str()) {
            bookmarks.push(Bookmark {
                name: name.to_string(),
                url: url.to_string(),
                web_title: None,
                description: None,
                keywords: None,
                status: "NoDetected".to_string(),
            });
        }
    } else if let Some(children) = node["children"].as_array() {
        for child in children {
            collect_bookmarks(child, bookmarks);
        }
    }
}

/**
 * Count bookmarks in JSON node
 *
 * @param node JSON node
 * @returns Bookmark count
 */
fn count_bookmarks(node: &serde_json::Value) -> usize {
    let mut count = 0;
    if node["type"].as_str() == Some("url") {
        count += 1;
    } else if let Some(children) = node["children"].as_array() {
        for child in children {
            count += count_bookmarks(child);
        }
    }
    count
}

/**
 * Get all bookmarks from browser file
 *
 * @param browser Browser name
 * @returns Bookmark list
 */
pub fn get_all_bookmarks(browser: &str) -> Result<Vec<Bookmark>, String> {
    let path = get_bookmarks_path(browser)?;
    let content = fs::read_to_string(path).map_err(|e| format!("读取书签文件失败: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let mut bookmarks = Vec::new();

    if let Some(root_node_1) = get_root_node_id_1(&json) {
        collect_bookmarks(root_node_1, &mut bookmarks);
    }

    Ok(bookmarks)
}

/**
 * Extract folder tree from JSON node
 *
 * @param node JSON node
 * @returns BookmarkFolder
 */
fn extract_folder_tree(node: &serde_json::Value) -> Option<BookmarkFolder> {
    let is_folder = node["type"].as_str().map(|s| s == "folder").unwrap_or(false);

    if is_folder {
        let id = node["id"].as_str()?.parse::<usize>().ok()?; 
        let name = node["name"].as_str()?.to_string();
        let guid = node["guid"].as_str()?.to_string();
        
        let mut children_folders = Vec::new();

        if let Some(children) = node["children"].as_array() {
            for child in children {
                if let Some(folder) = extract_folder_tree(child) {
                    children_folders.push(folder);
                }
            }
        }

        Some(BookmarkFolder {
            id,
            name,
            guid: Some(guid),
            url: None,
            type_: None,
            date_added: None,
            date_last_used: None,
            date_modified: None,
            children: children_folders,
        })
    } else {
        None
    }
}

/**
 * Get bookmarks by folder ID
 *
 * @param browser Browser name
 * @param folder_id Folder ID
 * @returns Bookmark list
 */
pub fn get_bookmarks_by_folder(browser: &str, folder_id: usize) -> Result<Vec<Bookmark>, String> {
    let path = get_bookmarks_path(browser)?;
    let content = fs::read_to_string(path).map_err(|e| format!("读取书签文件失败: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let mut bookmarks = Vec::new();

    if let Some(roots_obj) = json["roots"].as_object() {
        for (_, root_node) in roots_obj {
            if let Some(target_folder) = find_folder_node(root_node, folder_id) {
                if let Some(children) = target_folder["children"].as_array() {
                    for child in children {
                        if child["type"].as_str() == Some("url") {
                            if let (Some(name), Some(url)) =
                                (child["name"].as_str(), child["url"].as_str())
                            {
                                bookmarks.push(Bookmark {
                                    name: name.to_string(),
                                    url: url.to_string(),
                                    web_title: None,
                                    description: None,
                                    keywords: None,
                                    status: "NoDetected".to_string(),
                                });
                            }
                        }
                    }
                }
                return Ok(bookmarks);
            }
        }
    }

    Ok(bookmarks)
}

/**
 * Find folder node in JSON tree
 *
 * @param node JSON node
 * @param target_id Target folder ID
 * @returns Option<&'a serde_json::Value>
 */
fn find_folder_node<'a>(
    node: &'a serde_json::Value,
    target_id: usize,
) -> Option<&'a serde_json::Value> {
    if get_id_as_usize(node) == Some(target_id) {
        return Some(node);
    }

    if let Some(children) = node["children"].as_array() {
        for child in children {
            if let Some(found) = find_folder_node(child, target_id) {
                return Some(found);
            }
        }
    }
    None
}

/**
 * Get ID as usize from JSON node
 *
 * @param node JSON node
 * @returns Option<usize>
 */
fn get_id_as_usize(node: &serde_json::Value) -> Option<usize> {
    node["id"].as_u64()
        .map(|n| n as usize)
        .or_else(|| node["id"].as_str()?.parse().ok())
}

/**
 * Overlay bookmarks file with new content
 *
 * @param browser Browser name
 * @param new_content New content to overlay
 * @returns Result
 */
pub fn overlay_bookmarks_file(browser: &str, new_content: Vec<BookmarkFolder>) -> Result<bool, String> {
    let path = get_bookmarks_path(browser)?;
    let content = fs::read_to_string(&path).map_err(|e| format!("读取书签文件失败: {}", e))?;
    let mut json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let now_micros = Local::now().timestamp_micros() as u64;

    let target_folder = new_content.iter().find(|f| f.id == 1)
        .ok_or("未能在传入数据中找到 ID 为 1 的根文件夹")?;

    let processed_json = folder_to_json(target_folder, now_micros);

    let mut found = false;
    if let Some(roots_obj) = json["roots"].as_object_mut() {
        for (_, root_node) in roots_obj {
            if get_id_as_usize(root_node) == Some(1) {
                *root_node = processed_json.clone();
                found = true;
                break;
            }
        }
    }

    if !found {
        return Err("在本地书签文件中未找到 ID 为 1 的书签栏".into());
    }

    let new_json_str = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("序列化 JSON 失败: {}", e))?;
    
    fs::write(path, new_json_str).map_err(|e| format!("写入书签文件失败: {}", e))?;

    Ok(true)
}

/**
 * Convert BookmarkFolder to JSON value with timestamp
 *
 * @param folder BookmarkFolder to convert
 * @param timestamp Timestamp to use for date_added and date_last_used
 * @returns serde_json::Value
 */
fn folder_to_json(folder: &BookmarkFolder, timestamp: u64) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    
    map.insert("id".to_string(), serde_json::Value::String(folder.id.to_string()));
    map.insert("name".to_string(), serde_json::Value::String(folder.name.clone()));
    
    let guid = folder.guid.clone().unwrap_or_else(|| Uuid::new_v4().to_string().to_uppercase());
    map.insert("guid".to_string(), serde_json::Value::String(guid));

    let date_added = folder.date_added.unwrap_or(timestamp);
    map.insert("date_added".to_string(), serde_json::Value::String(date_added.to_string()));
    
    let type_str = if folder.url.is_some() { "url" } else { "folder" };
    map.insert("type".to_string(), serde_json::Value::String(type_str.to_string()));

    if let Some(url) = &folder.url {
        map.insert("url".to_string(), serde_json::Value::String(url.clone()));
        let date_last_used = folder.date_last_used.unwrap_or(timestamp);
        map.insert("date_last_used".to_string(), serde_json::Value::String(date_last_used.to_string()));
    } else {
        let children_values: Vec<serde_json::Value> = folder.children
            .iter()
            .map(|child| folder_to_json(child, timestamp))
            .collect();
        map.insert("children".to_string(), serde_json::Value::Array(children_values));
        map.insert("date_modified".to_string(), serde_json::Value::String(timestamp.to_string()));
    }

    serde_json::Value::Object(map)
}

/**
 * Get bookmarks file path
 *
 * @param browser Browser name
 * @returns Bookmarks file path
 */
fn get_bookmarks_path(browser: &str) -> Result<String, String> {
    let path;
    #[cfg(target_os = "windows")]
    {
        let app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
        path = match browser {
            "chrome" => format!(
                "{}\\Google\\Chrome\\User Data\\Default\\Bookmarks",
                app_data
            ),
            "edge" => format!(
                "{}\\Microsoft\\Edge\\User Data\\Default\\Bookmarks",
                app_data
            ),
            _ => return Err("Unsupported browser".into()),
        };
        Ok(path)
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        path = match browser {
            "chrome" => format!("{}/.config/google-chrome/Default/Bookmarks", home),
            "edge" => format!("{}/.config/microsoft-edge/Default/Bookmarks", home),
            _ => return Err("Unsupported browser".into()),
        };
        Ok(path)
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        path = match browser {
            "chrome" => format!(
                "{}/Library/Application Support/Google/Chrome/Default/Bookmarks",
                home
            ),
            "edge" => format!(
                "{}/Library/Application Support/Microsoft Edge/Default/Bookmarks",
                home
            ),
            _ => return Err("Unsupported browser".into()),
        };
        Ok(path)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        return Err("Unsupported platform".into());
    }
}

/**
 * Get bookmarks number
 *
 * @param browser Browser name
 * @returns Bookmarks number
 */
pub fn get_bookmarks_num(browser: &str) -> Result<usize, String> {
    let path = get_bookmarks_path(browser)?;
    let content = fs::read_to_string(path).map_err(|e| format!("读取书签文件失败: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let mut num = 0;
    
    if let Some(root_node_1) = get_root_node_id_1(&json) {
        num = count_bookmarks(root_node_1);
    }

    Ok(num)
}



#[derive(Serialize, Deserialize, Debug)]
pub struct BrowserStatus {
    pub chrome: bool,
    pub edge: bool,
}

/**
 * Check browser installation
 *
 * @returns Browser status
 */
pub fn check_browsers() -> BrowserStatus {
    BrowserStatus {
        chrome: is_chrome_installed(),
        edge: is_edge_installed(),
    }
}

/**
 * Backup bookmarks to a file
 *
 * @param browser Browser name
 * @returns Backup file path
 */
pub fn backup_bookmarks(browser: &str) -> Result<String, String> {
    let path = get_bookmarks_path(browser)?;
    let backup_path = format!("{}.backup", path);
    fs::copy(&path, &backup_path).map_err(|e| format!("Failed to backup bookmarks: {}", e))?;
    Ok(backup_path)
}

/**
 * Check if backup file exists
 *
 * @param browser Browser name
 * @returns True if backup file exists, False otherwise
 */
pub fn check_backup(browser: &str) -> bool {
    let backup_path = format!("{}.backup", get_bookmarks_path(browser).unwrap());
    fs::metadata(&backup_path).is_ok()
}

/**
 * Check if Chrome is installed
 *
 * @returns True if Chrome is installed, False otherwise
 */
fn is_chrome_installed() -> bool {
    #[cfg(target_os = "macos")]
    {
        std::path::Path::new("/Applications/Google Chrome.app").exists()
    }
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::RegKey;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        hklm.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe")
            .is_ok()
            || hklm
                .open_subkey(
                    r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe",
                )
                .is_ok()
    }
    #[cfg(target_os = "linux")]
    {
        std::path::Path::new("/usr/bin/google-chrome").exists()
            || std::path::Path::new("/usr/bin/google-chrome-stable").exists()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        false
    }
}

/**
 * Check if Edge is installed
 *
 * @returns True if Edge is installed, False otherwise
 */
fn is_edge_installed() -> bool {
    #[cfg(target_os = "macos")]
    {
        std::path::Path::new("/Applications/Microsoft Edge.app").exists()
    }
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::RegKey;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        hklm.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\msedge.exe")
            .is_ok()
            || hklm
                .open_subkey(
                    r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\App Paths\msedge.exe",
                )
                .is_ok()
    }
    #[cfg(target_os = "linux")]
    {
        std::path::Path::new("/usr/bin/microsoft-edge").exists()
            || std::path::Path::new("/usr/bin/microsoft-edge-stable").exists()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        false
    }
}