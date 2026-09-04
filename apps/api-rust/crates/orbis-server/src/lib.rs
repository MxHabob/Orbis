pub mod api;
pub mod state;

use axum::Router;
use state::AppState;

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .merge(api::iam_routes::routes())
        .merge(api::org_routes::routes())
        .merge(api::calendar_routes::routes())
        .merge(api::academic_routes::routes())
        .merge(api::student_routes::routes())
        .merge(api::scheduling_routes::routes())
        .merge(api::assessment_routes::routes())
        .with_state(state)
}
