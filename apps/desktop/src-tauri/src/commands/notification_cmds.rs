//! Tauri IPC commands — Notifications and User Notification Preferences.

use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_app::ports::repositories::NotificationFilter;
use orbis_domain::iam::actor::Actor;

#[derive(Serialize)]
pub struct NotificationDto {
    pub id: String,
    pub category: String,
    pub priority: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub body: String,
    pub body_ar: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub action_route: Option<String>,
    pub source_event: String,
    pub is_read: bool,
    pub is_dismissed: bool,
    pub read_at: Option<String>,
    pub created_at: String,
}

pub fn notification_dto(n: orbis_domain::entities::notification::Notification) -> NotificationDto {
    NotificationDto {
        id: n.id.to_string(),
        category: n.category.to_string(),
        priority: n.priority.to_string(),
        title: n.title,
        title_ar: n.title_ar,
        body: n.body,
        body_ar: n.body_ar,
        entity_type: n.entity_type,
        entity_id: n.entity_id.map(|e| e.to_string()),
        action_route: n.action_route,
        source_event: n.source_event,
        is_read: n.is_read,
        is_dismissed: n.is_dismissed,
        read_at: n.read_at.map(|t| t.to_rfc3339()),
        created_at: n.created_at.to_rfc3339(),
    }
}

#[derive(Deserialize)]
pub struct ListNotificationsParamsDto {
    pub category: Option<String>,
    pub is_read: Option<bool>,
    pub is_dismissed: Option<bool>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Serialize)]
pub struct UnreadCountDto {
    pub count: i64,
}

#[derive(Serialize)]
pub struct MarkAllReadDto {
    pub updated: u64,
}

#[derive(Serialize)]
pub struct NotificationPreferenceResponseDto {
    pub id: String,
    pub category: String,
    pub in_app_enabled: bool,
    pub email_enabled: bool,
    pub push_enabled: bool,
}

#[derive(Deserialize)]
pub struct NotificationPreferenceInputDto {
    pub category: String,
    pub in_app_enabled: bool,
    pub email_enabled: bool,
    pub push_enabled: bool,
}

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[tauri::command]
pub async fn notification_list(
    params: Option<ListNotificationsParamsDto>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<NotificationDto>>, String> {
    let ctx = make_ctx(&state);
    let filter = NotificationFilter {
        category: params.as_ref().and_then(|p| p.category.clone()),
        is_read: params.as_ref().and_then(|p| p.is_read),
        is_dismissed: params
            .as_ref()
            .and_then(|p| p.is_dismissed)
            .unwrap_or(false),
        limit: params
            .as_ref()
            .and_then(|p| p.limit)
            .map(|v| v as i64)
            .unwrap_or(50),
        offset: params
            .as_ref()
            .and_then(|p| p.offset)
            .map(|v| v as i64)
            .unwrap_or(0),
    };

    let notifications = state
        .notification_service
        .list(&ctx, &filter)
        .await
        .map_err(|e| e.to_string())?;

    let dtos: Vec<NotificationDto> = notifications.into_iter().map(notification_dto).collect();
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn notification_unread_count(
    state: State<'_, AppState>,
) -> Result<ApiResponse<UnreadCountDto>, String> {
    let ctx = make_ctx(&state);
    let count = state
        .notification_service
        .unread_count(&ctx)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(UnreadCountDto { count }))
}

#[tauri::command]
pub async fn notification_mark_read(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let notif_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    state
        .notification_service
        .mark_read(&ctx, notif_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(()))
}

#[tauri::command]
pub async fn notification_mark_all_read(
    state: State<'_, AppState>,
) -> Result<ApiResponse<MarkAllReadDto>, String> {
    let ctx = make_ctx(&state);
    let updated = state
        .notification_service
        .mark_all_read(&ctx)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(MarkAllReadDto { updated }))
}

#[tauri::command]
pub async fn notification_dismiss(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let notif_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    state
        .notification_service
        .dismiss(&ctx, notif_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(()))
}

#[tauri::command]
pub async fn notification_get_preferences(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<NotificationPreferenceResponseDto>>, String> {
    let ctx = make_ctx(&state);
    let prefs = state
        .notification_service
        .get_preferences(&ctx)
        .await
        .map_err(|e| e.to_string())?;

    let dtos: Vec<NotificationPreferenceResponseDto> = prefs
        .into_iter()
        .map(|p| NotificationPreferenceResponseDto {
            id: p.id.to_string(),
            category: p.category,
            in_app_enabled: p.in_app_enabled,
            email_enabled: p.email_enabled,
            push_enabled: p.push_enabled,
        })
        .collect();

    Ok(ok(dtos))
}

#[tauri::command]
pub async fn notification_update_preferences(
    preferences: Vec<NotificationPreferenceInputDto>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<NotificationPreferenceResponseDto>>, String> {
    let ctx = make_ctx(&state);
    let user_id = match ctx.actor {
        Actor::User(uid) => uid,
        _ => return Err("User session required".to_string()),
    };

    let domain_prefs: Vec<
        orbis_domain::entities::notification_preference::UserNotificationPreference,
    > = preferences
        .into_iter()
        .map(
            |p| orbis_domain::entities::notification_preference::UserNotificationPreference {
                id: Uuid::new_v4(),
                user_id,
                org_id: ctx.tenant_id,
                category: p.category,
                in_app_enabled: p.in_app_enabled,
                email_enabled: p.email_enabled,
                push_enabled: p.push_enabled,
            },
        )
        .collect();

    let updated = state
        .notification_service
        .update_preferences(&ctx, domain_prefs)
        .await
        .map_err(|e| e.to_string())?;

    let dtos: Vec<NotificationPreferenceResponseDto> = updated
        .into_iter()
        .map(|p| NotificationPreferenceResponseDto {
            id: p.id.to_string(),
            category: p.category,
            in_app_enabled: p.in_app_enabled,
            email_enabled: p.email_enabled,
            push_enabled: p.push_enabled,
        })
        .collect();

    Ok(ok(dtos))
}
