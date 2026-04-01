use serde::Deserialize;
use serde_json::json;
use crate::models::{BookmarkFolder, ClassifyOptions};
use reqwest::Client;
use crate::ai_supplier::{clean_json_content, parse_to_folder};

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub models: Option<Vec<OllamaModel>>,
}

#[derive(Deserialize)]
pub struct OllamaModel {
    pub model: Option<String>,
}


/**
 * Call Ollama API
 *
 * @param client Client instance
 * @param prompt Prompt to send
 * @returns Category string
 */
pub async fn call_ollama(
    client: &Client,
    prompt: &str,
    options: &ClassifyOptions,
    provider_url: &str,
) -> Result<BookmarkFolder, String> {
    let url = format!("{}/api/generate", provider_url.trim_end_matches('/'));
    let body = json!({
        "model": options.model,
        "prompt": prompt,
        "stream": false
    });
    // println!("Body: {:?}", body);

    let res = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("解析 Ollama 响应格式失败: {}", e))?;
    // println!("Response: {:?}", res);
    let raw_content = res["response"].as_str().ok_or("AI 返回内容为空")?.trim();

    let cleaned_json = clean_json_content(raw_content);
    // println!("Cleaned JSON: {}", cleaned_json);

    Ok(parse_to_folder(&cleaned_json, options))
}
