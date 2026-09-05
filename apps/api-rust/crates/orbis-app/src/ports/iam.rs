use async_trait::async_trait;
use orbis_domain::iam::permission::Permission;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use uuid::Uuid;

#[async_trait]
pub trait PermissionCheckerPort: Send + Sync {
    /// Checks if the actor in the RequestContext has the required permission.
    /// System and Autopilot bypass permissions.
    async fn check(&self, ctx: &RequestContext, permission: Permission) -> Result<(), DomainError>;

    /// Checks if a specific user has a named permission in an org.
    /// Returns false if the user lacks the permission or if there is an error.
    async fn has_permission(&self, user_id: Uuid, org_id: Uuid, permission: &str) -> Result<bool, DomainError> {
        // Default implementation — subclasses can override for efficiency.
        let _ = (user_id, org_id, permission);
        Ok(false)
    }
}
