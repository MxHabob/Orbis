//! Role Repository — SeaORM implementation of RoleRepository port.

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

use orbis_app::ports::repositories::RoleRepository;
use orbis_domain::entities::role::Role;
use orbis_domain::entities::role_permission::RolePermission;
use orbis_domain::entities::user_role_assignment::UserRoleAssignment;
use orbis_domain::errors::DomainError;

use crate::models::{role, role_permission, user_role_assignment};

pub struct RoleRepoImpl {
    pub db: DatabaseConnection,
}

impl RoleRepoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoleRepository for RoleRepoImpl {
    async fn find_by_id(&self, org_id: Uuid, role_id: &str) -> Result<Option<Role>, DomainError> {
        let model = role::Entity::find()
            .filter(role::Column::OrgId.eq(org_id))
            .filter(role::Column::Id.eq(role_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| Role {
            id: m.id,
            org_id: m.org_id,
            name: m.name,
            name_ar: m.name_ar,
            is_system: m.is_system,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn list_by_org(&self, org_id: Uuid) -> Result<Vec<Role>, DomainError> {
        let models = role::Entity::find()
            .filter(role::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| Role {
                id: m.id,
                org_id: m.org_id,
                name: m.name,
                name_ar: m.name_ar,
                is_system: m.is_system,
                created_at: m.created_at,
                updated_at: m.updated_at,
            })
            .collect())
    }

    async fn count_by_org(&self, org_id: Uuid) -> Result<u64, DomainError> {
        let count = role::Entity::find()
            .filter(role::Column::OrgId.eq(org_id))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(count)
    }

    async fn save_role(&self, r: &Role) -> Result<(), DomainError> {
        let existing = role::Entity::find()
            .filter(role::Column::OrgId.eq(r.org_id))
            .filter(role::Column::Id.eq(&r.id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let active_model = role::ActiveModel {
            id: Set(r.id.clone()),
            org_id: Set(r.org_id),
            name: Set(r.name.clone()),
            name_ar: Set(r.name_ar.clone()),
            is_system: Set(r.is_system),
            created_at: Set(r.created_at),
            updated_at: Set(r.updated_at),
        };

        if existing.is_some() {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    async fn save_role_permissions(
        &self,
        org_id: Uuid,
        role_id: &str,
        permissions: &[RolePermission],
    ) -> Result<(), DomainError> {
        // Delete existing permissions for this role/org
        let _ = role_permission::Entity::delete_many()
            .filter(role_permission::Column::OrgId.eq(org_id))
            .filter(role_permission::Column::RoleId.eq(role_id))
            .exec(&self.db)
            .await;

        for p in permissions {
            let active = role_permission::ActiveModel {
                id: Set(p.id),
                role_id: Set(p.role_id.clone()),
                org_id: Set(p.org_id),
                resource: Set(p.resource.clone()),
                action: Set(p.action.clone()),
                condition: Set(p.condition.clone()),
                created_at: Set(p.created_at),
            };
            let _ = active.insert(&self.db).await;
        }

        Ok(())
    }

    async fn assign_role(&self, assignment: &UserRoleAssignment) -> Result<(), DomainError> {
        let existing = user_role_assignment::Entity::find()
            .filter(
                Condition::all()
                    .add(user_role_assignment::Column::UserId.eq(assignment.user_id))
                    .add(user_role_assignment::Column::RoleId.eq(&assignment.role_id))
                    .add(user_role_assignment::Column::OrgId.eq(assignment.org_id)),
            )
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if existing.is_some() {
            return Ok(());
        }

        let active = user_role_assignment::ActiveModel {
            id: Set(assignment.id),
            user_id: Set(assignment.user_id),
            role_id: Set(assignment.role_id.clone()),
            org_id: Set(assignment.org_id),
            scope_type: Set(assignment.scope_type.clone()),
            scope_id: Set(assignment.scope_id),
            assigned_by: Set(assignment.assigned_by.clone()),
            created_at: Set(assignment.created_at),
        };

        active
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn find_assignment(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        role_id: &str,
    ) -> Result<Option<UserRoleAssignment>, DomainError> {
        let model = user_role_assignment::Entity::find()
            .filter(
                Condition::all()
                    .add(user_role_assignment::Column::UserId.eq(user_id))
                    .add(user_role_assignment::Column::RoleId.eq(role_id))
                    .add(user_role_assignment::Column::OrgId.eq(org_id)),
            )
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| UserRoleAssignment {
            id: m.id,
            user_id: m.user_id,
            role_id: m.role_id,
            org_id: m.org_id,
            scope_type: m.scope_type,
            scope_id: m.scope_id,
            assigned_by: m.assigned_by,
            created_at: m.created_at,
        }))
    }
}
