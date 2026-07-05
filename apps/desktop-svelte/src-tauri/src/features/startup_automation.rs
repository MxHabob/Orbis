//! Startup Automation — processes pending calendar events at app launch.
//!
//! Called once during app initialization (non-blocking, fire-and-forget).
//! Handles the three execution modes:
//!
//! - **Automatic**: Executes immediately if prerequisites are met
//! - **Suggested**: Emits a Tauri event so the frontend can show a notification
//! - **Manual**: Skipped entirely
//!
//! The frontend listens for `calendar:pending-suggestions` events and displays
//! a notification banner with actionable items.

use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use sea_orm::ConnectionTrait;

macro_rules! info {
    ($($arg:tt)*) => { println!($($arg)*) };
}
macro_rules! warn {
    ($($arg:tt)*) => { eprintln!($($arg)*) };
}

use crate::state::AppState;

/// Process pending calendar events at startup (fire-and-forget).
///
/// This is spawned as an async task during `setup()`, so it never blocks the UI.
pub async fn process_startup_events(app_handle: AppHandle) {
    // Give the app a moment to fully initialize before processing
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let state: tauri::State<'_, Arc<AppState>> = match app_handle.try_state() {
        Some(s) => s,
        None => {
            warn!("⚠️ Startup automation: AppState not available yet");
            return;
        }
    };

    // We need an org_id — get it from the current year
    let year_id = state.provider.current_year_id().await;
    if year_id.is_empty() {
        info!("📅 Startup automation: No active year — skipping event processing");
        return;
    }

    let service = state.service().await;
    let today = chrono::Local::now().date_naive().to_string();

    // Get org_id from the first organization in the DB
    let org_id = match service.repo.db.execute_unprepared(
        "SELECT id FROM organizations LIMIT 1"
    ).await {
        Ok(_) => {
            // Use a query to get the actual org_id
            let result = sea_orm::ConnectionTrait::query_one(
                &service.repo.db,
                sea_orm::Statement::from_string(
                    sea_orm::DatabaseBackend::Sqlite,
                    "SELECT id FROM organizations LIMIT 1".to_string(),
                ),
            ).await;
            match result {
                Ok(Some(row)) => {
                    sea_orm::QueryResult::try_get::<String>(&row, "", "id").unwrap_or_default()
                }
                _ => String::new(),
            }
        }
        Err(_) => String::new(),
    };

    if org_id.is_empty() {
        info!("📅 Startup automation: No organization found — skipping");
        return;
    }

    // Fetch pending events
    let pending = match service.get_pending_events(&org_id, &today).await {
        Ok(events) => events,
        Err(e) => {
            warn!("⚠️ Startup automation: Failed to fetch pending events: {}", e);
            return;
        }
    };

    if pending.is_empty() {
        info!("📅 Startup automation: No pending events");
        return;
    }

    info!("📅 Startup automation: Processing {} pending events", pending.len());

    let is_writable = !state.provider.is_current_archived().await;
    let mut auto_executed = 0u32;
    let mut suggestions = Vec::new();

    for event in &pending {
        match event.execution_mode.as_str() {
            "automatic" => {
                if !is_writable {
                    info!("  ⏭️ Skipping automatic event '{}' — year is archived", event.title);
                    continue;
                }
                match service.execute_calendar_event_action(event, "system").await {
                    Ok(result) => {
                        info!("  ✅ Auto-executed: {} ({})", event.title, result.message.as_deref().unwrap_or("ok"));
                        auto_executed += 1;
                    }
                    Err(e) => {
                        warn!("  ❌ Auto-execute failed for '{}': {}", event.title, e);
                    }
                }
            }
            "suggested" => {
                suggestions.push(serde_json::json!({
                    "event_id": event.id,
                    "event_type": event.event_type,
                    "title": event.title,
                    "title_ar": event.title_ar,
                    "start_date": event.start_date,
                    "execution_mode": "suggested",
                }));
            }
            _ => {
                // manual — skip
            }
        }
    }

    if auto_executed > 0 {
        info!("📅 Startup automation: {} events auto-executed", auto_executed);
    }

    // Emit suggested events to the frontend
    if !suggestions.is_empty() {
        info!("📅 Startup automation: {} events need user attention", suggestions.len());
        let _ = app_handle.emit("calendar:pending-suggestions", &suggestions);
    }
}
