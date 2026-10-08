//! License runtime — Mulverse Platform license verification, activation, and local state management.
//!
//! This is the core implementation of the `LicenseRuntime` port.
//! It handles:
//! - Ed25519 signature verification of signed entitlements
//! - Local license state persistence
//! - Trial management
//! - Clock rollback detection
//! - Background sync scheduling
//!
//! The runtime operates offline-first: network failures never cause license failures.

use async_trait::async_trait;
use chrono::Utc;
use orbis_app::ports::license::LicenseRuntime;
use orbis_domain::entities::license::*;
use orbis_domain::errors::DomainError;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::mulverse_client::MulverseClient;

// ═══════════════════════════════════════════════════════════════
// EMBEDDED PUBLIC KEY — Compile-time Ed25519 verification key
// ═══════════════════════════════════════════════════════════════

/// Placeholder for the Mulverse Ed25519 public key.
///
/// In production, this will be replaced with the actual base64-encoded
/// 32-byte Ed25519 public key from the Mulverse Platform.
///
/// Key rotation is supported: the runtime can fetch updated keys
/// from `/api/v1/public/keys/active` during background sync.
const EMBEDDED_PUBLIC_KEY_B64: &str = "";

/// Key ID for the embedded public key.
const EMBEDDED_KEY_ID: &str = "key_ed25519_2026_v1";

// ═══════════════════════════════════════════════════════════════
// LICENSE FILE PATHS
// ═══════════════════════════════════════════════════════════════

const LICENSE_SNAPSHOT_FILE: &str = "license_snapshot.json";
const INSTALLATION_ID_FILE: &str = "installation_id";

// ═══════════════════════════════════════════════════════════════
// MULVERSE LICENSE RUNTIME
// ═══════════════════════════════════════════════════════════════

/// The Mulverse license runtime — implements the `LicenseRuntime` port.
///
/// Manages the full license lifecycle including activation, verification,
/// trial management, and background synchronization with Mulverse Platform.
pub struct MulverseLicenseRuntime {
    /// Application data directory for persisting license state.
    app_data_dir: PathBuf,
    /// The current license snapshot (thread-safe mutable state).
    snapshot: RwLock<LicenseSnapshot>,
    /// Application version string.
    app_version: String,
    /// Mulverse Platform HTTP client.
    mulverse_client: MulverseClient,
    /// Trial policy configuration.
    trial_policy: TrialPolicy,
    /// Cached trusted public keys (kid → base64 public key).
    trusted_keys: RwLock<Vec<(String, String)>>,
}

impl MulverseLicenseRuntime {
    /// Create a new license runtime.
    ///
    /// Does NOT perform initialization — call `initialize()` after construction.
    pub fn new(
        app_data_dir: &Path,
        app_version: &str,
        mulverse_base_url: Option<&str>,
    ) -> Result<Self, String> {
        let installation_id = load_or_generate_installation_id(app_data_dir)?;

        let mulverse_client = match mulverse_base_url {
            Some(url) => MulverseClient::with_base_url(url)?,
            None => MulverseClient::new()?,
        };

        let initial_snapshot =
            LicenseSnapshot::new_unavailable(installation_id, app_version.to_string());

        let mut trusted_keys = vec![];
        if !EMBEDDED_PUBLIC_KEY_B64.is_empty() {
            trusted_keys.push((
                EMBEDDED_KEY_ID.to_string(),
                EMBEDDED_PUBLIC_KEY_B64.to_string(),
            ));
        }

        Ok(Self {
            app_data_dir: app_data_dir.to_path_buf(),
            snapshot: RwLock::new(initial_snapshot),
            app_version: app_version.to_string(),
            mulverse_client,
            trial_policy: TrialPolicy::default(),
            trusted_keys: RwLock::new(trusted_keys),
        })
    }

    /// Persist the current snapshot to disk.
    fn persist_snapshot(&self, snapshot: &LicenseSnapshot) -> Result<(), DomainError> {
        let path = self.app_data_dir.join(LICENSE_SNAPSHOT_FILE);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                DomainError::Internal(format!("Failed to create license directory: {e}"))
            })?;
        }

        let json = serde_json::to_string_pretty(snapshot).map_err(|e| {
            DomainError::Internal(format!("Failed to serialize license snapshot: {e}"))
        })?;

        std::fs::write(&path, json).map_err(|e| {
            DomainError::Internal(format!("Failed to write license snapshot: {e}"))
        })?;

        debug!("License snapshot persisted to {:?}", path);
        Ok(())
    }

    /// Load the persisted snapshot from disk.
    fn load_persisted_snapshot(&self) -> Option<LicenseSnapshot> {
        let path = self.app_data_dir.join(LICENSE_SNAPSHOT_FILE);
        if !path.exists() {
            return None;
        }

        match std::fs::read_to_string(&path) {
            Ok(json) => match serde_json::from_str::<LicenseSnapshot>(&json) {
                Ok(snapshot) => {
                    debug!("Loaded persisted license snapshot");
                    Some(snapshot)
                }
                Err(e) => {
                    warn!("Failed to parse license snapshot: {e}");
                    None
                }
            },
            Err(e) => {
                warn!("Failed to read license snapshot: {e}");
                None
            }
        }
    }

    /// Evaluate the current license state from the snapshot.
    ///
    /// This is the core state machine evaluation logic.
    fn evaluate_state(&self, snapshot: &mut LicenseSnapshot) {
        let now = Utc::now();

        // 1. Clock rollback detection
        if snapshot.detect_clock_rollback() {
            warn!("Clock rollback detected");
            snapshot.state = LicenseState::Invalid;
            return;
        }

        // Update last known time
        snapshot.last_known_time = now;

        // 2. If we have a signed entitlement, evaluate it
        if let Some(ref se) = snapshot.signed_entitlement {
            // Check expiration
            if let Some(valid_until) = se.payload.valid_until {
                if now > valid_until {
                    info!(
                        license_id = %se.payload.license_id,
                        "License expired"
                    );
                    snapshot.state = LicenseState::Expired;
                    return;
                }

                // Check if expiring soon (within 14 days)
                let days_remaining = valid_until
                    .signed_duration_since(now)
                    .num_days();
                if days_remaining <= 14 && days_remaining > 0 {
                    snapshot.state = LicenseState::Expiring;
                    return;
                }
            }

            // Check offline grace period
            if let Some(last_sync) = snapshot.last_sync_at {
                let days_offline = now
                    .signed_duration_since(last_sync)
                    .num_days();
                if days_offline > se.payload.offline_grace_period_days as i64 {
                    if !se.payload.offline_allowed {
                        warn!("Offline mode forbidden by license");
                        snapshot.state = LicenseState::Invalid;
                    } else {
                        warn!(
                            days_offline = days_offline,
                            grace_days = se.payload.offline_grace_period_days,
                            "Offline grace period exceeded"
                        );
                        snapshot.state = LicenseState::Expired;
                    }
                    return;
                }

                if days_offline > 0 {
                    snapshot.state = LicenseState::OfflineGrace;
                    return;
                }
            } else {
                // Never synced — if just activated offline, use OfflineGrace
                snapshot.state = LicenseState::OfflineGrace;
                return;
            }

            // All checks passed — license is active
            snapshot.state = LicenseState::Active;
            return;
        }

        // 3. No signed entitlement — evaluate trial
        if snapshot.trial_started_at.is_some() {
            if snapshot.is_trial_expired() {
                snapshot.state = LicenseState::Expired;
            } else {
                snapshot.state = LicenseState::Trial;
            }
            return;
        }

        // 4. No entitlement, no trial — unavailable
        snapshot.state = LicenseState::Unavailable;
    }

    /// Update the internal snapshot and persist.
    fn update_snapshot(&self, snapshot: LicenseSnapshot) -> Result<(), DomainError> {
        self.persist_snapshot(&snapshot)?;
        if let Ok(mut guard) = self.snapshot.write() {
            *guard = snapshot;
        }
        Ok(())
    }

    /// Verify the Ed25519 signature of a signed entitlement.
    fn verify_entitlement_signature(
        &self,
        entitlement: &SignedEntitlement,
    ) -> Result<(), LicenseVerificationError> {
        // If no embedded key and no fetched keys, we cannot verify
        let trusted_keys = self
            .trusted_keys
            .read()
            .map_err(|_| LicenseVerificationError::InvalidPublicKey)?;

        if trusted_keys.is_empty() {
            // During development or before Mulverse provides keys,
            // warn but allow operation. In production this must be enforced.
            warn!("No trusted Mulverse public keys configured — signature verification skipped");
            return Ok(());
        }

        // Find the key matching the entitlement's kid
        let key_b64 = trusted_keys
            .iter()
            .find(|(kid, _)| *kid == entitlement.kid)
            .map(|(_, key)| key.as_str())
            .ok_or(LicenseVerificationError::UnknownKeyId)?;

        // Decode the public key
        let key_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            key_b64,
        )
        .map_err(|_| LicenseVerificationError::InvalidPublicKey)?;

        if key_bytes.len() != 32 {
            return Err(LicenseVerificationError::InvalidPublicKey);
        }

        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&key_bytes);

        // Decode the signature
        let sig_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &entitlement.sig,
        )
        .map_err(|_| LicenseVerificationError::InvalidSignatureEncoding)?;

        // Canonicalize the payload to JSON for verification
        let canonical_payload = serde_json::to_string(&entitlement.payload)
            .map_err(|_| LicenseVerificationError::InvalidPayload)?;

        // Verify using ed25519-dalek
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        let verifying_key = VerifyingKey::from_bytes(&key_array)
            .map_err(|_| LicenseVerificationError::InvalidPublicKey)?;

        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|_| LicenseVerificationError::InvalidSignatureEncoding)?;

        verifying_key
            .verify(canonical_payload.as_bytes(), &signature)
            .map_err(|_| LicenseVerificationError::TamperedLicenseSignature)?;

        debug!(kid = %entitlement.kid, "Ed25519 signature verified successfully");
        Ok(())
    }

    /// Attempt to refresh the trusted key set from Mulverse Platform.
    async fn refresh_trusted_keys(&self) {
        match self.mulverse_client.get_active_key().await {
            Ok(key_info) => {
                if let Ok(mut keys) = self.trusted_keys.write() {
                    // Add or update the key
                    if let Some(existing) = keys.iter_mut().find(|(kid, _)| *kid == key_info.kid) {
                        existing.1 = key_info.public_key_b64;
                    } else {
                        keys.push((key_info.kid.clone(), key_info.public_key_b64));
                    }
                    info!(kid = %key_info.kid, "Updated trusted Mulverse signing key");
                }
            }
            Err(e) => {
                debug!("Failed to refresh Mulverse signing keys: {e}");
            }
        }
    }
}

#[async_trait]
impl LicenseRuntime for MulverseLicenseRuntime {
    fn current_snapshot(&self) -> LicenseSnapshot {
        self.snapshot
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| {
                LicenseSnapshot::new_unavailable(
                    "unknown".to_string(),
                    self.app_version.clone(),
                )
            })
    }

    fn current_state_dto(&self) -> LicenseStateDto {
        LicenseStateDto::from(&self.current_snapshot())
    }

    async fn initialize(&self) -> Result<(), DomainError> {
        info!("Initializing Orbis license runtime");

        // 1. Try to load persisted snapshot
        if let Some(mut snapshot) = self.load_persisted_snapshot() {
            // Verify signature if we have a signed entitlement
            if snapshot.signed_entitlement.is_some() {
                match self.verify_entitlement_signature(
                    snapshot.signed_entitlement.as_ref().unwrap(),
                ) {
                    Ok(()) => {
                        debug!("Persisted signed entitlement verified");
                    }
                    Err(e) => {
                        warn!("Persisted signed entitlement verification failed: {e}");
                        snapshot.signed_entitlement = None;
                        snapshot.state = LicenseState::Invalid;
                    }
                }
            }

            // Verify installation ID matches
            let current_id = load_or_generate_installation_id(&self.app_data_dir)
                .unwrap_or_else(|_| "unknown".to_string());

            if snapshot.installation_id != current_id {
                warn!(
                    expected = %current_id,
                    found = %snapshot.installation_id,
                    "Installation ID mismatch — license may have been copied"
                );
                // Don't immediately invalidate — the user might have re-installed
                snapshot.installation_id = current_id;
            }

            // Re-evaluate state
            self.evaluate_state(&mut snapshot);
            snapshot.app_version = self.app_version.clone();
            self.update_snapshot(snapshot)?;

            info!(state = ?self.current_snapshot().state, "License runtime initialized from persisted state");
        } else {
            // No persisted state — start as unavailable
            let installation_id = load_or_generate_installation_id(&self.app_data_dir)
                .unwrap_or_else(|_| generate_installation_id());

            let snapshot =
                LicenseSnapshot::new_unavailable(installation_id, self.app_version.clone());
            self.update_snapshot(snapshot)?;

            info!("License runtime initialized — no existing license found");
        }

        Ok(())
    }

    async fn activate_product_key(
        &self,
        product_key: &str,
    ) -> Result<ActivationResponse, DomainError> {
        // Validate product key format
        validate_product_key(product_key)
            .map_err(|e| DomainError::Validation(e))?;

        let installation_id = self.installation_id();

        let request = ActivationRequest {
            product_key: product_key.trim().to_uppercase(),
            license_id: None,
            installation_id: installation_id.clone(),
            platform: std::env::consts::OS.to_string(),
            app_version: self.app_version.clone(),
            machine_label: hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok()),
        };

        info!(
            installation_id = %installation_id,
            "Activating product key with Mulverse Platform"
        );

        let response = self
            .mulverse_client
            .activate(&request)
            .await
            .map_err(|e| {
                if e.is_network_error() {
                    DomainError::Internal(format!(
                        "Cannot reach Mulverse Platform for activation. Please check your internet connection. ({})",
                        e
                    ))
                } else {
                    DomainError::from(e)
                }
            })?;

        // Verify the received signed entitlement
        if let Err(e) = self.verify_entitlement_signature(&response.signed_entitlement) {
            error!("Received signed entitlement failed verification: {e}");
            return Err(DomainError::Internal(format!(
                "License verification failed: {e}"
            )));
        }

        // Verify installation ID matches
        if response.signed_entitlement.payload.installation_id != installation_id {
            error!("Activation response installation_id mismatch");
            return Err(DomainError::Internal(
                "Activation response installation ID mismatch".to_string(),
            ));
        }

        // Create snapshot with the new signed entitlement
        let mut snapshot = LicenseSnapshot {
            state: LicenseState::Active,
            signed_entitlement: Some(response.signed_entitlement.clone()),
            trial_started_at: None,
            trial_policy: self.trial_policy.clone(),
            last_sync_at: Some(Utc::now()),
            last_known_time: Utc::now(),
            installation_id,
            app_version: self.app_version.clone(),
        };

        self.evaluate_state(&mut snapshot);
        self.update_snapshot(snapshot)?;

        info!(
            license_id = %response.activation.license_id,
            edition = %response.signed_entitlement.payload.edition_key,
            plan = %response.signed_entitlement.payload.plan_key,
            "Product key activated successfully"
        );

        Ok(response)
    }

    async fn start_trial(&self) -> Result<LicenseSnapshot, DomainError> {
        let current = self.current_snapshot();

        // Prevent starting trial if one already existed
        if current.trial_started_at.is_some() {
            return Err(DomainError::Conflict(
                "A trial period has already been used on this installation".to_string(),
            ));
        }

        // Prevent starting trial if already licensed
        if current.signed_entitlement.is_some() {
            return Err(DomainError::Conflict(
                "Cannot start trial — a valid license is already active".to_string(),
            ));
        }

        let snapshot = LicenseSnapshot::new_trial(
            current.installation_id.clone(),
            self.app_version.clone(),
            self.trial_policy.clone(),
        );

        self.update_snapshot(snapshot.clone())?;

        info!(
            duration_days = self.trial_policy.duration_days,
            "Trial evaluation period started"
        );

        Ok(snapshot)
    }

    async fn refresh(&self) -> Result<(), DomainError> {
        let current = self.current_snapshot();

        // Only refresh if we have a signed entitlement with a license_id
        let se = match &current.signed_entitlement {
            Some(se) => se.clone(),
            None => {
                debug!("No signed entitlement to refresh");
                return Ok(());
            }
        };

        let request = RefreshRequest {
            license_id: se.payload.license_id.clone(),
            installation_id: current.installation_id.clone(),
            app_version: self.app_version.clone(),
        };

        match self.mulverse_client.refresh(&request).await {
            Ok(response) => {
                // Verify the new signed entitlement
                if let Err(e) = self.verify_entitlement_signature(&response.signed_entitlement) {
                    warn!("Refreshed entitlement failed verification: {e}");
                    return Ok(()); // Keep existing valid entitlement
                }

                // Check if the license has been revoked
                if response.activation.revoked_at.is_some() {
                    warn!(
                        license_id = %se.payload.license_id,
                        "License has been revoked by Mulverse Platform"
                    );
                    let mut snapshot = current;
                    snapshot.state = LicenseState::Revoked;
                    snapshot.last_sync_at = Some(Utc::now());
                    snapshot.last_known_time = Utc::now();
                    self.update_snapshot(snapshot)?;
                    return Ok(());
                }

                // Update with fresh entitlement
                let mut snapshot = current;
                snapshot.signed_entitlement = Some(response.signed_entitlement);
                snapshot.last_sync_at = Some(Utc::now());
                snapshot.last_known_time = Utc::now();
                self.evaluate_state(&mut snapshot);
                self.update_snapshot(snapshot)?;

                debug!("License refreshed successfully");
            }
            Err(e) => {
                if e.is_network_error() {
                    // Network failure is NOT a license failure
                    debug!("License refresh failed (network): {e}");
                } else if e.is_license_error() {
                    warn!("License refresh denied by Mulverse: {e}");
                    // The license may have been revoked
                    let mut snapshot = current;
                    snapshot.state = LicenseState::Revoked;
                    snapshot.last_known_time = Utc::now();
                    self.update_snapshot(snapshot)?;
                } else {
                    debug!("License refresh failed: {e}");
                }
            }
        }

        // Also try to refresh trusted keys
        self.refresh_trusted_keys().await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), DomainError> {
        let current = self.current_snapshot();

        if let Some(ref se) = current.signed_entitlement {
            let request = DeactivationRequest {
                license_id: se.payload.license_id.clone(),
                installation_id: current.installation_id.clone(),
            };

            // Best-effort deactivation — if network fails, still clear local state
            match self.mulverse_client.deactivate(&request).await {
                Ok(()) => {
                    info!(
                        license_id = %se.payload.license_id,
                        "Installation deactivated with Mulverse Platform"
                    );
                }
                Err(e) => {
                    warn!("Failed to deactivate with Mulverse (clearing local state anyway): {e}");
                }
            }
        }

        // Clear local state
        let snapshot = LicenseSnapshot::new_unavailable(
            current.installation_id.clone(),
            self.app_version.clone(),
        );
        self.update_snapshot(snapshot)?;

        // Remove persisted file
        let path = self.app_data_dir.join(LICENSE_SNAPSHOT_FILE);
        let _ = std::fs::remove_file(&path);

        info!("License state cleared — installation deactivated");
        Ok(())
    }

    fn is_feature_enabled(&self, key: &str) -> bool {
        self.current_snapshot().is_feature_enabled(key)
    }

    fn get_limit(&self, key: &str) -> Option<u64> {
        self.current_snapshot().get_limit(key)
    }

    fn check_limit(
        &self,
        key: &str,
        current_count: u64,
        amount: u64,
    ) -> Result<(), DomainError> {
        self.current_snapshot()
            .check_limit(key, current_count, amount)
            .map_err(|e| DomainError::Forbidden(e))
    }

    fn verify_signature(&self) -> Result<(), LicenseVerificationError> {
        let snapshot = self.current_snapshot();
        match &snapshot.signed_entitlement {
            Some(se) => self.verify_entitlement_signature(se),
            None => Ok(()), // No entitlement to verify
        }
    }

    fn installation_id(&self) -> String {
        self.current_snapshot().installation_id.clone()
    }
}

// ═══════════════════════════════════════════════════════════════
// BRIDGE — EntitlementProvider implementation
// ═══════════════════════════════════════════════════════════════

/// Bridge adapter: exposes `LicenseRuntime` as an `EntitlementProvider`.
///
/// This allows the existing Orbis services that depend on `EntitlementProvider`
/// to transparently use the Mulverse license runtime without modification.
pub struct LicenseEntitlementBridge {
    runtime: std::sync::Arc<dyn LicenseRuntime>,
}

impl LicenseEntitlementBridge {
    pub fn new(runtime: std::sync::Arc<dyn LicenseRuntime>) -> Self {
        Self { runtime }
    }
}

#[async_trait]
impl orbis_app::ports::entitlement::EntitlementProvider for LicenseEntitlementBridge {
    async fn check_entitlement(
        &self,
        _product_id: &str,
        _org_id: &str,
    ) -> Result<orbis_domain::entities::entitlement::EntitlementResult, DomainError> {
        let snapshot = self.runtime.current_snapshot();

        let status = match snapshot.state {
            LicenseState::Active | LicenseState::Expiring => {
                orbis_domain::entities::entitlement::EntitlementStatus::Active
            }
            LicenseState::Trial => {
                orbis_domain::entities::entitlement::EntitlementStatus::Trial
            }
            LicenseState::OfflineGrace => {
                orbis_domain::entities::entitlement::EntitlementStatus::GracePeriod
            }
            LicenseState::Expired | LicenseState::Revoked | LicenseState::Invalid => {
                orbis_domain::entities::entitlement::EntitlementStatus::Expired
            }
            LicenseState::Unavailable => {
                orbis_domain::entities::entitlement::EntitlementStatus::Unlicensed
            }
        };

        let plan = snapshot.plan_key().map(String::from);
        let max_users = snapshot.get_limit("users.max").map(|v| v as u32);
        let max_students = snapshot.get_limit("students.max").map(|v| v as u32);

        let features = match &snapshot.signed_entitlement {
            Some(se) => se
                .payload
                .entitlements
                .iter()
                .filter(|e| e.granted)
                .map(|e| e.key.clone())
                .collect(),
            None if snapshot.state == LicenseState::Trial => {
                snapshot.trial_policy.trial_features.clone()
            }
            _ => vec![],
        };

        let expires_at = snapshot
            .valid_until()
            .or_else(|| {
                snapshot.trial_started_at.map(|started| {
                    started
                        + chrono::Duration::days(snapshot.trial_policy.duration_days as i64)
                })
            })
            .map(|dt| dt.to_rfc3339());

        let grace_period_until = snapshot.signed_entitlement.as_ref().and_then(|se| {
            snapshot.last_sync_at.map(|sync| {
                (sync + chrono::Duration::days(se.payload.offline_grace_period_days as i64))
                    .to_rfc3339()
            })
        });

        Ok(orbis_domain::entities::entitlement::EntitlementResult {
            status,
            plan,
            max_users,
            max_students,
            features,
            expires_at,
            grace_period_until,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// INSTALLATION IDENTITY HELPERS
// ═══════════════════════════════════════════════════════════════

/// Load or generate a stable installation identity.
///
/// Persisted as a plain file in AppData. Uses the same pattern as JWT secret generation.
fn load_or_generate_installation_id(app_data_dir: &Path) -> Result<String, String> {
    let path = app_data_dir.join(INSTALLATION_ID_FILE);

    if path.exists() {
        std::fs::read_to_string(&path)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to read installation ID: {e}"))
    } else {
        let id = generate_installation_id();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create AppData dir: {e}"))?;
        }
        std::fs::write(&path, &id)
            .map_err(|e| format!("Failed to write installation ID: {e}"))?;

        info!(installation_id = %id, "Generated new installation identity");
        Ok(id)
    }
}

// ═══════════════════════════════════════════════════════════════
// BACKGROUND SYNC TASK
// ═══════════════════════════════════════════════════════════════

/// Spawn a background license sync task.
///
/// Periodically refreshes the license with Mulverse Platform.
/// Uses the `verification_interval_hours` from the signed entitlement,
/// defaulting to 24 hours.
///
/// This task:
/// - Does NOT block application startup
/// - Does NOT retry aggressively when offline
/// - Does NOT produce UI error messages for sync failures
/// - IS idempotent (repeated sync is safe)
pub fn spawn_background_sync(
    runtime: std::sync::Arc<dyn LicenseRuntime>,
    initial_delay_secs: u64,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        // Initial delay to avoid blocking startup
        tokio::time::sleep(std::time::Duration::from_secs(initial_delay_secs)).await;

        loop {
            match runtime.refresh().await {
                Ok(()) => debug!("Background license sync completed"),
                Err(e) => debug!("Background license sync failed (non-fatal): {e}"),
            }

            // Determine next sync interval from the signed entitlement
            let interval_hours = {
                let snapshot = runtime.current_snapshot();
                snapshot
                    .signed_entitlement
                    .as_ref()
                    .map(|se| se.payload.verification_interval_hours as u64)
                    .unwrap_or(24)
            };

            let interval = std::time::Duration::from_secs(interval_hours * 3600);
            tokio::time::sleep(interval).await;
        }
    })
}
