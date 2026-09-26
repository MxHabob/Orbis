//! Document routes — Upload, List, Get, Delete.

use axum::{extract::{State, Path}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::services::CreateDocumentInput;

#[derive(Serialize, ToSchema)]
pub struct DocumentDto {
    pub id: Uuid,
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
    #[serde(alias = "size")]
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    #[serde(alias = "path")]
    pub file_path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct UploadDocumentRequest {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
    #[serde(alias = "size")]
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: Option<String>,
    #[serde(alias = "path")]
    pub file_path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/documents", request_body = UploadDocumentRequest,
    responses((status = 201, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn upload_document(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<UploadDocumentRequest>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let uploaded_by = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
        _ => None,
    };
    let input = CreateDocumentInput {
        uploaded_by,
        entity_type: p.entity_type,
        entity_id: p.entity_id,
        name: p.name,
        description: p.description,
        category: p.category.unwrap_or_else(|| "general".to_string()),
        asset_type: p.asset_type.unwrap_or_else(|| "file".to_string()),
        school_year: p.school_year,
        term: p.term,
        tags: p.tags,
        visibility: p.visibility.unwrap_or_else(|| "internal".to_string()),
        file_size_bytes: p.file_size_bytes,
        mime_type: p.mime_type,
        extension: p.extension,
        storage_provider: p.storage_provider.unwrap_or_else(|| "local".to_string()),
        file_path: p.file_path,
        url: p.url,
        thumbnail_path: p.thumbnail_path,
        preview_path: p.preview_path,
    };
    let doc = state.document_service.create_document(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(DocumentDto {
        id: doc.id,
        org_id: doc.org_id,
        uploaded_by: doc.uploaded_by,
        entity_type: doc.entity_type,
        entity_id: doc.entity_id,
        name: doc.name,
        description: doc.description,
        category: doc.category,
        asset_type: doc.asset_type,
        school_year: doc.school_year,
        term: doc.term,
        tags: doc.tags,
        visibility: doc.visibility,
        file_size_bytes: doc.file_size_bytes,
        mime_type: doc.mime_type,
        extension: doc.extension,
        storage_provider: doc.storage_provider,
        file_path: doc.file_path,
        url: doc.url,
        thumbnail_path: doc.thumbnail_path,
        preview_path: doc.preview_path,
        created_at: doc.created_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/documents",
    responses((status = 200, body = Vec<DocumentDto>)), security(("bearer" = [])), tag = "Document")]
async fn list_documents(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentDto>>>, ApiError> {
    let docs = state.document_service.list_documents_by_org(ctx.tenant_id).await?;
    let dtos = docs.into_iter().map(|doc| DocumentDto {
        id: doc.id,
        org_id: doc.org_id,
        uploaded_by: doc.uploaded_by,
        entity_type: doc.entity_type,
        entity_id: doc.entity_id,
        name: doc.name,
        description: doc.description,
        category: doc.category,
        asset_type: doc.asset_type,
        school_year: doc.school_year,
        term: doc.term,
        tags: doc.tags,
        visibility: doc.visibility,
        file_size_bytes: doc.file_size_bytes,
        mime_type: doc.mime_type,
        extension: doc.extension,
        storage_provider: doc.storage_provider,
        file_path: doc.file_path,
        url: doc.url,
        thumbnail_path: doc.thumbnail_path,
        preview_path: doc.preview_path,
        created_at: doc.created_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path, description = "Document ID")),
    responses((status = 200, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn get_document(
    _ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let doc = state.document_service.get_document(id).await?.ok_or_else(|| ApiError::not_found("Document not found"))?;
    Ok(ApiResponse::ok(DocumentDto {
        id: doc.id,
        org_id: doc.org_id,
        uploaded_by: doc.uploaded_by,
        entity_type: doc.entity_type,
        entity_id: doc.entity_id,
        name: doc.name,
        description: doc.description,
        category: doc.category,
        asset_type: doc.asset_type,
        school_year: doc.school_year,
        term: doc.term,
        tags: doc.tags,
        visibility: doc.visibility,
        file_size_bytes: doc.file_size_bytes,
        mime_type: doc.mime_type,
        extension: doc.extension,
        storage_provider: doc.storage_provider,
        file_path: doc.file_path,
        url: doc.url,
        thumbnail_path: doc.thumbnail_path,
        preview_path: doc.preview_path,
        created_at: doc.created_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path, description = "Document deleted")),
    responses((status = 204, description = "Deleted")), security(("bearer" = [])), tag = "Document")]
async fn delete_document(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    state.document_service.delete_document(&ctx, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DocumentRequirementDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub target_type: String,
    pub target_id: Option<Uuid>,
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    pub is_mandatory: bool,
    pub blocks_enrollment: bool,
    pub condition: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateDocumentRequirementRequest {
    pub target_type: String,
    pub target_id: Option<Uuid>,
    pub code: String,
    pub name_ar: String,
    pub name_en: Option<String>,
    pub is_mandatory: bool,
    pub blocks_enrollment: bool,
    pub condition: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct StudentDocumentFulfillmentDto {
    pub requirement: DocumentRequirementDto,
    pub is_fulfilled: bool,
    pub is_applicable: bool,
    pub document_id: Option<Uuid>,
    pub document_name: Option<String>,
    pub file_path: Option<String>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct EvaluateRequirementsQuery {
    pub grade_level_id: Option<Uuid>,
    pub is_national: Option<bool>,
    pub is_transfer: Option<bool>,
}

// ── Document Requirement Handlers ───────────────────────────────

#[utoipa::path(post, path = "/api/v1/document-requirements", request_body = CreateDocumentRequirementRequest,
    responses((status = 201, body = DocumentRequirementDto)), security(("bearer" = [])), tag = "Document")]
async fn create_document_requirement(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateDocumentRequirementRequest>,
) -> Result<Json<ApiResponse<DocumentRequirementDto>>, ApiError> {
    use orbis_domain::entities::document_requirement::{RequirementTargetType, DocumentCondition};
    
    let target_type = match p.target_type.as_str() {
        "stage" => RequirementTargetType::Stage,
        "grade_level" => RequirementTargetType::GradeLevel,
        "program" => RequirementTargetType::Program,
        _ => RequirementTargetType::Global,
    };

    let condition = match p.condition.as_deref().unwrap_or("all_students") {
        "non_national_only" => DocumentCondition::NonNationalOnly,
        "transfer_students_only" => DocumentCondition::TransferStudentsOnly,
        "special_needs_only" => DocumentCondition::SpecialNeedsOnly,
        _ => DocumentCondition::AllStudents,
    };

    let input = orbis_app::services::CreateDocumentRequirementInput {
        target_type,
        target_id: p.target_id,
        code: p.code,
        name_ar: p.name_ar,
        name_en: p.name_en.unwrap_or_default(),
        is_mandatory: p.is_mandatory,
        blocks_enrollment: p.blocks_enrollment,
        condition,
    };

    let req = state.document_service.create_document_requirement(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(DocumentRequirementDto {
        id: req.id,
        org_id: req.org_id,
        target_type: req.target_type.to_string(),
        target_id: req.target_id,
        code: req.code,
        name_ar: req.name_ar,
        name_en: req.name_en,
        is_mandatory: req.is_mandatory,
        blocks_enrollment: req.blocks_enrollment,
        condition: req.condition.to_string(),
        created_at: req.created_at,
        updated_at: req.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/document-requirements",
    responses((status = 200, body = Vec<DocumentRequirementDto>)), security(("bearer" = [])), tag = "Document")]
async fn list_document_requirements(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentRequirementDto>>>, ApiError> {
    let reqs = state.document_service.list_document_requirements_by_org(ctx.tenant_id).await?;
    let dtos = reqs.into_iter().map(|req| DocumentRequirementDto {
        id: req.id,
        org_id: req.org_id,
        target_type: req.target_type.to_string(),
        target_id: req.target_id,
        code: req.code,
        name_ar: req.name_ar,
        name_en: req.name_en,
        is_mandatory: req.is_mandatory,
        blocks_enrollment: req.blocks_enrollment,
        condition: req.condition.to_string(),
        created_at: req.created_at,
        updated_at: req.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(delete, path = "/api/v1/document-requirements/{id}", params(("id" = Uuid, Path, description = "Requirement ID")),
    responses((status = 204, description = "Deleted")), security(("bearer" = [])), tag = "Document")]
async fn delete_document_requirement(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    state.document_service.delete_document_requirement(&ctx, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/v1/students/{id}/document-requirements", params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = Vec<StudentDocumentFulfillmentDto>)), security(("bearer" = [])), tag = "Document")]
async fn evaluate_student_requirements(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
    axum::extract::Query(q): axum::extract::Query<EvaluateRequirementsQuery>,
) -> Result<Json<ApiResponse<Vec<StudentDocumentFulfillmentDto>>>, ApiError> {
    let fulfillments = state.document_service.evaluate_student_requirements(
        ctx.tenant_id,
        student_id,
        q.grade_level_id,
        q.is_national.unwrap_or(true),
        q.is_transfer.unwrap_or(false),
    ).await?;

    let dtos = fulfillments.into_iter().map(|f| StudentDocumentFulfillmentDto {
        requirement: DocumentRequirementDto {
            id: f.requirement.id,
            org_id: f.requirement.org_id,
            target_type: f.requirement.target_type.to_string(),
            target_id: f.requirement.target_id,
            code: f.requirement.code,
            name_ar: f.requirement.name_ar,
            name_en: f.requirement.name_en,
            is_mandatory: f.requirement.is_mandatory,
            blocks_enrollment: f.requirement.blocks_enrollment,
            condition: f.requirement.condition.to_string(),
            created_at: f.requirement.created_at,
            updated_at: f.requirement.updated_at,
        },
        is_fulfilled: f.is_fulfilled,
        is_applicable: f.is_applicable,
        document_id: f.uploaded_document.as_ref().map(|d| d.id),
        document_name: f.uploaded_document.as_ref().map(|d| d.name.clone()),
        file_path: f.uploaded_document.as_ref().map(|d| d.file_path.clone()),
        uploaded_at: f.uploaded_document.as_ref().map(|d| d.created_at),
    }).collect();

    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/documents", post(upload_document).get(list_documents))
        .route("/api/v1/documents/:id", get(get_document).delete(delete_document))
        .route("/api/v1/document-requirements", post(create_document_requirement).get(list_document_requirements))
        .route("/api/v1/document-requirements/:id", axum::routing::delete(delete_document_requirement))
        .route("/api/v1/students/:id/document-requirements", get(evaluate_student_requirements))
}
