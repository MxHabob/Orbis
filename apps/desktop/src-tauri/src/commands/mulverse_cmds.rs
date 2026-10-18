use crate::commands::make_ctx;
use crate::state::AppState;
use orbis_domain::iam::permission::Permission;
use orbis_infra::adapters::mulverse_client::{
    CreateOrbisDeploymentRequest, CreateOrbisEndpointRequest, MulverseApiError, MulverseClient,
    OrbisConnectionResponse, OrbisDeployment, OrbisEndpoint, UpdateOrbisDeploymentRequest,
    UpdateOrbisEndpointRequest,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

const KEYRING_SERVICE: &str = "com.orbis.desktop.mulverse";

#[derive(Serialize)]
pub struct ApiResponse<T> {
    data: T,
}

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Deserialize)]
pub struct ConnectMulverseInput {
    api_key: String,
}

#[derive(Deserialize)]
pub struct CreateDeploymentInput {
    name: String,
    environment: String,
}

#[derive(Deserialize)]
pub struct UpdateDeploymentInput {
    name: Option<String>,
    environment: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateEndpointInput {
    url: String,
    network_scope: String,
}

#[derive(Deserialize)]
pub struct UpdateEndpointInput {
    url: Option<String>,
    status: Option<String>,
}

fn api_client() -> Result<MulverseClient, String> {
    match std::env::var("MULVERSE_API_URL") {
        Ok(url) => MulverseClient::with_base_url(&url),
        Err(_) => MulverseClient::new(),
    }
}

fn keyring_entry(org_id: Uuid) -> Result<keyring::Entry, String> {
    keyring::Entry::new(KEYRING_SERVICE, &org_id.to_string()).map_err(|error| error.to_string())
}

async fn check_permission(state: &AppState) -> Result<Uuid, String> {
    let ctx = make_ctx(state);
    state
        .permission_checker
        .check(&ctx, Permission::OrganizationWrite)
        .await
        .map_err(|error| error.to_string())?;
    Ok(ctx.tenant_id)
}

async fn get_key(org_id: Uuid) -> Result<String, String> {
    keyring_entry(org_id)?
        .get_password()
        .map_err(|error| match error {
            keyring::Error::NoEntry => "Connect this institution to Mulverse first".to_owned(),
            other => format!("Could not read the Mulverse key from secure storage: {other}"),
        })
}

async fn check_orbis_permission(state: &AppState) -> Result<(Uuid, String), String> {
    let org_id = check_permission(state).await?;
    let key = get_key(org_id).await?;
    Ok((org_id, key))
}

fn map_error(error: MulverseApiError) -> String {
    error.to_string()
}

fn connection_with_organization(connection: OrbisConnectionResponse) -> OrbisConnectionResponse {
    connection
}

#[tauri::command]
pub async fn mulverse_get_connection(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Option<OrbisConnectionResponse>>, String> {
    let org_id = check_permission(&state).await?;
    let entry = keyring_entry(org_id)?;
    let key = match entry.get_password() {
        Ok(key) => key,
        Err(keyring::Error::NoEntry) => return Ok(ok(None)),
        Err(error) => {
            return Err(format!(
                "Could not read the Mulverse key from secure storage: {error}"
            ));
        }
    };
    let connection = api_client()?
        .verify_orbis_key(&key)
        .await
        .map_err(map_error)?;
    Ok(ok(Some(connection_with_organization(connection))))
}

#[tauri::command]
pub async fn mulverse_connect(
    request: ConnectMulverseInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OrbisConnectionResponse>, String> {
    let org_id = check_permission(&state).await?;
    let key = request.api_key.trim();
    if key.is_empty() {
        return Err("Mulverse API key is required".to_owned());
    }
    let connection = api_client()?
        .verify_orbis_key(key)
        .await
        .map_err(map_error)?;
    if !connection.connected {
        return Err("Mulverse API key is not active".to_owned());
    }
    keyring_entry(org_id)?
        .set_password(key)
        .map_err(|error| format!("Could not save the Mulverse key securely: {error}"))?;
    Ok(ok(connection))
}

#[tauri::command]
pub async fn mulverse_disconnect(state: State<'_, AppState>) -> Result<ApiResponse<bool>, String> {
    let org_id = check_permission(&state).await?;
    match keyring_entry(org_id)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(ok(true)),
        Err(error) => Err(format!(
            "Could not remove the Mulverse key from secure storage: {error}"
        )),
    }
}

#[tauri::command]
pub async fn mulverse_list_deployments(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<OrbisDeployment>>, String> {
    let (_, key) = check_orbis_permission(&state).await?;
    let deployments = api_client()?
        .list_orbis_deployments(&key)
        .await
        .map_err(map_error)?;
    Ok(ok(deployments))
}

#[tauri::command]
pub async fn mulverse_create_deployment(
    request: CreateDeploymentInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OrbisDeployment>, String> {
    let (_, key) = check_orbis_permission(&state).await?;
    let deployment = api_client()?
        .create_orbis_deployment(
            &key,
            &CreateOrbisDeploymentRequest {
                name: request.name,
                environment: request.environment,
            },
        )
        .await
        .map_err(map_error)?;
    Ok(ok(deployment))
}

#[tauri::command]
pub async fn mulverse_update_deployment(
    id: String,
    request: UpdateDeploymentInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OrbisDeployment>, String> {
    let (_, key) = check_orbis_permission(&state).await?;
    let deployment = api_client()?
        .update_orbis_deployment(
            &key,
            &id,
            &UpdateOrbisDeploymentRequest {
                name: request.name,
                environment: request.environment,
            },
        )
        .await
        .map_err(map_error)?;
    Ok(ok(deployment))
}

#[tauri::command]
pub async fn mulverse_list_endpoints(
    deployment_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<OrbisEndpoint>>, String> {
    let (_, key) = check_orbis_permission(&state).await?;
    let endpoints = api_client()?
        .list_orbis_endpoints(&key, &deployment_id)
        .await
        .map_err(map_error)?;
    Ok(ok(endpoints))
}

#[tauri::command]
pub async fn mulverse_create_endpoint(
    deployment_id: String,
    request: CreateEndpointInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OrbisEndpoint>, String> {
    if !matches!(request.network_scope.as_str(), "public" | "local_network") {
        return Err("Unsupported endpoint network scope".to_owned());
    }
    let (_, key) = check_orbis_permission(&state).await?;
    let endpoint = api_client()?
        .create_orbis_endpoint(
            &key,
            &deployment_id,
            &CreateOrbisEndpointRequest {
                url: request.url,
                network_scope: request.network_scope,
            },
        )
        .await
        .map_err(map_error)?;
    Ok(ok(endpoint))
}

#[tauri::command]
pub async fn mulverse_update_endpoint(
    id: String,
    request: UpdateEndpointInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OrbisEndpoint>, String> {
    if request
        .status
        .as_deref()
        .is_some_and(|status| !matches!(status, "active" | "pending" | "disabled"))
    {
        return Err("Only Mulverse staff can revoke an endpoint".to_owned());
    }
    let (_, key) = check_orbis_permission(&state).await?;
    let endpoint = api_client()?
        .update_orbis_endpoint(
            &key,
            &id,
            &UpdateOrbisEndpointRequest {
                url: request.url,
                status: request.status,
            },
        )
        .await
        .map_err(map_error)?;
    Ok(ok(endpoint))
}
