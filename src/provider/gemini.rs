use super::{AIProvider, ProviderError};
use crate::ai_prompt::AIPrompt;
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct GeminiConfig {
    api_key: String,
    model: String,
    api_url_template: String, // Template like "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
}

impl GeminiConfig {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gemini-1.5-flash-latest".to_string()),
            // Using v1beta as it's commonly available
            api_url_template: "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent".to_string(),
        }
    }

    fn get_api_url(&self) -> String {
        self.api_url_template.replace("{model}", &self.model)
    }
}

// Structs for Gemini API Request
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    // Add generationConfig here if needed
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

// Structs for Gemini API Response
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    // Add promptFeedback here if needed for error checking
    error: Option<GeminiErrorDetail>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<ContentResponse>,
    // Add finishReason, safetyRatings etc. if needed
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Option<Vec<PartResponse>>,
    // role: Option<String> // Usually "model"
}

#[derive(Deserialize)]
struct PartResponse {
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GeminiErrorDetail {
    code: Option<i32>,
    message: Option<String>,
    status: Option<String>,
}


pub struct GeminiProvider {
    client: reqwest::Client,
    config: GeminiConfig,
}

impl GeminiProvider {
    pub fn new(client: reqwest::Client, config: GeminiConfig) -> Self {
        Self { client, config }
    }

    async fn complete(&self, prompt: AIPrompt) -> Result<String, ProviderError> {
        // Gemini's simpler API often works well combining system and user prompts
        let combined_prompt = format!("{}\n\n{}", prompt.system_prompt, prompt.user_prompt);

        let request_payload = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: combined_prompt }],
            }],
        };

        let api_url = format!("{}?key={}", self.config.get_api_url(), self.config.api_key);

        let response = self
            .client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .json(&request_payload)
            .send()
            .await?;

        let status = response.status();
        let response_bytes = response.bytes().await?; // Read bytes first for better error reporting

        // Try parsing as success response
        if status == StatusCode::OK {
             match serde_json::from_slice::<GeminiResponse>(&response_bytes) {
                 Ok(parsed_response) => {
                    let text = parsed_response
                        .candidates
                        .and_then(|mut c| c.pop()) // Take the first candidate
                        .and_then(|c| c.content)
                        .and_then(|co| co.parts)
                        .and_then(|mut p| p.pop()) // Take the first part
                        .and_then(|p| p.text)
                        .ok_or(ProviderError::NoCompletionChoice)?;
                    return Ok(text);
                 }
                 Err(e) => {
                    // If parsing success response fails, return unexpected response
                    eprintln!("Failed to parse successful Gemini response: {}", e);
                    return Err(ProviderError::UnexpectedResponse);
                 }
             }
        }

        // If status is not OK, try parsing as error response
        match serde_json::from_slice::<GeminiResponse>(&response_bytes) {
            Ok(error_response) => {
                let error_message = error_response
                    .error
                    .and_then(|e| {
                        let code_info = e.code.map_or("".to_string(), |c| format!(" (Code: {})", c));
                        let status_info = e.status.map_or("".to_string(), |s| format!(" Status: {}", s));
                        Some(format!("{}{}{}", e.message.unwrap_or_default(), code_info, status_info))
                    })
                    .unwrap_or_else(|| String::from_utf8_lossy(&response_bytes).to_string()); // Fallback to raw response
                Err(ProviderError::APIError(status, error_message))
            },
            Err(_) => {
                // If parsing error response also fails, return raw response
                 Err(ProviderError::APIError(
                    status,
                    String::from_utf8_lossy(&response_bytes).to_string(),
                ))
            }
        }
    }
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn complete(&self, prompt: AIPrompt) -> Result<String, ProviderError> {
        self.complete(prompt).await
    }
} 