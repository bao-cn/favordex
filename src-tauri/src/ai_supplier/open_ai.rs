use serde::Deserialize;
use serde_json::json;
use crate::models::{BookmarkFolder, ClassifyOptions};
use reqwest::Client;
use crate::ai_supplier::{clean_json_content, parse_to_folder};

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub data: Option<Vec<OpenAIModel>>,
}

#[derive(Deserialize)]
pub struct OpenAIModel {
    pub id: Option<String>,
}


/**
 * Call OpenAI API
 *
 * @param client Client instance
 * @param prompt Prompt to send
 * @param api_key API key for OpenAI
 * @returns Category string
 */
pub async fn call_openai(
    client: &Client,
    prompt: &str,
    options: &ClassifyOptions,
) -> Result<BookmarkFolder, String> {
    let key = options.api_key.as_deref().ok_or("OpenAI API Key 未设置")?;
    let url = "https://api.openai.com/v1/chat/completions";

    let body = json!({
        "model": options.model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.3
    });

    let res = client
        .post(url)
        .bearer_auth(key)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("解析 OpenAI 响应格式失败: {}", e))?;

    let raw_content = res["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("AI 返回内容为空")?
        .trim();

    let cleaned_json = clean_json_content(raw_content);

    Ok(parse_to_folder(&cleaned_json, options))
}
