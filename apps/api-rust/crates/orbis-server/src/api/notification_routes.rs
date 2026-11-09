//! Notification routes — Notification center, preferences, unread counts, and real-time SSE stream.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, patch, post},
};
use chrono::{DateTime, Utc};
use orbis_domain::entities::notification::Notification;
use orbis_domain::iam::actor::Actor;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::BroadcastStream;
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::ports::repositories::NotificationFilter;

// ── DTOs ───────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct NotificationDto {
    pub id: Uuid,
    pub category: String,
    pub priority: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub body: String,
    pub body_ar: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub action_route: Option<String>,
    pub source_event: String,
    pub is_read: bool,
    pub is_dismissed: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<&Notification> for NotificationDto {
    fn from(n: &Notification) -> Self {
        Self {
            id: n.id,
            category: n.category.clone(),
            priority: n.priority.clone(),
            title: n.title.clone(),
            title_ar: n.title_ar.clone(),
            body: n.body.clone(),
            body_ar: n.body_ar.clone(),
            entity_type: n.entity_type.clone(),
            entity_id: n.entity_id,
            action_route: n.action_route.clone(),
            source_event: n.source_event.clone(),
            is_read: n.is_read,
            is_dismissed: n.is_dismissed,
            read_at: n.read_at,
            created_at: n.created_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UnreadCountDto {
    pub count: i64,
}

#[derive(Serialize, ToSchema)]
pub struct MarkAllReadDto {
    pub updated: u64,
}

#[derive(Deserialize, ToSchema, utoipa::IntoParams)]
pub struct ListNotificationsQuery {
    pub category: Option<String>,
    pub is_read: Option<bool>,
    pub is_dismissed: Option<bool>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct NotificationPreferenceResponseDto {
    pub id: Uuid,
    pub category: String,
    pub in_app_enabled: bool,
    pub email_enabled: bool,
    pub push_enabled: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct NotificationPreferenceDto {
    pub category: String,
    pub in_app_enabled: bool,
    pub email_enabled: bool,
    pub push_enabled: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdatePreferencesRequest {
    pub preferences: Vec<NotificationPreferenceDto>,
}

// ── Handlers ───────────────────────────────────────────────────

/// List notifications for the authenticated user.
#[utoipa::path(
    get,
    path = "/api/v1/notifications",
    tag = "Notifications",
    params(ListNotificationsQuery),
    responses(
        (status = 200, description = "List of user notifications", body = ApiResponse<Vec<NotificationDto>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_notifications(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(query): Query<ListNotificationsQuery>,
) -> Result<Json<ApiResponse<Vec<NotificationDto>>>, ApiError> {
    let filter = NotificationFilter {
        category: query.category,
        is_read: query.is_read,
        is_dismissed: query.is_dismissed.unwrap_or(false),
        limit: query.limit.map(|v| v as i64).unwrap_or(50),
        offset: query.offset.map(|v| v as i64).unwrap_or(0),
    };

    let notifications = state.notification_service.list(&ctx.0, &filter).await?;

    let dtos: Vec<NotificationDto> = notifications.iter().map(NotificationDto::from).collect();

    Ok(ApiResponse::ok(dtos))
}

/// Stream notifications in real-time via Server-Sent Events (SSE).
#[utoipa::path(
    get,
    path = "/api/v1/notifications/stream",
    tag = "Notifications",
    responses(
        (status = 200, description = "SSE stream of real-time notifications", content_type = "text/event-stream"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn notification_stream(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, std::convert::Infallible>>>, ApiError>
{
    let rx = state
        .notification_service
        .broadcast_tx
        .as_ref()
        .map(|tx| tx.subscribe())
        .ok_or_else(|| ApiError::internal("Notification broadcaster not initialized"))?;

    let user_id = match ctx.0.actor {
        Actor::User(uid) => uid,
        _ => return Err(ApiError::unauthorized("User session required")),
    };
    let org_id = ctx.0.tenant_id;

    let stream = BroadcastStream::new(rx).filter_map(move |result| match result {
        Ok(notif) => {
            if notif.recipient_user_id == user_id && notif.org_id == org_id {
                let dto = NotificationDto::from(&notif);
                let json = serde_json::to_string(&dto).unwrap_or_default();
                Some(Ok(Event::default().event("notification").data(json)))
            } else {
                None
            }
        }
        Err(_) => None,
    });

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

/// Get unread notification count for badge display.
#[utoipa::path(
    get,
    path = "/api/v1/notifications/unread-count",
    tag = "Notifications",
    responses(
        (status = 200, description = "Unread notification count", body = ApiResponse<UnreadCountDto>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_unread_count(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<UnreadCountDto>>, ApiError> {
    let count = state.notification_service.unread_count(&ctx.0).await?;

    Ok(ApiResponse::ok(UnreadCountDto { count }))
}

/// Mark a single notification as read.
#[utoipa::path(
    patch,
    path = "/api/v1/notifications/{id}/read",
    tag = "Notifications",
    params(("id" = Uuid, Path, description = "Notification ID")),
    responses(
        (status = 200, description = "Notification marked as read"),
        (status = 404, description = "Notification not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn mark_read(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.notification_service.mark_read(&ctx.0, id).await?;

    Ok(ApiResponse::ok(()))
}

/// Mark all notifications as read for the authenticated user.
#[utoipa::path(
    post,
    path = "/api/v1/notifications/read-all",
    tag = "Notifications",
    responses(
        (status = 200, description = "All notifications marked as read", body = ApiResponse<MarkAllReadDto>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn mark_all_read(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MarkAllReadDto>>, ApiError> {
    let updated = state.notification_service.mark_all_read(&ctx.0).await?;

    Ok(ApiResponse::ok(MarkAllReadDto { updated }))
}

/// Dismiss (soft-delete) a notification.
#[utoipa::path(
    patch,
    path = "/api/v1/notifications/{id}/dismiss",
    tag = "Notifications",
    params(("id" = Uuid, Path, description = "Notification ID")),
    responses(
        (status = 200, description = "Notification dismissed"),
        (status = 404, description = "Notification not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn dismiss_notification(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.notification_service.dismiss(&ctx.0, id).await?;

    Ok(ApiResponse::ok(()))
}

/// Get user's notification channel preferences.
#[utoipa::path(
    get,
    path = "/api/v1/notifications/preferences",
    tag = "Notifications",
    responses(
        (status = 200, description = "User notification preferences", body = ApiResponse<Vec<NotificationPreferenceResponseDto>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_preferences(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<NotificationPreferenceResponseDto>>>, ApiError> {
    let preferences = state.notification_service.get_preferences(&ctx.0).await?;

    let dtos: Vec<NotificationPreferenceResponseDto> = preferences
        .into_iter()
        .map(|p| NotificationPreferenceResponseDto {
            id: p.id,
            category: p.category,
            in_app_enabled: p.in_app_enabled,
            email_enabled: p.email_enabled,
            push_enabled: p.push_enabled,
        })
        .collect();

    Ok(ApiResponse::ok(dtos))
}

/// Update user's notification channel preferences.
#[utoipa::path(
    put,
    path = "/api/v1/notifications/preferences",
    tag = "Notifications",
    request_body = UpdatePreferencesRequest,
    responses(
        (status = 200, description = "Updated notification preferences", body = ApiResponse<Vec<NotificationPreferenceResponseDto>>),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_preferences(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(body): Json<UpdatePreferencesRequest>,
) -> Result<Json<ApiResponse<Vec<NotificationPreferenceResponseDto>>>, ApiError> {
    let user_id = match ctx.0.actor {
        Actor::User(uid) => uid,
        _ => return Err(ApiError::unauthorized("User session required")),
    };

    let preferences: Vec<
        orbis_domain::entities::notification_preference::UserNotificationPreference,
    > = body
        .preferences
        .into_iter()
        .map(
            |p| orbis_domain::entities::notification_preference::UserNotificationPreference {
                id: uuid::Uuid::new_v4(),
                user_id,
                org_id: ctx.0.tenant_id,
                category: p.category,
                in_app_enabled: p.in_app_enabled,
                email_enabled: p.email_enabled,
                push_enabled: p.push_enabled,
            },
        )
        .collect();

    let updated = state
        .notification_service
        .update_preferences(&ctx.0, preferences)
        .await?;

    let dtos: Vec<NotificationPreferenceResponseDto> = updated
        .into_iter()
        .map(|p| NotificationPreferenceResponseDto {
            id: p.id,
            category: p.category,
            in_app_enabled: p.in_app_enabled,
            email_enabled: p.email_enabled,
            push_enabled: p.push_enabled,
        })
        .collect();

    Ok(ApiResponse::ok(dtos))
}

// ── Router ─────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/notifications", get(list_notifications))
        .route("/api/v1/notifications/stream", get(notification_stream))
        .route("/api/v1/notifications/unread-count", get(get_unread_count))
        .route("/api/v1/notifications/read-all", post(mark_all_read))
        .route(
            "/api/v1/notifications/preferences",
            get(get_preferences).put(update_preferences),
        )
        .route("/api/v1/notifications/:id/read", patch(mark_read))
        .route(
            "/api/v1/notifications/:id/dismiss",
            patch(dismiss_notification),
        )
}
