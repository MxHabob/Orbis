use async_trait::async_trait;
use orbis_app::ports::gateways::LlmGatewayPort;
use orbis_domain::errors::DomainError;
use orbis_app::context::RequestContext;
use reqwest::Client;
use serde_json::json;

pub struct OrbisAiAdapter {
    pub endpoint_url: String,
    pub client: Client,
}

impl OrbisAiAdapter {
    pub fn new(endpoint_url: String) -> Self {
        Self { endpoint_url, client: Client::new() }
    }
}

#[async_trait]
impl LlmGatewayPort for OrbisAiAdapter {
    async fn generate_text(&self, _ctx: &RequestContext, model: &str, system_prompt: &str, user_prompt: &str, temperature: f64) -> Result<String, DomainError> {
        // Assuming OpenAI-compatible endpoint for local LLMs (e.g., Llama via vLLM, LM Studio, or Ollama OpenAI wrapper)
        let endpoint = format!("{}/v1/chat/completions", self.endpoint_url);
        
        let payload = json!({
            "model": model, // e.g. "llama-3-8b"
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
            ],
            "temperature": temperature
        });

        let res = self.client.post(&endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Orbis Local AI Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!("Orbis Local AI API Error: {}", error_text)));
        }

        let body: serde_json::Value = res.json().await.map_err(|e| DomainError::Validation(format!("Invalid JSON from Orbis Local AI: {}", e)))?;
        
        let generated_text = body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response generated")
            .to_string();

        Ok(generated_text)
    }
}
