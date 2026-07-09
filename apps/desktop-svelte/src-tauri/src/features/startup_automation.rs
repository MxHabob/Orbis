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
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

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

    let db = state.active_db().await;
    let today = chrono::Local::now().date_naive().to_string();

    // Get org_id from the first organization in the DB
    let org_id = match domain_lifecycle::entities::organization::Entity::find()
        .one(&db)
        .await
    {
        Ok(Some(org)) => org.id,
        _ => String::new(),
    };

    if org_id.is_empty() {
        info!("📅 Startup automation: No organization found — skipping");
        return;
    }

    // Fetch completed event IDs from autopilot tasks
    let completed_event_ids: std::collections::HashSet<String> = match autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::OrgId.eq(&org_id))
        .filter(autopilot::entities::autopilot_task::Column::Status.eq("completed"))
        .all(state.provider.registry())
        .await
    {
        Ok(tasks) => tasks.into_iter().flat_map(|t| t.calendar_event_id).collect(),
        Err(e) => {
            warn!("⚠️ Startup automation: Failed to fetch completed tasks: {}", e);
            std::collections::HashSet::new()
        }
    };

    // Fetch active events
    let active_events = match domain_lifecycle::entities::academic_calendar_event::Entity::find()
        .filter(domain_lifecycle::entities::academic_calendar_event::Column::OrgId.eq(&org_id))
        .filter(domain_lifecycle::entities::academic_calendar_event::Column::StartDate.lte(&today))
        .filter(domain_lifecycle::entities::academic_calendar_event::Column::IsActive.eq(1))
        .all(&db)
        .await
    {
        Ok(events) => events,
        Err(e) => {
            warn!("⚠️ Startup automation: Failed to fetch active events: {}", e);
            return;
        }
    };

    let pending: Vec<domain_lifecycle::entities::academic_calendar_event::Model> = active_events
        .into_iter()
        .filter(|e| !completed_event_ids.contains(&e.id))
        .collect();

    if pending.is_empty() {
        info!("📅 Startup automation: No pending events");
        return;
    }

    info!("📅 Startup automation: Processing {} pending events", pending.len());

    let is_writable = !state.provider.is_current_archived().await;
    
    // Fetch default execution mode from autopilot config
    let config = autopilot::entities::autopilot_config::Entity::find()
        .filter(autopilot::entities::autopilot_config::Column::OrgId.eq(&org_id))
        .one(state.provider.registry())
        .await
        .unwrap_or(None);
    let default_mode = config.map(|c| c.mode).unwrap_or("suggested".to_string());
    
    let mut auto_executed = 0u32;
    let mut suggestions = Vec::new();

    for event in &pending {
        let execution_mode = if event.event_type == "backup" {
            "automatic".to_string()
        } else {
            default_mode.clone()
        };

        match execution_mode.as_str() {
            "automatic" => {
                if !is_writable {
                    info!("  Skip automatic event '{}' — year is archived", event.title);
                    continue;
                }
                match crate::commands::lifecycle::calendar::execute_calendar_event_action(&db, event, "system", state.provider.registry()).await {
                    Ok(result) => {
                        info!("  Auto-executed: {} ({})", event.title, result.message.as_deref().unwrap_or("ok"));
                        auto_executed += 1;
                    }
                    Err(e) => {
                        warn!("  Auto-execute failed for '{}': {}", event.title, e);
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
