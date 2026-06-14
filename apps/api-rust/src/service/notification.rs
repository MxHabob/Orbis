use uuid::Uuid;
use chrono::Utc;
use tracing::info;
use crate::entities::notification;

impl super::Service {
    pub async fn send_notification(
        &self,
        user_id: &str,
        org_id: &str,
        title: &str,
        title_ar: &str,
        message: &str,
        message_ar: &str,
        priority: &str,
        n_type: &str,
    ) -> Result<notification::Model, String> {
        // 1. Create notification in database
        let n = notification::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            user_id: user_id.to_string(),
            title: title.to_string(),
            title_ar: Some(title_ar.to_string()),
            message: message.to_string(),
            message_ar: Some(message_ar.to_string()),
            notification_type: n_type.to_string(),
            priority: priority.to_string(),
            is_read: 0,
            created_at: Utc::now().to_rfc3339(),
        };

        let created_n = self.repo.create_notification(n).await
            .map_err(|e| e.to_string())?;

        // 2. Dynamic priority-based routing (Infrastructure integration)
        match priority {
            "low" => {
                info!(
                    notification_id = %created_n.id,
                    user_id = %user_id,
                    "Low priority: notification sent to In-App feed only"
                );
            }
            "medium" => {
                info!(
                    notification_id = %created_n.id,
                    user_id = %user_id,
                    "Medium priority: dispatching In-App notification & sending mock Email"
                );
                // Mock dispatch to email channel
                info!(
                    to = %format!("{}@orbis.edu", user_id),
                    subject = %title,
                    body = %message,
                    "✉️ [INFRASTRUCTURE NOTIFIER] Email dispatched successfully"
                );
            }
            "high" => {
                info!(
                    notification_id = %created_n.id,
                    user_id = %user_id,
                    "⚡ HIGH priority escalation: dispatching In-App notification, mock Email & mock SMS call"
                );
                // Mock dispatch to email and SMS channels
                info!(
                    to = %format!("{}@orbis.edu", user_id),
                    subject = %title,
                    body = %message,
                    "✉️ [INFRASTRUCTURE NOTIFIER] Email dispatched successfully"
                );
                info!(
                    to = "+9671234567",
                    body = %message,
                    "📱 [INFRASTRUCTURE NOTIFIER] SMS alert dispatched successfully"
                );
            }
            _ => {}
        }

        Ok(created_n)
    }

    pub async fn get_notifications(&self, user_id: &str) -> Result<Vec<notification::Model>, String> {
        self.repo.get_notifications(user_id).await.map_err(|e| e.to_string())
    }

    pub async fn mark_notification_read(&self, id: &str) -> Result<(), String> {
        self.repo.mark_notification_read(id).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
