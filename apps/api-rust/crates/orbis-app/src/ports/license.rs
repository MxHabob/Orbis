//! License port — Application-layer contract for the Orbis license runtime.
//!
//! This trait defines the boundary between Orbis application services
//! and the Mulverse Platform license integration.
//!
//! The license runtime is responsible for:
//! - Loading and verifying the local license state
//! - Activating product keys (online or offline)
//! - Managing trial evaluation periods
//! - Background synchronization with Mulverse Platform
//! - Entitlement and limit evaluation
//!
//! # Architecture
//!
//! ```text
//! Application Services ──▶ LicenseRuntime (port)
//!                                │
//!                     ┌──────────┴──────────┐
//!                     │                     │
//!              MulverseLicenseRuntime   (test mock)
//!              (infra adapter)
//! ```

use async_trait::async_trait;
use orbis_domain::entities::license::{
    ActivationResponse, LicenseSnapshot, LicenseStateDto, LicenseVerificationError,
};
use orbis_domain::errors::DomainError;
use uuid::Uuid;

/// Port for the Orbis license runtime.
///
/// Implementations must be `Send + Sync` for use across async tasks.
/// The runtime manages the full license lifecycle including activation,
/// verification, trial management, and background sync.
#[async_trait]
pub trait LicenseRuntime: Send + Sync {
    /// Get the current license snapshot.
    ///
    /// This is the primary method used by the application to query license state.
    /// It returns the locally cached state — no network call is made.
    fn current_snapshot(&self) -> LicenseSnapshot;

    /// Get the current license state as a frontend-safe DTO.
    ///
    /// Strips cryptographic material and sensitive data.
    fn current_state_dto(&self) -> LicenseStateDto;

    /// Initialize the license runtime.
    ///
    /// Called once during application startup. Loads persisted state,
    /// verifies signatures, evaluates trial, and starts background sync.
    /// Must NOT block on network — operates offline-first.
    async fn initialize(&self) -> Result<(), DomainError>;

    /// Activate a product key.
    ///
    /// If internet is available, contacts Mulverse Platform for activation.
    /// Stores the resulting signed entitlement locally.
    /// Returns the activation response on success.
    async fn activate_product_key(
        &self,
        product_key: &str,
        organization_id: uuid::Uuid,
    ) -> Result<ActivationResponse, DomainError>;

    /// Start a trial evaluation period.
    ///
    /// Creates a local trial state with the configured trial policy.
    /// Does NOT contact Mulverse Platform.
    /// Returns an error if a trial has already been used.
    async fn start_trial(
        &self,
        organization: orbis_domain::entities::license::TrialOrganizationProfile,
    ) -> Result<LicenseSnapshot, DomainError>;

    /// Refresh the license with Mulverse Platform.
    ///
    /// Called periodically by the background sync task.
    /// Updates the local signed entitlement if Mulverse returns a newer one.
    /// Network failure is NOT a license failure — the existing cached state remains valid.
    async fn refresh(&self, organization_id: Option<uuid::Uuid>) -> Result<(), DomainError>;

    /// Deactivate the current installation.
    ///
    /// Contacts Mulverse Platform to release the device seat.
    /// Clears local license state and returns to Unavailable.
    async fn deactivate(&self) -> Result<(), DomainError>;

    /// Check whether a boolean entitlement is granted.
    ///
    /// Reads from the cached license state — no network call.
    fn is_feature_enabled(&self, key: &str) -> bool;

    /// Authorize an operation for a tenant and optional feature entitlement.
    fn authorize(&self, organization_id: Uuid, feature: Option<&str>) -> Result<(), DomainError> {
        let snapshot = self.current_snapshot();
        if !snapshot.state.is_operational() {
            return Err(DomainError::LicenseRequired(format!(
                "License state {:?} does not permit this operation",
                snapshot.state
            )));
        }

        self.verify_signature()
            .map_err(|error| DomainError::LicenseRequired(error.to_string()))?;

        if let Some(entitlement) = snapshot.signed_entitlement.as_ref() {
            let licensed_organization = Uuid::parse_str(&entitlement.payload.organization_id)
                .map_err(|_| {
                    DomainError::LicenseRequired(
                        "The license is not bound to a valid organization".to_string(),
                    )
                })?;
            if licensed_organization != organization_id {
                return Err(DomainError::Forbidden(
                    "The license is not valid for the active organization".to_string(),
                ));
            }
        }

        if let Some(feature) = feature
            && !self.is_feature_enabled(feature)
        {
            return Err(DomainError::LicenseRequired(format!(
                "The '{feature}' license entitlement is required"
            )));
        }

        Ok(())
    }

    /// Get the limit value for a given limit key.
    ///
    /// Returns `None` if no limit is set (unlimited).
    fn get_limit(&self, key: &str) -> Option<u64>;

    /// Check if creating resources would exceed a license limit.
    ///
    /// Returns `Ok(())` if within limits, `Err` with detail if exceeded.
    fn check_limit(&self, key: &str, current_count: u64, amount: u64) -> Result<(), DomainError>;

    /// Verify the current signed entitlement's cryptographic signature.
    ///
    /// Used during startup and periodic re-verification.
    fn verify_signature(&self) -> Result<(), LicenseVerificationError>;

    /// Get the installation identity for this Orbis instance.
    fn installation_id(&self) -> String;
}
