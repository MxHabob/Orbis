//! License domain — Mulverse Platform integration types and license state machine.
//!
//! This module defines the core types for Orbis ↔ Mulverse Platform licensing.
//! Mulverse Platform is the authoritative commercial licensing system.
//! Orbis consumes and verifies signed entitlement documents locally.
//!
//! # Architecture
//!
//! ```text
//! MULVERSE PLATFORM (authority)
//!        │
//!  SignedEntitlement (Ed25519)
//!        │
//!  ORBIS LICENSE RUNTIME (consumer)
//!        │
//!  ┌─────┼──────┐
//!  │     │      │
//! State  Entitlements  Limits
//! ```
//!
//! The private signing key NEVER exists in Orbis.
//! Only the Ed25519 public verification key is embedded.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════
// PRODUCT IDENTITY
// ═══════════════════════════════════════════════════════════════

/// Canonical Mulverse product stable key for Orbis.
pub const MULVERSE_PRODUCT_KEY: &str = "ORBIS";

/// Product key prefix used by Mulverse for Orbis product keys.
pub const PRODUCT_KEY_PREFIX: &str = "ORBIS-";

/// Number of segments in a Mulverse product key (e.g. `ORBIS-XXXXX-XXXXX-XXXXX-XXXXX`).
pub const PRODUCT_KEY_SEGMENTS: usize = 5;

/// Characters per segment in a Mulverse product key.
pub const PRODUCT_KEY_SEGMENT_LENGTH: usize = 5;

// ═══════════════════════════════════════════════════════════════
// TRIAL POLICY
// ═══════════════════════════════════════════════════════════════

/// Centralized trial policy — configurable evaluation period settings.
///
/// Do NOT scatter trial durations throughout Orbis.
/// All trial-related decisions must read from this policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialPolicy {
    /// Duration of the trial period in days.
    pub duration_days: u32,
    /// Maximum users allowed during trial.
    pub max_users: Option<u32>,
    /// Maximum students allowed during trial.
    pub max_students: Option<u32>,
    /// Features available during trial.
    pub trial_features: Vec<String>,
}

impl Default for TrialPolicy {
    fn default() -> Self {
        Self {
            duration_days: 30,
            max_users: Some(10),
            max_students: Some(100),
            trial_features: vec!["students".to_string()],
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// SIGNED ENTITLEMENT — Mulverse Platform Cryptographic Artifact
// ═══════════════════════════════════════════════════════════════

/// The top-level signed entitlement envelope issued by Mulverse Platform.
///
/// This is the offline-verifiable cryptographic document.
/// Orbis verifies `sig` against the canonical JSON of `payload`
/// using the Ed25519 public key identified by `kid`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedEntitlement {
    /// Schema version (currently 1).
    pub v: u32,
    /// Key identifier for signature rotation (e.g. "key_ed25519_2026_v1").
    pub kid: String,
    /// Algorithm identifier (always "Ed25519").
    pub alg: String,
    /// The entitlement payload data.
    pub payload: EntitlementPayload,
    /// Base64-encoded Ed25519 signature over the canonical JSON of `payload`.
    pub sig: String,
}

/// The entitlement payload — the data signed by Mulverse Platform.
///
/// This is the source of truth for license state, entitlements, and limits.
/// Orbis must never modify this payload — it is verified cryptographically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementPayload {
    /// License UUID from Mulverse.
    pub license_id: String,
    /// The product key used for activation (e.g. "ORBIS-ABCDE-FGHIJ-KLMNO-PQRST").
    pub product_key: String,
    /// Mulverse product UUID.
    pub product_id: String,
    /// Organization UUID that owns this license.
    pub organization_id: String,
    /// Edition key (e.g. "SCHOOL", "UNIVERSITY").
    pub edition_key: String,
    /// Plan key (e.g. "BASIC", "PRO", "ENTERPRISE").
    pub plan_key: String,
    /// Boolean feature capabilities.
    pub entitlements: Vec<EntitlementEntry>,
    /// Quantitative limits.
    pub limits: Vec<LimitEntry>,
    /// Activation UUID for this specific installation.
    pub activation_id: String,
    /// Installation identity this entitlement is bound to.
    pub installation_id: String,
    /// When this entitlement was issued.
    pub issued_at: DateTime<Utc>,
    /// When the license validity begins.
    pub valid_from: DateTime<Utc>,
    /// When the license expires. `None` = perpetual.
    pub valid_until: Option<DateTime<Utc>>,
    /// Whether offline operation is allowed.
    pub offline_allowed: bool,
    /// Days the app can operate without contacting Mulverse.
    pub offline_grace_period_days: u32,
    /// How often (hours) the app should verify/refresh online.
    pub verification_interval_hours: u32,
}

/// A boolean entitlement entry from the Mulverse catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementEntry {
    /// Canonical entitlement key (e.g. "students", "ai_grading", "multi_branch").
    pub key: String,
    /// Whether this capability is granted.
    pub granted: bool,
}

/// A quantitative limit entry from the Mulverse catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitEntry {
    /// Canonical limit key (e.g. "students.max", "users.max", "branches.max").
    pub key: String,
    /// Maximum allowed value.
    pub max_value: u64,
}

// ═══════════════════════════════════════════════════════════════
// ACTIVATION — Mulverse API DTOs
// ═══════════════════════════════════════════════════════════════

/// Request to activate a product key with Mulverse Platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationRequest {
    pub product_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
    pub installation_id: String,
    pub platform: String,
    pub app_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_label: Option<String>,
}

/// Response from Mulverse Platform activation endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationResponse {
    pub activation: ActivationInfo,
    pub signed_entitlement: SignedEntitlement,
}

/// Activation record from Mulverse Platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationInfo {
    pub id: String,
    pub license_id: String,
    pub product_id: String,
    pub organization_id: String,
    pub installation_id: String,
    pub platform: String,
    pub app_version: String,
    pub status: String,
    pub activated_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub machine_label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to refresh/heartbeat with Mulverse Platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub license_id: String,
    pub installation_id: String,
    pub app_version: String,
}

/// Request to deactivate an installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeactivationRequest {
    pub license_id: String,
    pub installation_id: String,
}

/// Public key metadata from Mulverse key rotation endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyInfo {
    pub kid: String,
    pub algorithm: String,
    pub public_key_b64: String,
}

/// Update check response from Mulverse.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResponse {
    pub update_available: bool,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub release_notes: Option<String>,
    pub artifact: Option<UpdateArtifact>,
    pub min_upgrade_version: Option<String>,
    pub published_at: Option<String>,
}

/// Update artifact metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArtifact {
    pub file_name: String,
    pub download_url: String,
    pub checksum_sha256: String,
    pub signature: Option<String>,
    pub file_size_bytes: u64,
}

// ═══════════════════════════════════════════════════════════════
// LICENSE STATE MACHINE
// ═══════════════════════════════════════════════════════════════

/// The runtime license state — evaluated locally by the Orbis license runtime.
///
/// State transitions:
/// ```text
///                   ┌──────────────┐
///            ┌──────│ UNAVAILABLE  │──────┐
///            │      └──────────────┘      │
///            │ (first run, no key)        │ (key provided)
///            ▼                            ▼
///     ┌────────────┐              ┌────────────┐
///     │   TRIAL    │───(expired)──│   ACTIVE   │
///     └────────────┘              └────────────┘
///            │                     │    │    │
///            │ (expired)           │    │    │ (grace period)
///            ▼                     │    │    ▼
///     ┌────────────┐               │    │  ┌──────────────┐
///     │  EXPIRED   │◄──(expired)───┘    │  │ OFFLINE_GRACE│
///     └────────────┘                    │  └──────────────┘
///                                       │         │
///                                       │ (revoked)│ (expired)
///                                       ▼         ▼
///                                ┌────────────┐
///                                │  REVOKED   │
///                                └────────────┘
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseState {
    /// No license or trial — initial state, activation screen shown.
    Unavailable,
    /// Trial period is active. No product key has been activated.
    Trial,
    /// License is active and valid. Signed entitlement verified.
    Active,
    /// License is active but nearing expiration (configurable threshold).
    Expiring,
    /// Operating offline within the allowed grace period.
    OfflineGrace,
    /// License or trial has expired.
    Expired,
    /// License has been revoked by Mulverse Platform.
    Revoked,
    /// License artifact is corrupt, tampered, or unverifiable.
    Invalid,
}

impl LicenseState {
    /// Whether the application should operate normally in this state.
    pub fn is_operational(&self) -> bool {
        matches!(
            self,
            LicenseState::Trial
                | LicenseState::Active
                | LicenseState::Expiring
                | LicenseState::OfflineGrace
        )
    }

    /// Whether the application should enter read-only / restricted mode.
    pub fn is_restricted(&self) -> bool {
        matches!(
            self,
            LicenseState::Expired | LicenseState::Revoked | LicenseState::Invalid
        )
    }

    /// Whether the activation screen should be shown.
    pub fn requires_activation(&self) -> bool {
        matches!(self, LicenseState::Unavailable)
    }
}

// ═══════════════════════════════════════════════════════════════
// LICENSE RUNTIME SNAPSHOT — Local cached state
// ═══════════════════════════════════════════════════════════════

/// The complete license runtime state persisted locally.
///
/// This is the local cache that allows Orbis to operate offline.
/// It includes the signed entitlement, sync metadata, and trial state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseSnapshot {
    /// Current evaluated license state.
    pub state: LicenseState,
    /// The signed entitlement from Mulverse (if activated).
    pub signed_entitlement: Option<SignedEntitlement>,
    /// Trial start timestamp (if in trial mode).
    pub trial_started_at: Option<DateTime<Utc>>,
    /// Trial policy applied.
    pub trial_policy: TrialPolicy,
    /// Last successful online sync with Mulverse.
    pub last_sync_at: Option<DateTime<Utc>>,
    /// Last known system time — for clock rollback detection.
    pub last_known_time: DateTime<Utc>,
    /// Installation identity for this Orbis instance.
    pub installation_id: String,
    /// Application version at last sync.
    pub app_version: String,
}

impl LicenseSnapshot {
    /// Create a new snapshot in the initial unavailable state.
    pub fn new_unavailable(installation_id: String, app_version: String) -> Self {
        Self {
            state: LicenseState::Unavailable,
            signed_entitlement: None,
            trial_started_at: None,
            trial_policy: TrialPolicy::default(),
            last_sync_at: None,
            last_known_time: Utc::now(),
            installation_id,
            app_version,
        }
    }

    /// Create a new snapshot in trial state.
    pub fn new_trial(installation_id: String, app_version: String, policy: TrialPolicy) -> Self {
        Self {
            state: LicenseState::Trial,
            signed_entitlement: None,
            trial_started_at: Some(Utc::now()),
            trial_policy: policy,
            last_sync_at: None,
            last_known_time: Utc::now(),
            installation_id,
            app_version,
        }
    }

    /// Days remaining in trial. Returns 0 if not in trial.
    pub fn trial_days_remaining(&self) -> i64 {
        match self.trial_started_at {
            Some(started) => {
                let end = started
                    + chrono::Duration::days(self.trial_policy.duration_days as i64);
                let remaining = end.signed_duration_since(Utc::now()).num_days();
                remaining.max(0)
            }
            None => 0,
        }
    }

    /// Whether the trial has expired.
    pub fn is_trial_expired(&self) -> bool {
        self.state == LicenseState::Trial && self.trial_days_remaining() == 0
    }

    /// Detect obvious clock rollback.
    ///
    /// If the current system time is significantly before the last known time,
    /// the clock has likely been rolled back. This is a heuristic — a desktop
    /// application cannot perfectly protect against a malicious admin with
    /// full OS access. The objective is robust commercial licensing, not DRM.
    pub fn detect_clock_rollback(&self) -> bool {
        let now = Utc::now();
        // Allow up to 5 minutes of drift (NTP corrections, DST, etc.)
        let threshold = chrono::Duration::minutes(5);
        now < (self.last_known_time - threshold)
    }

    /// Check if a boolean entitlement is granted.
    pub fn is_feature_enabled(&self, key: &str) -> bool {
        match &self.signed_entitlement {
            Some(se) => se
                .payload
                .entitlements
                .iter()
                .any(|e| e.key == key && e.granted),
            None => {
                // In trial, check trial features
                if self.state == LicenseState::Trial {
                    self.trial_policy.trial_features.iter().any(|f| f == key)
                } else {
                    false
                }
            }
        }
    }

    /// Get the limit value for a given limit key.
    /// Returns `None` if no limit is set (unlimited).
    pub fn get_limit(&self, key: &str) -> Option<u64> {
        match &self.signed_entitlement {
            Some(se) => se
                .payload
                .limits
                .iter()
                .find(|l| l.key == key)
                .map(|l| l.max_value),
            None => {
                // In trial, return trial limits
                if self.state == LicenseState::Trial {
                    match key {
                        "users.max" => self.trial_policy.max_users.map(|v| v as u64),
                        "students.max" => self.trial_policy.max_students.map(|v| v as u64),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Check if creating `amount` more of a resource would exceed the limit.
    /// Returns `Ok(())` if within limits, `Err(message)` if exceeded.
    pub fn check_limit(&self, key: &str, current_count: u64, amount: u64) -> Result<(), String> {
        match self.get_limit(key) {
            Some(max) => {
                let new_total = current_count.saturating_add(amount);
                if new_total > max {
                    Err(format!(
                        "License limit exceeded for '{}': current={}, adding={}, max={}",
                        key, current_count, amount, max
                    ))
                } else {
                    Ok(())
                }
            }
            None => Ok(()), // No limit = unlimited
        }
    }

    /// The edition key (e.g. "SCHOOL", "UNIVERSITY") from the active license.
    pub fn edition_key(&self) -> Option<&str> {
        self.signed_entitlement
            .as_ref()
            .map(|se| se.payload.edition_key.as_str())
    }

    /// The plan key (e.g. "BASIC", "PRO", "ENTERPRISE") from the active license.
    pub fn plan_key(&self) -> Option<&str> {
        self.signed_entitlement
            .as_ref()
            .map(|se| se.payload.plan_key.as_str())
    }

    /// The license ID from the active license.
    pub fn license_id(&self) -> Option<&str> {
        self.signed_entitlement
            .as_ref()
            .map(|se| se.payload.license_id.as_str())
    }

    /// The organization ID from the active license.
    pub fn organization_id(&self) -> Option<&str> {
        self.signed_entitlement
            .as_ref()
            .map(|se| se.payload.organization_id.as_str())
    }

    /// License expiration date.
    pub fn valid_until(&self) -> Option<DateTime<Utc>> {
        self.signed_entitlement
            .as_ref()
            .and_then(|se| se.payload.valid_until)
    }

    /// Days until license expiration. Returns `None` for perpetual licenses.
    pub fn days_until_expiry(&self) -> Option<i64> {
        self.valid_until().map(|until| {
            until
                .signed_duration_since(Utc::now())
                .num_days()
                .max(0)
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// LICENSE VERIFICATION ERRORS
// ═══════════════════════════════════════════════════════════════

/// Errors that can occur during license verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LicenseVerificationError {
    /// Ed25519 signature is invalid — document has been tampered.
    TamperedLicenseSignature,
    /// The installation_id in the entitlement doesn't match this machine.
    InstallationIdMismatch,
    /// The license has expired (valid_until is in the past).
    LicenseExpired,
    /// Offline mode is not allowed by this license.
    OfflineModeForbidden,
    /// The offline grace period has been exceeded.
    OfflineGraceExpired,
    /// The public key identified by `kid` is not trusted.
    UnknownKeyId,
    /// The entitlement payload could not be parsed.
    InvalidPayload,
    /// The signature encoding is invalid.
    InvalidSignatureEncoding,
    /// The public key is invalid.
    InvalidPublicKey,
    /// Clock rollback detected.
    ClockRollbackDetected,
    /// Product identity mismatch.
    ProductMismatch,
}

impl std::fmt::Display for LicenseVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TamperedLicenseSignature => write!(f, "License signature verification failed"),
            Self::InstallationIdMismatch => write!(f, "Installation ID does not match"),
            Self::LicenseExpired => write!(f, "License has expired"),
            Self::OfflineModeForbidden => write!(f, "Offline mode not allowed by license"),
            Self::OfflineGraceExpired => write!(f, "Offline grace period exceeded"),
            Self::UnknownKeyId => write!(f, "Unknown signing key ID"),
            Self::InvalidPayload => write!(f, "Invalid entitlement payload"),
            Self::InvalidSignatureEncoding => write!(f, "Invalid signature encoding"),
            Self::InvalidPublicKey => write!(f, "Invalid public verification key"),
            Self::ClockRollbackDetected => write!(f, "System clock rollback detected"),
            Self::ProductMismatch => write!(f, "Product identity mismatch"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// INSTALLATION IDENTITY
// ═══════════════════════════════════════════════════════════════

/// Generate a new installation identity.
///
/// Format: `inst_{platform}_{uuid}`
/// This is generated once during first launch and persisted.
/// It is NOT a hardware fingerprint — it is a stable random identifier.
pub fn generate_installation_id() -> String {
    let platform = std::env::consts::OS;
    let id = uuid::Uuid::new_v4()
        .to_string()
        .replace('-', "")
        .chars()
        .take(13)
        .collect::<String>();
    format!("inst_{}_{}", platform, id)
}

/// Validate a product key format.
///
/// Mulverse product keys follow: `ORBIS-XXXXX-XXXXX-XXXXX-XXXXX`
/// (5 segments of 5 Base32 characters each, separated by hyphens)
pub fn validate_product_key(key: &str) -> Result<(), String> {
    let trimmed = key.trim().to_uppercase();

    if trimmed.is_empty() {
        return Err("Product key cannot be empty".to_string());
    }

    if !trimmed.starts_with(PRODUCT_KEY_PREFIX) {
        return Err(format!(
            "Product key must start with '{}'",
            PRODUCT_KEY_PREFIX
        ));
    }

    let segments: Vec<&str> = trimmed.split('-').collect();
    if segments.len() != PRODUCT_KEY_SEGMENTS {
        return Err(format!(
            "Product key must have {} segments separated by hyphens (e.g. ORBIS-XXXXX-XXXXX-XXXXX-XXXXX)",
            PRODUCT_KEY_SEGMENTS
        ));
    }

    for (i, segment) in segments.iter().enumerate() {
        if i == 0 {
            // First segment is the product prefix
            if *segment != "ORBIS" {
                return Err("Product key must start with 'ORBIS-'".to_string());
            }
            continue;
        }
        if segment.len() != PRODUCT_KEY_SEGMENT_LENGTH {
            return Err(format!(
                "Each key segment must be {} characters",
                PRODUCT_KEY_SEGMENT_LENGTH
            ));
        }
        if !segment.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err("Key segments must contain only alphanumeric characters".to_string());
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════
// FRONTEND DTO — Serialized license state for the UI
// ═══════════════════════════════════════════════════════════════

/// DTO exposing license state to the frontend.
///
/// This exposes only what the UI needs to display.
/// No cryptographic material, no signing keys, no raw entitlement blobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStateDto {
    /// Current license state.
    pub state: LicenseState,
    /// Product identifier.
    pub product: String,
    /// Edition (e.g. "SCHOOL", "UNIVERSITY").
    pub edition: Option<String>,
    /// Plan (e.g. "BASIC", "PRO", "ENTERPRISE").
    pub plan: Option<String>,
    /// Organization ID associated with the license.
    pub organization_id: Option<String>,
    /// License expiration date.
    pub valid_until: Option<DateTime<Utc>>,
    /// Days until expiration.
    pub days_until_expiry: Option<i64>,
    /// Trial days remaining (0 if not in trial).
    pub trial_days_remaining: i64,
    /// Whether the application is operational.
    pub is_operational: bool,
    /// Whether the application is in restricted mode.
    pub is_restricted: bool,
    /// Installation ID (safe to display).
    pub installation_id: String,
    /// Last successful Mulverse sync.
    pub last_sync_at: Option<DateTime<Utc>>,
    /// Enabled entitlement keys.
    pub enabled_features: Vec<String>,
    /// Active limits.
    pub limits: Vec<LimitEntry>,
    /// Application version.
    pub app_version: String,
}

impl From<&LicenseSnapshot> for LicenseStateDto {
    fn from(snapshot: &LicenseSnapshot) -> Self {
        let enabled_features = match &snapshot.signed_entitlement {
            Some(se) => se
                .payload
                .entitlements
                .iter()
                .filter(|e| e.granted)
                .map(|e| e.key.clone())
                .collect(),
            None => {
                if snapshot.state == LicenseState::Trial {
                    snapshot.trial_policy.trial_features.clone()
                } else {
                    vec![]
                }
            }
        };

        let limits = match &snapshot.signed_entitlement {
            Some(se) => se.payload.limits.clone(),
            None => {
                if snapshot.state == LicenseState::Trial {
                    let mut trial_limits = vec![];
                    if let Some(max) = snapshot.trial_policy.max_users {
                        trial_limits.push(LimitEntry {
                            key: "users.max".to_string(),
                            max_value: max as u64,
                        });
                    }
                    if let Some(max) = snapshot.trial_policy.max_students {
                        trial_limits.push(LimitEntry {
                            key: "students.max".to_string(),
                            max_value: max as u64,
                        });
                    }
                    trial_limits
                } else {
                    vec![]
                }
            }
        };

        Self {
            state: snapshot.state.clone(),
            product: MULVERSE_PRODUCT_KEY.to_string(),
            edition: snapshot.edition_key().map(String::from),
            plan: snapshot.plan_key().map(String::from),
            organization_id: snapshot.organization_id().map(String::from),
            valid_until: snapshot.valid_until(),
            days_until_expiry: snapshot.days_until_expiry(),
            trial_days_remaining: snapshot.trial_days_remaining(),
            is_operational: snapshot.state.is_operational(),
            is_restricted: snapshot.state.is_restricted(),
            installation_id: snapshot.installation_id.clone(),
            last_sync_at: snapshot.last_sync_at,
            enabled_features,
            limits,
            app_version: snapshot.app_version.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_product_key_valid() {
        assert!(validate_product_key("ORBIS-ABCDE-FGHIJ-KLMNO-PQRST").is_ok());
        assert!(validate_product_key("ORBIS-12345-67890-ABCDE-FGHIJ").is_ok());
        // Case-insensitive
        assert!(validate_product_key("orbis-abcde-fghij-klmno-pqrst").is_ok());
    }

    #[test]
    fn test_validate_product_key_invalid() {
        assert!(validate_product_key("").is_err());
        assert!(validate_product_key("INVALID").is_err());
        assert!(validate_product_key("XXXX-XXXX-XXXX-XXXX").is_err());
        assert!(validate_product_key("ORBIS-ABC-DEF-GHI-JKL").is_err());
        assert!(validate_product_key("NOTORBIS-ABCDE-FGHIJ-KLMNO-PQRST").is_err());
    }

    #[test]
    fn test_installation_id_format() {
        let id = generate_installation_id();
        assert!(id.starts_with("inst_"));
        assert!(id.len() > 10);
    }

    #[test]
    fn test_license_state_operational() {
        assert!(LicenseState::Trial.is_operational());
        assert!(LicenseState::Active.is_operational());
        assert!(LicenseState::Expiring.is_operational());
        assert!(LicenseState::OfflineGrace.is_operational());
        assert!(!LicenseState::Expired.is_operational());
        assert!(!LicenseState::Revoked.is_operational());
        assert!(!LicenseState::Invalid.is_operational());
        assert!(!LicenseState::Unavailable.is_operational());
    }

    #[test]
    fn test_license_state_restricted() {
        assert!(LicenseState::Expired.is_restricted());
        assert!(LicenseState::Revoked.is_restricted());
        assert!(LicenseState::Invalid.is_restricted());
        assert!(!LicenseState::Active.is_restricted());
        assert!(!LicenseState::Trial.is_restricted());
    }

    #[test]
    fn test_trial_snapshot() {
        let snapshot = LicenseSnapshot::new_trial(
            "inst_windows_abc123".to_string(),
            "1.0.0".to_string(),
            TrialPolicy::default(),
        );
        assert_eq!(snapshot.state, LicenseState::Trial);
        assert!(snapshot.trial_days_remaining() > 0);
        assert!(snapshot.is_feature_enabled("students"));
        assert!(!snapshot.is_feature_enabled("ai_grading"));
        assert_eq!(snapshot.get_limit("students.max"), Some(100));
        assert_eq!(snapshot.get_limit("users.max"), Some(10));
        assert_eq!(snapshot.get_limit("branches.max"), None);
    }

    #[test]
    fn test_limit_check() {
        let snapshot = LicenseSnapshot::new_trial(
            "inst_windows_abc123".to_string(),
            "1.0.0".to_string(),
            TrialPolicy::default(),
        );
        // Within limits
        assert!(snapshot.check_limit("students.max", 50, 10).is_ok());
        // Exceeds limits
        assert!(snapshot.check_limit("students.max", 95, 10).is_err());
        // No limit defined = unlimited
        assert!(snapshot.check_limit("unknown.max", 999999, 1).is_ok());
    }

    #[test]
    fn test_license_state_dto_from_trial() {
        let snapshot = LicenseSnapshot::new_trial(
            "inst_windows_abc123".to_string(),
            "1.0.0".to_string(),
            TrialPolicy::default(),
        );
        let dto = LicenseStateDto::from(&snapshot);
        assert_eq!(dto.state, LicenseState::Trial);
        assert_eq!(dto.product, "ORBIS");
        assert!(dto.is_operational);
        assert!(!dto.is_restricted);
        assert!(dto.trial_days_remaining > 0);
        assert!(dto.enabled_features.contains(&"students".to_string()));
    }
}
