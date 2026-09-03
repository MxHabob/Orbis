use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct NotificationCreatedPayload {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct SyncStatusChangedPayload {
    pub status: String,
}

pub fn emit_notification_created(app: &AppHandle, payload: NotificationCreatedPayload) -> Result<(), tauri::Error> {
    app.emit("notification-created", payload)
}

pub fn emit_sync_status_changed(app: &AppHandle, payload: SyncStatusChangedPayload) -> Result<(), tauri::Error> {
    app.emit("sync-status-changed", payload)
}
