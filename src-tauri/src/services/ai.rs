use crate::ai_supplier::{google::call_google, ollama::call_ollama, open_ai::call_openai};
use crate::models::{BookmarkFolder, ClassificationTask, ClassifyOptions};
use reqwest::Client;

/**
 * Get category for a classification task
 *
 * @param task Classification task
 * @param options ClassifyOptions
 * @returns BookmarkFolder
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
    let taxonomy_text = task.taxonomy.clone();

    let mut context = format!("Title: {}\nURL: {}\n", task.title, task.url);
    if let Some(desc) = &task.description {
        context.push_str(&format!("Description: {}\n", desc));
    }
    if let Some(keys) = &task.keywords {
        context.push_str(&format!("Keywords: {}\n", keys));
    }

format!(
    "You are a professional, precise bookmark classification assistant. 
    Analyze the provided context and map it to the most relevant category. 
    Follow ALL rules STRICTLY — NO exceptions, NO extra content, NO deviations.

    ### CLASSIFICATION SYSTEM (MANDATORY, ONLY USE THESE CATEGORIES)
    {}

    ### WEBPAGE CONTENT TO CLASSIFY
    {}

    ### MANDATORY RULES (HIGHEST PRIORITY)
    1. **Data Priority**: Extract core information in this order: Description > Keywords > Title > URL.
    2. **Deep Inference (Step-by-Step)**:
       - First, extract core utility from 'description' and 'keywords'.
       - Second, if the title is generic, use the URL path to infer the site's niche.
       - Third, map findings to the most specific LEAF category.
    3. **Category Selection**:
       - Select the MOST SPECIFIC (leaf) category in the system.
       - If no matching subcategory, use the closest parent category.
       - NEVER create, modify, or invent categories — ONLY use the provided system.
    4. **Proactive Matching (Anti-999)**:
       - Do NOT use 'Others' (999) unless the content is completely unrecognizable.
       - If a site has >50% semantic overlap with a category, select that category instead of 999.
    5. **Strict ID Exclusion**: NEVER use ID 997 or 998 (reserved for system-level logic).

    ### AMBIGUOUS CASES (MANDATORY)
    If the input is essentially empty, total gibberish, or you CANNOT confidently determine the category, return this EXACT JSON:
    {{\"id\":999,\"name\":\"Others\",\"children\":[]}}

    ### OUTPUT REQUIREMENTS (STRICTLY ENFORCED)
    - RETURN ONLY A SINGLE VALID JSON OBJECT — NO Markdown blocks, NO preamble, NO explanations, NO extra text.
    - JSON STRUCTURE (FIXED):
      - \"id\": number (matches the system category ID)
      - \"name\": string (EXACT match with system category name)
      - \"children\": empty array []
    - OUTPUT EXAMPLE:
      {{\"id\":3001,\"name\":\"Food Recipes\",\"children\":[]}}

    ### FINAL VERIFICATION (BEFORE OUTPUT)
    1. Is the ID a valid leaf node (or closest parent) from the system (excluding 997, 998)?
    2. Is the name an EXACT string match with the system category?
    3. Is the output a single, valid JSON object with no extra characters?
    4. Did you avoid 997, 998, and only use 999 when absolutely necessary?",
    serde_json::to_string(&taxonomy_text).unwrap(),
    context
)
}
