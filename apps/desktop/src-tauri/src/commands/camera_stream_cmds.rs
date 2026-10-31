use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::features::camera_stream::CameraStreamSource;
use crate::state::AppState;
use orbis_domain::iam::permission::Permission;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CameraStreamDto {
    id: String,
    name: String,
    address: String,
    location: String,
    status: String,
    #[serde(rename = "whepUrl")]
    whep_url: Option<String>,
}

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[tauri::command]
pub async fn camera_stream_start(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<CameraStreamDto>>, String> {
    let ctx = make_ctx(&state);
    state
        .integration_service
        .permission_checker
        .check(&ctx, Permission::OrganizationRead)
        .await
        .map_err(|error| error.to_string())?;

    let integrations = state
        .integration_service
        .list_integrations(ctx.tenant_id)
        .await
        .map_err(|error| error.to_string())?;
    let mut sources = Vec::new();
    let mut cameras = Vec::new();

    for integration in integrations.into_iter().filter(|integration| {
        integration.is_enabled && integration.category.eq_ignore_ascii_case("surveillance")
    }) {
        let Some(provider) = camera_provider_label(&integration.provider_name) else {
            continue;
        };
        let settings = integration.settings.unwrap_or(Value::Null);
        let rtsp_url = setting_string(&settings, "rtsp_url");
        let username = setting_string(&settings, "username");
        let location = setting_string(&settings, "location").unwrap_or_default();
        let address = setting_string(&settings, "ip_address")
            .or_else(|| rtsp_url.as_deref().and_then(rtsp_host))
            .unwrap_or_default();
        let name = setting_string(&settings, "camera_name")
            .filter(|name| !name.trim().is_empty())
            .unwrap_or_else(|| format!("{provider} {address}").trim().to_string());

        let integration_id = integration.id;
        if let Some(rtsp_url) = rtsp_url {
            let password = state
                .integration_repo
                .get_secrets_by_config(integration_id)
                .await
                .map_err(|error| error.to_string())?
                .into_iter()
                .find(|secret| secret.key_name == "password")
                .map(|secret| secret.secret_key);
            sources.push(CameraStreamSource {
                id: integration_id,
                rtsp_url,
                username,
                password,
            });
        }

        cameras.push((integration_id, name, address, location));
    }

    let stream_map: HashMap<Uuid, String> =
        state.camera_stream_manager.start_streams(&sources).await?;

    Ok(ok(cameras
        .into_iter()
        .map(|(id, name, address, location)| {
            let whep_url = stream_map.get(&id).cloned();
            CameraStreamDto {
                id: id.to_string(),
                name,
                address,
                location,
                status: if whep_url.is_some() {
                    "online".to_string()
                } else {
                    "error".to_string()
                },
                whep_url,
            }
        })
        .collect()))
}

#[tauri::command]
pub async fn camera_stream_stop(state: State<'_, AppState>) -> Result<ApiResponse<bool>, String> {
    state.camera_stream_manager.stop().await;
    Ok(ok(true))
}

fn setting_string(settings: &Value, key: &str) -> Option<String> {
    settings
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn camera_provider_label(provider_name: &str) -> Option<&'static str> {
    match provider_name.to_ascii_lowercase().as_str() {
        "hikvision_camera" | "hikvision-camera" | "hikvision" => Some("Hikvision"),
        "dahua_camera" | "dahua-camera" | "dahua" => Some("Dahua"),
        "onvif_camera" | "onvif-camera" | "onvif" => Some("ONVIF"),
        "rtsp_camera" | "rtsp-camera" | "rtsp" => Some("RTSP"),
        _ => None,
    }
}

fn rtsp_host(endpoint: &str) -> Option<String> {
    url::Url::parse(endpoint)
        .ok()
        .and_then(|url| url.host_str().map(str::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_provider_names_are_case_insensitive() {
        assert_eq!(camera_provider_label("Dahua-Camera"), Some("Dahua"));
        assert_eq!(camera_provider_label("unknown"), None);
    }

    #[test]
    fn rtsp_host_does_not_include_credentials() {
        assert_eq!(
            rtsp_host("rtsp://user:secret@192.168.1.25:554/stream"),
            Some("192.168.1.25".to_string())
        );
    }
}
