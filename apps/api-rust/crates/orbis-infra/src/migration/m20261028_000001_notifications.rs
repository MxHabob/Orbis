use sea_orm_migration::prelude::*;

use crate::models::{notification, notification_preference};

/// Adds notifications and notification_preferences tables for the
/// in-app notification center, unread counts, and per-user channel preferences.
#[derive(DeriveMigrationName)]
pub struct Migration;

async fn create_table_from_entity<E>(manager: &SchemaManager<'_>, entity: E) -> Result<(), DbErr>
where
    E: sea_orm::EntityTrait,
{
    let builder = manager.get_database_backend();
    let schema = sea_orm::Schema::new(builder);
    let mut stmt = schema.create_table_from_entity(entity);
    stmt.if_not_exists();
    manager.create_table(stmt).await
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ── Notifications table ──────────────────────────────────
        create_table_from_entity(manager, notification::Entity).await?;

        // Index: fast lookup by recipient within an org, ordered newest-first
        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_recipient_org")
                    .table(notification::Entity)
                    .col(notification::Column::OrgId)
                    .col(notification::Column::RecipientUserId)
                    .col(notification::Column::CreatedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Index: unread notifications for badge count queries
        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_unread")
                    .table(notification::Entity)
                    .col(notification::Column::OrgId)
                    .col(notification::Column::RecipientUserId)
                    .col(notification::Column::IsRead)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Index: deduplication via correlation_id
        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_correlation")
                    .table(notification::Entity)
                    .col(notification::Column::CorrelationId)
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Index: expiration cleanup
        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_expires")
                    .table(notification::Entity)
                    .col(notification::Column::ExpiresAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // ── Notification preferences table ───────────────────────
        create_table_from_entity(manager, notification_preference::Entity).await?;

        // Index: unique constraint per user+org+category
        manager
            .create_index(
                Index::create()
                    .name("idx_notification_prefs_unique")
                    .table(notification_preference::Entity)
                    .col(notification_preference::Column::UserId)
                    .col(notification_preference::Column::OrgId)
                    .col(notification_preference::Column::Category)
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(notification_preference::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(notification::Entity).to_owned())
            .await?;
        Ok(())
    }
}
