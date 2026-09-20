use async_trait::async_trait;
use orbis_app::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use orbis_domain::errors::DomainError;
use orbis_app::context::RequestContext;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::models::{user_role_assignment, role_permission};

pub struct DbPermissionChecker {
    pub db: DatabaseConnection,
}

impl DbPermissionChecker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PermissionCheckerPort for DbPermissionChecker {
    async fn check(&self, ctx: &RequestContext, permission: Permission) -> Result<(), DomainError> {
        // System and Autopilot bypass permission checks.
        if ctx.actor.is_system() {
            return Ok(());
        }

        let actor_id = ctx.actor.id().ok_or_else(|| DomainError::Validation("Missing actor ID".into()))?;
        let perm_str = permission.as_str();

        // 1. Get all roles assigned to the user
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(actor_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if user_roles.is_empty() {
            return Err(DomainError::Unauthorized(format!("User has no roles, missing permission: {}", perm_str)));
        }

        let role_ids: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        // 2. Query all permissions granted to these roles
        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(role_ids))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut parts = perm_str.split(':');
        let target_resource = parts.next().unwrap_or("").to_lowercase();
        let target_action = parts.next().unwrap_or("").to_lowercase();

        let is_granted = role_permissions.iter().any(|rp| {
            let r_res = rp.resource.to_lowercase();
            let r_act = rp.action.to_lowercase();

            // Universal wildcard permission
            if r_res == "*" {
                return true;
            }
            // Exact compound string match
            if r_res == perm_str {
                return true;
            }
            // Domain wildcard or action-level matching
            let res_ok = r_res == target_resource || r_res == "*";
            let act_ok = r_act == target_action || r_act == "*" || r_act == "admin" || (r_act == "write" && target_action == "read");
            res_ok && act_ok
        });

        if !is_granted {
            return Err(DomainError::Unauthorized(format!("Missing permission: {}", perm_str)));
        }

        Ok(())
    }

    async fn has_permission(&self, user_id: uuid::Uuid, _org_id: uuid::Uuid, permission: &str) -> Result<bool, DomainError> {
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if user_roles.is_empty() {
            return Ok(false);
        }

        let role_ids: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(role_ids))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut parts = permission.split(':');
        let target_resource = parts.next().unwrap_or("").to_lowercase();
        let target_action = parts.next().unwrap_or("").to_lowercase();

        let is_granted = role_permissions.iter().any(|rp| {
            let r_res = rp.resource.to_lowercase();
            let r_act = rp.action.to_lowercase();

            if r_res == "*" {
                return true;
            }
            if r_res == permission {
                return true;
            }
            let res_ok = r_res == target_resource || r_res == "*";
            let act_ok = r_act == target_action || r_act == "*" || r_act == "admin" || (r_act == "write" && target_action == "read");
            res_ok && act_ok
        });

        Ok(is_granted)
    }

    async fn get_user_permissions(&self, user_id: uuid::Uuid, _org_id: uuid::Uuid) -> Result<Vec<String>, DomainError> {
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if user_roles.is_empty() {
            return Ok(vec![]);
        }

        let role_ids: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(role_ids))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut perms: Vec<String> = role_permissions
            .into_iter()
            .map(|rp| {
                if rp.resource == "*" {
                    "*".to_string()
                } else if rp.resource.contains(':') {
                    rp.resource.to_lowercase()
                } else if rp.action.is_empty() || rp.action == "*" {
                    rp.resource.to_lowercase()
                } else {
                    format!("{}:{}", rp.resource.to_lowercase(), rp.action.to_lowercase())
                }
            })
            .collect();

        perms.sort();
        perms.dedup();
        Ok(perms)
    }

    async fn get_user_roles(&self, user_id: uuid::Uuid, _org_id: uuid::Uuid) -> Result<Vec<String>, DomainError> {
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut roles: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();
        roles.sort();
        roles.dedup();
        Ok(roles)
    }
}
