//! Concrete `NotificationSender` implementation.
//!
//! For now, this is a logging-only implementation that records notifications
//! to tracing. Future versions will integrate push notifications, email, SMS.

use async_trait::async_trait;
use tracing::info;

use shared_kernel::errors::AppError;
use shared_kernel::traits::{Notification, NotificationSender};

/// Logging-only `NotificationSender` — records to tracing.
///
/// Replace with real push/email/SMS integration in production.
pub struct LogNotificationSender;

impl LogNotificationSender {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NotificationSender for LogNotificationSender {
    async fn send(&self, notification: Notification) -> Result<(), AppError> {
        info!(
            org_id = %notification.org_id,
            title = %notification.title,
            channel = ?notification.channel,
            priority = ?notification.priority,
            targets = ?notification.target_user_ids.len(),
            "📬 Notification sent (log-only)"
        );
        Ok(())
    }
}
