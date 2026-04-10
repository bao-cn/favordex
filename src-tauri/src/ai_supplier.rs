pub mod google;
pub mod ollama;
pub mod open_ai;
pub mod custom;

use crate::models::{BookmarkFolder, ClassifyOptions};
use google::GeminiModelList;
use ollama::OllamaResponse;
use open_ai::OpenAIResponse;
use reqwest::Client;
use std::str::FromStr;

#[derive(Debug)]
pub enum AiProvider {
    Ollama,
    Openai,
    Google,
    Custom,
}

impl FromStr for AiProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(AiProvider::Openai),
            "ollama" => Ok(AiProvider::Ollama),
            "google" => Ok(AiProvider::Google),
            "custom" => Ok(AiProvider::Custom),
            _ => Err(format!("不支持的 provider: {}", s)),
        }
    }
}

pub const FALLBACK_ID: usize = 999;
pub const FALLBACK_NAME: &str = "Others";

/**
 * Get models from AI provider
 *
 * @param client Client instance
 * @param provider AI provider
 * @param api_url API URL
 * @param api_key API key for OpenAI
 * @returns List of models
 */
pub async fn get_models(
    client: &Client,
    provider: &AiProvider,
    api_url: &str,
    api_key: Option<String>,
) -> Result<Vec<String>, String> {
    match provider {
        AiProvider::Ollama => {
            let url = format!("{}/api/tags", api_url.trim_end_matches("/"));

            let res: OllamaResponse = client
                .get(url)
                .send()
                .await
                .map_err(|e| format!("请求 Ollama 失败: {}", e))?
                .json()
                .await
                .map_err(|e| format!("解析 Ollama 响应失败: {}", e))?;

            let models = res
                .models
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| m.model)
                .collect();

            Ok(models)
        }

        AiProvider::Openai => {
            let key = api_key.ok_or("OpenAI API Key 未设置")?;
            let url = "https://api.openai.com/v1/models";

            let res: OpenAIResponse = client
                .get(url)
                .bearer_auth(key)
                .send()
                .await
                .map_err(|e| format!("请求 OpenAI 失败: {}", e))?
                .json()
                .await
                .map_err(|e| format!("解析 OpenAI 响应失败: {}", e))?;

            let models = res
                .data
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| m.id)
                .collect();

            Ok(models)
        }
        AiProvider::Google => {
            let key = api_key.ok_or("Google API Key 未设置")?;
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models?key={}",
                key
            );

            let res: GeminiModelList = client
                .get(url)
                .send()
                .await
                .map_err(|e| format!("请求 Google Gemini 失败: {}", e))?
                .json()
                .await
                .map_err(|e| {
                    format!(
                        "解析 Google Gemini 响应失败（可能是 Key 无效或格式变动）: {}",
                        e
                    )
                })?;

            let model_names: Vec<String> = res
                .models
                .into_iter()
                .filter(|m| m.methods.iter().any(|s| s == "generateContent"))
                .map(|m| {
                    m.name
                        .strip_prefix("models/")
                        .unwrap_or(&m.name)
                        .to_string()
                })
                .filter(|name| !name.contains("vision") || name.contains("1.5"))
                .collect();

            if model_names.is_empty() {
                return Err("未发现可用的 Gemini 生成模型".to_string());
            }

            Ok(model_names)
        }
        AiProvider::Custom => {
            // Custom provider logic, Compatible with OpenAI's API format
            let key = api_key.ok_or("Custom API Key 未设置")?;

            let url = format!("{}/models", api_url.trim_end_matches("/"));

            let res: OpenAIResponse = client
                .get(&url)
                .bearer_auth(key)
                .send()
                .await
                .map_err(|e| format!("请求 Custom Provider 失败: {}", e))?
                .json()
                .await
                .map_err(|e| format!("解析 Custom Provider 响应失败: {}", e))?;

            let models = res
                .data
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| m.id)
                .collect::<Vec<String>>();

            if models.is_empty() {
                return Err("未获取到任何模型（请确认接口是否兼容 OpenAI /v1/models）".to_string());
            }

            Ok(models)
        }
    }
}

/**
 * Get fallback bookmark folder
 *
 * @param fallback_id Fallback ID
 * @param fallback_name Fallback name
 * @returns BookmarkFolder
 */
fn get_fallback_folder(fallback_id: usize, fallback_name: &str) -> BookmarkFolder {
    BookmarkFolder {
        id: fallback_id,
        name: fallback_name.to_string(),
        guid: None,
        url: None,
        type_: None,
        date_added: None,
        date_last_used: None,
        date_modified: None,
        children: vec![],
    }
}

/**
 * Parse JSON string to BookmarkFolder
 *
 * @param json_str JSON string
 * @param options ClassifyOptions
 * @returns BookmarkFolder
 */
fn parse_to_folder(json_str: &str, options: &ClassifyOptions) -> BookmarkFolder {
    match serde_json::from_str::<BookmarkFolder>(json_str) {
        Ok(parsed) if parsed.id > 0 => {
            println!("成功解析: {:?}", parsed.name);
            parsed
        }
        Err(e) => {
            println!("Serde 解析失败: {}", e);
            println!("失败的字符串原文: {:?}", json_str);

            get_fallback_folder(
                options.fallback_id.unwrap_or(FALLBACK_ID),
                options.fallback_name.as_deref().unwrap_or(FALLBACK_NAME),
            )
        }
        _ => get_fallback_folder(FALLBACK_ID, FALLBACK_NAME),
    }
}
/**
 * Clean and extract JSON content from raw AI response
 *
 * @param raw Raw string from AI response
 * @returns Cleaned JSON string
 */
fn clean_json_content(raw: &str) -> String {
    let mut s = raw.trim();

    s = s.strip_prefix('\u{FEFF}').unwrap_or(s);

    if let Some(content) = s
        .strip_prefix("```json")
        .or_else(|| s.strip_prefix("```JSON"))
    {
        s = content;
    } else if let Some(content) = s.strip_prefix("```") {
        s = content;
    }
    s = s.strip_suffix("```").unwrap_or(s).trim();

    let s = s.replace('\u{a0}', " ");

    let mut s = if s.starts_with('"') {
        match serde_json::from_str::<String>(&s) {
            Ok(unescaped) => unescaped,
            Err(_) => s,
        }
    } else {
        s
    };

    if let Some(start) = s.find('{') {
        let mut brace_count = 0;
        let mut end_index = None;

        for (i, ch) in s.char_indices().skip(start) {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        end_index = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }

        if let Some(end) = end_index {
            s = s[start..=end].to_string();
        }
    }

    s
}
