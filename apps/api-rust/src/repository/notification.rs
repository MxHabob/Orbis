use crate::entities::notification;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    pub async fn create_notification(&self, n: notification::Model) -> Result<notification::Model, DbErr> {
        let active_model = notification::ActiveModel {
            id: Set(n.id),
            org_id: Set(n.org_id),
            user_id: Set(n.user_id),
            title: Set(n.title),
            title_ar: Set(n.title_ar),
            message: Set(n.message),
            message_ar: Set(n.message_ar),
            notification_type: Set(n.notification_type),
            priority: Set(n.priority),
            is_read: Set(n.is_read),
            created_at: Set(n.created_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_notifications(&self, user_id: &str) -> Result<Vec<notification::Model>, DbErr> {
        notification::Entity::find()
            .filter(notification::Column::UserId.eq(user_id))
            .order_by_desc(notification::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    pub async fn mark_notification_read(&self, id: &str) -> Result<u64, DbErr> {
        let update_res = notification::Entity::update_many()
            .col_expr(notification::Column::IsRead, Expr::value(1))
            .filter(notification::Column::Id.eq(id))
            .exec(&self.db)
            .await?;
        Ok(update_res.rows_affected)
    }
}
