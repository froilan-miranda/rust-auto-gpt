use crate::models::general::llm::{ Message };
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{ HeaderMap, HeaderValue };

//Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(message: Vec<Message>) {
    dotenv().ok();

    // Extract API Ket Information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in env vars");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in env vars");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/completions";

    // Create Headers
    let mut headers = HeaderMap::new();
    
    // Create api key header
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());

    // Create Open AI org header
    headers.insert("OpenAI-Organization", HeaderValue::from_str(api_org.as_str()).unwrap());

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
}
