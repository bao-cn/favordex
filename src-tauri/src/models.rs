use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookmarkFolder {
    pub id: usize,
    pub name: String,
    #[serde(default)]
    pub guid: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
    pub children: Vec<BookmarkFolder>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub name: String,
    pub url: String,
    pub web_title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryGroup {
    pub primary: String,
    pub secondaries: Vec<String>,
}

// 最终传给 AI 的上下文
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClassificationTask {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub taxonomy: Vec<BookmarkFolder>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClassifyOptions {
    pub auto_sort_local: bool,
    pub auto_sort_dead: bool,
    pub provider: String,
    pub fallback_id: Option<usize>,
    pub fallback_name: Option<String>,
    pub provider_url: Option<String>,
    pub api_key: Option<String>,
    pub model: String,
    pub auto_delete: bool,
    pub system_prompt: Option<String>,
    pub system_proxy: bool,
    pub task_timeout_secs: Option<u64>,
    pub max_tasks: Option<u32>,
}


