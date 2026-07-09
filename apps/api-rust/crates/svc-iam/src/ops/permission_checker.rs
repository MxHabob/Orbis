//! Concrete `PermissionChecker` implementation — wraps `AuthzService`.
//!
//! This bridges the existing `AuthzService` (which takes `user::Model`)
//! to the `PermissionChecker` trait (which takes `Actor`).

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};

use shared_kernel::errors::AppError;
use shared_kernel::traits::{Actor, AuthzContext, Permission, PermissionChecker};

use crate::entities::user;
use crate::services::authz::AuthzService;

/// Concrete `PermissionChecker` backed by the existing `AuthzService`.
///
/// Resolves `Actor` → `user::Model` and delegates to `AuthzService::check_permission`.
pub struct IamPermissionChecker {
    db: DatabaseConnection,
    authz: AuthzService,
}

impl IamPermissionChecker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            authz: AuthzService::new(),
        }
    }
}

#[async_trait]
impl PermissionChecker for IamPermissionChecker {
    async fn check(&self, actor: &Actor, permission: Permission) -> Result<(), AppError> {
        match actor {
            // System and Autopilot bypass all permission checks
            Actor::System | Actor::Autopilot => Ok(()),

            // AI agents also bypass (they have their own risk-level gates)
            Actor::Agent(_) => Ok(()),

            // Users go through the full permission check
            Actor::User(user_id) => {
                let user_model = user::Entity::find()
                    .filter(user::Column::Id.eq(user_id.as_str()))
                    .one(&self.db)
                    .await?
                    .ok_or_else(|| AppError::unauthorized("User not found"))?;

                // Map shared-kernel Permission to domain-iam Permission
                let iam_permission = map_permission(permission);
                self.authz.check_permission(&self.db, &user_model, iam_permission, None).await
            }
        }
    }

    async fn check_with_context(
        &self,
        actor: &Actor,
        permission: Permission,
        ctx: &AuthzContext,
    ) -> Result<(), AppError> {
        match actor {
            Actor::System | Actor::Autopilot | Actor::Agent(_) => Ok(()),
            Actor::User(user_id) => {
                let user_model = user::Entity::find()
                    .filter(user::Column::Id.eq(user_id.as_str()))
                    .one(&self.db)
                    .await?
                    .ok_or_else(|| AppError::unauthorized("User not found"))?;

                let iam_ctx = crate::services::authz::AuthzContext {
                    target_user_id: None,
                    target_class_id: ctx.class_id.clone(),
                    target_org_id: ctx.org_unit_id.clone(),
                };

                let iam_permission = map_permission(permission);
                self.authz.check_permission(&self.db, &user_model, iam_permission, Some(&iam_ctx)).await
            }
        }
    }
}

/// Map shared-kernel Permission to domain-iam Permission.
///
/// Both enums have identical variants. This is a temporary bridge
/// until domain-iam uses shared-kernel's Permission directly.
fn map_permission(p: Permission) -> crate::services::authz::Permission {
    // Since both enums have the same variants and layout, we can map by name.
    // Using serde round-trip as a safe mapping strategy.
    let json = serde_json::to_string(&p).unwrap_or_default();
    serde_json::from_str(&json).unwrap_or(crate::services::authz::Permission::OrganizationRead)
}
