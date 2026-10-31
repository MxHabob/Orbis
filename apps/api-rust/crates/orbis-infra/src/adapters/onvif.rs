use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;

use super::{hikvision::HikvisionAdapter, surveillance::camera_http_url};

const DEVICE_INFORMATION_REQUEST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
  <s:Body><tds:GetDeviceInformation xmlns:tds="http://www.onvif.org/ver10/device/wsdl"/></s:Body>
</s:Envelope>"#;

pub struct OnvifAdapter;

impl OnvifAdapter {
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
                    "The ONVIF integration is missing the required 'username' setting".into(),
                )
            })?;
        let password = secrets
            .get("password")
            .map(String::as_str)
            .filter(|password| !password.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "The ONVIF integration is missing the required 'password' credential".into(),
                )
            })?;
        let device_url = camera_http_url(settings, "ONVIF", 80)?;
        let response = HikvisionAdapter::check_digest_endpoint(
            &device_url,
            "/onvif/device_service",
            "ONVIF",
            reqwest::Method::POST,
            username,
            password,
            Some(DEVICE_INFORMATION_REQUEST),
        )
        .await?;
        let body = response.text().await.map_err(|_| {
            DomainError::Validation("ONVIF camera returned an unreadable device response".into())
        })?;
        if !body.contains("GetDeviceInformationResponse") {
            return Err(DomainError::Validation(
                "The endpoint did not return a valid ONVIF device-information response".into(),
            ));
        }
        Ok(())
    }
}
