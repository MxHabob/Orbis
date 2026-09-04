use axum::{extract::{State, Path}, Json, routing::{post, delete}, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::document::{
    upload_document::UploadDocumentUseCase,
    delete_document::DeleteDocumentUseCase,
};
use uuid::Uuid;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct UploadDocumentRequest {
    pub org_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: String,
    pub size: Option<i32>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    pub path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
}

#[derive(Serialize)]
pub struct DocumentResponse {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

async fn upload_document(
    State(state): State<AppState>,
    Json(payload): Json<UploadDocumentRequest>,
) -> Result<Json<DocumentResponse>, String> {
    let ctx = RequestContext { tenant_id: payload.org_id, actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    
    let usecase = UploadDocumentUseCase { 
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone()
    };
    
    match usecase.execute(
        &ctx, payload.org_id, payload.uploaded_by, payload.entity_type, payload.entity_id, 
        payload.name, payload.description, payload.category, payload.asset_type, 
        payload.school_year, payload.term, payload.tags, payload.visibility, 
        payload.size, payload.mime_type, payload.extension, payload.storage_provider, 
        payload.path, payload.url, payload.thumbnail_path, payload.preview_path
    ).await {
        Ok(doc) => Ok(Json(DocumentResponse { id: doc.id })),
        Err(e) => Err(e.to_string()),
    }
}

async fn delete_document(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<SuccessResponse>, String> {
    let ctx = RequestContext { tenant_id: Uuid::new_v4(), actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    
    let usecase = DeleteDocumentUseCase { 
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone()
    };
    
    match usecase.execute(&ctx, document_id).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/documents/upload", post(upload_document))
        .route("/documents/:id", delete(delete_document))
}
