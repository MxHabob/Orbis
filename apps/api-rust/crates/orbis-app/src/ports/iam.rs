use crate::context::RequestContext;
use async_trait::async_trait;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use uuid::Uuid;

#[async_trait]
pub trait PermissionCheckerPort: Send + Sync {
    /// Checks if the actor in the RequestContext has the required permission.
    /// System and Autopilot bypass permissions.
    async fn check(&self, ctx: &RequestContext, permission: Permission) -> Result<(), DomainError>;

    /// Checks if a specific user has a named permission in an org.
    /// Returns false if the user lacks the permission or if there is an error.
    async fn has_permission(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        permission: &str,
    ) -> Result<bool, DomainError> {
        let _ = (user_id, org_id, permission);
        Ok(false)
    }

    /// Retrieves all effective permission names for a user in an organization.
    async fn get_user_permissions(
        &self,
        user_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<String>, DomainError> {
        let _ = (user_id, org_id);
        Ok(vec![])
    }

    /// Retrieves all assigned role IDs for a user in an organization.
    async fn get_user_roles(
        &self,
        user_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<String>, DomainError> {
        let _ = (user_id, org_id);
        Ok(vec![])
    }
}
