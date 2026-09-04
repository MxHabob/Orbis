//! Email Notification Adapter — skeleton for sending emails via SMTP or API.
//!
//! Supports:
//!   - SMTP (generic)
//!   - SendGrid API
//!   - Mailgun API
//!
//! Used for:
//!   - Parent notifications (absence alerts, grade reports)
//!   - Staff communications
//!   - System alerts (autopilot failures, data quality issues)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{NotificationProvider, Notification, NotificationChannel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub provider: String, // "smtp" | "sendgrid" | "mailgun"
    pub from_address: String,
    pub from_name: String,
    // SMTP-specific
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_tls: Option<bool>,
    // API-specific
    pub api_key: Option<String>,
}

pub struct EmailAdapter {
    config: EmailConfig,
}

impl EmailAdapter {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl NotificationProvider for EmailAdapter {
    fn provider_name(&self) -> &str {
        "email"
    }

    fn supported_channels(&self) -> Vec<NotificationChannel> {
        vec![NotificationChannel::Email]
    }

    async fn send(&self, notification: &Notification) -> Result<String, String> {
        let _ = notification;
        Err(format!(
            "Email provider '{}' is configured but sending is not implemented",
            self.config.provider
        ))
    }

    async fn send_bulk(&self, notifications: &[Notification]) -> Result<Vec<String>, String> {
        let mut ids = Vec::new();
        for n in notifications {
            ids.push(self.send(n).await?);
        }
        Ok(ids)
    }
}
