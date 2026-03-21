use crate::models::{Bookmark, BookmarkFolder};
use serde::{Deserialize, Serialize};
use std::fs;

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

    if let Some(roots_obj) = json["roots"].as_object() {
        for (_, root_node) in roots_obj {
            if let Some(folder) = extract_folder_tree(root_node) {
                roots.push(folder);
            }
        }
    }

    Ok(roots)
}

/**
 * Extract folder tree from JSON node
 *
 * @param node JSON node
 * @returns BookmarkFolder
 */
fn extract_folder_tree(node: &serde_json::Value) -> Option<BookmarkFolder> {
    let node_type = node["type"].as_str().unwrap_or("folder");

    if node_type == "folder" {
        let id = node["id"].as_str()?.to_string();
        let name = node["name"].as_str()?.to_string();
        let mut children_folders = Vec::new();

        if let Some(children) = node["children"].as_array() {
            for child in children {
                if child["type"].as_str() == Some("folder") {
                    if let Some(folder) = extract_folder_tree(child) {
                        children_folders.push(folder);
                    }
                }
            }
        }
        Some(BookmarkFolder {
            id,
            name,
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
pub fn get_bookmarks_by_folder(browser: &str, folder_id: &str) -> Result<Vec<Bookmark>, String> {
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
    target_id: &str,
) -> Option<&'a serde_json::Value> {
    if node["id"].as_str() == Some(target_id) {
        return Some(node);
    }

    if let Some(children) = node["children"].as_array() {
        for child in children {
            if child["type"].as_str() == Some("folder") || child["type"].as_str() == Some("url") {
                if let Some(found) = find_folder_node(child, target_id) {
                    return Some(found);
                }
            }
        }
    }
    None
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
    if let Some(roots_obj) = json["roots"].as_object() {
        for (_, root_node) in roots_obj {
            if let Some(children) = root_node["children"].as_array() {
                num += children.len();
            }
        }
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
