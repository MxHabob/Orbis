use sea_orm::EntityTrait;
use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::academic_calendar_event,
    entities::org_academic_policy,
    service::calendar::{
        CreateCalendarEventRequest,
        UpdateCalendarEventRequest,
        CreateOrgPolicyRequest,
        CalendarProcessingReport,
        EventProcessingResult,
    },
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Calendar Events ────────────────────────────────────────────────

#[tauri::command]
pub async fn get_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<academic_calendar_event::Model>> {
    state.service().await
        .get_calendar_events(&org_id, &cycle_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_pending_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<academic_calendar_event::Model>> {
    let today = chrono::Local::now().date_naive().to_string();
    state.service().await
        .get_pending_events(&org_id, &today)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_calendar_event(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateCalendarEventRequest,
) -> CmdResult<academic_calendar_event::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .create_calendar_event(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_calendar_event(
    state: State<'_, Arc<AppState>>,
    event_id: String,
    request: UpdateCalendarEventRequest,
) -> CmdResult<academic_calendar_event::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .update_calendar_event(&event_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_calendar_event(
    state: State<'_, Arc<AppState>>,
    event_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .delete_calendar_event(&event_id)
        .await
        .map_err(CommandError::from)
}

/// Execute a single calendar event's action (manually triggered by user).
#[tauri::command]
pub async fn execute_calendar_event(
    state: State<'_, Arc<AppState>>,
    event_id: String,
    user_id: String,
) -> CmdResult<EventProcessingResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;

    let service = state.service().await;

    // Find the event
    let event = core_domain::entities::academic_calendar_event::Entity::find_by_id(&event_id)
        .one(&service.repo.db)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?
        .ok_or_else(|| CommandError::from("Calendar event not found".to_string()))?;

    service
        .execute_calendar_event_action(&event, &user_id)
        .await
        .map_err(CommandError::from)
}

/// Generate default calendar events for a cycle.
#[tauri::command]
pub async fn generate_default_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<academic_calendar_event::Model>> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;

    let service = state.service().await;

    // Get the cycle
    let cycle = service.get_cycle(&cycle_id).await.map_err(CommandError::from)?;

    // Get default execution mode from policy
    let policy = service
        .get_org_academic_policy(&org_id)
        .await
        .map_err(CommandError::from)?;

    service
        .generate_default_calendar_events(&org_id, &cycle, &policy.default_event_execution_mode)
        .await
        .map_err(CommandError::from)
}

/// Process pending calendar events (called on app startup).
/// Returns a report of what was executed, suggested, skipped, or blocked.
#[tauri::command]
pub async fn process_pending_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
) -> CmdResult<CalendarProcessingReport> {
    let service = state.service().await;
    let today = chrono::Local::now().date_naive().to_string();

    let pending = service
        .get_pending_events(&org_id, &today)
        .await
        .map_err(CommandError::from)?;

    let mut report = CalendarProcessingReport::new();
    let is_writable = !state.provider.is_current_archived().await;

    for event in &pending {
        match event.execution_mode.as_str() {
            "automatic" => {
                if !is_writable {
                    report.blocked.push(EventProcessingResult {
                        event_id: event.id.clone(),
                        event_type: event.event_type.clone(),
                        title: event.title.clone(),
                        execution_mode: event.execution_mode.clone(),
                        outcome: "blocked".to_string(),
                        message: Some("Year is archived (read-only)".to_string()),
                    });
                    continue;
                }
                match service.execute_calendar_event_action(event, &user_id).await {
                    Ok(result) => report.executed.push(result),
                    Err(e) => report.blocked.push(EventProcessingResult {
                        event_id: event.id.clone(),
                        event_type: event.event_type.clone(),
                        title: event.title.clone(),
                        execution_mode: event.execution_mode.clone(),
                        outcome: "blocked".to_string(),
                        message: Some(e),
                    }),
                }
            }
            "suggested" => {
                report.suggested.push(EventProcessingResult {
                    event_id: event.id.clone(),
                    event_type: event.event_type.clone(),
                    title: event.title.clone(),
                    execution_mode: event.execution_mode.clone(),
                    outcome: "suggested".to_string(),
                    message: Some(format!("Event '{}' is due. Review and execute.", event.title)),
                });
            }
            _ => {
                // manual — skip entirely
                report.skipped.push(EventProcessingResult {
                    event_id: event.id.clone(),
                    event_type: event.event_type.clone(),
                    title: event.title.clone(),
                    execution_mode: event.execution_mode.clone(),
                    outcome: "skipped".to_string(),
                    message: Some("Manual mode — requires explicit user action".to_string()),
                });
            }
        }
    }

    Ok(report)
}

// ── Org Academic Policies ──────────────────────────────────────────

#[tauri::command]
pub async fn get_org_academic_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<org_academic_policy::Model> {
    state.service().await
        .get_org_academic_policy(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_org_academic_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateOrgPolicyRequest,
) -> CmdResult<org_academic_policy::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .update_org_academic_policy(&org_id, request)
        .await
        .map_err(CommandError::from)
}
