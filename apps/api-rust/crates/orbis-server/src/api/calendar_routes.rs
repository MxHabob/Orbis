use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::calendar::{
    academic_year::CreateAcademicYearUseCase,
    academic_period::CreateAcademicPeriodUseCase,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateYearRequest {
    pub org_id: Uuid,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct YearResponse {
    pub id: Uuid,
    pub name: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreatePeriodRequest {
    pub org_id: Uuid,
    pub year_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub period_type: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PeriodResponse {
    pub id: Uuid,
    pub name: String,
    pub period_type: String,
}

async fn create_year(
    State(state): State<AppState>,
    Json(payload): Json<CreateYearRequest>,
) -> Result<Json<YearResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateAcademicYearUseCase {
        year_repo: state.year_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.name, payload.start_date, payload.end_date).await {
        Ok(year) => Ok(Json(YearResponse {
            id: year.id,
            name: year.name,
            status: year.status,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn create_period(
    State(state): State<AppState>,
    Json(payload): Json<CreatePeriodRequest>,
) -> Result<Json<PeriodResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateAcademicPeriodUseCase {
        period_repo: state.period_repo.clone(),
        year_repo: state.year_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(
        &ctx, payload.org_id, payload.year_id, payload.parent_id,
        payload.name, payload.period_type, payload.start_date, payload.end_date
    ).await {
        Ok(period) => Ok(Json(PeriodResponse {
            id: period.id,
            name: period.name,
            period_type: period.period_type,
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/calendar/years", post(create_year))
        .route("/calendar/periods", post(create_period))
}
