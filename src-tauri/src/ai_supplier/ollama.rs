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
 * @param options ClassifyOptions
 * @param provider_url Ollama server URL
 * @returns BookmarkFolder
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

    let res_result = client
        .post(url)
        .json(&body)
        .send()
        .await;

    let res = match res_result {
        Ok(response) => response.json::<serde_json::Value>().await.ok(),
        Err(_) => None,
    };

    let extracted_text = res.as_ref().and_then(|json| json["response"].as_str());

    let final_folder = match extracted_text {
        Some(raw_content) => {
            let cleaned_json = clean_json_content(raw_content.trim());
            parse_to_folder(&cleaned_json, options)
        }
        None => {
            println!("警告: Ollama 响应提取失败或网络异常。响应原文: {:?}", res);
            crate::ai_supplier::get_fallback_folder(
                options.fallback_id.unwrap_or(crate::ai_supplier::FALLBACK_ID),
                options.fallback_name.as_deref().unwrap_or(crate::ai_supplier::FALLBACK_NAME),
            )
        }
    };

    Ok(final_folder)
}