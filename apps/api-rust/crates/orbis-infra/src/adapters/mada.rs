use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::PaymentGatewayPort;
use orbis_domain::errors::DomainError;
use reqwest::Client;
use serde_json::json;

pub struct MadaAdapter {
    pub secret_key: String,
    pub client: Client,
}

impl MadaAdapter {
    pub fn new(secret_key: String) -> Self {
        Self {
            secret_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl PaymentGatewayPort for MadaAdapter {
    async fn process_payment(
        &self,
        _ctx: &RequestContext,
        amount: f64,
        currency: &str,
        source: &str,
    ) -> Result<String, DomainError> {
        // Note: Mada is often processed through gateways like Payfort, Moyasar, or Geidea.
        // Assuming a standard REST gateway for Mada
        let payload = json!({
            "amount": amount,
            "currency": currency,
            "payment_method": "mada",
            "token": source
        });

        let res = self
            .client
            .post("https://api.madagateway.sa/v1/charge") // Placeholder endpoint
            .bearer_auth(&self.secret_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Mada Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Mada API Error: {}",
                error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Mada: {}", e)))?;
        let txn_id = body["transaction_id"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        Ok(txn_id)
    }
}
