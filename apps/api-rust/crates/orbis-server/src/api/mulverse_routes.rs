use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, put},
};
use orbis_infra::adapters::mulverse_client::{
    CreateOrbisDeploymentRequest, CreateOrbisEndpointRequest, MulverseApiError, OrbisDeployment,
    OrbisEndpoint, UpdateOrbisDeploymentRequest, UpdateOrbisEndpointRequest,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_domain::iam::permission::Permission;

const PROVIDER_NAME: &str = "mulverse";
const SECRET_NAME: &str = "orbis_api_key";

#[derive(Serialize, ToSchema)]
pub struct MulverseConnectionDto {
    pub connected: bool,
    pub organization_id: Option<String>,
    pub product_organization_id: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ConnectMulverseRequest {
    pub api_key: String,
}

#[derive(Serialize, ToSchema)]
pub struct DeploymentDto {
    pub id: String,
    pub name: String,
    pub environment: String,
    pub status: String,
    pub verification_status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct EndpointDto {
    pub id: String,
    pub deployment_id: String,
    pub url: String,
    pub network_scope: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateDeploymentRequest {
    pub name: String,
    pub environment: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateDeploymentRequest {
    pub name: Option<String>,
    pub environment: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateEndpointRequest {
    pub url: String,
    pub network_scope: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateEndpointRequest {
    pub url: Option<String>,
    pub status: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/orbis-mobile", get(get_status).put(connect))
        .route("/api/v1/orbis-mobile", delete(disconnect))
        .route(
            "/api/v1/orbis-mobile/deployments",
            get(list_deployments).post(create_deployment),
        )
        .route(
            "/api/v1/orbis-mobile/deployments/{id}",
            put(update_deployment),
        )
        .route(
            "/api/v1/orbis-mobile/deployments/{id}/endpoints",
            get(list_endpoints).post(create_endpoint),
        )
        .route("/api/v1/orbis-mobile/endpoints/{id}", put(update_endpoint))
}

#[utoipa::path(
    get,
    path = "/api/v1/orbis-mobile",
    responses((status = 200, body = MulverseConnectionDto)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn get_status(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MulverseConnectionDto>>, ApiError> {
    ensure_write_access(&state, &ctx).await?;
    let Some((config, key)) = get_connection(&state, ctx.tenant_id).await? else {
        return Ok(ApiResponse::ok(MulverseConnectionDto {
            connected: false,
            organization_id: None,
            product_organization_id: None,
        }));
    };
    let verified = state
        .mulverse_client
        .verify_orbis_key(&key)
        .await
        .map_err(map_mulverse_error)?;
    let settings = config.settings.unwrap_or_default();
    let organization_id = settings
        .get("organization_id")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    let product_organization_id = settings
        .get("product_organization_id")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    Ok(ApiResponse::ok(MulverseConnectionDto {
        connected: verified.connected,
        organization_id,
        product_organization_id,
    }))
}

#[utoipa::path(
    put,
    path = "/api/v1/orbis-mobile",
    request_body = ConnectMulverseRequest,
    responses((status = 200, body = MulverseConnectionDto)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn connect(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<ConnectMulverseRequest>,
) -> Result<Json<ApiResponse<MulverseConnectionDto>>, ApiError> {
    ensure_write_access(&state, &ctx).await?;
    if request.api_key.trim().is_empty() {
        return Err(ApiError::bad_request("Mulverse API key is required"));
    }
    let verified = state
        .mulverse_client
        .verify_orbis_key(request.api_key.trim())
        .await
        .map_err(map_mulverse_error)?;
    if !verified.connected {
        return Err(ApiError::unauthorized("Mulverse API key is not active"));
    }
    let encrypted_key = encrypt_key(request.api_key.trim())?;
    let mut secrets = HashMap::new();
    secrets.insert(SECRET_NAME.to_owned(), encrypted_key);
    state
        .integration_service
        .connect_app(
            &ctx,
            ctx.tenant_id,
            PROVIDER_NAME.to_owned(),
            "platform".to_owned(),
            "production".to_owned(),
            serde_json::json!({
                "organization_id": verified.organization_id,
                "product_organization_id": verified.product_organization_id
            }),
            secrets,
        )
        .await
        .map_err(ApiError::from)?;
    Ok(ApiResponse::ok(MulverseConnectionDto {
        connected: true,
        organization_id: Some(verified.organization_id),
        product_organization_id: Some(verified.product_organization_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/api/v1/orbis-mobile",
    responses((status = 204)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn disconnect(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    ensure_write_access(&state, &ctx).await?;
    state
        .integration_service
        .delete_integration(&ctx, ctx.tenant_id, PROVIDER_NAME)
        .await
        .map_err(ApiError::from)?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(
    get,
    path = "/api/v1/orbis-mobile/deployments",
    responses((status = 200, body = Vec<DeploymentDto>)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn list_deployments(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DeploymentDto>>>, ApiError> {
    ensure_write_access(&state, &ctx).await?;
    let (_, key) = require_connection(&state, ctx.tenant_id).await?;
    let deployments = state
        .mulverse_client
        .list_orbis_deployments(&key)
        .await
        .map_err(map_mulverse_error)?;
    Ok(ApiResponse::ok(
        deployments.into_iter().map(Into::into).collect(),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/orbis-mobile/deployments",
    request_body = CreateDeploymentRequest,
    responses((status = 201, body = DeploymentDto)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn create_deployment(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<CreateDeploymentRequest>,
) -> Result<(axum::http::StatusCode, Json<ApiResponse<DeploymentDto>>), ApiError> {
    ensure_write_access(&state, &ctx).await?;
    let (_, key) = require_connection(&state, ctx.tenant_id).await?;
    let deployment = state
        .mulverse_client
        .create_orbis_deployment(
            &key,
            &CreateOrbisDeploymentRequest {
                name: request.name,
                environment: request.environment,
            },
        )
        .await
        .map_err(map_mulverse_error)?;
    Ok((
        axum::http::StatusCode::CREATED,
        ApiResponse::ok(deployment.into()),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/orbis-mobile/deployments/{id}",
    request_body = UpdateDeploymentRequest,
    params(("id" = String, Path, description = "Deployment ID")),
    responses((status = 200, body = DeploymentDto)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn update_deployment(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateDeploymentRequest>,
) -> Result<Json<ApiResponse<DeploymentDto>>, ApiError> {
    ensure_write_access(&state, &ctx).await?;
    let (_, key) = require_connection(&state, ctx.tenant_id).await?;
    let deployment = state
        .mulverse_client
        .update_orbis_deployment(
            &key,
            &id,
            &UpdateOrbisDeploymentRequest {
                name: request.name,
                environment: request.environment,
            },
        )
        .await
        .map_err(map_mulverse_error)?;
    Ok(ApiResponse::ok(deployment.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/orbis-mobile/deployments/{id}/endpoints",
    params(("id" = String, Path, description = "Deployment ID")),
    responses((status = 200, body = Vec<EndpointDto>)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn list_endpoints(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Vec<EndpointDto>>>, ApiError> {
    ensure_write_access(&state, &ctx).await?;
    let (_, key) = require_connection(&state, ctx.tenant_id).await?;
    let endpoints = state
        .mulverse_client
        .list_orbis_endpoints(&key, &id)
        .await
        .map_err(map_mulverse_error)?;
    Ok(ApiResponse::ok(
        endpoints.into_iter().map(Into::into).collect(),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/orbis-mobile/deployments/{id}/endpoints",
    params(("id" = String, Path, description = "Deployment ID")),
    request_body = CreateEndpointRequest,
    responses((status = 201, body = EndpointDto)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn create_endpoint(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateEndpointRequest>,
) -> Result<(axum::http::StatusCode, Json<ApiResponse<EndpointDto>>), ApiError> {
    ensure_write_access(&state, &ctx).await?;
    if !matches!(request.network_scope.as_str(), "public" | "local_network") {
        return Err(ApiError::bad_request("Unsupported endpoint network scope"));
    }
    let (_, key) = require_connection(&state, ctx.tenant_id).await?;
    let endpoint = state
        .mulverse_client
        .create_orbis_endpoint(
            &key,
            &id,
            &CreateOrbisEndpointRequest {
                url: request.url,
                network_scope: request.network_scope,
            },
        )
        .await
        .map_err(map_mulverse_error)?;
    Ok((
        axum::http::StatusCode::CREATED,
        ApiResponse::ok(endpoint.into()),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/orbis-mobile/endpoints/{id}",
    params(("id" = String, Path, description = "Endpoint ID")),
    request_body = UpdateEndpointRequest,
    responses((status = 200, body = EndpointDto)),
    security(("bearer" = [])),
    tag = "Orbis Mobile"
)]
pub async fn update_endpoint(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateEndpointRequest>,
) -> Result<Json<ApiResponse<EndpointDto>>, ApiError> {
    ensure_write_access(&state, &ctx).await?;
    if let Some(status) = request.status.as_deref()
        && !matches!(status, "active" | "pending" | "disabled")
    {
        return Err(ApiError::bad_request(
            "Organizations may activate, disable, or re-pend endpoints; only Mulverse staff can revoke them",
        ));
    }
    let (_, key) = require_connection(&state, ctx.tenant_id).await?;
    let endpoint = state
        .mulverse_client
        .update_orbis_endpoint(
            &key,
            &id,
            &UpdateOrbisEndpointRequest {
                url: request.url,
                status: request.status,
            },
        )
        .await
        .map_err(map_mulverse_error)?;
    Ok(ApiResponse::ok(endpoint.into()))
}

async fn ensure_write_access(state: &AppState, ctx: &AuthContext) -> Result<(), ApiError> {
    state
        .permission_checker
        .check(ctx, Permission::OrganizationWrite)
        .await
        .map_err(ApiError::from)
}

async fn get_connection(
    state: &AppState,
    organization_id: Uuid,
) -> Result<
    Option<(
        orbis_domain::entities::integration::IntegrationConfig,
        String,
    )>,
    ApiError,
> {
    let Some(config) = state
        .integration_service
        .get_integration_by_provider(organization_id, PROVIDER_NAME)
        .await
        .map_err(ApiError::from)?
    else {
        return Ok(None);
    };
    let secrets = state
        .integration_repo
        .get_secrets_by_config(config.id)
        .await
        .map_err(ApiError::from)?;
    let encrypted_key = secrets
        .into_iter()
        .find(|secret| secret.key_name == SECRET_NAME)
        .map(|secret| secret.secret_key)
        .ok_or_else(|| ApiError::internal("Mulverse connection key is missing"))?;
    Ok(Some((config, decrypt_key(&encrypted_key)?)))
}

async fn require_connection(state: &AppState, org_id: Uuid) -> Result<(Uuid, String), ApiError> {
    get_connection(state, org_id)
        .await?
        .map(|(config, key)| (config.id, key))
        .ok_or_else(|| ApiError::conflict("Connect this institution to Mulverse first"))
}

fn encryption_key() -> Result<aes_gcm::Aes256Gcm, ApiError> {
    use aes_gcm::KeyInit;

    let encoded = std::env::var("MULVERSE_ORBIS_ENCRYPTION_KEY").map_err(|_| {
        ApiError::internal("MULVERSE_ORBIS_ENCRYPTION_KEY must be configured to store the key")
    })?;
    let bytes = hex::decode(encoded.trim()).map_err(|_| {
        ApiError::internal("Mulverse encryption key must be 64 hexadecimal characters")
    })?;
    if bytes.len() != 32 {
        return Err(ApiError::internal(
            "Mulverse encryption key must be exactly 32 bytes",
        ));
    }
    aes_gcm::Aes256Gcm::new_from_slice(&bytes)
        .map_err(|_| ApiError::internal("Invalid Mulverse encryption key"))
}

fn encrypt_key(api_key: &str) -> Result<String, ApiError> {
    use aes_gcm::{
        AeadCore,
        aead::{Aead, OsRng},
    };

    let cipher = encryption_key()?;
    let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, api_key.as_bytes())
        .map_err(|_| ApiError::internal("Could not encrypt the Mulverse API key"))?;
    Ok(format!(
        "v1:{}:{}",
        hex::encode(nonce),
        hex::encode(ciphertext)
    ))
}

fn decrypt_key(stored: &str) -> Result<String, ApiError> {
    use aes_gcm::aead::Aead;

    let mut parts = stored.split(':');
    if parts.next() != Some("v1") {
        return Err(ApiError::internal(
            "Stored Mulverse key has an unsupported format",
        ));
    }
    let nonce = hex::decode(parts.next().unwrap_or_default())
        .map_err(|_| ApiError::internal("Stored Mulverse key nonce is invalid"))?;
    let ciphertext = hex::decode(parts.next().unwrap_or_default())
        .map_err(|_| ApiError::internal("Stored Mulverse key ciphertext is invalid"))?;
    if parts.next().is_some() || nonce.len() != 12 {
        return Err(ApiError::internal("Stored Mulverse key is malformed"));
    }
    let cipher = encryption_key()?;
    let plaintext = cipher
        .decrypt(aes_gcm::Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| ApiError::internal("Could not decrypt the stored Mulverse API key"))?;
    String::from_utf8(plaintext)
        .map_err(|_| ApiError::internal("Stored Mulverse key is not valid UTF-8"))
}

fn map_mulverse_error(error: MulverseApiError) -> ApiError {
    match error {
        MulverseApiError::ValidationError(message) => ApiError::bad_request(message),
        MulverseApiError::Forbidden(message) => ApiError::forbidden(message),
        MulverseApiError::NotFound(message) => ApiError::not_found(message),
        MulverseApiError::Conflict(message) => ApiError::conflict(message),
        MulverseApiError::NetworkError(message) => {
            tracing::warn!(error = %message, "Mulverse is unavailable");
            ApiError::internal("Mulverse is currently unavailable")
        }
        MulverseApiError::RateLimited => ApiError::internal("Mulverse rate limit was reached"),
        MulverseApiError::ServerError(message)
        | MulverseApiError::ParseError(message)
        | MulverseApiError::UnexpectedStatus(_, message) => {
            tracing::warn!(error = %message, "Mulverse request failed");
            ApiError::internal("Mulverse request failed")
        }
    }
}

impl From<OrbisDeployment> for DeploymentDto {
    fn from(value: OrbisDeployment) -> Self {
        Self {
            id: value.id,
            name: value.name,
            environment: value.environment,
            status: value.status,
            verification_status: value.verification_status,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<OrbisEndpoint> for EndpointDto {
    fn from(value: OrbisEndpoint) -> Self {
        Self {
            id: value.id,
            deployment_id: value.deployment_id,
            url: value.url,
            network_scope: value.network_scope,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
