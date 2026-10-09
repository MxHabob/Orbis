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
// PRODUCT IDENTITY & CONSTANTS
// ═══════════════════════════════════════════════════════════════

/// Canonical Mulverse product stable key for Orbis.
pub const MULVERSE_PRODUCT_KEY: &str = "ORBIS";

/// Canonical Product ID for Orbis in Mulverse catalog.
pub const CANONICAL_PRODUCT_ID: &str = "11111111-1111-4111-8111-111111111111";

/// Product key prefix used by Mulverse for Orbis product keys.
pub const PRODUCT_KEY_PREFIX: &str = "ORBIS-";

/// Number of segments in a Mulverse product key (e.g. `ORBIS-XXXXX-XXXXX-XXXXX-XXXXX`).
pub const PRODUCT_KEY_SEGMENTS: usize = 5;

/// Characters per segment in a Mulverse product key.
pub const PRODUCT_KEY_SEGMENT_LENGTH: usize = 5;

/// Total length of formatted product key string (including hyphens).
pub const PRODUCT_KEY_LENGTH: usize = 29;

/// Default offline grace period in days.
pub const DEFAULT_OFFLINE_GRACE_PERIOD_DAYS: u32 = 30;

/// Default online verification interval in hours.
pub const DEFAULT_VERIFICATION_INTERVAL_HOURS: u32 = 24;

// ═══════════════════════════════════════════════════════════════
// STANDARD ENTITLEMENTS DICTIONARY (Features)
// ═══════════════════════════════════════════════════════════════

pub const ENTITLEMENT_STUDENTS: &str = "students";
pub const ENTITLEMENT_AI_GRADING: &str = "ai_grading";
pub const ENTITLEMENT_MULTI_BRANCH: &str = "multi_branch";
pub const ENTITLEMENT_OFFLINE_SYNC: &str = "offline_sync";
pub const ENTITLEMENT_PARENT_PORTAL: &str = "parent_portal";
pub const ENTITLEMENT_CAMPUS_ANALYTICS: &str = "campus_analytics";
pub const ENTITLEMENT_SSO: &str = "sso";
pub const ENTITLEMENT_EXAM_TOOLS: &str = "exam_tools";

pub const ALL_ENTITLEMENT_KEYS: &[&str] = &[
    ENTITLEMENT_STUDENTS,
    ENTITLEMENT_AI_GRADING,
    ENTITLEMENT_MULTI_BRANCH,
    ENTITLEMENT_OFFLINE_SYNC,
    ENTITLEMENT_PARENT_PORTAL,
    ENTITLEMENT_CAMPUS_ANALYTICS,
    ENTITLEMENT_SSO,
    ENTITLEMENT_EXAM_TOOLS,
];

// ═══════════════════════════════════════════════════════════════
// STANDARD LIMITS DICTIONARY (Quotas)
// ═══════════════════════════════════════════════════════════════

pub const LIMIT_STUDENTS_MAX: &str = "students.max";
pub const LIMIT_STAFF_MAX: &str = "staff.max";
pub const LIMIT_DEVICES_MAX: &str = "devices.max";
pub const LIMIT_BRANCHES_MAX: &str = "branches.max";
pub const LIMIT_CLASSES_MAX: &str = "classes.max";
pub const LIMIT_STORAGE_GB_MAX: &str = "storage_gb.max";

pub const ALL_LIMIT_KEYS: &[&str] = &[
    LIMIT_STUDENTS_MAX,
    LIMIT_STAFF_MAX,
    LIMIT_DEVICES_MAX,
    LIMIT_BRANCHES_MAX,
    LIMIT_CLASSES_MAX,
    LIMIT_STORAGE_GB_MAX,
];

/// Helper to get standard unit for a limit key.
pub fn get_limit_unit(key: &str) -> &'static str {
    match key {
        LIMIT_STUDENTS_MAX => "students",
        LIMIT_STAFF_MAX => "users",
        LIMIT_DEVICES_MAX => "devices",
        LIMIT_BRANCHES_MAX => "branches",
        LIMIT_CLASSES_MAX => "classes",
        LIMIT_STORAGE_GB_MAX => "GB",
        _ => "",
    }
}

// ═══════════════════════════════════════════════════════════════
// TRIAL POLICY
// ═══════════════════════════════════════════════════════════════

/// Centralized trial policy — configurable evaluation period settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialPolicy {
    /// Duration of the trial period in days.
    pub duration_days: u32,
    /// Maximum staff accounts allowed during trial.
    pub max_staff: Option<u32>,
    /// Maximum students allowed during trial.
    pub max_students: Option<u32>,
    /// Features available during trial.
    pub trial_features: Vec<String>,
}

impl Default for TrialPolicy {
    fn default() -> Self {
        Self {
            duration_days: DEFAULT_OFFLINE_GRACE_PERIOD_DAYS,
            max_staff: Some(25),
            max_students: Some(100),
            trial_features: vec![
                ENTITLEMENT_STUDENTS.to_string(),
                ENTITLEMENT_OFFLINE_SYNC.to_string(),
                ENTITLEMENT_EXAM_TOOLS.to_string(),
            ],
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// SIGNED ENTITLEMENT — Mulverse Platform Cryptographic Artifact
// ═══════════════════════════════════════════════════════════════

/// The top-level signed entitlement envelope issued by Mulverse Platform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedEntitlement {
    /// Schema version (currently 1).
    pub v: u32,
    /// Key identifier for signature rotation (e.g. "key_ed25519_2026_v1").
    pub kid: String,
    /// Algorithm identifier (always "Ed25519").
    pub alg: String,
    /// The entitlement payload data.
    pub payload: EntitlementPayload,
    /// Base64-encoded Ed25519 signature over canonical JSON of `payload`.
    pub sig: String,
}

/// The entitlement payload — the authoritative data signed by Mulverse Platform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntitlementPayload {
    /// License UUID from Mulverse.
    pub license_id: String,
    /// The product key used for activation (e.g. "ORBIS-ABCDE-FGHIJ-KLMNO-PQRST").
    pub product_key: String,
    /// Mulverse product UUID ("11111111-1111-4111-8111-111111111111").
    pub product_id: String,
    /// Organization UUID that owns this license.
    pub organization_id: String,
    /// Edition key (e.g. "SCHOOL", "UNIVERSITY").
    pub edition_key: String,
    /// Plan key (e.g. "BASIC", "PRO", "ENTERPRISE", "SCHOOL_PRO").
    pub plan_key: String,
    /// Boolean feature capabilities.
    pub entitlements: Vec<EntitlementItem>,
    /// Quantitative limits.
    pub limits: Vec<LimitItem>,
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
    /// Days the app can operate without contacting Mulverse (default 30).
    pub offline_grace_period_days: u32,
    /// How often (hours) the app should verify/refresh online (default 24).
    pub verification_interval_hours: u32,
}

/// A boolean entitlement entry from the Mulverse catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntitlementItem {
    /// Canonical entitlement key (e.g. "students", "ai_grading", "multi_branch").
    pub key: String,
    /// Whether this capability is granted.
    pub granted: bool,
}

/// Alias for backwards compatibility.
pub type EntitlementEntry = EntitlementItem;

/// A quantitative limit entry from the Mulverse catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimitItem {
    /// Canonical limit key (e.g. "students.max", "staff.max", "devices.max").
    pub key: String,
    /// Maximum allowed value.
    pub max_value: i64,
}

/// Alias for backwards compatibility.
pub type LimitEntry = LimitItem;

// ═══════════════════════════════════════════════════════════════
// ACTIVATION & REFRESH — Mulverse API DTOs
// ═══════════════════════════════════════════════════════════════

/// Request to activate a product key with Mulverse Platform.
/// `POST /api/v1/public/licenses/activate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationRequest {
    pub product_key: String,
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
/// `POST /api/v1/public/licenses/refresh`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub license_id: String,
    pub installation_id: String,
    pub app_version: String,
}

/// Request to deactivate an installation.
/// `POST /api/v1/public/licenses/deactivate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeactivationRequest {
    pub license_id: String,
    pub installation_id: String,
}

/// Public key metadata from Mulverse key rotation endpoint.
/// `GET /api/v1/public/keys/active`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyInfo {
    pub kid: String,
    pub algorithm: String,
    pub public_key_b64: String,
}

/// Update manifest from Tauri updater endpoint.
/// `GET /api/v1/public/updates/tauri`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriUpdateManifest {
    pub version: String,
    pub notes: Option<String>,
    pub pub_date: Option<String>,
    pub url: String,
    pub signature: String,
}

// ═══════════════════════════════════════════════════════════════
// LICENSE STATE MACHINE
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseState {
    /// No license or trial — initial state, activation screen shown.
    Unavailable,
    /// License is valid but its `valid_from` time has not yet arrived.
    Pending,
    /// Trial period is active. No product key has been activated.
    Trial,
    /// License is active and valid. Signed entitlement verified.
    Active,
    /// License is active but nearing expiration (<= 14 days).
    Expiring,
    /// Operating offline within the allowed grace period.
    OfflineGrace,
    /// License or trial has expired.
    Expired,
    /// License is temporarily disabled by the licensing authority.
    Suspended,
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
            LicenseState::Unavailable
                | LicenseState::Pending
                | LicenseState::Expired
                | LicenseState::Suspended
                | LicenseState::Revoked
                | LicenseState::Invalid
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
                let end = started + chrono::Duration::days(self.trial_policy.duration_days as i64);
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
    pub fn detect_clock_rollback(&self) -> bool {
        let now = Utc::now();
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
    pub fn get_limit(&self, key: &str) -> Option<i64> {
        match &self.signed_entitlement {
            Some(se) => se
                .payload
                .limits
                .iter()
                .find(|l| l.key == key)
                .map(|l| l.max_value),
            None => {
                if self.state == LicenseState::Trial {
                    match key {
                        LIMIT_STAFF_MAX | "users.max" => {
                            self.trial_policy.max_staff.map(|v| v as i64)
                        }
                        LIMIT_STUDENTS_MAX => self.trial_policy.max_students.map(|v| v as i64),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Check if creating `amount` more of a resource would exceed the limit.
    pub fn check_limit(&self, key: &str, current_count: u64, amount: u64) -> Result<(), String> {
        match self.get_limit(key) {
            Some(max) if max >= 0 => {
                let max_u64 = max as u64;
                let new_total = current_count.saturating_add(amount);
                if new_total > max_u64 {
                    Err(format!(
                        "License limit exceeded for '{}': current={}, adding={}, max={}",
                        key, current_count, amount, max
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()), // No limit or negative = unlimited
        }
    }

    /// The edition key (e.g. "SCHOOL", "UNIVERSITY") from the active license.
    pub fn edition_key(&self) -> Option<&str> {
        self.signed_entitlement
            .as_ref()
            .map(|se| se.payload.edition_key.as_str())
    }

    /// The plan key (e.g. "BASIC", "PRO", "SCHOOL_PRO") from the active license.
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
        self.valid_until()
            .map(|until| until.signed_duration_since(Utc::now()).num_days().max(0))
    }
}

// ═══════════════════════════════════════════════════════════════
// LICENSE VERIFICATION ERRORS
// ═══════════════════════════════════════════════════════════════

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
    /// No pinned public key is configured for signature verification.
    NoTrustedKeysConfigured,
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
            Self::NoTrustedKeysConfigured => {
                write!(f, "No trusted license verification keys are configured")
            }
            Self::ClockRollbackDetected => write!(f, "System clock rollback detected"),
            Self::ProductMismatch => write!(f, "Product identity mismatch"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// INSTALLATION IDENTITY & PRODUCT KEY VALIDATION
// ═══════════════════════════════════════════════════════════════

/// Generate a new installation identity.
/// Format: `inst_{platform}_{uuid_short}`
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

/// Validate a product key format: `ORBIS-XXXXX-XXXXX-XXXXX-XXXXX`
/// 5 segments of 5 alphanumeric characters each, separated by hyphens (29 chars total).
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

/// DTO for individual limit entry exposed to UI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimitEntryDto {
    pub key: String,
    pub max_value: i64,
    pub unit: String,
}

/// DTO exposing license state to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStateDto {
    /// Current license state.
    pub state: LicenseState,
    /// Product identifier.
    pub product: String,
    /// Edition (e.g. "SCHOOL", "UNIVERSITY").
    pub edition: Option<String>,
    /// Plan (e.g. "BASIC", "PRO", "SCHOOL_PRO").
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
    /// Entitlement keys available when starting this installation's trial.
    pub trial_features: Vec<String>,
    /// Active limits.
    pub limits: Vec<LimitEntryDto>,
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

        let limits: Vec<LimitEntryDto> = match &snapshot.signed_entitlement {
            Some(se) => se
                .payload
                .limits
                .iter()
                .map(|l| LimitEntryDto {
                    key: l.key.clone(),
                    max_value: l.max_value,
                    unit: get_limit_unit(&l.key).to_string(),
                })
                .collect(),
            None => {
                if snapshot.state == LicenseState::Trial {
                    let mut trial_limits = vec![];
                    if let Some(max) = snapshot.trial_policy.max_staff {
                        trial_limits.push(LimitEntryDto {
                            key: LIMIT_STAFF_MAX.to_string(),
                            max_value: max as i64,
                            unit: get_limit_unit(LIMIT_STAFF_MAX).to_string(),
                        });
                    }
                    if let Some(max) = snapshot.trial_policy.max_students {
                        trial_limits.push(LimitEntryDto {
                            key: LIMIT_STUDENTS_MAX.to_string(),
                            max_value: max as i64,
                            unit: get_limit_unit(LIMIT_STUDENTS_MAX).to_string(),
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
            trial_features: snapshot.trial_policy.trial_features.clone(),
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
        assert!(!LicenseState::Suspended.is_operational());
        assert!(!LicenseState::Revoked.is_operational());
        assert!(!LicenseState::Invalid.is_operational());
        assert!(!LicenseState::Unavailable.is_operational());
        assert!(!LicenseState::Pending.is_operational());
        assert!(LicenseState::Unavailable.is_restricted());
        assert!(LicenseState::Pending.is_restricted());
        assert!(LicenseState::Suspended.is_restricted());
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
        assert!(snapshot.is_feature_enabled(ENTITLEMENT_STUDENTS));
        assert!(!snapshot.is_feature_enabled(ENTITLEMENT_AI_GRADING));
        assert_eq!(snapshot.get_limit(LIMIT_STUDENTS_MAX), Some(100));
        assert_eq!(snapshot.get_limit(LIMIT_STAFF_MAX), Some(25));
        assert_eq!(snapshot.get_limit(LIMIT_BRANCHES_MAX), None);
    }

    #[test]
    fn license_state_dto_exposes_trial_features_before_trial_starts() {
        let snapshot = LicenseSnapshot::new_unavailable(
            "inst_windows_abc123".to_string(),
            "1.0.0".to_string(),
        );
        let dto = LicenseStateDto::from(&snapshot);

        assert_eq!(dto.trial_features, snapshot.trial_policy.trial_features);
        assert!(dto.trial_features.contains(&ENTITLEMENT_STUDENTS.to_string()));
        assert!(!dto.trial_features.contains(&ENTITLEMENT_MULTI_BRANCH.to_string()));
    }

    #[test]
    fn test_limit_check() {
        let snapshot = LicenseSnapshot::new_trial(
            "inst_windows_abc123".to_string(),
            "1.0.0".to_string(),
            TrialPolicy::default(),
        );
        assert!(snapshot.check_limit(LIMIT_STUDENTS_MAX, 50, 10).is_ok());
        assert!(snapshot.check_limit(LIMIT_STUDENTS_MAX, 95, 10).is_err());
        assert!(snapshot.check_limit("unknown.max", 999999, 1).is_ok());
    }
}
