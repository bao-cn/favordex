use crate::ai_supplier::{google::call_google, ollama::call_ollama, open_ai::call_openai};
use crate::models::{BookmarkFolder, ClassificationTask, ClassifyOptions};
use reqwest::Client;

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
    options: &ClassifyOptions,
) -> Result<BookmarkFolder, String> {
    let client = Client::new();
    let prompt = options
        .system_prompt
        .clone()
        .unwrap_or_else(|| build_classification_prompt(&task));
    // println!("{:?}", prompt);

    match options.provider.as_str() {
        "ollama" => {
            let provider_url = options
                .provider_url
                .as_deref()
                .unwrap_or("http://localhost:11434");

            call_ollama(&client, &prompt, &options, provider_url).await
        }
        "openai" => call_openai(&client, &prompt, options).await,
        "google" => call_google(&client, &prompt, options).await,
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
    let mut taxonomy_text = task.taxonomy.clone();

    let mut context = format!("Title: {}\nURL: {}\n", task.title, task.url);
    if let Some(desc) = &task.description {
        context.push_str(&format!("Description: {}\n", desc));
    }
    if let Some(keys) = &task.keywords {
        context.push_str(&format!("Keywords: {}\n", keys));
    }

    format!(
        "You are a professional, precise bookmark classification assistant.
        Follow ALL rules STRICTLY — NO exceptions, NO extra content, NO deviations.
        ### CLASSIFICATION SYSTEM (MANDATORY, ONLY USE THESE CATEGORIES)
        {}
        ### WEBPAGE CONTENT TO CLASSIFY
        {}
        ### CLASSIFICATION RULES (HIGHEST PRIORITY)
        1. Core judgment priority: description/keywords > title > URL
        2. Select the MOST SPECIFIC (leaf) category in the system
        3. If no matching subcategory, use the closest parent category
        4. NEVER create, modify, or invent categories — ONLY use the provided system
        ### AMBIGUOUS CASES (MANDATORY)
        If you CANNOT confidently determine the category, RETURN THIS EXACT JSON:
        {{\"id\":115,\"name\":\"Others\",\"children\":[]}}
        ### OUTPUT REQUIREMENTS (STRICT, ENFORCE 100%)
        YOU MUST RETURN ONLY A SINGLE VALID JSON OBJECT.
        NO explanations, NO markdown, NO extra text, NO arrays, NO quotes wrapping JSON.
        JSON STRUCTURE (FIXED):
        - \"id\": number (matches the system category ID)
        - \"name\": string (EXACT match with system category name)
        - \"children\": empty array []
        ### OUTPUT EXAMPLE
        {{\"id\":3001,\"name\":\"Food Recipes\",\"children\":[]}}
        ### FINAL CHECK (BEFORE OUTPUT)
        1. Correct classification from the given system
        2. Valid, complete JSON with no extra characters
        3. Only the JSON object — nothing else",
        serde_json::to_string(&taxonomy_text).unwrap(),
        context
    )
}
