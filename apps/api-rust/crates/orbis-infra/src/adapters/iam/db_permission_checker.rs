use crate::models::{
    group_permission, role_permission, user_group_membership, user_role_assignment,
};
use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::iam::PermissionCheckerPort;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

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

        let actor_id = ctx
            .actor
            .id()
            .ok_or_else(|| DomainError::Validation("Missing actor ID".into()))?;
        let perm_str = permission.as_str();

        let has_perm = self
            .has_permission(actor_id, ctx.tenant_id, perm_str)
            .await?;
        if !has_perm {
            return Err(DomainError::Unauthorized(format!(
                "Missing permission: {}",
                perm_str
            )));
        }

        Ok(())
    }

    async fn has_permission(
        &self,
        user_id: uuid::Uuid,
        org_id: uuid::Uuid,
        permission: &str,
    ) -> Result<bool, DomainError> {
        // 1. Check direct role permissions
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .filter(user_role_assignment::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let role_ids: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        let mut parts = permission.split(':');
        let target_resource = parts.next().unwrap_or("").to_lowercase();
        let target_action = parts.next().unwrap_or("").to_lowercase();

        if !role_ids.is_empty() {
            let role_permissions = role_permission::Entity::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids))
                .filter(role_permission::Column::OrgId.eq(org_id))
                .all(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            let is_granted = role_permissions.iter().any(|rp| {
                let r_res = rp.resource.to_lowercase();
                let r_act = rp.action.to_lowercase();

                if r_res == "*" {
                    return true;
                }
                if r_res == permission {
                    return true;
                }
                let res_ok = r_res == target_resource
                    || r_res == "*"
                    || (r_res == "class" && target_resource == "section")
                    || (r_res == "section" && target_resource == "class");
                let act_ok = r_act == target_action
                    || r_act == "*"
                    || r_act == "admin"
                    || (r_act == "write" && target_action == "read");
                res_ok && act_ok
            });

            if is_granted {
                return Ok(true);
            }
        }

        // 2. Check group permissions
        let user_groups = user_group_membership::Entity::find()
            .filter(user_group_membership::Column::UserId.eq(user_id))
            .filter(user_group_membership::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let group_ids: Vec<uuid::Uuid> = user_groups.into_iter().map(|ug| ug.group_id).collect();
        if !group_ids.is_empty() {
            let group_perms = group_permission::Entity::find()
                .filter(group_permission::Column::GroupId.is_in(group_ids))
                .filter(group_permission::Column::OrgId.eq(org_id))
                .all(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            let is_granted = group_perms.iter().any(|gp| {
                let p = gp.permission.to_lowercase();
                if p == "*" || p == permission {
                    return true;
                }
                let mut gp_parts = p.split(':');
                let g_res = gp_parts.next().unwrap_or("");
                let g_act = gp_parts.next().unwrap_or("");

                let res_ok = g_res == target_resource
                    || g_res == "*"
                    || (g_res == "class" && target_resource == "section")
                    || (g_res == "section" && target_resource == "class");
                let act_ok = g_act == target_action
                    || g_act == "*"
                    || g_act == "admin"
                    || (g_act == "write" && target_action == "read");
                res_ok && act_ok
            });

            if is_granted {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn get_user_permissions(
        &self,
        user_id: uuid::Uuid,
        org_id: uuid::Uuid,
    ) -> Result<Vec<String>, DomainError> {
        let mut perms: Vec<String> = Vec::new();

        // 1. Direct role permissions
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .filter(user_role_assignment::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let role_ids: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();
        if !role_ids.is_empty() {
            let role_permissions = role_permission::Entity::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids))
                .filter(role_permission::Column::OrgId.eq(org_id))
                .all(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            for rp in role_permissions {
                if rp.resource == "*" {
                    perms.push("*".to_string());
                } else if rp.resource.contains(':') {
                    perms.push(rp.resource.to_lowercase());
                } else if rp.action.is_empty() || rp.action == "*" {
                    perms.push(rp.resource.to_lowercase());
                } else {
                    perms.push(format!(
                        "{}:{}",
                        rp.resource.to_lowercase(),
                        rp.action.to_lowercase()
                    ));
                }
            }
        }

        // 2. Group permissions
        let user_groups = user_group_membership::Entity::find()
            .filter(user_group_membership::Column::UserId.eq(user_id))
            .filter(user_group_membership::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let group_ids: Vec<uuid::Uuid> = user_groups.into_iter().map(|ug| ug.group_id).collect();
        if !group_ids.is_empty() {
            let group_perms = group_permission::Entity::find()
                .filter(group_permission::Column::GroupId.is_in(group_ids))
                .filter(group_permission::Column::OrgId.eq(org_id))
                .all(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            for gp in group_perms {
                perms.push(gp.permission.to_lowercase());
            }
        }

        perms.sort();
        perms.dedup();
        Ok(perms)
    }

    async fn get_user_roles(
        &self,
        user_id: uuid::Uuid,
        org_id: uuid::Uuid,
    ) -> Result<Vec<String>, DomainError> {
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .filter(user_role_assignment::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut roles: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();
        roles.sort();
        roles.dedup();
        Ok(roles)
    }

    async fn get_org_unit_scope(
        &self,
        user_id: uuid::Uuid,
        org_id: uuid::Uuid,
    ) -> Result<Option<Vec<uuid::Uuid>>, DomainError> {
        let assignments = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .filter(user_role_assignment::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if assignments.iter().any(|assignment| {
            assignment.scope_type.eq_ignore_ascii_case("global")
                || assignment.scope_type.eq_ignore_ascii_case("organization")
        }) {
            return Ok(None);
        }

        let mut unit_ids: Vec<_> = assignments
            .into_iter()
            .filter(|assignment| {
                assignment.scope_type.eq_ignore_ascii_case("org_unit")
                    || assignment.scope_type.eq_ignore_ascii_case("unit")
            })
            .filter_map(|assignment| assignment.scope_id)
            .collect();
        unit_ids.sort();
        unit_ids.dedup();
        Ok(Some(unit_ids))
    }
}
