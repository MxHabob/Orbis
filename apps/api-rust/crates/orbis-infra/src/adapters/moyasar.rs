use async_trait::async_trait;
use orbis_app::ports::gateways::PaymentGatewayPort;
use orbis_domain::errors::DomainError;
use orbis_app::context::RequestContext;
use reqwest::Client;
use serde_json::json;

pub struct MoyasarAdapter {
    pub secret_key: String,
    pub client: Client,
}

impl MoyasarAdapter {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key, client: Client::new() }
    }
}

#[async_trait]
impl PaymentGatewayPort for MoyasarAdapter {
    async fn process_payment(&self, _ctx: &RequestContext, amount: f64, currency: &str, source: &str) -> Result<String, DomainError> {
        // Source in this context would typically be a token from frontend
        // We expect source to be a JSON string representing the funding source, or just a token ID
        let source_val: serde_json::Value = serde_json::from_str(source).unwrap_or_else(|_| json!({"type": "token", "token": source}));
        
        let payload = json!({
            // Moyasar expects amount in cents/halalas
            "amount": (amount * 100.0) as i64, 
            "currency": currency,
            "source": source_val,
            "description": "Orbis App Store Integration Payment"
        });

        let res = self.client.post("https://api.moyasar.com/v1/payments")
            .basic_auth(&self.secret_key, Some(""))
            .json(&payload)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Moyasar Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!("Moyasar API Error: {}", error_text)));
        }

        let body: serde_json::Value = res.json().await.map_err(|e| DomainError::Validation(format!("Invalid JSON from Moyasar: {}", e)))?;
        
        let payment_id = body["id"].as_str().unwrap_or_default().to_string();
        Ok(payment_id)
    }
}
