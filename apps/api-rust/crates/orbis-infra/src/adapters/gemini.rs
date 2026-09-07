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
    async fn generate_text(&self, _ctx: &orbis_app::context::RequestContext, model: &str, system_prompt: &str, user_prompt: &str, temperature: f64) -> Result<String, DomainError> {
        const CURRENT_FLASH_MODEL: &str = "gemini-3.0-flash";
        let mut selected_model = model.to_string();
        let prompt_text = format!("{}\n\n{}", system_prompt, user_prompt);
        let payload = json!({
            "contents": [{
                "parts": [{"text": prompt_text}]
            }],
            "generationConfig": {"temperature": temperature.clamp(0.0, 2.0)}
        });

        for attempt in 0..2 {
            let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent", selected_model);
            let res = self.client.post(&url)
                .header("x-goog-api-key", &self.api_key)
                .json(&payload)
                .send()
                .await
                .map_err(|e| DomainError::Validation(format!("Gemini API Network Error: {}", e)))?;

            if !res.status().is_success() {
                let status = res.status();
                let error_text = res.text().await.unwrap_or_default();
                let is_retired_model = status.as_u16() == 404
                    && error_text.contains("no longer available");
                if attempt == 0 && is_retired_model && selected_model != CURRENT_FLASH_MODEL {
                    selected_model = CURRENT_FLASH_MODEL.to_string();
                    continue;
                }
                return Err(DomainError::Validation(format!("Gemini API Error: {}", error_text)));
            }

            let body: serde_json::Value = res.json().await.map_err(|e| DomainError::Validation(format!("Invalid JSON from Gemini: {}", e)))?;
            let generated_text = body["candidates"][0]["content"]["parts"][0]["text"]
                .as_str()
                .unwrap_or("No response generated")
                .to_string();

            return Ok(generated_text);
        }

        Err(DomainError::Validation("Gemini model retry failed".into()))
    }
}
