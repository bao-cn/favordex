use serde::Deserialize;
use serde_json::json;
use crate::models::{BookmarkFolder, ClassifyOptions};
use reqwest::Client;
use crate::ai_supplier::{clean_json_content, parse_to_folder};

#[derive(Debug, Deserialize)]
pub struct GeminiModelList {
    pub models: Vec<GeminiModel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiModel {
    pub name: String,           
    #[serde(rename = "supportedGenerationMethods")]
    pub methods: Vec<String>,   
}

/**
 * Call Google API
 *
 * @param client Client instance
 * @param prompt Prompt to send
 * @param options ClassifyOptions
 * @returns BookmarkFolder
 */
pub async fn call_google(
    client: &Client,
    prompt: &str,
    options: &ClassifyOptions,
) -> Result<BookmarkFolder, String> {
    let key = options.api_key.as_deref().ok_or("Google API Key 未设置")?;
    
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        options.model, key
    );

    let body = json!({
        "contents": [{ 
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 1,
            "maxOutputTokens": 2048,
            "topP": 0.8,
            "responseMimeType": "text/plain" 
        }
    });

    let res_result = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    let res = match res_result {
        Ok(response) => response.json::<serde_json::Value>().await.ok(),
        Err(_) => None,
    };

    let extracted_text = res.as_ref().and_then(|json| {
        json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.trim())
    });

    let final_folder = match extracted_text {
        Some(raw_content) => {
            let cleaned_json = clean_json_content(raw_content);
            parse_to_folder(&cleaned_json, options)
        }
        None => {
            if res.is_none() {
                println!("错误: Gemini 请求或解析失败");
            } else {
                println!("警告: Gemini 结构解析失败或内容被拦截。响应原文: {:?}", res);
            }
            
            crate::ai_supplier::get_fallback_folder(
                options.fallback_id.unwrap_or(crate::ai_supplier::FALLBACK_ID),
                options.fallback_name.as_deref().unwrap_or(crate::ai_supplier::FALLBACK_NAME),
            )
        }
    };

    Ok(final_folder)
}