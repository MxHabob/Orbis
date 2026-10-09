//! Mulverse Platform HTTP client — typed integration boundary.
//!
//! This is the ONLY place in Orbis that communicates with Mulverse Platform.
//! All Mulverse HTTP calls are centralized here.
//!
//! # Endpoints (from ORBIS_INTEGRATION_CONTRACT.md)
//!
//! - `POST /api/v1/public/licenses/activate`  — Activate a product key
//! - `POST /api/v1/public/licenses/refresh`   — Periodic heartbeat/refresh
//! - `POST /api/v1/public/licenses/deactivate` — Release a device seat
//! - `GET  /api/v1/public/keys/active`        — Fetch active signing key
//! - `GET  /api/v1/public/updates/tauri`      — Tauri Updater endpoint

use orbis_domain::entities::license::{
    ActivationRequest, ActivationResponse, DeactivationRequest, MULVERSE_PRODUCT_KEY,
    PublicKeyInfo, RefreshRequest, SignedEntitlement, TauriUpdateManifest,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::time::Duration;
use tracing::{debug, warn};

/// Default Mulverse Platform base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.mulverse.com";

/// HTTP request timeout.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

/// Maximum retry attempts for transient failures.
const MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (milliseconds).
const BASE_BACKOFF_MS: u64 = 1000;

#[derive(Debug, Deserialize)]
pub struct TrialChallenge {
    pub challenge_id: String,
    pub challenge: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterTrialRequest {
    pub device_public_key: String,
    pub device_signature: String,
    pub challenge_id: String,
    pub challenge: String,
    pub installation_id: String,
    pub platform: String,
    pub app_version: String,
    pub machine_label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RefreshTrialRequest {
    pub trial_id: String,
    pub device_public_key: String,
    pub device_signature: String,
    pub challenge_id: String,
    pub challenge: String,
    pub installation_id: String,
    pub app_version: String,
}

#[derive(Debug, Deserialize)]
pub struct TrialDevice {
    pub id: String,
    pub status: String,
    pub installation_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TrialRegistrationResponse {
    pub trial: TrialDevice,
    pub signed_entitlement: SignedEntitlement,
}

#[derive(Deserialize)]
struct ApiErrorEnvelope {
    error: ApiErrorDetails,
}

#[derive(Deserialize)]
struct ApiErrorDetails {
    code: Option<String>,
    message: Option<String>,
}

fn format_api_error_body(body: &str) -> String {
    if let Ok(envelope) = serde_json::from_str::<ApiErrorEnvelope>(body) {
        let message = envelope.error.message.unwrap_or_default();
        return match (envelope.error.code, message.is_empty()) {
            (Some(code), false) => format!("{code}: {message}"),
            (Some(code), true) => code,
            (None, false) => message,
            (None, true) => body.to_string(),
        };
    }

    body.to_string()
}

/// Mulverse Platform HTTP client.
///
/// Centralized integration boundary — no Mulverse HTTP calls should exist
/// outside this module. Uses exponential backoff with jitter for resilience.
#[derive(Clone)]
pub struct MulverseClient {
    base_url: String,
    client: reqwest::Client,
}

impl MulverseClient {
    /// Create a new Mulverse client with the default production URL.
    pub fn new() -> Result<Self, String> {
        Self::with_base_url(DEFAULT_BASE_URL)
    }

    /// Create a Mulverse client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .user_agent("Orbis-Desktop/1.0")
            .timeout(REQUEST_TIMEOUT)
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| format!("Failed to create Mulverse HTTP client: {e}"))?;

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        })
    }

    /// Base URL accessor.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Activate a product key with Mulverse Platform.
    ///
    /// `POST /api/v1/public/licenses/activate`
    pub async fn activate(
        &self,
        request: &ActivationRequest,
    ) -> Result<ActivationResponse, MulverseApiError> {
        let url = format!("{}/api/v1/public/licenses/activate", self.base_url);
        let prefix = if request.product_key.len() >= 10 {
            &request.product_key[..10]
        } else {
            &request.product_key
        };
        debug!(product_key_prefix = %prefix, "Activating with Mulverse Platform");

        let response = self.post_with_retry(&url, request).await?;

        if response.status() == reqwest::StatusCode::CREATED
            || response.status() == reqwest::StatusCode::OK
        {
            let body = response
                .json::<ActivationResponse>()
                .await
                .map_err(|e| MulverseApiError::ParseError(e.to_string()))?;
            Ok(body)
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    /// Refresh the license (periodic heartbeat).
    ///
    /// `POST /api/v1/public/licenses/refresh`
    pub async fn refresh(
        &self,
        request: &RefreshRequest,
    ) -> Result<ActivationResponse, MulverseApiError> {
        let url = format!("{}/api/v1/public/licenses/refresh", self.base_url);
        debug!(license_id = %request.license_id, "Refreshing license with Mulverse Platform");

        let response = self.post_with_retry(&url, request).await?;

        if response.status().is_success() {
            let body = response
                .json::<ActivationResponse>()
                .await
                .map_err(|e| MulverseApiError::ParseError(e.to_string()))?;
            Ok(body)
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    pub async fn create_trial_challenge(&self) -> Result<TrialChallenge, MulverseApiError> {
        self.post_once(
            &format!("{}/api/v1/public/trials/challenge", self.base_url),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn register_trial(
        &self,
        request: &RegisterTrialRequest,
    ) -> Result<TrialRegistrationResponse, MulverseApiError> {
        self.post_once(
            &format!("{}/api/v1/public/trials/register", self.base_url),
            request,
        )
        .await
    }

    pub async fn refresh_trial(
        &self,
        request: &RefreshTrialRequest,
    ) -> Result<TrialRegistrationResponse, MulverseApiError> {
        self.post_once(
            &format!("{}/api/v1/public/trials/refresh", self.base_url),
            request,
        )
        .await
    }

    /// Deactivate an installation (release device seat).
    ///
    /// `POST /api/v1/public/licenses/deactivate`
    pub async fn deactivate(&self, request: &DeactivationRequest) -> Result<(), MulverseApiError> {
        let url = format!("{}/api/v1/public/licenses/deactivate", self.base_url);
        debug!(license_id = %request.license_id, "Deactivating with Mulverse Platform");

        let response = self.post_with_retry(&url, request).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    /// Fetch the active signing public key for key rotation.
    ///
    /// `GET /api/v1/public/keys/active`
    pub async fn get_active_key(&self) -> Result<PublicKeyInfo, MulverseApiError> {
        let url = format!("{}/api/v1/public/keys/active", self.base_url);
        debug!("Fetching active signing key from Mulverse Platform");

        let response = self.get_with_retry(&url).await?;

        if response.status().is_success() {
            let body = response
                .json::<PublicKeyInfo>()
                .await
                .map_err(|e| MulverseApiError::ParseError(e.to_string()))?;
            Ok(body)
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    /// Check for Tauri updates via the official Tauri updater endpoint.
    ///
    /// `GET /api/v1/public/updates/tauri`
    pub async fn check_tauri_updates(
        &self,
        channel: &str,
        current_version: &str,
        target: &str,
        arch: &str,
    ) -> Result<Option<TauriUpdateManifest>, MulverseApiError> {
        let mut url =
            reqwest::Url::parse(&format!("{}/api/v1/public/updates/tauri", self.base_url))
                .map_err(|e| MulverseApiError::ParseError(e.to_string()))?;
        url.query_pairs_mut()
            .append_pair("product_key", MULVERSE_PRODUCT_KEY)
            .append_pair("channel", channel)
            .append_pair("current_version", current_version)
            .append_pair("target", target)
            .append_pair("arch", arch);
        let url = url.to_string();
        debug!(version = %current_version, target = %target, arch = %arch, "Checking for updates with Mulverse Platform");

        let response = self.get_with_retry(&url).await?;

        if response.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(None);
        }

        if response.status().is_success() {
            let body = response
                .json::<TauriUpdateManifest>()
                .await
                .map_err(|e| MulverseApiError::ParseError(e.to_string()))?;
            Ok(Some(body))
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    /// Check if the Mulverse Platform is reachable.
    pub async fn is_reachable(&self) -> bool {
        let url = format!("{}/api/v1/public/keys/active", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    // ── Internal helpers ─────────────────────────────────────────

    async fn post_once<T, R>(&self, url: &str, body: &T) -> Result<R, MulverseApiError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let response = self
            .client
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|error| MulverseApiError::NetworkError(error.to_string()))?;
        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|error| MulverseApiError::ParseError(error.to_string()))
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    /// POST with exponential backoff and jitter for transient failures.
    async fn post_with_retry<T: serde::Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<reqwest::Response, MulverseApiError> {
        let mut last_error = None;

        for attempt in 0..MAX_RETRIES {
            match self.client.post(url).json(body).send().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if e.is_timeout() || e.is_connect() {
                        let delay = backoff_with_jitter(attempt);
                        warn!(
                            attempt = attempt + 1,
                            max = MAX_RETRIES,
                            delay_ms = delay.as_millis() as u64,
                            error = %e,
                            "Mulverse request failed, retrying"
                        );
                        tokio::time::sleep(delay).await;
                        last_error = Some(e);
                    } else {
                        return Err(MulverseApiError::NetworkError(e.to_string()));
                    }
                }
            }
        }

        Err(MulverseApiError::NetworkError(
            last_error
                .map(|e| e.to_string())
                .unwrap_or_else(|| "Max retries exceeded".to_string()),
        ))
    }

    /// GET with exponential backoff and jitter.
    async fn get_with_retry(&self, url: &str) -> Result<reqwest::Response, MulverseApiError> {
        let mut last_error = None;

        for attempt in 0..MAX_RETRIES {
            match self.client.get(url).send().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if e.is_timeout() || e.is_connect() {
                        let delay = backoff_with_jitter(attempt);
                        warn!(
                            attempt = attempt + 1,
                            max = MAX_RETRIES,
                            delay_ms = delay.as_millis() as u64,
                            error = %e,
                            "Mulverse request failed, retrying"
                        );
                        tokio::time::sleep(delay).await;
                        last_error = Some(e);
                    } else {
                        return Err(MulverseApiError::NetworkError(e.to_string()));
                    }
                }
            }
        }

        Err(MulverseApiError::NetworkError(
            last_error
                .map(|e| e.to_string())
                .unwrap_or_else(|| "Max retries exceeded".to_string()),
        ))
    }

    /// Parse a non-success HTTP response into a typed error.
    async fn parse_error_response(&self, response: reqwest::Response) -> MulverseApiError {
        let status = response.status();
        let body = match response.text().await {
            Ok(body) => format_api_error_body(&body),
            Err(error) => {
                return MulverseApiError::ParseError(format!(
                    "HTTP {status} error response could not be read: {error}"
                ));
            }
        };

        match status.as_u16() {
            400 => MulverseApiError::ValidationError(body),
            403 => MulverseApiError::Forbidden(body),
            404 => MulverseApiError::NotFound(body),
            409 => MulverseApiError::Conflict(body),
            429 => MulverseApiError::RateLimited,
            500..=599 => MulverseApiError::ServerError(body),
            _ => MulverseApiError::UnexpectedStatus(status.as_u16(), body),
        }
    }
}

/// Calculate exponential backoff delay with jitter.
fn backoff_with_jitter(attempt: u32) -> Duration {
    let base = BASE_BACKOFF_MS * 2u64.pow(attempt);
    let jitter = rand_jitter(base);
    Duration::from_millis(base + jitter)
}

/// Simple jitter without external RNG dependency.
fn rand_jitter(max_ms: u64) -> u64 {
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    seed % (max_ms.max(1))
}

// ═══════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum MulverseApiError {
    /// Network/connectivity error — Mulverse is unreachable.
    NetworkError(String),
    /// 400 — Validation error.
    ValidationError(String),
    /// 403 — Forbidden (key/license revoked or expired).
    Forbidden(String),
    /// 404 — Not found (key not found).
    NotFound(String),
    /// 409 — Conflict, including seat limits and license-key assignment errors.
    Conflict(String),
    /// 429 — Rate limited.
    RateLimited,
    /// 5xx — Server error.
    ServerError(String),
    /// Response body could not be parsed.
    ParseError(String),
    /// Unexpected HTTP status code.
    UnexpectedStatus(u16, String),
}

impl std::fmt::Display for MulverseApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "Network error: {e}"),
            Self::ValidationError(e) => write!(f, "Validation error: {e}"),
            Self::Forbidden(e) => write!(f, "Forbidden: {e}"),
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::Conflict(e) => write!(f, "Conflict: {e}"),
            Self::RateLimited => write!(f, "Rate limited by Mulverse Platform"),
            Self::ServerError(e) => write!(f, "Mulverse server error: {e}"),
            Self::ParseError(e) => write!(f, "Response parse error: {e}"),
            Self::UnexpectedStatus(code, e) => write!(f, "Unexpected status {code}: {e}"),
        }
    }
}

impl MulverseApiError {
    pub fn is_network_error(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::RateLimited | Self::ServerError(_)
        )
    }

    pub fn is_license_error(&self) -> bool {
        matches!(
            self,
            Self::Forbidden(_) | Self::NotFound(_) | Self::Conflict(_)
        )
    }
}

impl From<MulverseApiError> for orbis_domain::errors::DomainError {
    fn from(err: MulverseApiError) -> Self {
        match err {
            MulverseApiError::NetworkError(e) => {
                orbis_domain::errors::DomainError::Internal(format!("Mulverse unreachable: {e}"))
            }
            MulverseApiError::ValidationError(e) => {
                orbis_domain::errors::DomainError::Validation(format!("Mulverse: {e}"))
            }
            MulverseApiError::Forbidden(e) => {
                orbis_domain::errors::DomainError::Forbidden(format!("License: {e}"))
            }
            MulverseApiError::NotFound(e) => {
                orbis_domain::errors::DomainError::NotFound(format!("License: {e}"))
            }
            MulverseApiError::Conflict(e) => {
                orbis_domain::errors::DomainError::Conflict(format!("Mulverse: {e}"))
            }
            MulverseApiError::RateLimited => {
                orbis_domain::errors::DomainError::Internal("Mulverse rate limited".to_string())
            }
            MulverseApiError::ServerError(e) => {
                orbis_domain::errors::DomainError::Internal(format!("Mulverse server: {e}"))
            }
            MulverseApiError::ParseError(e) => {
                orbis_domain::errors::DomainError::Internal(format!("Mulverse parse: {e}"))
            }
            MulverseApiError::UnexpectedStatus(code, e) => {
                orbis_domain::errors::DomainError::Internal(format!("Mulverse {code}: {e}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_api_error_body;

    #[test]
    fn api_error_envelope_displays_message_instead_of_raw_json() {
        assert_eq!(
            format_api_error_body(
                r#"{"error":{"code":"CONFLICT","message":"No license is assigned to this product key"}}"#
            ),
            "CONFLICT: No license is assigned to this product key"
        );
    }

    #[test]
    fn api_error_body_falls_back_to_plain_text() {
        assert_eq!(
            format_api_error_body("upstream unavailable"),
            "upstream unavailable"
        );
    }
}
