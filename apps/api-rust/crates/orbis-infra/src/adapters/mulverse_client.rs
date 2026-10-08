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
//! - `GET  /api/v1/public/updates/check`      — Check for updates

use orbis_domain::entities::license::{
    ActivationRequest, ActivationResponse, DeactivationRequest, PublicKeyInfo, RefreshRequest,
    UpdateCheckResponse,
};
use std::time::Duration;
use tracing::{debug, warn};

/// Default Mulverse Platform base URL.
const DEFAULT_BASE_URL: &str = "https://platform.mulverse.com";

/// HTTP request timeout.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

/// Maximum retry attempts for transient failures.
const MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (milliseconds).
const BASE_BACKOFF_MS: u64 = 1000;

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

    /// Activate a product key with Mulverse Platform.
    ///
    /// `POST /api/v1/public/licenses/activate`
    pub async fn activate(
        &self,
        request: &ActivationRequest,
    ) -> Result<ActivationResponse, MulverseApiError> {
        let url = format!("{}/api/v1/public/licenses/activate", self.base_url);
        debug!(product_key_prefix = %&request.product_key[..10], "Activating with Mulverse Platform");

        let response = self
            .post_with_retry(&url, request)
            .await?;

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

        let response = self
            .post_with_retry(&url, request)
            .await?;

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

    /// Deactivate an installation (release device seat).
    ///
    /// `POST /api/v1/public/licenses/deactivate`
    pub async fn deactivate(
        &self,
        request: &DeactivationRequest,
    ) -> Result<(), MulverseApiError> {
        let url = format!("{}/api/v1/public/licenses/deactivate", self.base_url);
        debug!(license_id = %request.license_id, "Deactivating with Mulverse Platform");

        let response = self
            .post_with_retry(&url, request)
            .await?;

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

        let response = self
            .get_with_retry(&url)
            .await?;

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

    /// Check for application updates.
    ///
    /// `GET /api/v1/public/updates/check`
    pub async fn check_updates(
        &self,
        product_key: &str,
        channel: &str,
        current_version: &str,
        platform: &str,
        architecture: &str,
    ) -> Result<UpdateCheckResponse, MulverseApiError> {
        let url = format!(
            "{}/api/v1/public/updates/check?product_key={}&channel={}&current_version={}&platform={}&architecture={}",
            self.base_url, product_key, channel, current_version, platform, architecture
        );
        debug!(version = %current_version, "Checking for updates with Mulverse Platform");

        let response = self
            .get_with_retry(&url)
            .await?;

        if response.status().is_success() {
            let body = response
                .json::<UpdateCheckResponse>()
                .await
                .map_err(|e| MulverseApiError::ParseError(e.to_string()))?;
            Ok(body)
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
    async fn get_with_retry(
        &self,
        url: &str,
    ) -> Result<reqwest::Response, MulverseApiError> {
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
        let body = response.text().await.unwrap_or_default();

        match status.as_u16() {
            400 => MulverseApiError::ValidationError(body),
            403 => MulverseApiError::Forbidden(body),
            404 => MulverseApiError::NotFound(body),
            409 => MulverseApiError::SeatLimitExceeded(body),
            429 => MulverseApiError::RateLimited,
            500..=599 => MulverseApiError::ServerError(body),
            _ => MulverseApiError::UnexpectedStatus(status.as_u16(), body),
        }
    }
}

/// Calculate exponential backoff delay with jitter.
fn backoff_with_jitter(attempt: u32) -> Duration {
    let base = BASE_BACKOFF_MS * 2u64.pow(attempt);
    // Add jitter: random value between 0 and base
    let jitter = rand_jitter(base);
    Duration::from_millis(base + jitter)
}

/// Simple jitter using a basic approach (no external RNG dependency for this).
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

/// Errors from the Mulverse Platform API.
///
/// These map directly to the error codes defined in the integration contract.
/// Network failure is distinct from API errors — network failure must NOT
/// be treated as a license failure.
#[derive(Debug, Clone)]
pub enum MulverseApiError {
    /// Network/connectivity error — Mulverse is unreachable.
    /// This must NOT be treated as a license failure.
    NetworkError(String),
    /// 400 — Validation error (e.g. empty installation_id).
    ValidationError(String),
    /// 403 — Forbidden (key/license revoked or expired).
    Forbidden(String),
    /// 404 — Not found (key not found).
    NotFound(String),
    /// 409 — Seat limit exceeded.
    SeatLimitExceeded(String),
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
            Self::SeatLimitExceeded(e) => write!(f, "Seat limit exceeded: {e}"),
            Self::RateLimited => write!(f, "Rate limited by Mulverse Platform"),
            Self::ServerError(e) => write!(f, "Mulverse server error: {e}"),
            Self::ParseError(e) => write!(f, "Response parse error: {e}"),
            Self::UnexpectedStatus(code, e) => write!(f, "Unexpected status {code}: {e}"),
        }
    }
}

impl MulverseApiError {
    /// Whether this error is a transient network issue (not a license problem).
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::NetworkError(_) | Self::RateLimited | Self::ServerError(_))
    }

    /// Whether this error indicates the license/key is invalid or revoked.
    pub fn is_license_error(&self) -> bool {
        matches!(self, Self::Forbidden(_) | Self::NotFound(_) | Self::SeatLimitExceeded(_))
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
            MulverseApiError::SeatLimitExceeded(e) => {
                orbis_domain::errors::DomainError::Conflict(format!("Seat limit: {e}"))
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
