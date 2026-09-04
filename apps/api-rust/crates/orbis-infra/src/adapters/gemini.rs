use async_trait::async_trait;
use orbis_app::ports::gateways::LlmGatewayPort;
use orbis_domain::errors::DomainError;
use reqwest::Client;
use serde_json::json;

pub struct GeminiAdapter {
    pub api_key: String,
    pub client: Client,
}

impl GeminiAdapter {
    pub fn new(api_key: String) -> Self {
        Self { api_key, client: Client::new() }
    }
}

#[async_trait]
impl LlmGatewayPort for GeminiAdapter {
    async fn generate_text(&self, _ctx: &orbis_app::context::RequestContext, model: &str, system_prompt: &str, user_prompt: &str, _temperature: f64) -> Result<String, DomainError> {
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, self.api_key);
        
        let prompt_text = format!("{}\n\n{}", system_prompt, user_prompt);
        let payload = json!({
            "contents": [{
                "parts": [{"text": prompt_text}]
            }]
        });

        let res = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Gemini API Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!("Gemini API Error: {}", error_text)));
        }

        let body: serde_json::Value = res.json().await.map_err(|e| DomainError::Validation(format!("Invalid JSON from Gemini: {}", e)))?;
        
        // Extract text from the response structure
        let generated_text = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No response generated")
            .to_string();

        Ok(generated_text)
    }
}
