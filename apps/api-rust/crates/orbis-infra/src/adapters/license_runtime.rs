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
use base64::Engine;
use chrono::Utc;
use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};
use orbis_app::ports::license::LicenseRuntime;
use orbis_domain::entities::license::*;
use orbis_domain::errors::DomainError;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

use super::mulverse_client::MulverseClient;
use super::mulverse_client::{
    MulverseApiError, RefreshTrialRequest, RegisterTrialRequest, TrialChallenge,
    TrialRegistrationResponse,
};
use super::trial_device_identity;

// ═══════════════════════════════════════════════════════════════
// EMBEDDED PUBLIC KEY — Compile-time Ed25519 verification key
// ═══════════════════════════════════════════════════════════════

/// Release builds pin the matching Mulverse Ed25519 public key at compile time.
const EMBEDDED_PUBLIC_KEY_B64: Option<&str> = option_env!("MULVERSE_LICENSE_PUBLIC_KEY_B64");

/// Key ID for the embedded public key.
const EMBEDDED_KEY_ID: &str = match option_env!("MULVERSE_LICENSE_KEY_ID") {
    Some(key_id) => key_id,
    None => "key_ed25519_2026_v1",
};

// ═══════════════════════════════════════════════════════════════
// LICENSE FILE PATHS
// ═══════════════════════════════════════════════════════════════

const LICENSE_SNAPSHOT_FILE: &str = "license_snapshot.json";
const INSTALLATION_ID_FILE: &str = "installation_id";
const PROVISIONAL_TRIAL_DAYS: u32 = 7;

enum TrialSyncError {
    Api(MulverseApiError),
    Domain(DomainError),
}

// ═══════════════════════════════════════════════════════════════
// OFFLINE CRYPTOGRAPHIC VERIFICATION (Contract Section 7)
// ═══════════════════════════════════════════════════════════════

/// يتحقق من صحة الترخيص محلياً بدون إنترنت
pub fn verify_offline_license(
    signed_doc: &SignedEntitlement,
    public_key_base64: &str,
    current_installation_id: &str,
) -> Result<bool, LicenseVerificationError> {
    if signed_doc.v != 1 || signed_doc.alg != "Ed25519" {
        return Err(LicenseVerificationError::InvalidPayload);
    }

    // 1. فحص تطابق معرف التثبيت للجهاز
    if signed_doc.payload.installation_id != current_installation_id {
        return Err(LicenseVerificationError::InstallationIdMismatch);
    }

    // 2. فحص تطابق معرف المنتج
    if signed_doc.payload.product_id != CANONICAL_PRODUCT_ID {
        return Err(LicenseVerificationError::ProductMismatch);
    }

    // 3. التحقق من التوقيع التشفيري Ed25519
    let pub_key_bytes = base64::engine::general_purpose::STANDARD
        .decode(public_key_base64)
        .map_err(|_| LicenseVerificationError::InvalidPublicKey)?;

    let verifying_key = VerifyingKey::from_bytes(
        pub_key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| LicenseVerificationError::InvalidPublicKey)?,
    )
    .map_err(|_| LicenseVerificationError::InvalidPublicKey)?;

    let canonical_payload = serde_json::to_string(&signed_doc.payload)
        .map_err(|_| LicenseVerificationError::InvalidPayload)?;

    let sig_bytes = base64::engine::general_purpose::STANDARD
        .decode(&signed_doc.sig)
        .map_err(|_| LicenseVerificationError::InvalidSignatureEncoding)?;

    let signature = Signature::from_slice(&sig_bytes)
        .map_err(|_| LicenseVerificationError::InvalidSignatureEncoding)?;

    verifying_key
        .verify(canonical_payload.as_bytes(), &signature)
        .map_err(|_| LicenseVerificationError::TamperedLicenseSignature)?;

    Ok(true)
}

// ═══════════════════════════════════════════════════════════════
// MULVERSE LICENSE RUNTIME
// ═══════════════════════════════════════════════════════════════

/// The Mulverse license runtime — implements the `LicenseRuntime` port.
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
    /// Public keys pinned into the application build (kid → base64 public key).
    trusted_keys: Vec<(String, String)>,
    lifecycle_lock: Mutex<()>,
}

impl MulverseLicenseRuntime {
    /// Create a new license runtime.
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

        let trusted_keys = match EMBEDDED_PUBLIC_KEY_B64 {
            Some(key) if !key.trim().is_empty() => {
                vec![(EMBEDDED_KEY_ID.to_string(), key.to_string())]
            }
            _ => Vec::new(),
        };

        Ok(Self {
            app_data_dir: app_data_dir.to_path_buf(),
            snapshot: RwLock::new(initial_snapshot),
            app_version: app_version.to_string(),
            mulverse_client,
            trial_policy: TrialPolicy::default(),
            trusted_keys,
            lifecycle_lock: Mutex::new(()),
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

        std::fs::write(&path, json)
            .map_err(|e| DomainError::Internal(format!("Failed to write license snapshot: {e}")))?;

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
    fn evaluate_state(&self, snapshot: &mut LicenseSnapshot) {
        let now = Utc::now();

        if matches!(
            snapshot.state,
            LicenseState::Suspended
                | LicenseState::Revoked
                | LicenseState::DeactivationPending
                | LicenseState::Invalid
        ) {
            return;
        }

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
            if now < se.payload.valid_from {
                snapshot.state = LicenseState::Pending;
                return;
            }

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
                let days_remaining = valid_until.signed_duration_since(now).num_days();
                if days_remaining <= 14 && days_remaining > 0 {
                    snapshot.state = LicenseState::Expiring;
                    return;
                }
            }

            if se.payload.plan_key.eq_ignore_ascii_case("TRIAL")
                && se.payload.organization_id.is_empty()
            {
                snapshot.state = LicenseState::Pending;
                return;
            }

            // Check offline grace period
            if let Some(last_sync) = snapshot.last_sync_at {
                let days_offline = now.signed_duration_since(last_sync).num_days();
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
        if self.trusted_keys.is_empty() {
            return Err(LicenseVerificationError::NoTrustedKeysConfigured);
        }

        let key_b64 = self
            .trusted_keys
            .iter()
            .find(|(kid, _)| *kid == entitlement.kid)
            .map(|(_, key)| key.as_str())
            .ok_or(LicenseVerificationError::UnknownKeyId)?;

        let current_id = self.installation_id();
        verify_offline_license(entitlement, key_b64, &current_id)?;

        debug!(kid = %entitlement.kid, "Ed25519 signature verified successfully");
        Ok(())
    }

    async fn sign_trial_challenge(
        &self,
        challenge: &TrialChallenge,
        fields: &[String],
        installation_id: &str,
    ) -> Result<(String, String), DomainError> {
        let app_data_dir = self.app_data_dir.clone();
        let installation_id = installation_id.to_owned();
        let key = tokio::task::spawn_blocking(move || {
            trial_device_identity::load_or_create(&app_data_dir, &installation_id)
        })
        .await
        .map_err(|error| DomainError::Internal(format!("Trial identity worker failed: {error}")))?
        .map_err(DomainError::Internal)?;
        let mut message = String::from("mulverse-trial-v1");
        for field in fields {
            message.push('\n');
            message.push_str(field);
        }
        message.push('\n');
        message.push_str(&challenge.challenge);
        let signature = key.sign(message.as_bytes());
        Ok((
            base64::engine::general_purpose::STANDARD.encode(key.verifying_key().to_bytes()),
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes()),
        ))
    }

    async fn register_central_trial(&self) -> Result<TrialRegistrationResponse, TrialSyncError> {
        let challenge = self
            .mulverse_client
            .create_trial_challenge()
            .await
            .map_err(TrialSyncError::Api)?;
        let installation_id = self.installation_id();
        let fields = vec![
            "register".to_owned(),
            challenge.challenge_id.clone(),
            installation_id.clone(),
            std::env::consts::OS.to_owned(),
            self.app_version.clone(),
        ];
        let (device_public_key, device_signature) = self
            .sign_trial_challenge(&challenge, &fields, &installation_id)
            .await
            .map_err(TrialSyncError::Domain)?;
        self.mulverse_client
            .register_trial(&RegisterTrialRequest {
                device_public_key,
                device_signature,
                challenge_id: challenge.challenge_id,
                challenge: challenge.challenge,
                installation_id,
                platform: std::env::consts::OS.to_owned(),
                app_version: self.app_version.clone(),
                machine_label: hostname::get()
                    .ok()
                    .and_then(|value| value.into_string().ok()),
            })
            .await
            .map_err(TrialSyncError::Api)
    }

    async fn refresh_central_trial(
        &self,
        trial_id: &str,
    ) -> Result<TrialRegistrationResponse, TrialSyncError> {
        let challenge = self
            .mulverse_client
            .create_trial_challenge()
            .await
            .map_err(TrialSyncError::Api)?;
        let installation_id = self.installation_id();
        let fields = vec![
            "refresh".to_owned(),
            challenge.challenge_id.clone(),
            trial_id.to_owned(),
            installation_id.clone(),
            self.app_version.clone(),
        ];
        let (device_public_key, device_signature) = self
            .sign_trial_challenge(&challenge, &fields, &installation_id)
            .await
            .map_err(TrialSyncError::Domain)?;
        self.mulverse_client
            .refresh_trial(&RefreshTrialRequest {
                trial_id: trial_id.to_owned(),
                device_public_key,
                device_signature,
                challenge_id: challenge.challenge_id,
                challenge: challenge.challenge,
                installation_id,
                app_version: self.app_version.clone(),
            })
            .await
            .map_err(TrialSyncError::Api)
    }

    fn snapshot_from_trial_response(
        &self,
        current: &LicenseSnapshot,
        response: TrialRegistrationResponse,
    ) -> Result<LicenseSnapshot, DomainError> {
        if response.trial.status != "active"
            || response.trial.installation_id != current.installation_id
        {
            return Err(DomainError::Forbidden(
                "Mulverse returned an inactive or mismatched trial device".to_owned(),
            ));
        }
        let entitlement = response.signed_entitlement;
        if entitlement.payload.license_id != response.trial.id
            || !entitlement.payload.plan_key.eq_ignore_ascii_case("TRIAL")
            || entitlement.payload.product_id != CANONICAL_PRODUCT_ID
            || entitlement.payload.installation_id != current.installation_id
        {
            return Err(DomainError::Internal(
                "Mulverse returned a trial entitlement bound to different device data".to_owned(),
            ));
        }
        self.verify_entitlement_signature(&entitlement)
            .map_err(|error| {
                DomainError::Internal(format!(
                    "Central trial entitlement signature verification failed: {error}"
                ))
            })?;
        if entitlement
            .payload
            .valid_until
            .map_or(true, |expires_at| expires_at <= Utc::now())
        {
            return Err(DomainError::Forbidden(
                "Mulverse returned an expired trial entitlement".to_owned(),
            ));
        }

        let mut snapshot = LicenseSnapshot::new_unavailable(
            current.installation_id.clone(),
            self.app_version.clone(),
        );
        snapshot.signed_entitlement = Some(entitlement.clone());
        snapshot.trial_started_at = Some(entitlement.payload.valid_from);
        snapshot.trial_policy.duration_days = self.trial_policy.duration_days;
        snapshot.last_sync_at = Some(Utc::now());
        snapshot.last_known_time = Utc::now();
        self.evaluate_state(&mut snapshot);
        Ok(snapshot)
    }

    fn provisional_trial_snapshot(
        &self,
        current: &LicenseSnapshot,
    ) -> Result<LicenseSnapshot, DomainError> {
        let mut snapshot = current.clone();
        if snapshot.trial_started_at.is_none() {
            snapshot = LicenseSnapshot::new_trial(
                current.installation_id.clone(),
                self.app_version.clone(),
                self.trial_policy.clone(),
            );
        }
        snapshot.trial_policy.duration_days = PROVISIONAL_TRIAL_DAYS;
        snapshot.last_known_time = Utc::now();
        self.evaluate_state(&mut snapshot);
        if snapshot.state != LicenseState::Trial {
            return Err(DomainError::Conflict(
                "The seven-day offline trial period has expired. Connect to Mulverse to continue."
                    .to_owned(),
            ));
        }
        Ok(snapshot)
    }
}

#[async_trait]
impl LicenseRuntime for MulverseLicenseRuntime {
    fn current_snapshot(&self) -> LicenseSnapshot {
        let mut snapshot = match self.snapshot.write() {
            Ok(mut guard) => {
                if guard.signed_entitlement.is_none() && guard.trial_started_at.is_some() {
                    guard.trial_policy.duration_days =
                        guard.trial_policy.duration_days.min(PROVISIONAL_TRIAL_DAYS);
                }
                self.evaluate_state(&mut guard);
                guard.clone()
            }
            Err(_) => {
                LicenseSnapshot::new_unavailable("unknown".to_string(), self.app_version.clone())
            }
        };
        if snapshot.signed_entitlement.is_none() && snapshot.trial_started_at.is_some() {
            snapshot.trial_policy.duration_days = snapshot
                .trial_policy
                .duration_days
                .min(PROVISIONAL_TRIAL_DAYS);
        }
        snapshot
    }

    fn current_state_dto(&self) -> LicenseStateDto {
        LicenseStateDto::from(&self.current_snapshot())
    }

    async fn initialize(&self) -> Result<(), DomainError> {
        info!("Initializing Orbis license runtime");

        if let Some(mut snapshot) = self.load_persisted_snapshot() {
            let mut entitlement_invalid = false;
            if let Some(entitlement) = snapshot.signed_entitlement.as_ref() {
                match self.verify_entitlement_signature(entitlement) {
                    Ok(()) => {
                        debug!("Persisted signed entitlement verified");
                    }
                    Err(e) => {
                        warn!("Persisted signed entitlement verification failed: {e}");
                        snapshot.signed_entitlement = None;
                        snapshot.state = LicenseState::Invalid;
                        entitlement_invalid = true;
                    }
                }
            }

            let current_id = load_or_generate_installation_id(&self.app_data_dir)
                .unwrap_or_else(|_| "unknown".to_string());

            if snapshot.installation_id != current_id {
                warn!(
                    expected = %current_id,
                    found = %snapshot.installation_id,
                    "Installation ID mismatch — updating to current installation"
                );
                snapshot.installation_id = current_id;
            }

            if snapshot.signed_entitlement.is_none() && snapshot.trial_started_at.is_some() {
                snapshot.trial_policy.duration_days = snapshot
                    .trial_policy
                    .duration_days
                    .min(PROVISIONAL_TRIAL_DAYS);
            }

            if !entitlement_invalid && snapshot.state != LicenseState::Suspended {
                self.evaluate_state(&mut snapshot);
            }
            snapshot.app_version = self.app_version.clone();
            self.update_snapshot(snapshot)?;

            info!(state = ?self.current_snapshot().state, "License runtime initialized from persisted state");
        } else {
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
        let _lifecycle_guard = self.lifecycle_lock.lock().await;
        validate_product_key(product_key).map_err(DomainError::Validation)?;

        let installation_id = self.installation_id();

        let request = ActivationRequest {
            product_key: product_key.trim().to_uppercase(),
            installation_id: installation_id.clone(),
            platform: std::env::consts::OS.to_string(),
            app_version: self.app_version.clone(),
            machine_label: hostname::get().ok().and_then(|h| h.into_string().ok()),
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

        if let Err(e) = self.verify_entitlement_signature(&response.signed_entitlement) {
            error!("Received signed entitlement failed verification: {e}");
            return Err(DomainError::Internal(format!(
                "License verification failed: {e}"
            )));
        }

        if response.signed_entitlement.payload.installation_id != installation_id {
            error!("Activation response installation_id mismatch");
            return Err(DomainError::Internal(
                "Activation response installation ID mismatch".to_string(),
            ));
        }

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
        if response.activation.status.eq_ignore_ascii_case("suspended") {
            snapshot.state = LicenseState::Suspended;
        }
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
        let _lifecycle_guard = self.lifecycle_lock.lock().await;
        let current = self.current_snapshot();

        if current.signed_entitlement.is_some() {
            return Err(DomainError::Conflict(
                "Cannot start trial — a valid license is already active".to_string(),
            ));
        }

        match self.register_central_trial().await {
            Ok(response) => {
                let snapshot = self.snapshot_from_trial_response(&current, response)?;
                self.update_snapshot(snapshot.clone())?;
                info!("Centralized trial registration succeeded");
                Ok(snapshot)
            }
            Err(TrialSyncError::Api(error)) if error.is_network_error() => {
                let snapshot = self.provisional_trial_snapshot(&current)?;
                self.update_snapshot(snapshot.clone())?;
                warn!("Mulverse unavailable; started a seven-day provisional offline trial");
                Ok(snapshot)
            }
            Err(TrialSyncError::Api(error)) => Err(DomainError::from(error)),
            Err(TrialSyncError::Domain(error)) => Err(error),
        }
    }

    async fn refresh(&self) -> Result<(), DomainError> {
        let _lifecycle_guard = self.lifecycle_lock.lock().await;
        let current = self.current_snapshot();

        if current.state == LicenseState::DeactivationPending {
            return Err(DomainError::Conflict(
                "License deactivation is pending; retry deactivation before refreshing".to_string(),
            ));
        }

        let se = match &current.signed_entitlement {
            Some(se) => se.clone(),
            None => {
                if current.trial_started_at.is_some() {
                    return match self.register_central_trial().await {
                        Ok(response) => {
                            let snapshot = self.snapshot_from_trial_response(&current, response)?;
                            self.update_snapshot(snapshot)?;
                            Ok(())
                        }
                        Err(TrialSyncError::Api(error)) if error.is_network_error() => {
                            let mut snapshot = current;
                            self.evaluate_state(&mut snapshot);
                            self.update_snapshot(snapshot)?;
                            Ok(())
                        }
                        Err(TrialSyncError::Api(error)) if error.is_license_error() => {
                            let mut snapshot = current;
                            snapshot.state = LicenseState::Revoked;
                            snapshot.last_known_time = Utc::now();
                            self.update_snapshot(snapshot)?;
                            Err(DomainError::from(error))
                        }
                        Err(TrialSyncError::Api(error)) => Err(DomainError::from(error)),
                        Err(TrialSyncError::Domain(error)) => Err(error),
                    };
                }
                debug!("No signed entitlement to refresh");
                return Ok(());
            }
        };

        if se.payload.plan_key.eq_ignore_ascii_case("TRIAL") {
            return match self.refresh_central_trial(&se.payload.license_id).await {
                Ok(response) => {
                    let snapshot = self.snapshot_from_trial_response(&current, response)?;
                    self.update_snapshot(snapshot)?;
                    Ok(())
                }
                Err(TrialSyncError::Api(error)) if error.is_network_error() => {
                    let mut snapshot = current;
                    self.evaluate_state(&mut snapshot);
                    self.update_snapshot(snapshot)?;
                    Ok(())
                }
                Err(TrialSyncError::Api(error)) if error.is_license_error() => {
                    let mut snapshot = current;
                    snapshot.state = LicenseState::Revoked;
                    snapshot.last_known_time = Utc::now();
                    self.update_snapshot(snapshot)?;
                    warn!(error = %error, "Central trial was denied by Mulverse");
                    Ok(())
                }
                Err(TrialSyncError::Api(error)) => Err(DomainError::from(error)),
                Err(TrialSyncError::Domain(error)) => Err(error),
            };
        }

        let request = RefreshRequest {
            license_id: se.payload.license_id.clone(),
            installation_id: current.installation_id.clone(),
            app_version: self.app_version.clone(),
        };

        match self.mulverse_client.refresh(&request).await {
            Ok(response) => {
                if let Err(e) = self.verify_entitlement_signature(&response.signed_entitlement) {
                    warn!("Refreshed entitlement failed verification: {e}");
                    return Ok(());
                }

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

                let mut snapshot = current;
                snapshot.signed_entitlement = Some(response.signed_entitlement);
                snapshot.last_sync_at = Some(Utc::now());
                snapshot.last_known_time = Utc::now();
                snapshot.state = LicenseState::Active;
                self.evaluate_state(&mut snapshot);
                if response.activation.status.eq_ignore_ascii_case("suspended") {
                    snapshot.state = LicenseState::Suspended;
                }
                self.update_snapshot(snapshot)?;

                debug!("License refreshed successfully");
            }
            Err(e) => {
                if e.is_network_error() {
                    debug!("License refresh failed (network): {e}");
                } else if e.is_license_error() {
                    warn!("License refresh denied by Mulverse: {e}");
                    let mut snapshot = current;
                    snapshot.state = LicenseState::Revoked;
                    snapshot.last_known_time = Utc::now();
                    self.update_snapshot(snapshot)?;
                } else {
                    debug!("License refresh failed: {e}");
                }
            }
        }

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), DomainError> {
        let _lifecycle_guard = self.lifecycle_lock.lock().await;
        let current = self.current_snapshot();

        if let Some(ref se) = current.signed_entitlement
            && !se.payload.plan_key.eq_ignore_ascii_case("TRIAL")
        {
            let request = DeactivationRequest {
                license_id: se.payload.license_id.clone(),
                installation_id: current.installation_id.clone(),
            };

            match self.mulverse_client.deactivate(&request).await {
                Ok(()) => {
                    info!(
                        license_id = %se.payload.license_id,
                        "Installation deactivated with Mulverse Platform"
                    );
                }
                Err(e) => {
                    warn!("Failed to deactivate with Mulverse; preserving state for retry: {e}");
                    let mut pending = current.clone();
                    pending.state = LicenseState::DeactivationPending;
                    self.update_snapshot(pending)?;
                    return Err(DomainError::Internal(format!(
                        "License deactivation is pending because Mulverse could not confirm it: {e}"
                    )));
                }
            }
        }

        let mut snapshot = LicenseSnapshot::new_unavailable(
            current.installation_id.clone(),
            self.app_version.clone(),
        );
        snapshot.trial_started_at = current.trial_started_at;
        snapshot.trial_policy = current.trial_policy;
        self.update_snapshot(snapshot)?;

        info!("License deactivation confirmed and local state cleared");
        Ok(())
    }

    fn is_feature_enabled(&self, key: &str) -> bool {
        self.current_snapshot().is_feature_enabled(key)
    }

    fn get_limit(&self, key: &str) -> Option<u64> {
        self.current_snapshot()
            .get_limit(key)
            .filter(|value| *value >= 0)
            .map(|value| value as u64)
    }

    fn check_limit(&self, key: &str, current_count: u64, amount: u64) -> Result<(), DomainError> {
        self.current_snapshot()
            .check_limit(key, current_count, amount)
            .map_err(DomainError::Forbidden)
    }

    fn verify_signature(&self) -> Result<(), LicenseVerificationError> {
        let snapshot = self.current_snapshot();
        match &snapshot.signed_entitlement {
            Some(se) => self.verify_entitlement_signature(se),
            None => Ok(()),
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
            LicenseState::Trial => orbis_domain::entities::entitlement::EntitlementStatus::Trial,
            LicenseState::Pending => {
                orbis_domain::entities::entitlement::EntitlementStatus::Pending
            }
            LicenseState::OfflineGrace => {
                orbis_domain::entities::entitlement::EntitlementStatus::GracePeriod
            }
            LicenseState::Expired | LicenseState::Revoked | LicenseState::Invalid => {
                orbis_domain::entities::entitlement::EntitlementStatus::Expired
            }
            LicenseState::Suspended => {
                orbis_domain::entities::entitlement::EntitlementStatus::Suspended
            }
            LicenseState::DeactivationPending => {
                orbis_domain::entities::entitlement::EntitlementStatus::Suspended
            }
            LicenseState::Unavailable => {
                orbis_domain::entities::entitlement::EntitlementStatus::Unlicensed
            }
        };

        let plan = snapshot.plan_key().map(String::from);
        let max_users = snapshot
            .get_limit(LIMIT_STAFF_MAX)
            .or_else(|| snapshot.get_limit("users.max"))
            .map(|v| v as u32);
        let max_students = snapshot.get_limit(LIMIT_STUDENTS_MAX).map(|v| v as u32);

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
                    started + chrono::Duration::days(snapshot.trial_policy.duration_days as i64)
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
        std::fs::write(&path, &id).map_err(|e| format!("Failed to write installation ID: {e}"))?;

        info!(installation_id = %id, "Generated new installation identity");
        Ok(id)
    }
}

// ═══════════════════════════════════════════════════════════════
// BACKGROUND SYNC TASK
// ═══════════════════════════════════════════════════════════════

pub fn spawn_background_sync(
    runtime: std::sync::Arc<dyn LicenseRuntime>,
    initial_delay_secs: u64,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(initial_delay_secs)).await;

        loop {
            match runtime.refresh().await {
                Ok(()) => debug!("Background license sync completed"),
                Err(e) => debug!("Background license sync failed (non-fatal): {e}"),
            }

            let interval_hours = {
                let snapshot = runtime.current_snapshot();
                snapshot
                    .signed_entitlement
                    .as_ref()
                    .map(|se| se.payload.verification_interval_hours as u64)
                    .unwrap_or(DEFAULT_VERIFICATION_INTERVAL_HOURS as u64)
            };

            let interval = std::time::Duration::from_secs(interval_hours * 3600);
            tokio::time::sleep(interval).await;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signed_entitlement(
        installation_id: &str,
        valid_from: chrono::DateTime<Utc>,
    ) -> SignedEntitlement {
        SignedEntitlement {
            v: 1,
            kid: EMBEDDED_KEY_ID.to_string(),
            alg: "Ed25519".to_string(),
            payload: EntitlementPayload {
                license_id: "license-test".to_string(),
                product_key: "ORBIS-ABCDE-FGHIJ-KLMNO-PQRST".to_string(),
                product_id: CANONICAL_PRODUCT_ID.to_string(),
                organization_id: "org-test".to_string(),
                edition_key: "SCHOOL".to_string(),
                plan_key: "SCHOOL_PRO".to_string(),
                entitlements: vec![],
                limits: vec![],
                activation_id: "activation-test".to_string(),
                installation_id: installation_id.to_string(),
                issued_at: Utc::now(),
                valid_from,
                valid_until: Some(Utc::now() + chrono::Duration::days(30)),
                offline_allowed: true,
                offline_grace_period_days: 30,
                verification_interval_hours: 24,
            },
            sig: String::new(),
        }
    }

    fn test_runtime() -> (MulverseLicenseRuntime, PathBuf) {
        let app_data_dir = std::env::temp_dir().join(format!(
            "orbis-license-runtime-test-{}",
            uuid::Uuid::new_v4()
        ));
        let runtime = MulverseLicenseRuntime::new(&app_data_dir, "test", Some("http://127.0.0.1"))
            .expect("test runtime should initialize");
        (runtime, app_data_dir)
    }

    #[test]
    fn missing_pinned_key_fails_license_verification_closed() {
        let (mut runtime, app_data_dir) = test_runtime();
        runtime.trusted_keys.clear();

        let entitlement = signed_entitlement(
            &runtime.installation_id(),
            Utc::now() + chrono::Duration::days(1),
        );
        assert_eq!(
            runtime.verify_entitlement_signature(&entitlement),
            Err(LicenseVerificationError::NoTrustedKeysConfigured)
        );

        std::fs::remove_dir_all(app_data_dir).expect("test data should be cleaned up");
    }

    #[test]
    fn entitlement_outside_validity_window_is_pending() {
        let (runtime, app_data_dir) = test_runtime();
        let mut snapshot = runtime.current_snapshot();
        snapshot.signed_entitlement = Some(signed_entitlement(
            &snapshot.installation_id,
            Utc::now() + chrono::Duration::days(1),
        ));

        runtime.evaluate_state(&mut snapshot);

        assert_eq!(snapshot.state, LicenseState::Pending);
        std::fs::remove_dir_all(app_data_dir).expect("test data should be cleaned up");
    }

    #[test]
    fn unsupported_signature_algorithm_is_rejected() {
        let entitlement = signed_entitlement("inst_test", Utc::now() + chrono::Duration::days(1));
        let mut entitlement = entitlement;
        entitlement.alg = "RS256".to_string();

        assert_eq!(
            verify_offline_license(&entitlement, "", "inst_test"),
            Err(LicenseVerificationError::InvalidPayload)
        );
    }

    #[test]
    fn provisional_offline_trial_expires_after_seven_days() {
        let (runtime, app_data_dir) = test_runtime();
        let mut snapshot = runtime.current_snapshot();
        snapshot.trial_started_at = Some(Utc::now() - chrono::Duration::days(8));
        snapshot.trial_policy.duration_days = 30;
        snapshot.state = LicenseState::Trial;
        runtime
            .update_snapshot(snapshot)
            .expect("trial snapshot should persist");

        let current = runtime.current_snapshot();
        assert_eq!(current.trial_policy.duration_days, PROVISIONAL_TRIAL_DAYS);
        assert_eq!(current.state, LicenseState::Expired);
        assert_eq!(runtime.current_snapshot().state, LicenseState::Expired);
        std::fs::remove_dir_all(app_data_dir).expect("test data should be cleaned up");
    }

    #[test]
    fn provisional_offline_trial_is_operational_within_seven_days() {
        let (runtime, app_data_dir) = test_runtime();
        let mut snapshot = runtime.current_snapshot();
        snapshot.trial_started_at = Some(Utc::now() - chrono::Duration::days(6));
        snapshot.trial_policy.duration_days = 30;
        snapshot.state = LicenseState::Trial;
        runtime
            .update_snapshot(snapshot)
            .expect("trial snapshot should persist");

        let current = runtime.current_snapshot();
        assert_eq!(current.trial_policy.duration_days, PROVISIONAL_TRIAL_DAYS);
        assert_eq!(current.state, LicenseState::Trial);
        std::fs::remove_dir_all(app_data_dir).expect("test data should be cleaned up");
    }

    #[test]
    fn unbound_central_trial_waits_for_organization_assignment() {
        let (runtime, app_data_dir) = test_runtime();
        let mut snapshot = runtime.current_snapshot();
        let mut entitlement = signed_entitlement(
            &snapshot.installation_id,
            Utc::now() - chrono::Duration::days(1),
        );
        entitlement.payload.plan_key = "TRIAL".to_owned();
        entitlement.payload.organization_id.clear();
        entitlement.payload.offline_grace_period_days = PROVISIONAL_TRIAL_DAYS;
        snapshot.signed_entitlement = Some(entitlement);
        snapshot.last_sync_at = Some(Utc::now());
        snapshot.state = LicenseState::Active;

        runtime.evaluate_state(&mut snapshot);

        assert_eq!(snapshot.state, LicenseState::Pending);
        std::fs::remove_dir_all(app_data_dir).expect("test data should be cleaned up");
    }
}
