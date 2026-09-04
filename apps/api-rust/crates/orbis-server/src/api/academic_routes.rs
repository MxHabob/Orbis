use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::academic::{
    subject::CreateSubjectUseCase,
    class::CreateClassUseCase,
    class_subject::AssignClassSubjectUseCase,
};
use uuid::Uuid;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateSubjectRequest {
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub credit_hours: Option<i32>,
    pub is_core: bool,
}

#[derive(Serialize)]
pub struct SubjectResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateClassRequest {
    pub org_id: Uuid,
    pub unit_id: Uuid,
    pub period_id: Uuid,
    pub name: String,
    pub level: String,
}

#[derive(Serialize)]
pub struct ClassResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AssignClassSubjectRequest {
    pub class_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub primary_teacher_id: Uuid,
    pub weekly_hours: Option<i32>,
}

#[derive(Serialize)]
pub struct ClassSubjectResponse {
    pub id: Uuid,
}

async fn create_subject(
    State(state): State<AppState>,
    Json(payload): Json<CreateSubjectRequest>,
) -> Result<Json<SubjectResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateSubjectUseCase {
        subject_repo: state.subject_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.unit_id, payload.code, payload.name, payload.credit_hours, payload.is_core).await {
        Ok(subject) => Ok(Json(SubjectResponse {
            id: subject.id,
            name: subject.name,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn create_class(
    State(state): State<AppState>,
    Json(payload): Json<CreateClassRequest>,
) -> Result<Json<ClassResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateClassUseCase {
        class_repo: state.class_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.unit_id, payload.period_id, payload.name, payload.level).await {
        Ok(class) => Ok(Json(ClassResponse {
            id: class.id,
            name: class.name,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn assign_class_subject(
    State(state): State<AppState>,
    Json(payload): Json<AssignClassSubjectRequest>,
) -> Result<Json<ClassSubjectResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(), // Should be derived from session
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = AssignClassSubjectUseCase {
        class_subject_repo: state.class_subject_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.class_id, payload.subject_id, payload.period_id, payload.primary_teacher_id, payload.weekly_hours).await {
        Ok(class_subject) => Ok(Json(ClassSubjectResponse {
            id: class_subject.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/academic/subjects", post(create_subject))
        .route("/academic/classes", post(create_class))
        .route("/academic/class-subjects", post(assign_class_subject))
}
