use async_trait::async_trait;
use orbis_domain::iam::permission::Permission;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;

#[async_trait]
pub trait PermissionCheckerPort: Send + Sync {
    /// Checks if the actor in the RequestContext has the required permission.
    /// System and Autopilot bypass permissions.
    async fn check(&self, ctx: &RequestContext, permission: Permission) -> Result<(), DomainError>;
}
