use crate::models::ClassificationTask;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;

#[derive(Debug)]
pub enum AiProvider {
    Ollama,
    Openai,
}

impl FromStr for AiProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(AiProvider::Openai),
            "ollama" => Ok(AiProvider::Ollama),
            _ => Err(format!("不支持的 provider: {}", s)),
        }
    }
}

#[derive(Deserialize)]
struct OllamaResponse {
    models: Option<Vec<OllamaModel>>,
}

#[derive(Deserialize)]
struct OllamaModel {
    model: Option<String>,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    data: Option<Vec<OpenAIModel>>,
}

#[derive(Deserialize)]
struct OpenAIModel {
    id: Option<String>,
}

/**
 * Get category for a classification task
 *
 * @param task Classification task
 * @param provider AI provider
 * @param api_key API key for OpenAI
 * @returns Category string
 */
pub async fn get_category(
    task: ClassificationTask,
    provider: &str,
    api_key: Option<String>,
) -> Result<String, String> {
    let client = Client::new();
    let prompt = build_classification_prompt(&task);

    match provider {
        "ollama" => call_ollama(&client, &prompt).await,
        "openai" => call_openai(&client, &prompt, api_key).await,
        _ => Err("未知的 AI 供应商".to_string()),
    }
}

/**
 * Build classification prompt
 *
 * @param task Classification task
 * @returns Classification prompt
 */
pub fn build_classification_prompt(task: &ClassificationTask) -> String {
    let mut taxonomy_text = String::new();
    for group in &task.taxonomy {
        taxonomy_text.push_str(&format!(
            "- {}: {}\n",
            group.primary,
            group.secondaries.join(", ")
        ));
    }

    let mut context = format!("标题: {}\n链接: {}\n", task.title, task.url);
    if let Some(desc) = &task.description {
        context.push_str(&format!("描述: {}\n", desc));
    }
    if let Some(keys) = &task.keywords {
        context.push_str(&format!("关键词: {}\n", keys));
    }

    // 2. 组合最终 Prompt
    format!(
        "你是一个专业的书签整理助手。请根据提供的分类体系和网页元数据，为网页选择最合适的分类。\n\n\
        ### 分类体系（一级：二级）:\n\
        {}\n\n\
        ### 目标网页信息:\n\
        {}\n\n\
        ### 要求:\n\
        1. 必须从提供的体系中选择，严禁自创分类。\n\
        2. 返回格式严格为：一级分类 > 二级分类\n\
        3. 优先参考描述和关键词来判断网页真实用途。\n\
        4. 只返回结果，不要解释说明。",
        taxonomy_text, context
    )
}

/**
 * Call Ollama API
 *
 * @param client Client instance
 * @param prompt Prompt to send
 * @returns Category string
 */
async fn call_ollama(client: &Client, prompt: &str) -> Result<String, String> {
    let url = "http://127.0.0.1:11434/api/generate";
    let body = json!({
        "model": "qwen2.5:7b",
        "prompt": prompt,
        "stream": false
    });

    let res = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res["response"]
        .as_str()
        .unwrap_or("未分类")
        .trim()
        .to_string())
}

/**
 * Call OpenAI API
 *
 * @param client Client instance
 * @param prompt Prompt to send
 * @param api_key API key for OpenAI
 * @returns Category string
 */
async fn call_openai(
    client: &Client,
    prompt: &str,
    api_key: Option<String>,
) -> Result<String, String> {
    let key = api_key.ok_or("OpenAI API Key 未设置")?;
    let url = "https://api.openai.com/v1/chat/completions";

    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.3
    });

    let res = client
        .post(url)
        .bearer_auth(key)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    let content = res["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("未分类");

    Ok(content.trim().to_string())
}

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
            let url = format!("{}/tags", api_url.trim_end_matches("/"));

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

        _ => Err("未知的 AI 供应商".to_string()),
    }
}
