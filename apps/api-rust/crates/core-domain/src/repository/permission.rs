use crate::entities::{permission, role_permission};
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryFilter, EntityTrait};
use chrono::Utc;

impl super::Repository {
    pub async fn get_all_permissions(&self) -> Result<Vec<permission::Model>, DbErr> {
        permission::Entity::find()
            .all(&self.db)
            .await
    }

    pub async fn get_role_permissions(&self) -> Result<Vec<role_permission::Model>, DbErr> {
        role_permission::Entity::find()
            .all(&self.db)
            .await
    }

    pub async fn save_role_permissions(
        &self,
        role_id: &str,
        rules: Vec<role_permission::Model>,
    ) -> Result<(), DbErr> {
        // Delete all existing role_permission rows for the given role
        role_permission::Entity::delete_many()
            .filter(role_permission::Column::Role.eq(role_id.to_string()))
            .exec(&self.db)
            .await?;

        // Insert new role_permission models
        let mut new_models = Vec::new();
        for r in rules {
            new_models.push(role_permission::ActiveModel {
                role: Set(r.role),
                document_type: Set(r.document_type),
                level: Set(r.level),
                only_if_creator: Set(r.only_if_creator),
                actions: Set(r.actions),
                created_at: Set(r.created_at),
            });
        }

        if !new_models.is_empty() {
            role_permission::Entity::insert_many(new_models)
                .exec(&self.db)
                .await?;
        }

        Ok(())
    }

    pub async fn get_user_permissions(&self, user_id: &str) -> Result<Vec<crate::entities::user_permission::Model>, DbErr> {
        crate::entities::user_permission::Entity::find()
            .filter(crate::entities::user_permission::Column::UserId.eq(user_id.to_string()))
            .filter(crate::entities::user_permission::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    pub async fn save_user_permissions(
        &self,
        user_id: &str,
        permissions: Vec<(i32, String)>,
    ) -> Result<(), DbErr> {
        // Delete all existing user_permissions rows for the given user
        crate::entities::user_permission::Entity::delete_many()
            .filter(crate::entities::user_permission::Column::UserId.eq(user_id.to_string()))
            .exec(&self.db)
            .await?;

        // Insert new user_permission models
        let now = Utc::now().naive_utc().to_string();
        let mut new_models = Vec::new();
        for (perm_id, grant_type) in permissions {
            new_models.push(crate::entities::user_permission::ActiveModel {
                id: Set(uuid::Uuid::new_v4().to_string()),
                user_id: Set(user_id.to_string()),
                permission_id: Set(perm_id),
                grant_type: Set(grant_type),
                conditions: Set(None),
                valid_from: Set(None),
                valid_until: Set(None),
                reason: Set(None),
                granted_by: Set(Some("system".to_string())),
                granted_at: Set(now.clone()),
                is_active: Set(1),
            });
        }

        if !new_models.is_empty() {
            crate::entities::user_permission::Entity::insert_many(new_models)
                .exec(&self.db)
                .await?;
        }

        Ok(())
    }
}
