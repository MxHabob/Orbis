//! Default entitlement provider — always returns Unlicensed (unrestricted).
//!
//! This is the default implementation for the current phase.
//! When the external Licensing Service is integrated, this will be replaced
//! by a provider that calls the licensing API.

use async_trait::async_trait;
use orbis_app::ports::entitlement::EntitlementProvider;
use orbis_domain::entities::entitlement::EntitlementResult;
use orbis_domain::errors::DomainError;

/// Default entitlement provider — no licensing service integrated.
/// Always returns `Unlicensed` with no restrictions.
pub struct UnlicensedEntitlementProvider;

#[async_trait]
impl EntitlementProvider for UnlicensedEntitlementProvider {
    async fn check_entitlement(
        &self,
        _product_id: &str,
        _org_id: &str,
    ) -> Result<EntitlementResult, DomainError> {
        Ok(EntitlementResult::unlicensed())
    }
}
