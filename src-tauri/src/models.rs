use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookmarkFolder {
    pub id: String,
    pub name: String,
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
    pub taxonomy: Vec<CategoryGroup>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClassifyOptions {
    pub auto_sort_local: bool,
    pub auto_sort_dead: bool,
    pub provider: String,
    pub api_key: Option<String>,
}
