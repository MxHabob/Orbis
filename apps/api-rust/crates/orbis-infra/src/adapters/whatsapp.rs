use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::NotificationPort;
use orbis_domain::errors::DomainError;
use reqwest::Client;

pub struct WhatsAppAdapter {
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    pub client: Client,
}

impl WhatsAppAdapter {
    pub fn new(account_sid: String, auth_token: String, from_number: String) -> Self {
        Self {
            account_sid,
            auth_token,
            from_number,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl NotificationPort for WhatsAppAdapter {
    async fn send_email(
        &self,
        _ctx: &RequestContext,
        _to: &str,
        _subject: &str,
        _body: &str,
    ) -> Result<(), DomainError> {
        Err(DomainError::Validation(
            "WhatsAppAdapter cannot send Email messages".into(),
        ))
    }

    async fn send_whatsapp(
        &self,
        _ctx: &RequestContext,
        to: &str,
        message: &str,
    ) -> Result<(), DomainError> {
        let endpoint = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );

        // Twilio requires form urlencoded
        let params = [
            ("To", format!("whatsapp:{}", to)),
            ("From", format!("whatsapp:{}", self.from_number)),
            ("Body", message.to_string()),
        ];

        let res = self
            .client
            .post(&endpoint)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Twilio Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Twilio API Error: {}",
                error_text
            )));
        }

        Ok(())
    }

    async fn send_sms(
        &self,
        _ctx: &RequestContext,
        to: &str,
        message: &str,
    ) -> Result<(), DomainError> {
        let endpoint = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );

        let params = [
            ("To", to.to_string()),
            ("From", self.from_number.clone()),
            ("Body", message.to_string()),
        ];

        let res = self
            .client
            .post(&endpoint)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Twilio SMS Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Twilio SMS API Error: {}",
                error_text
            )));
        }

        Ok(())
    }
}
