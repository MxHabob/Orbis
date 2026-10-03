use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::SurveillancePort;
use orbis_domain::errors::DomainError;
use reqwest::Client;
use serde_json::json;

pub struct AuthenAdapter {
    pub api_url: String,
    pub api_key: String,
    pub client: Client,
}

impl AuthenAdapter {
    pub fn new(api_url: String, api_key: String) -> Self {
        Self {
            api_url,
            api_key,
            client: Client::new(),
        }
    }
}

use base64::prelude::*;

#[async_trait]
impl SurveillancePort for AuthenAdapter {
    async fn detect_face(
        &self,
        _ctx: &RequestContext,
        image_data: &[u8],
    ) -> Result<Option<String>, DomainError> {
        let endpoint = format!("{}/api/v1/vision/detect", self.api_url); // Assuming a standard REST structure for Authen-AI

        let payload = json!({
            // Send image as base64 or via multipart, assuming base64 JSON payload for simplicity
            "image": BASE64_STANDARD.encode(image_data)
        });

        let res = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Authen-AI Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Authen-AI API Error: {}",
                error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Authen-AI: {}", e)))?;

        // Assume response contains matched user_id
        if let Some(user_id) = body.get("user_id").and_then(|id| id.as_str()) {
            if !user_id.is_empty() {
                return Ok(Some(user_id.to_string()));
            }
        }

        Ok(None)
    }
}
