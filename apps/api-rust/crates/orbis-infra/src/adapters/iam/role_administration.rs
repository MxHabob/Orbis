use orbis_app::context::RequestContext;
use orbis_app::ports::iam::{PermissionCheckerPort, PermissionScope};
use orbis_domain::errors::DomainError;
use orbis_domain::iam::{actor::Actor, permission::Permission};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use std::collections::HashSet;
use uuid::Uuid;

use crate::models::{role, user_role_assignment};

pub async fn require_role_administrator(
    checker: &dyn PermissionCheckerPort,
    ctx: &RequestContext,
) -> Result<Uuid, DomainError> {
    let Actor::User(actor) = ctx.actor else {
        return Err(DomainError::Forbidden(
            "An authenticated administrator is required".into(),
        ));
    };
    let org = ctx.tenant_id;
    if checker
        .permission_scope(actor, org, Permission::UserRoleWrite.as_str())
        .await?
        != PermissionScope::Organization
        || (checker.role_scope(actor, org, "org_admin").await? != PermissionScope::Organization
            && checker.role_scope(actor, org, "super_admin").await?
                != PermissionScope::Organization)
    {
        return Err(DomainError::Forbidden(
            "Organization-wide role administration is required".into(),
        ));
    }
    Ok(actor)
}

pub async fn validate_role_change(
    db: &DatabaseConnection,
    checker: &dyn PermissionCheckerPort,
    ctx: &RequestContext,
    target_user: Option<Uuid>,
    roles: &[String],
) -> Result<Uuid, DomainError> {
    let actor = require_role_administrator(checker, ctx).await?;
    let org = ctx.tenant_id;
    if target_user == Some(actor) {
        return Err(DomainError::Forbidden(
            "You cannot change your own role assignments".into(),
        ));
    }
    let super_admin =
        checker.role_scope(actor, org, "super_admin").await? == PermissionScope::Organization;
    for id in roles {
        role::Entity::find()
            .filter(role::Column::OrgId.eq(org))
            .filter(role::Column::Id.eq(id))
            .one(db)
            .await
            .map_err(db_error)?
            .ok_or_else(|| {
                DomainError::Validation("Role does not belong to this organization".into())
            })?;
        if matches!(id.as_str(), "super_admin" | "org_admin") && !super_admin {
            return Err(DomainError::Forbidden("You cannot assign this role".into()));
        }
    }
    if let Some(target) = target_user {
        protect_administrator(checker, ctx, actor, target).await?;
    }
    Ok(actor)
}

pub async fn protect_administrator(
    checker: &dyn PermissionCheckerPort,
    ctx: &RequestContext,
    actor: Uuid,
    target: Uuid,
) -> Result<(), DomainError> {
    let org = ctx.tenant_id;
    if target == actor {
        return Ok(());
    }
    if checker.role_scope(actor, org, "super_admin").await? == PermissionScope::Organization {
        return Ok(());
    }
    if checker.role_scope(target, org, "super_admin").await? == PermissionScope::Organization
        || checker.role_scope(target, org, "org_admin").await? == PermissionScope::Organization
    {
        return Err(DomainError::Forbidden(
            "Managing a central administrator requires a super administrator".into(),
        ));
    }
    if checker
        .role_scope(target, org, "principal")
        .await?
        .has_access()
        && checker.role_scope(actor, org, "org_admin").await? != PermissionScope::Organization
    {
        return Err(DomainError::Forbidden(
            "Managing a branch reviewer requires a central administrator".into(),
        ));
    }
    Ok(())
}

pub async fn replace_organization_roles(
    db: &DatabaseConnection,
    org_id: Uuid,
    user_id: Uuid,
    actor: Uuid,
    roles: &[String],
) -> Result<(), DomainError> {
    let transaction = db.begin().await.map_err(db_error)?;
    let assignments = user_role_assignment::Entity::find()
        .filter(user_role_assignment::Column::OrgId.eq(org_id))
        .filter(user_role_assignment::Column::UserId.eq(user_id))
        .all(&transaction)
        .await
        .map_err(db_error)?;
    let mut scoped = HashSet::new();
    let mut organization_wide = HashSet::new();
    for assignment in assignments {
        if assignment.scope_id.is_none()
            && matches!(assignment.scope_type.as_str(), "global" | "organization")
        {
            organization_wide.insert(assignment.role_id);
        } else {
            scoped.insert(assignment.role_id);
        }
    }
    user_role_assignment::Entity::delete_many()
        .filter(user_role_assignment::Column::OrgId.eq(org_id))
        .filter(user_role_assignment::Column::UserId.eq(user_id))
        .filter(user_role_assignment::Column::ScopeType.is_in(["global", "organization"]))
        .filter(user_role_assignment::Column::ScopeId.is_null())
        .exec(&transaction)
        .await
        .map_err(db_error)?;
    let mut roles = roles.to_vec();
    roles.sort_unstable();
    roles.dedup();
    for role_id in roles {
        if scoped.contains(&role_id) && !organization_wide.contains(&role_id) {
            // The user editor exposes role IDs without scopes, so preserve scoped-only roles.
            continue;
        }
        user_role_assignment::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            role_id: Set(role_id),
            org_id: Set(org_id),
            scope_type: Set("organization".into()),
            scope_id: Set(None),
            assigned_by: Set(Some(actor.to_string())),
            created_at: Set(chrono::Utc::now()),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
    }
    transaction.commit().await.map_err(db_error)
}

fn db_error(error: sea_orm::DbErr) -> DomainError {
    DomainError::Internal(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ConnectionTrait, Database, Schema};

    #[tokio::test]
    async fn replacing_roles_leaves_existing_branch_and_other_tenant_assignments_intact() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        let backend = db.get_database_backend();
        let table = Schema::new(backend).create_table_from_entity(user_role_assignment::Entity);
        db.execute(backend.build(&table)).await.unwrap();
        let (org, other_org, user, actor, branch) = (
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        );
        for (org_id, role, scope, unit) in [
            (org, "principal", "org_unit", Some(branch)),
            (org, "old_role", "global", None),
            (other_org, "other_role", "global", None),
        ] {
            user_role_assignment::ActiveModel {
                id: Set(Uuid::new_v4()),
                org_id: Set(org_id),
                user_id: Set(user),
                role_id: Set(role.into()),
                scope_type: Set(scope.into()),
                scope_id: Set(unit),
                assigned_by: Set(None),
                created_at: Set(chrono::Utc::now()),
            }
            .insert(&db)
            .await
            .unwrap();
        }
        replace_organization_roles(
            &db,
            org,
            user,
            actor,
            &["principal".into(), "teacher".into()],
        )
        .await
        .unwrap();
        let assignments = user_role_assignment::Entity::find().all(&db).await.unwrap();
        assert_eq!(assignments.len(), 3);
        assert!(
            assignments
                .iter()
                .any(|item| item.role_id == "principal" && item.scope_id == Some(branch))
        );
        assert!(assignments.iter().any(|item| item.role_id == "teacher"
            && item.org_id == org
            && item.scope_id.is_none()));
        assert!(
            assignments
                .iter()
                .any(|item| item.role_id == "other_role" && item.org_id == other_org)
        );
    }
}
