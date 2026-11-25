use crate::models::{
    group_permission, role_permission, user_group_membership, user_role_assignment,
};
use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::iam::{PermissionCheckerPort, PermissionScope};
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct DbPermissionChecker {
    pub db: DatabaseConnection,
}

fn assignment_scope(
    assignments: impl IntoIterator<Item = user_role_assignment::Model>,
) -> PermissionScope {
    let mut units = Vec::new();
    for assignment in assignments {
        match assignment.scope_type.to_ascii_lowercase().as_str() {
            "global" | "organization" if assignment.scope_id.is_none() => {
                return PermissionScope::Organization;
            }
            "org_unit" | "unit" => units.extend(assignment.scope_id),
            _ => {}
        }
    }
    units.sort_unstable();
    units.dedup();
    PermissionScope::Units(units)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ActiveModelTrait, ConnectionTrait, Database, Schema, Set};
    use uuid::Uuid;

    #[tokio::test]
    async fn organization_wide_grants_only_apply_to_the_matching_operation() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        let schema = Schema::new(db.get_database_backend());
        db.execute(
            db.get_database_backend()
                .build(&schema.create_table_from_entity(user_role_assignment::Entity)),
        )
        .await
        .unwrap();
        db.execute(
            db.get_database_backend()
                .build(&schema.create_table_from_entity(role_permission::Entity)),
        )
        .await
        .unwrap();
        db.execute(
            db.get_database_backend()
                .build(&schema.create_table_from_entity(user_group_membership::Entity)),
        )
        .await
        .unwrap();
        let now = chrono::Utc::now();
        let (user_id, org_id, branch_id) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        for (role, scope, unit, resource, action) in [
            (
                "dashboard_viewer",
                "organization",
                None,
                "dashboard",
                "read",
            ),
            (
                "branch_editor",
                "org_unit",
                Some(branch_id),
                "org_unit",
                "write",
            ),
            (
                "principal",
                "org_unit",
                Some(branch_id),
                "workflow",
                "approve",
            ),
        ] {
            user_role_assignment::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                org_id: Set(org_id),
                role_id: Set(role.into()),
                scope_type: Set(scope.into()),
                scope_id: Set(unit),
                assigned_by: Set(None),
                created_at: Set(now),
            }
            .insert(&db)
            .await
            .unwrap();
            role_permission::ActiveModel {
                id: Set(Uuid::new_v4()),
                org_id: Set(org_id),
                role_id: Set(role.into()),
                resource: Set(resource.into()),
                action: Set(action.into()),
                condition: Set(None),
                created_at: Set(now),
            }
            .insert(&db)
            .await
            .unwrap();
        }
        let checker = DbPermissionChecker::new(db);
        assert_eq!(
            checker
                .role_scope(user_id, org_id, "principal")
                .await
                .unwrap(),
            PermissionScope::Units(vec![branch_id])
        );
        assert_eq!(
            checker
                .permission_scope(user_id, org_id, "workflow:approve")
                .await
                .unwrap(),
            PermissionScope::Units(vec![branch_id])
        );
        assert!(
            !checker
                .role_scope(user_id, org_id, "org_admin")
                .await
                .unwrap()
                .has_access()
        );
        assert_eq!(
            checker
                .permission_scope(user_id, org_id, "dashboard:read")
                .await
                .unwrap(),
            PermissionScope::Organization
        );
        assert_eq!(
            checker
                .permission_scope(user_id, org_id, "org_unit:write")
                .await
                .unwrap(),
            PermissionScope::Units(vec![branch_id])
        );
        assert_eq!(
            checker
                .permission_scope(user_id, Uuid::new_v4(), "dashboard:read")
                .await
                .unwrap(),
            PermissionScope::Units(vec![])
        );
    }
}

fn matches_permission(resource: &str, action: &str, permission: &str) -> bool {
    let resource = resource.to_ascii_lowercase();
    let action = action.to_ascii_lowercase();
    let Some((target_resource, target_action)) = permission.split_once(':') else {
        return resource == permission && action.is_empty();
    };
    resource == permission
        || ((resource == target_resource
            || resource == "*"
            || matches!(
                (resource.as_str(), target_resource),
                ("class", "section") | ("section", "class")
            ))
            && (action == target_action
                || matches!(action.as_str(), "*" | "admin")
                || (action == "write" && target_action == "read")))
}

impl DbPermissionChecker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PermissionCheckerPort for DbPermissionChecker {
    async fn role_scope(
        &self,
        user_id: uuid::Uuid,
        org_id: uuid::Uuid,
        role: &str,
    ) -> Result<PermissionScope, DomainError> {
        let assignments = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .filter(user_role_assignment::Column::OrgId.eq(org_id))
            .filter(user_role_assignment::Column::RoleId.eq(role))
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(assignment_scope(assignments))
    }

    async fn check(&self, ctx: &RequestContext, permission: Permission) -> Result<(), DomainError> {
        // System and Autopilot actors may bypass transport IAM only for explicitly
        // internal execution sources. API, Tauri, and integration requests must
        // use authenticated user or agent actors.
        if ctx.actor.is_system()
            && matches!(
                &ctx.source,
                orbis_app::context::ActionSource::System
                    | orbis_app::context::ActionSource::Autopilot
            )
        {
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
        Ok(self
            .permission_scope(user_id, org_id, permission)
            .await?
            .has_access())
    }

    async fn permission_scope(
        &self,
        user_id: uuid::Uuid,
        org_id: uuid::Uuid,
        permission: &str,
    ) -> Result<PermissionScope, DomainError> {
        let assignments = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .filter(user_role_assignment::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        let mut units = Vec::new();

        if !assignments.is_empty() {
            let role_ids: Vec<_> = assignments
                .iter()
                .map(|assignment| assignment.role_id.clone())
                .collect();
            let grants = role_permission::Entity::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids))
                .filter(role_permission::Column::OrgId.eq(org_id))
                .all(&self.db)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            let allowed_roles: std::collections::HashSet<_> = grants
                .iter()
                .filter(|grant| {
                    grant.condition.is_none()
                        && matches_permission(&grant.resource, &grant.action, permission)
                })
                .map(|grant| grant.role_id.as_str())
                .collect();

            for assignment in assignments
                .iter()
                .filter(|assignment| allowed_roles.contains(assignment.role_id.as_str()))
            {
                match assignment.scope_type.to_ascii_lowercase().as_str() {
                    "global" | "organization" if assignment.scope_id.is_none() => {
                        return Ok(PermissionScope::Organization);
                    }
                    "org_unit" | "unit" => units.extend(assignment.scope_id),
                    _ => {}
                }
            }
        }

        // Workflow authority must have an explicit role scope. Group membership has no branch scope.
        if permission.starts_with("workflow:") {
            units.sort_unstable();
            units.dedup();
            return Ok(PermissionScope::Units(units));
        }

        let groups = user_group_membership::Entity::find()
            .filter(user_group_membership::Column::UserId.eq(user_id))
            .filter(user_group_membership::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if !groups.is_empty() {
            let grants = group_permission::Entity::find()
                .filter(
                    group_permission::Column::GroupId
                        .is_in(groups.into_iter().map(|group| group.group_id)),
                )
                .filter(group_permission::Column::OrgId.eq(org_id))
                .all(&self.db)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            if grants.iter().any(|grant| {
                grant.permission == "*"
                    || grant
                        .permission
                        .split_once(':')
                        .is_some_and(|(resource, action)| {
                            matches_permission(resource, action, permission)
                        })
            }) {
                return Ok(PermissionScope::Organization);
            }
        }

        units.sort_unstable();
        units.dedup();
        Ok(PermissionScope::Units(units))
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
