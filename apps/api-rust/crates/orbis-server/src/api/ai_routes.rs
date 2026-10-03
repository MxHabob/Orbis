//! AI Platform API Routes — Stateless Multi-turn Chat & Tool Discovery.

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_domain::entities::ai::{AiChatRequest, AiChatResponse, AiToolDefinition};

#[utoipa::path(
    post,
    path = "/api/v1/ai/chat",
    request_body = AiChatRequest,
    responses((status = 200, description = "AI chat response with executed domain tools and proposed actions", body = AiChatResponse)),
    security(("bearer" = [])),
    tag = "AI"
)]
async fn chat(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<AiChatRequest>,
) -> Result<Json<ApiResponse<AiChatResponse>>, ApiError> {
    let response = state.ai_service.chat(&ctx, payload).await?;
    Ok(ApiResponse::ok(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/ai/tools",
    responses((status = 200, description = "List registered domain AI tools with schemas and permissions", body = Vec<AiToolDefinition>)),
    security(("bearer" = [])),
    tag = "AI"
)]
async fn list_tools(
    _ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AiToolDefinition>>>, ApiError> {
    let tools = state.ai_service.get_available_tools();
    Ok(ApiResponse::ok(tools))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/ai/chat", post(chat))
        .route("/api/v1/ai/tools", get(list_tools))
}
