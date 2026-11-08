use orbis_app::context::RequestContext;
use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use tokio::net::TcpStream;

use super::{
    authen::AuthenAdapter, camera::verify_connection as verify_camera_connection,
    mulverse_client::MulverseClient, notification::NotificationAdapter,
    resolver::IntegrationResolver, solver_adapter::RemoteTimefoldSolver, storage::StorageResolver,
};

const CONNECTION_CHECK_TIMEOUT: Duration = Duration::from_secs(10);

fn connection_check_client() -> Result<reqwest::Client, DomainError> {
    reqwest::Client::builder()
        .timeout(CONNECTION_CHECK_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| DomainError::Internal("Could not initialize connection check".into()))
}

fn required_secret<'a>(
    secrets: &'a HashMap<String, String>,
    name: &str,
    provider: &str,
) -> Result<&'a str, DomainError> {
    secrets
        .get(name)
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "The {provider} integration is missing the required '{name}' credential"
            ))
        })
}

fn reject_connection_status(provider: &str, status: reqwest::StatusCode) -> DomainError {
    DomainError::Validation(format!(
        "{provider} rejected the connection check (HTTP {})",
        status.as_u16()
    ))
}

async fn verify_moyasar_connection(secrets: &HashMap<String, String>) -> Result<(), DomainError> {
    let secret_key = required_secret(secrets, "secret_key", "Moyasar")?;
    let response = connection_check_client()?
        .get("https://api.moyasar.com/v1/payments?limit=1")
        .basic_auth(secret_key, Some(""))
        .send()
        .await
        .map_err(|_| DomainError::Validation("Moyasar connection check failed".into()))?;
    if !response.status().is_success() {
        return Err(reject_connection_status("Moyasar", response.status()));
    }
    Ok(())
}

async fn verify_stripe_connection(secrets: &HashMap<String, String>) -> Result<(), DomainError> {
    let secret_key = required_secret(secrets, "secret_key", "Stripe")?;
    let response = connection_check_client()?
        .get("https://api.stripe.com/v1/account")
        .basic_auth(secret_key, Some(""))
        .send()
        .await
        .map_err(|_| DomainError::Validation("Stripe connection check failed".into()))?;
    if !response.status().is_success() {
        return Err(reject_connection_status("Stripe", response.status()));
    }
    Ok(())
}

async fn verify_orbis_ai_connection(
    settings: &Value,
    secrets: &HashMap<String, String>,
) -> Result<(), DomainError> {
    required_setting(settings, "model", "Orbis AI")?;
    let api_key = required_secret(secrets, "api_key", "Orbis AI")?;
    let response = MulverseClient::new()
        .map_err(|_| {
            DomainError::Internal("Could not initialize Orbis AI connection check".into())
        })?
        .verify_orbis_key(api_key)
        .await
        .map_err(|_| DomainError::Validation("Orbis AI rejected the connection check".into()))?;
    if !response.connected {
        return Err(DomainError::Validation(
            "The Mulverse API key is not connected to Orbis AI".into(),
        ));
    }
    Ok(())
}

async fn verify_tabby_connection(
    settings: &Value,
    secrets: &HashMap<String, String>,
) -> Result<(), DomainError> {
    let secret_key = required_secret(secrets, "secret_key", "Tabby")?;
    required_setting(settings, "public_key", "Tabby")?;
    required_setting(settings, "merchant_code", "Tabby")?;
    let response = connection_check_client()?
        .get("https://api.tabby.sa/api/v2/payments?limit=1")
        .bearer_auth(secret_key)
        .send()
        .await
        .map_err(|_| DomainError::Validation("Tabby connection check failed".into()))?;
    if !response.status().is_success() {
        return Err(reject_connection_status("Tabby", response.status()));
    }
    Ok(())
}

async fn verify_tamara_connection(
    settings: &Value,
    secrets: &HashMap<String, String>,
) -> Result<(), DomainError> {
    let api_token = required_secret(secrets, "api_token", "Tamara")?;
    required_setting(settings, "public_key", "Tamara")?;
    let base_url = if settings
        .get("sandbox_mode")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        "https://api-sandbox.tamara.co"
    } else {
        "https://api.tamara.co"
    };
    let response = connection_check_client()?
        .get(format!(
            "{base_url}/orders/00000000-0000-0000-0000-000000000000"
        ))
        .bearer_auth(api_token)
        .send()
        .await
        .map_err(|_| DomainError::Validation("Tamara connection check failed".into()))?;
    if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
        return Err(reject_connection_status("Tamara", response.status()));
    }
    Ok(())
}

async fn verify_hyperpay_connection(
    settings: &Value,
    secrets: &HashMap<String, String>,
) -> Result<(), DomainError> {
    let access_token = required_secret(secrets, "access_token", "HyperPay")?;
    let base_url = if settings
        .get("test_mode")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        "https://eu-test.oppwa.com"
    } else {
        "https://eu-prod.oppwa.com"
    };
    let client = connection_check_client()?;

    for entity_setting in ["entity_id_mada", "entity_id_cards"] {
        let entity_id = required_setting(settings, entity_setting, "HyperPay")?;
        let response = client
            .get(format!(
                "{base_url}/v1/payments/00000000000000000000000000000000"
            ))
            .bearer_auth(access_token)
            .query(&[("entityId", entity_id)])
            .send()
            .await
            .map_err(|_| DomainError::Validation("HyperPay connection check failed".into()))?;
        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
            return Err(reject_connection_status("HyperPay", response.status()));
        }
    }
    Ok(())
}

async fn verify_google_workspace_connection(
    settings: &Value,
    secrets: &HashMap<String, String>,
) -> Result<(), DomainError> {
    let domain = required_setting(settings, "domain", "Google Workspace")?;
    if !matches!(url::Host::parse(domain), Ok(url::Host::Domain(_))) {
        return Err(DomainError::Validation(
            "Google Workspace organization domain is invalid".into(),
        ));
    }
    let client_id = settings
        .get("client_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    let client_secret = secrets
        .get("client_secret")
        .map(String::as_str)
        .map(str::trim)
        .unwrap_or_default();
    if client_id.is_empty() != client_secret.is_empty() {
        return Err(DomainError::Validation(
            "Google Workspace OAuth client ID and secret must be provided together".into(),
        ));
    }

    let response = connection_check_client()?
        .get("https://accounts.google.com/.well-known/openid-configuration")
        .send()
        .await
        .map_err(|_| DomainError::Validation("Google Workspace connection check failed".into()))?;
    if !response.status().is_success() {
        return Err(reject_connection_status(
            "Google Workspace",
            response.status(),
        ));
    }
    let discovery: Value = response.json().await.map_err(|_| {
        DomainError::Validation("Google returned an invalid OpenID configuration".into())
    })?;
    if discovery.get("issuer").and_then(Value::as_str) != Some("https://accounts.google.com")
        || discovery
            .get("authorization_endpoint")
            .and_then(Value::as_str)
            .is_none()
        || discovery
            .get("token_endpoint")
            .and_then(Value::as_str)
            .is_none()
    {
        return Err(DomainError::Validation(
            "Google returned an incomplete OpenID configuration".into(),
        ));
    }
    Ok(())
}

async fn verify_microsoft_365_connection(
    settings: &Value,
    secrets: &HashMap<String, String>,
) -> Result<(), DomainError> {
    let tenant_id = required_setting(settings, "tenant_id", "Microsoft 365")?
        .parse::<uuid::Uuid>()
        .map_err(|_| {
            DomainError::Validation("Microsoft 365 tenant ID must be a valid UUID".into())
        })?
        .to_string();
    let client_id = required_setting(settings, "client_id", "Microsoft 365")?;
    let client_secret = required_secret(secrets, "client_secret", "Microsoft 365")?;

    let mut token_url = url::Url::parse("https://login.microsoftonline.com")
        .map_err(|_| DomainError::Internal("Could not build Microsoft OAuth URL".into()))?;
    token_url
        .path_segments_mut()
        .map_err(|_| DomainError::Internal("Could not build Microsoft OAuth URL".into()))?
        .push(&tenant_id)
        .push("oauth2")
        .push("v2.0")
        .push("token");

    let response = connection_check_client()?
        .post(token_url)
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("scope", "https://graph.microsoft.com/.default"),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .await
        .map_err(|_| DomainError::Validation("Microsoft 365 connection check failed".into()))?;
    if !response.status().is_success() {
        return Err(reject_connection_status("Microsoft 365", response.status()));
    }

    let token: Value = response.json().await.map_err(|_| {
        DomainError::Validation("Microsoft returned an invalid OAuth token response".into())
    })?;
    if token
        .get("access_token")
        .and_then(Value::as_str)
        .map_or(true, str::is_empty)
    {
        return Err(DomainError::Validation(
            "Microsoft did not return an access token".into(),
        ));
    }
    Ok(())
}

async fn verify_fingerprint_connection(settings: &Value) -> Result<(), DomainError> {
    let device_ip = required_setting(settings, "device_ip", "Fingerprint")?
        .parse::<IpAddr>()
        .map_err(|_| DomainError::Validation("Fingerprint device IP address is invalid".into()))?;
    let port = settings
        .get("device_port")
        .and_then(|value| {
            value
                .as_u64()
                .or_else(|| value.as_str().and_then(|port| port.parse().ok()))
        })
        .filter(|port| (1..=u16::MAX as u64).contains(port))
        .ok_or_else(|| {
            DomainError::Validation("Fingerprint device port must be between 1 and 65535".into())
        })?;
    let address = std::net::SocketAddr::new(device_ip, port as u16);

    tokio::time::timeout(CONNECTION_CHECK_TIMEOUT, TcpStream::connect(address))
        .await
        .map_err(|_| DomainError::Validation("Fingerprint device connection timed out".into()))?
        .map_err(|_| DomainError::Validation("Could not reach the fingerprint device".into()))?;
    Ok(())
}

fn required_setting<'a>(
    settings: &'a Value,
    name: &str,
    provider: &str,
) -> Result<&'a str, DomainError> {
    settings
        .get(name)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "The {provider} integration is missing the required '{name}' setting"
            ))
        })
}

#[cfg(test)]
fn hikvision_camera_url(settings: &Value) -> Result<String, DomainError> {
    super::surveillance::camera_http_url(settings, "Hikvision", 80)
}

pub async fn verify_integration_connection(
    ctx: &RequestContext,
    provider_name: &str,
    category: &str,
    settings: &Value,
    secrets: &HashMap<String, String>,
    integration_resolver: &IntegrationResolver,
    notification_adapter: &NotificationAdapter,
    storage_resolver: &StorageResolver,
) -> Result<(), DomainError> {
    if category.eq_ignore_ascii_case("storage") {
        storage_resolver
            .check_connection(ctx.tenant_id, provider_name, category, settings, secrets)
            .await
            .map_err(|error| DomainError::Validation(error.to_string()))
    } else if category.eq_ignore_ascii_case("ai-vision")
        && provider_name.eq_ignore_ascii_case("authen")
    {
        let api_url = settings
            .get("api_url")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|url| !url.is_empty())
            .ok_or_else(|| {
                DomainError::Validation("The Authen integration is missing its API URL".into())
            })?;
        let api_key = secrets
            .get("api_key")
            .map(String::as_str)
            .map(str::trim)
            .filter(|key| !key.is_empty())
            .ok_or_else(|| {
                DomainError::Validation("The Authen integration is missing its API key".into())
            })?;
        AuthenAdapter::new(api_url.to_string(), api_key.to_string())
            .check_connection()
            .await
    } else if category.eq_ignore_ascii_case("surveillance") {
        verify_camera_connection(provider_name, settings, secrets).await
    } else if category.eq_ignore_ascii_case("ai-provider")
        && provider_name.eq_ignore_ascii_case("orbis_ai")
    {
        verify_orbis_ai_connection(settings, secrets).await
    } else if category.eq_ignore_ascii_case("identity")
        && provider_name.eq_ignore_ascii_case("orbis_mobile")
    {
        if !settings.as_object().is_some_and(serde_json::Map::is_empty) || !secrets.is_empty() {
            return Err(DomainError::Validation(
                "Orbis Mobile activation does not accept integration settings or secrets".into(),
            ));
        }
        Ok(())
    } else if category.eq_ignore_ascii_case("identity")
        && provider_name.eq_ignore_ascii_case("google-workspace")
    {
        verify_google_workspace_connection(settings, secrets).await
    } else if category.eq_ignore_ascii_case("identity")
        && provider_name.eq_ignore_ascii_case("microsoft-365")
    {
        verify_microsoft_365_connection(settings, secrets).await
    } else if category.eq_ignore_ascii_case("hardware")
        && provider_name.eq_ignore_ascii_case("fingerprint")
    {
        verify_fingerprint_connection(settings).await
    } else if category.eq_ignore_ascii_case("communication") {
        notification_adapter
            .check_connection(ctx.tenant_id, provider_name, category, settings, secrets)
            .await
    } else if category.eq_ignore_ascii_case("ai-provider")
        || category.eq_ignore_ascii_case("ai")
        || category.eq_ignore_ascii_case("llm")
        || category.eq_ignore_ascii_case("ai-gateway")
    {
        integration_resolver
            .check_llm_connection(ctx, provider_name, settings, secrets)
            .await
    } else if category.eq_ignore_ascii_case("automation")
        && (provider_name.eq_ignore_ascii_case("solver_service")
            || provider_name.eq_ignore_ascii_case("solver-service"))
    {
        let endpoint = settings
            .get("endpoint_url")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|endpoint| !endpoint.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "Solver service endpoint is required to verify the connection".into(),
                )
            })?;
        let mode = settings
            .get("mode")
            .and_then(Value::as_str)
            .unwrap_or("remote");
        if !matches!(mode, "embedded" | "remote") {
            return Err(DomainError::Validation(format!(
                "Unsupported solver deployment mode '{}'",
                mode
            )));
        }
        RemoteTimefoldSolver::new(endpoint.to_string())
            .check_health()
            .await
    } else if category.eq_ignore_ascii_case("payment-gateway")
        && provider_name.eq_ignore_ascii_case("moyasar")
    {
        verify_moyasar_connection(secrets).await
    } else if category.eq_ignore_ascii_case("payment-gateway")
        && provider_name.eq_ignore_ascii_case("stripe")
    {
        verify_stripe_connection(secrets).await
    } else if category.eq_ignore_ascii_case("payment-gateway")
        && provider_name.eq_ignore_ascii_case("tabby")
    {
        verify_tabby_connection(settings, secrets).await
    } else if category.eq_ignore_ascii_case("payment-gateway")
        && provider_name.eq_ignore_ascii_case("tamara")
    {
        verify_tamara_connection(settings, secrets).await
    } else if category.eq_ignore_ascii_case("payment-gateway")
        && provider_name.eq_ignore_ascii_case("hyperpay")
    {
        verify_hyperpay_connection(settings, secrets).await
    } else {
        Err(DomainError::Validation(format!(
            "No live connection verifier is available for provider '{}' in category '{}'",
            provider_name, category
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_hikvision_url_from_ipv4_settings() {
        let settings = serde_json::json!({ "ip_address": "192.168.1.20", "port": 65002 });

        assert_eq!(
            hikvision_camera_url(&settings).expect("valid camera address"),
            "http://192.168.1.20:65002"
        );
    }

    #[test]
    fn brackets_ipv6_hikvision_addresses() {
        let settings = serde_json::json!({ "ip_address": "fe80::1", "port": "80" });

        assert_eq!(
            hikvision_camera_url(&settings).expect("valid camera address"),
            "http://[fe80::1]:80"
        );
    }

    #[test]
    fn builds_hikvision_url_from_ddns_hostname() {
        let settings =
            serde_json::json!({ "ip_address": "9002A9553EB9.DahuaDDNS.com", "port": 6600 });

        assert_eq!(
            hikvision_camera_url(&settings).expect("valid camera hostname"),
            "http://9002a9553eb9.dahuaddns.com:6600"
        );
    }

    #[test]
    fn rejects_urls_and_ports_in_hikvision_host_field() {
        for address in ["https://camera.example.com", "camera.example.com:8443"] {
            let settings = serde_json::json!({ "ip_address": address, "port": 80 });

            assert!(hikvision_camera_url(&settings).is_err(), "{address}");
        }
    }

    #[test]
    fn rejects_invalid_hikvision_ports() {
        let settings = serde_json::json!({ "ip_address": "192.168.1.20", "port": 0 });

        assert!(hikvision_camera_url(&settings).is_err());
    }

    #[tokio::test]
    async fn requires_orbis_ai_model_before_checking_credentials() {
        assert!(
            verify_orbis_ai_connection(&Value::Null, &HashMap::new())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn rejects_invalid_fingerprint_address_before_network_access() {
        let settings = serde_json::json!({ "device_ip": "not-an-ip", "device_port": 4370 });

        assert!(verify_fingerprint_connection(&settings).await.is_err());
    }

    #[tokio::test]
    async fn rejects_invalid_microsoft_tenant_before_network_access() {
        let settings = serde_json::json!({ "tenant_id": "not-a-uuid" });

        assert!(
            verify_microsoft_365_connection(&settings, &HashMap::new())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn requires_stripe_secret_without_network_access() {
        assert!(verify_stripe_connection(&HashMap::new()).await.is_err());
    }
}
