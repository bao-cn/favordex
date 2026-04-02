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
 * @param options ClassifyOptions
 * @returns BookmarkFolder
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

    let res_result = client
        .post(url)
        .bearer_auth(key)
        .json(&body)
        .send()
        .await;

    let res = match res_result {
        Ok(response) => response.json::<serde_json::Value>().await.ok(),
        Err(_) => None,
    };

    let extracted_text = res.as_ref().and_then(|json| {
        json["choices"][0]["message"]["content"].as_str()
    });

    let final_folder = match extracted_text {
        Some(raw_content) => {
            let cleaned_json = clean_json_content(raw_content.trim());
            parse_to_folder(&cleaned_json, options)
        }
        None => {
            println!("警告: OpenAI 响应提取失败或网络异常。响应原文: {:?}", res);
            crate::ai_supplier::get_fallback_folder(
                options.fallback_id.unwrap_or(crate::ai_supplier::FALLBACK_ID),
                options.fallback_name.as_deref().unwrap_or(crate::ai_supplier::FALLBACK_NAME),
            )
        }
    };

    Ok(final_folder)
}