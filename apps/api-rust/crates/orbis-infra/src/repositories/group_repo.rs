use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use orbis_app::ports::repositories::GroupRepository;
use orbis_domain::entities::group::{Group, GroupDetail, GroupMemberInfo, UserGroupMembership};
use orbis_domain::errors::DomainError;

use crate::models::group::{self, Entity as GroupEntity};
use crate::models::group_permission::{self, Entity as GroupPermissionEntity};
use crate::models::person::Entity as PersonEntity;
use crate::models::user::Entity as UserEntity;
use crate::models::user_group_membership::{self, Entity as UserGroupMembershipEntity};

pub struct GroupRepoImpl {
    pub db: DatabaseConnection,
}

impl GroupRepoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_group(m: group::Model) -> Group {
        Group {
            id: m.id,
            org_id: m.org_id,
            name: m.name,
            name_ar: m.name_ar,
            description: m.description,
            is_system: m.is_system,
            status: m.status,
            created_by: m.created_by,
            updated_by: m.updated_by,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[async_trait]
impl GroupRepository for GroupRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Group>, DomainError> {
        let model = GroupEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(Self::model_to_group))
    }

    async fn find_by_name(&self, org_id: Uuid, name: &str) -> Result<Option<Group>, DomainError> {
        let model = GroupEntity::find()
            .filter(group::Column::OrgId.eq(org_id))
            .filter(group::Column::Name.eq(name))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(Self::model_to_group))
    }

    async fn find_detail_by_id(&self, id: Uuid) -> Result<Option<GroupDetail>, DomainError> {
        let group_model = match GroupEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))? {
            Some(m) => m,
            None => return Ok(None),
        };

        let member_count = UserGroupMembershipEntity::find()
            .filter(user_group_membership::Column::GroupId.eq(id))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))? as usize;

        let permissions = GroupPermissionEntity::find()
            .filter(group_permission::Column::GroupId.eq(id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .into_iter()
            .map(|p| p.permission)
            .collect();

        Ok(Some(GroupDetail {
            group: Self::model_to_group(group_model),
            member_count,
            permissions,
        }))
    }

    async fn list_by_org(&self, org_id: Uuid) -> Result<Vec<GroupDetail>, DomainError> {
        let groups = GroupEntity::find()
            .filter(group::Column::OrgId.eq(org_id))
            .order_by_asc(group::Column::Name)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut results = Vec::new();
        for g in groups {
            let member_count = UserGroupMembershipEntity::find()
                .filter(user_group_membership::Column::GroupId.eq(g.id))
                .count(&self.db)
                .await
                .unwrap_or(0) as usize;

            let permissions = GroupPermissionEntity::find()
                .filter(group_permission::Column::GroupId.eq(g.id))
                .all(&self.db)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|p| p.permission)
                .collect();

            results.push(GroupDetail {
                group: Self::model_to_group(g),
                member_count,
                permissions,
            });
        }

        Ok(results)
    }

    async fn save(&self, group: &Group) -> Result<(), DomainError> {
        let active_model = group::ActiveModel {
            id: Set(group.id),
            org_id: Set(group.org_id),
            name: Set(group.name.clone()),
            name_ar: Set(group.name_ar.clone()),
            description: Set(group.description.clone()),
            is_system: Set(group.is_system),
            status: Set(group.status.clone()),
            created_by: Set(group.created_by),
            updated_by: Set(group.updated_by),
            created_at: Set(group.created_at),
            updated_at: Set(group.updated_at),
        };

        if GroupEntity::find_by_id(group.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
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

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        // Delete memberships and permissions first
        let _ = UserGroupMembershipEntity::delete_many()
            .filter(user_group_membership::Column::GroupId.eq(id))
            .exec(&self.db)
            .await;

        let _ = GroupPermissionEntity::delete_many()
            .filter(group_permission::Column::GroupId.eq(id))
            .exec(&self.db)
            .await;

        let _ = GroupEntity::delete_by_id(id).exec(&self.db).await;
        Ok(())
    }

    // --- Membership Operations ---

    async fn get_members(&self, group_id: Uuid) -> Result<Vec<GroupMemberInfo>, DomainError> {
        let memberships = UserGroupMembershipEntity::find()
            .filter(user_group_membership::Column::GroupId.eq(group_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut members = Vec::new();
        for m in memberships {
            if let Ok(Some(user_model)) = UserEntity::find_by_id(m.user_id).one(&self.db).await {
                let person = PersonEntity::find_by_id(user_model.person_id)
                    .one(&self.db)
                    .await
                    .ok()
                    .flatten();

                let (full_name, full_name_ar, avatar_url, phone) = match person {
                    Some(p) => (
                        p.first_name.clone() + " " + &p.last_name,
                        p.first_name_ar.clone().map(|f| f + " " + &p.last_name_ar.clone().unwrap_or_default()),
                        p.avatar_url,
                        p.phone,
                    ),
                    None => ("User".into(), None, None, None),
                };

                members.push(GroupMemberInfo {
                    user_id: user_model.id,
                    person_id: user_model.person_id,
                    full_name,
                    full_name_ar,
                    email: user_model.email,
                    phone,
                    avatar_url,
                    user_status: user_model.status,
                    joined_at: m.created_at,
                });
            }
        }

        Ok(members)
    }

    async fn get_user_groups(&self, user_id: Uuid) -> Result<Vec<Group>, DomainError> {
        let memberships = UserGroupMembershipEntity::find()
            .filter(user_group_membership::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let group_ids: Vec<Uuid> = memberships.into_iter().map(|m| m.group_id).collect();
        if group_ids.is_empty() {
            return Ok(vec![]);
        }

        let models = GroupEntity::find()
            .filter(group::Column::Id.is_in(group_ids))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(Self::model_to_group).collect())
    }

    async fn add_member(&self, membership: &UserGroupMembership) -> Result<(), DomainError> {
        let existing = UserGroupMembershipEntity::find()
            .filter(user_group_membership::Column::UserId.eq(membership.user_id))
            .filter(user_group_membership::Column::GroupId.eq(membership.group_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if existing.is_none() {
            let active_model = user_group_membership::ActiveModel {
                id: Set(membership.id),
                org_id: Set(membership.org_id),
                user_id: Set(membership.user_id),
                group_id: Set(membership.group_id),
                created_at: Set(membership.created_at),
            };
            active_model.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    async fn remove_member(&self, user_id: Uuid, group_id: Uuid) -> Result<(), DomainError> {
        UserGroupMembershipEntity::delete_many()
            .filter(user_group_membership::Column::UserId.eq(user_id))
            .filter(user_group_membership::Column::GroupId.eq(group_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn bulk_add_members(&self, memberships: &[UserGroupMembership]) -> Result<u64, DomainError> {
        let mut count = 0u64;
        for m in memberships {
            let existing = UserGroupMembershipEntity::find()
                .filter(user_group_membership::Column::UserId.eq(m.user_id))
                .filter(user_group_membership::Column::GroupId.eq(m.group_id))
                .one(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            if existing.is_none() {
                let active_model = user_group_membership::ActiveModel {
                    id: Set(m.id),
                    org_id: Set(m.org_id),
                    user_id: Set(m.user_id),
                    group_id: Set(m.group_id),
                    created_at: Set(m.created_at),
                };
                if active_model.insert(&self.db).await.is_ok() {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    async fn is_member(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DomainError> {
        let existing = UserGroupMembershipEntity::find()
            .filter(user_group_membership::Column::UserId.eq(user_id))
            .filter(user_group_membership::Column::GroupId.eq(group_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(existing.is_some())
    }

    // --- Permissions Operations ---

    async fn get_group_permissions(&self, group_id: Uuid) -> Result<Vec<String>, DomainError> {
        let perms = GroupPermissionEntity::find()
            .filter(group_permission::Column::GroupId.eq(group_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(perms.into_iter().map(|p| p.permission).collect())
    }

    async fn set_group_permissions(&self, org_id: Uuid, group_id: Uuid, permissions: &[String]) -> Result<(), DomainError> {
        let _ = GroupPermissionEntity::delete_many()
            .filter(group_permission::Column::GroupId.eq(group_id))
            .exec(&self.db)
            .await;

        let now = chrono::Utc::now();
        for perm in permissions {
            let active_model = group_permission::ActiveModel {
                id: Set(Uuid::new_v4()),
                org_id: Set(org_id),
                group_id: Set(group_id),
                permission: Set(perm.clone()),
                created_at: Set(now),
            };
            let _ = active_model.insert(&self.db).await;
        }

        Ok(())
    }

    async fn get_user_group_permissions(&self, user_id: Uuid) -> Result<Vec<String>, DomainError> {
        let memberships = UserGroupMembershipEntity::find()
            .filter(user_group_membership::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let group_ids: Vec<Uuid> = memberships.into_iter().map(|m| m.group_id).collect();
        if group_ids.is_empty() {
            return Ok(vec![]);
        }

        let perms = GroupPermissionEntity::find()
            .filter(group_permission::Column::GroupId.is_in(group_ids))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut list: Vec<String> = perms.into_iter().map(|p| p.permission).collect();
        list.sort();
        list.dedup();
        Ok(list)
    }
}
