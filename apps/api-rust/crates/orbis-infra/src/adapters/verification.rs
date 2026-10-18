use orbis_app::context::RequestContext;
use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;

use super::{
    notification::NotificationAdapter, resolver::IntegrationResolver,
    solver_adapter::RemoteTimefoldSolver, storage::StorageResolver,
};

async fn verify_moyasar_connection(secrets: &HashMap<String, String>) -> Result<(), DomainError> {
    let secret_key = secrets
        .get("secret_key")
        .map(String::as_str)
        .map(str::trim)
        .filter(|key| !key.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(
                "The Moyasar integration is missing a required secret key".into(),
            )
        })?;
    let response = reqwest::Client::new()
        .get("https://api.moyasar.com/v1/payments?limit=1")
        .basic_auth(secret_key, Some(""))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|_| DomainError::Validation("Moyasar connection check failed".into()))?;
    if !response.status().is_success() {
        return Err(DomainError::Validation(format!(
            "Moyasar rejected the connection check (HTTP {})",
            response.status().as_u16()
        )));
    }
    Ok(())
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
    } else {
        Err(DomainError::Validation(format!(
            "No live connection verifier is available for category '{}'",
            category
        )))
    }
}
