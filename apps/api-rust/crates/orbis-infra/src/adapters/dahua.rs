use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;

use super::{hikvision::HikvisionAdapter, surveillance::camera_http_url};

pub struct DahuaAdapter;

impl DahuaAdapter {
    pub async fn check_connection(
        settings: &Value,
        secrets: &HashMap<String, String>,
    ) -> Result<(), DomainError> {
        let username = settings
            .get("username")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|username| !username.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "The Dahua integration is missing the required 'username' setting".into(),
                )
            })?;
        let password = secrets
            .get("password")
            .map(String::as_str)
            .filter(|password| !password.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "The Dahua integration is missing the required 'password' credential".into(),
                )
            })?;
        let api_url = camera_http_url(settings, "Dahua", 80)?;
        HikvisionAdapter::check_digest_endpoint(
            &api_url,
            "/cgi-bin/magicBox.cgi?action=getSystemInfo",
            "Dahua",
            reqwest::Method::GET,
            username,
            password,
            None,
        )
        .await?;
        Ok(())
    }
}
