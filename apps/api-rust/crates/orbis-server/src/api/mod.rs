//! Orbis API — Shared contracts, error handling, and module re-exports.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use orbis_domain::errors::DomainError;

// ── Shared DTOs ────────────────────────────────────────────────

/// Uniform success envelope.
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Json<Self> {
        Json(Self { data, request_id: None })
    }
    pub fn ok_with_id(data: T, request_id: String) -> Json<Self> {
        Json(Self { data, request_id: Some(request_id) })
    }
}

/// Paginated response envelope.
#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub count: usize,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn from_cursor_page(page: orbis_domain::queries::pagination::CursorPage<T>) -> Json<ApiResponse<Self>> {
        ApiResponse::ok(Self {
            count: page.count,
            next_cursor: page.next_cursor,
            has_more: page.has_more,
            items: page.items,
        })
    }
}

/// Uniform error response body.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct OrbisErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

// ── ApiError ───────────────────────────────────────────────────

/// Central API error type that converts to proper HTTP responses.
pub struct ApiError {
    pub status: StatusCode,
    pub body: OrbisErrorBody,
}

impl ApiError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            body: OrbisErrorBody {
                code: "NOT_FOUND".into(),
                message: msg.into(),
                details: None,
                request_id: None,
            },
        }
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: OrbisErrorBody {
                code: "VALIDATION_ERROR".into(),
                message: msg.into(),
                details: None,
                request_id: None,
            },
        }
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            body: OrbisErrorBody {
                code: "UNAUTHORIZED".into(),
                message: msg.into(),
                details: None,
                request_id: None,
            },
        }
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            body: OrbisErrorBody {
                code: "FORBIDDEN".into(),
                message: msg.into(),
                details: None,
                request_id: None,
            },
        }
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            body: OrbisErrorBody {
                code: "CONFLICT".into(),
                message: msg.into(),
                details: None,
                request_id: None,
            },
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: OrbisErrorBody {
                code: "INTERNAL_ERROR".into(),
                message: msg.into(),
                details: None,
                request_id: None,
            },
        }
    }

    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.body.request_id = Some(id.into());
        self
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match &err {
            DomainError::NotFound(_) => Self::not_found(err.to_string()),
            DomainError::Validation(_) => Self::bad_request(err.to_string()),
            DomainError::Conflict(_) => Self::conflict(err.to_string()),
            DomainError::BusinessRule(_) => Self::bad_request(err.to_string()),
            DomainError::AuthenticationFailed => Self::unauthorized("Authentication failed"),
            DomainError::Unauthorized(_) => Self::unauthorized(err.to_string()),
            DomainError::Forbidden(_) => Self::forbidden(err.to_string()),
            DomainError::AlreadyInitialized => Self::conflict("System has already been initialized"),
            DomainError::BootstrapFailed(msg) => Self::bad_request(format!("Bootstrap failed: {msg}")),
            DomainError::Internal(_) => Self::internal(err.to_string()),
        }
    }
}

// ── Route modules ──────────────────────────────────────────────

pub mod extractors;
pub mod iam_routes;
pub mod org_routes;
pub mod calendar_routes;
pub mod academic_routes;
pub mod student_routes;
pub mod scheduling_routes;
pub mod assessment_routes;
pub mod exam_routes;
pub mod finance_routes;
pub mod document_routes;
pub mod integration_routes;
pub mod report_routes;
pub mod override_routes;
pub mod progression_routes;
pub mod ai_routes;
pub mod autopilot_routes;
