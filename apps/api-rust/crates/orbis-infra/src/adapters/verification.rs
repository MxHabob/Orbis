use orbis_app::context::RequestContext;
use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;

use super::{
    notification::NotificationAdapter, resolver::IntegrationResolver, storage::StorageResolver,
    solver_adapter::RemoteTimefoldSolver,
};

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
        && matches!(provider_name, "solver_service" | "solver-service")
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
    } else {
        Err(DomainError::Validation(format!(
            "No live connection verifier is available for category '{}'",
            category
        )))
    }
}
