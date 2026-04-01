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
    pub version: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub description: Option<String>,
    #[serde(rename = "supportedGenerationMethods")]
    pub methods: Vec<String>,   
}


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

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("解析 Gemini 响应格式失败: {}", e))?;

    let raw_content = res["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| {
            println!("Gemini Error Response: {:?}", res);
            "AI 返回内容为空或被安全拦截"
        })?
        .trim();

    let cleaned_json = clean_json_content(raw_content);
        println!("Cleaned JSON: {:?}", cleaned_json);
    Ok(parse_to_folder(&cleaned_json, options))
}