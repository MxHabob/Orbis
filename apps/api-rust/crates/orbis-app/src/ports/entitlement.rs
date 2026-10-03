//! Entitlement port — Application-layer contract for checking licensing/entitlement status.
//!
//! In this phase, the default implementation always returns `Unlicensed` (unrestricted).
//! When a future external Licensing Service is integrated, only the implementation changes.

use async_trait::async_trait;
use orbis_domain::entities::entitlement::EntitlementResult;
use orbis_domain::errors::DomainError;

/// Port for checking entitlement status.
///
/// Orbis consumes this trait; the Licensing Service produces the implementation.
/// In this phase, the default provider (`UnlicensedEntitlementProvider`) always
/// returns `Unlicensed` with no restrictions.
#[async_trait]
pub trait EntitlementProvider: Send + Sync {
    /// Check current entitlement status for the given organization.
    async fn check_entitlement(
        &self,
        product_id: &str,
        org_id: &str,
    ) -> Result<EntitlementResult, DomainError>;
}
