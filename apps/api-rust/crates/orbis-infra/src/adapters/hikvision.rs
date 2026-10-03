use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::SurveillancePort;
use orbis_domain::errors::DomainError;
use reqwest::Client;

pub struct HikvisionAdapter {
    pub api_url: String,
    pub api_key: String,
    pub client: Client,
}

impl HikvisionAdapter {
    pub fn new(api_url: String, api_key: String) -> Self {
        Self {
            api_url,
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl SurveillancePort for HikvisionAdapter {
    async fn detect_face(
        &self,
        _ctx: &RequestContext,
        image_data: &[u8],
    ) -> Result<Option<String>, DomainError> {
        let endpoint = format!("{}/ISAPI/Intelligent/face/detect", self.api_url);

        let res = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "image/jpeg")
            .body(image_data.to_vec())
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Hikvision Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Hikvision API Error: {}",
                error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Hikvision: {}", e)))?;

        if let Some(matches) = body.get("matches").and_then(|m| m.as_array()) {
            if !matches.is_empty() {
                return Ok(Some(
                    matches[0]["subject_id"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                ));
            }
        }

        Ok(None)
    }
}
