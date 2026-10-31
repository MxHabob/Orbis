use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;

use super::{
    dahua::DahuaAdapter,
    hikvision::HikvisionAdapter,
    onvif::OnvifAdapter,
    rtsp::RtspAdapter,
    surveillance::{camera_credentials, camera_http_url, validate_whep_url},
};

pub async fn verify_connection(
    provider_name: &str,
    settings: &Value,
    secrets: &HashMap<String, String>,
) -> Result<(), DomainError> {
    let provider = match provider_name.to_ascii_lowercase().as_str() {
        "hikvision_camera" | "hikvision-camera" | "hikvision" => "hikvision",
        "dahua_camera" | "dahua-camera" | "dahua" => "dahua",
        "onvif_camera" | "onvif-camera" | "onvif" => "onvif",
        "rtsp_camera" | "rtsp-camera" | "rtsp" => "rtsp",
        _ => {
            return Err(DomainError::Validation(format!(
                "Unsupported camera provider '{provider_name}'"
            )));
        }
    };

    if settings.get("whep_url").is_some() {
        validate_whep_url(settings, provider)?;
    }
    match provider {
        "hikvision" => {
            let (username, password) = camera_credentials(settings, secrets, "Hikvision")?;
            let api_url = camera_http_url(settings, "Hikvision", 80)?;
            HikvisionAdapter::check_connection(&api_url, username, password).await
        }
        "dahua" => DahuaAdapter::check_connection(settings, secrets).await,
        "onvif" => OnvifAdapter::check_connection(settings, secrets).await,
        "rtsp" => Ok(()),
        _ => Err(DomainError::Validation(format!(
            "Unsupported camera provider '{provider_name}'"
        ))),
    }?;
    RtspAdapter::check_connection(settings, secrets).await
}
