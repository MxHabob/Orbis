use async_trait::async_trait;
use chrono::Utc;
use orbis_app::ports::repositories::{
    NotificationFilter, NotificationPreferenceRepository, NotificationRepository,
};
use orbis_domain::entities::notification::Notification;
use orbis_domain::entities::notification_preference::UserNotificationPreference;
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::models::{notification, notification_preference};

// =========================================================================
// NotificationRepository implementation
// =========================================================================

pub struct NotificationRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl NotificationRepository for NotificationRepoImpl {
    async fn save(&self, n: &Notification) -> Result<(), DomainError> {
        let active = notification::ActiveModel {
            id: Set(n.id),
            org_id: Set(n.org_id),
            recipient_user_id: Set(n.recipient_user_id),
            category: Set(n.category.clone()),
            priority: Set(n.priority.clone()),
            title: Set(n.title.clone()),
            title_ar: Set(n.title_ar.clone()),
            body: Set(n.body.clone()),
            body_ar: Set(n.body_ar.clone()),
            entity_type: Set(n.entity_type.clone()),
            entity_id: Set(n.entity_id),
            action_route: Set(n.action_route.clone()),
            source_event: Set(n.source_event.clone()),
            correlation_id: Set(n.correlation_id.clone()),
            schema_version: Set(n.schema_version),
            is_read: Set(n.is_read),
            read_at: Set(n.read_at),
            is_dismissed: Set(n.is_dismissed),
            dismissed_at: Set(n.dismissed_at),
            expires_at: Set(n.expires_at),
            created_at: Set(n.created_at),
        };

        if notification::Entity::find_by_id(n.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        org_id: Uuid,
        notification_id: Uuid,
    ) -> Result<Option<Notification>, DomainError> {
        let model = notification::Entity::find_by_id(notification_id)
            .filter(notification::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(map_notification_model))
    }

    async fn find_by_correlation_id(
        &self,
        correlation_id: &str,
    ) -> Result<Option<Notification>, DomainError> {
        let model = notification::Entity::find()
            .filter(notification::Column::CorrelationId.eq(correlation_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(map_notification_model))
    }

    async fn list_for_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        filter: &NotificationFilter,
    ) -> Result<Vec<Notification>, DomainError> {
        let mut query = notification::Entity::find()
            .filter(notification::Column::OrgId.eq(org_id))
            .filter(notification::Column::RecipientUserId.eq(user_id))
            .filter(notification::Column::IsDismissed.eq(filter.is_dismissed));

        if let Some(ref category) = filter.category {
            query = query.filter(notification::Column::Category.eq(category.clone()));
        }
        if let Some(is_read) = filter.is_read {
            query = query.filter(notification::Column::IsRead.eq(is_read));
        }

        let models = query
            .order_by_desc(notification::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        // Apply offset/limit in Rust since SeaORM query builder chaining is
        // cleanest this way for small result sets. For production scale,
        // switch to .paginate() with cursor-based pagination.
        let result: Vec<Notification> = models
            .into_iter()
            .skip(filter.offset as usize)
            .take(filter.limit as usize)
            .map(map_notification_model)
            .collect();

        Ok(result)
    }

    async fn count_unread(&self, org_id: Uuid, user_id: Uuid) -> Result<i64, DomainError> {
        let count = notification::Entity::find()
            .filter(notification::Column::OrgId.eq(org_id))
            .filter(notification::Column::RecipientUserId.eq(user_id))
            .filter(notification::Column::IsRead.eq(false))
            .filter(notification::Column::IsDismissed.eq(false))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(count as i64)
    }

    async fn mark_read(&self, org_id: Uuid, notification_id: Uuid) -> Result<(), DomainError> {
        let model = notification::Entity::find_by_id(notification_id)
            .filter(notification::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(_existing) = model {
            let active = notification::ActiveModel {
                id: Set(notification_id),
                is_read: Set(true),
                read_at: Set(Some(Utc::now())),
                ..Default::default()
            };
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn mark_all_read(&self, org_id: Uuid, user_id: Uuid) -> Result<u64, DomainError> {
        use sea_orm::sea_query::Expr;

        let result = notification::Entity::update_many()
            .col_expr(notification::Column::IsRead, Expr::value(true))
            .col_expr(notification::Column::ReadAt, Expr::value(Utc::now()))
            .filter(notification::Column::OrgId.eq(org_id))
            .filter(notification::Column::RecipientUserId.eq(user_id))
            .filter(notification::Column::IsRead.eq(false))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(result.rows_affected)
    }

    async fn dismiss(&self, org_id: Uuid, notification_id: Uuid) -> Result<(), DomainError> {
        let model = notification::Entity::find_by_id(notification_id)
            .filter(notification::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(_existing) = model {
            let active = notification::ActiveModel {
                id: Set(notification_id),
                is_dismissed: Set(true),
                dismissed_at: Set(Some(Utc::now())),
                ..Default::default()
            };
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, DomainError> {
        let now = Utc::now();
        let result = notification::Entity::delete_many()
            .filter(notification::Column::ExpiresAt.lte(now))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(result.rows_affected)
    }
}

fn map_notification_model(m: notification::Model) -> Notification {
    Notification {
        id: m.id,
        org_id: m.org_id,
        recipient_user_id: m.recipient_user_id,
        category: m.category,
        priority: m.priority,
        title: m.title,
        title_ar: m.title_ar,
        body: m.body,
        body_ar: m.body_ar,
        entity_type: m.entity_type,
        entity_id: m.entity_id,
        action_route: m.action_route,
        source_event: m.source_event,
        correlation_id: m.correlation_id,
        schema_version: m.schema_version,
        is_read: m.is_read,
        read_at: m.read_at,
        is_dismissed: m.is_dismissed,
        dismissed_at: m.dismissed_at,
        expires_at: m.expires_at,
        created_at: m.created_at,
    }
}

// =========================================================================
// NotificationPreferenceRepository implementation
// =========================================================================

pub struct NotificationPreferenceRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl NotificationPreferenceRepository for NotificationPreferenceRepoImpl {
    async fn find_for_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<UserNotificationPreference>, DomainError> {
        let models = notification_preference::Entity::find()
            .filter(notification_preference::Column::OrgId.eq(org_id))
            .filter(notification_preference::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(map_preference_model).collect())
    }

    async fn save(&self, pref: &UserNotificationPreference) -> Result<(), DomainError> {
        let active = notification_preference::ActiveModel {
            id: Set(pref.id),
            user_id: Set(pref.user_id),
            org_id: Set(pref.org_id),
            category: Set(pref.category.clone()),
            in_app_enabled: Set(pref.in_app_enabled),
            email_enabled: Set(pref.email_enabled),
            push_enabled: Set(pref.push_enabled),
        };

        if notification_preference::Entity::find_by_id(pref.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_all(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        preferences: &[UserNotificationPreference],
    ) -> Result<(), DomainError> {
        // Delete existing preferences for this user+org, then insert all new ones.
        notification_preference::Entity::delete_many()
            .filter(notification_preference::Column::OrgId.eq(org_id))
            .filter(notification_preference::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        for pref in preferences {
            let active = notification_preference::ActiveModel {
                id: Set(pref.id),
                user_id: Set(pref.user_id),
                org_id: Set(pref.org_id),
                category: Set(pref.category.clone()),
                in_app_enabled: Set(pref.in_app_enabled),
                email_enabled: Set(pref.email_enabled),
                push_enabled: Set(pref.push_enabled),
            };
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }
}

fn map_preference_model(m: notification_preference::Model) -> UserNotificationPreference {
    UserNotificationPreference {
        id: m.id,
        user_id: m.user_id,
        org_id: m.org_id,
        category: m.category,
        in_app_enabled: m.in_app_enabled,
        email_enabled: m.email_enabled,
        push_enabled: m.push_enabled,
    }
}
