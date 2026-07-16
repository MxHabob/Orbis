//! Concrete `NotificationSender` implementation.
//!
//! For now, this is a logging-only implementation that records notifications
//! to tracing. Future versions will integrate push notifications, email, SMS.

use async_trait::async_trait;
use tracing::info;

use shared_kernel::errors::AppError;
use shared_kernel::traits::{Notification, NotificationSender};

use sea_orm::{DatabaseConnection, Statement, DbBackend, ConnectionTrait};

/// Real `NotificationSender` — saves to `app_notifications` table.
pub struct DbNotificationSender {
    db: DatabaseConnection,
}

impl DbNotificationSender {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl NotificationSender for DbNotificationSender {
    async fn send(&self, notification: Notification) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let notif_type = match notification.channel {
            shared_kernel::traits::NotificationChannel::InApp => "in_app",
            shared_kernel::traits::NotificationChannel::Push => "push",
            shared_kernel::traits::NotificationChannel::Email => "email",
            shared_kernel::traits::NotificationChannel::Sms => "sms",
        };

        // If target_user_ids is empty but target_roles is not, we should ideally resolve roles to users.
        // For simplicity, we just insert for all explicitly targeted users.
        if notification.target_user_ids.is_empty() {
            info!("No target users for notification '{}'", notification.title);
            return Ok(());
        }

        let sql = r#"
            INSERT INTO app_notifications (id, org_id, user_id, title, message, notification_type, is_read, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, 0, $7)
        "#;

        for user_id in &notification.target_user_ids {
            let id = uuid::Uuid::new_v4().to_string();
            self.db.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                sql,
                vec![
                    id.into(),
                    notification.org_id.clone().into(),
                    user_id.clone().into(),
                    notification.title.clone().into(),
                    notification.body.clone().into(),
                    notif_type.into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| AppError::internal(format!("Failed to insert notification: {}", e)))?;
        }

        info!(
            org_id = %notification.org_id,
            title = %notification.title,
            targets = ?notification.target_user_ids.len(),
            "📬 Notification saved to DB"
        );
        Ok(())
    }
}
