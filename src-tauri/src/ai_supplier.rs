pub mod google;
pub mod ollama;
pub mod open_ai;

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
}

impl FromStr for AiProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(AiProvider::Openai),
            "ollama" => Ok(AiProvider::Ollama),
            "google" => Ok(AiProvider::Google),
            _ => Err(format!("不支持的 provider: {}", s)),
        }
    }
}

const FALLBACK_ID: usize = 115;
const FALLBACK_NAME: &str = "Others";

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
            // 增加一个过滤参数：只有支持 generateContent 的模型才是我们要的
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
                    // 这里建议打印出原始文本，因为 API Key 错误或欠费时，
                    // Google 会返回一个不同结构的 Error 对象，导致 json 解析失败
                    format!(
                        "解析 Google Gemini 响应失败（可能是 Key 无效或格式变动）: {}",
                        e
                    )
                })?;

            let model_names: Vec<String> = res
                .models
                .into_iter()
                // 过滤：必须支持生成内容
                .filter(|m| {
                    m.methods
                        .iter()
                        .any(|s| s == "generateContent")
                })
                // 转换：去掉 "models/" 前缀
                .map(|m| {
                    m.name
                        .strip_prefix("models/")
                        .unwrap_or(&m.name)
                        .to_string()
                })
                // 额外过滤：排除一些预览版或实验性模型（可选）
                .filter(|name| !name.contains("vision") || name.contains("1.5"))
                .collect();

            if model_names.is_empty() {
                return Err("未发现可用的 Gemini 生成模型".to_string());
            }

            Ok(model_names)
        }
        _ => Err("未知的 AI 供应商".to_string()),
    }
}

fn get_fallback_folder(fallback_id: usize, fallback_name: &str) -> BookmarkFolder {
    BookmarkFolder {
        id: fallback_id,
        name: fallback_name.to_string(),
        guid: Some(fallback_id.to_string().replace(" ", "").to_string()),
        url: None,
        type_: None,
        children: vec![],
    }
}

fn parse_to_folder(json_str: &str, options: &ClassifyOptions) -> BookmarkFolder {
    match serde_json::from_str::<BookmarkFolder>(json_str) {
        Ok(parsed) if parsed.id > 0 => {
            println!("成功解析: {:?}", parsed.name);
            parsed
        },
        Err(e) => {
            println!("Serde 解析失败: {}", e);
            println!("失败的字符串原文: {:?}", json_str);
            
            get_fallback_folder(
                options.fallback_id.unwrap_or(FALLBACK_ID),
                options.fallback_name.as_deref().unwrap_or(FALLBACK_NAME),
            )
        },
        _ => get_fallback_folder(FALLBACK_ID, FALLBACK_NAME),
    }
}

fn clean_json_content(raw: &str) -> String {
    let mut s = raw.trim();

    s = s.strip_prefix('\u{FEFF}').unwrap_or(s);

    if let Some(content) = s.strip_prefix("```json").or_else(|| s.strip_prefix("```JSON")) {
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