//! Report routes — Report generation, job status, file download.
//! These are kept minimal here — the full implementation is in report_cmds.rs on the desktop side.

use axum::{extract::State, routing::{get, post}, Json, Router};
use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;

// Placeholder routes — the report system already has its own dedicated use cases.
// These will be wired to the existing report_cmds patterns.

pub fn routes() -> Router<AppState> {
    Router::new()
}
