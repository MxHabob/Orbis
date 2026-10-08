use orbis_app::context::RequestContext;
use orbis_domain::errors::DomainError;
use reqwest::Client;
use serde_json::json;

pub struct EmailAdapter {
    pub api_key: String,
    pub client: Client,
}

impl EmailAdapter {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn send_email(
        &self,
        _ctx: &RequestContext,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), DomainError> {
        let payload = json!({
            "personalizations": [{
                "to": [{"email": to}]
            }],
            "from": {"email": "no-reply@orbis-edu.com"},
            "subject": subject,
            "content": [{
                "type": "text/html",
                "value": body
            }]
        });

        let res = self
            .client
            .post("https://api.sendgrid.com/v3/mail/send")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("SendGrid Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "SendGrid API Error: {}",
                error_text
            )));
        }

        Ok(())
    }
}
