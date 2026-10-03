use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::NotificationPort;
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
}

// Partial implementation for NotificationPort
// Note: NotificationPort requires both send_email and send_whatsapp,
// but in a real system we might split the traits.
// For now, EmailAdapter just errors on whatsapp.
#[async_trait]
impl NotificationPort for EmailAdapter {
    async fn send_email(
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

    async fn send_whatsapp(
        &self,
        _ctx: &RequestContext,
        _to: &str,
        _message: &str,
    ) -> Result<(), DomainError> {
        Err(DomainError::Validation(
            "EmailAdapter cannot send WhatsApp messages".into(),
        ))
    }
}
