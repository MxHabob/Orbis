//! Document routes — Upload, List, Get, Update, Delete, Requirements & Verification.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::{
    CreateDocumentInput, UpdateDocumentInput, UpdateDocumentRequirementInput,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DocumentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub org_unit_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub uploaded_by: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub academic_year_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub tags: Option<String>,
    pub visibility: String,
    #[serde(alias = "size")]
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub checksum_sha256: Option<String>,
    pub storage_provider: String,
    #[serde(alias = "path")]
    pub file_path: String,
    pub storage_key: Option<String>,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub is_archived: bool,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
    pub archive_reason: Option<String>,
    pub is_legal_hold: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn doc_to_dto(doc: orbis_domain::entities::document::Document) -> DocumentDto {
    DocumentDto {
        id: doc.id,
        org_id: doc.org_id,
        org_unit_id: doc.org_unit_id,
        folder_id: doc.folder_id,
        uploaded_by: doc.uploaded_by,
        entity_type: doc.entity_type,
        entity_id: doc.entity_id,
        name: doc.name,
        description: doc.description,
        category: doc.category,
        asset_type: doc.asset_type,
        academic_year_id: doc.academic_year_id,
        term_id: doc.term_id,
        tags: doc.tags,
        visibility: doc.visibility,
        file_size_bytes: doc.file_size_bytes,
        mime_type: doc.mime_type,
        extension: doc.extension,
        checksum_sha256: doc.checksum_sha256,
        storage_provider: doc.storage_provider,
        file_path: doc.file_path,
        storage_key: doc.storage_key,
        url: doc.url,
        thumbnail_path: doc.thumbnail_path,
        preview_path: doc.preview_path,
        is_archived: doc.is_archived,
        archived_at: doc.archived_at,
        archived_by: doc.archived_by,
        archive_reason: doc.archive_reason,
        is_legal_hold: doc.is_legal_hold,
        deleted_at: doc.deleted_at,
        deleted_by: doc.deleted_by,
        created_at: doc.created_at,
        updated_at: doc.updated_at,
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UploadDocumentRequest {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub academic_year_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
    #[serde(alias = "size")]
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub checksum_sha256: Option<String>,
    pub storage_provider: Option<String>,
    #[serde(alias = "path")]
    pub file_path: String,
    pub storage_key: Option<String>,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateDocumentRequest {
    pub name: Option<String>,
    pub folder_id: Option<Uuid>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub academic_year_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct ListDocumentsQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/documents", request_body = UploadDocumentRequest,
    responses((status = 201, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn upload_document(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<UploadDocumentRequest>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let uploaded_by = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
        _ => None,
    };
    let input = CreateDocumentInput {
        org_unit_id: p.org_unit_id,
        folder_id: p.folder_id,
        uploaded_by,
        entity_type: p.entity_type,
        entity_id: p.entity_id,
        name: p.name,
        description: p.description,
        category: p.category.unwrap_or_else(|| "general".to_string()),
        asset_type: p.asset_type.unwrap_or_else(|| "file".to_string()),
        academic_year_id: p.academic_year_id,
        term_id: p.term_id,
        tags: p.tags,
        visibility: p.visibility.unwrap_or_else(|| "internal".to_string()),
        file_size_bytes: p.file_size_bytes,
        mime_type: p.mime_type,
        extension: p.extension,
        checksum_sha256: p.checksum_sha256,
        storage_provider: p.storage_provider.unwrap_or_else(|| "local".to_string()),
        file_path: p.file_path.clone(),
        storage_key: p.storage_key.or(Some(p.file_path)),
        url: p.url,
        thumbnail_path: p.thumbnail_path,
        preview_path: p.preview_path,
    };
    let doc = state
        .document_service
        .create_document(&ctx, ctx.tenant_id, input)
        .await?;
    Ok(ApiResponse::ok(doc_to_dto(doc)))
}

#[utoipa::path(get, path = "/api/v1/documents", params(ListDocumentsQuery),
    responses((status = 200, body = Vec<DocumentDto>)), security(("bearer" = [])), tag = "Document")]
async fn list_documents(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(query): Query<ListDocumentsQuery>,
) -> Result<Json<ApiResponse<Vec<DocumentDto>>>, ApiError> {
    let docs = if let Some(fid) = query.folder_id {
        state
            .document_service
            .document_repo
            .find_by_folder(ctx.tenant_id, fid)
            .await?
    } else if let (Some(ref et), Some(eid)) = (query.entity_type, query.entity_id) {
        state
            .document_service
            .list_documents_by_entity(et, eid)
            .await?
    } else {
        state
            .document_service
            .list_documents_by_org(ctx.tenant_id)
            .await?
    };
    Ok(ApiResponse::ok(docs.into_iter().map(doc_to_dto).collect()))
}

#[utoipa::path(get, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path, description = "Document ID")),
    responses((status = 200, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn get_document(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let doc = state
        .document_service
        .get_document(id)
        .await?
        .ok_or_else(|| ApiError::not_found("Document not found"))?;
    Ok(ApiResponse::ok(doc_to_dto(doc)))
}

#[utoipa::path(patch, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path, description = "Document ID")),
    request_body = UpdateDocumentRequest,
    responses((status = 200, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn update_document(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateDocumentRequest>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let input = UpdateDocumentInput {
        name: p.name,
        folder_id: p.folder_id,
        description: p.description,
        category: p.category,
        asset_type: p.asset_type,
        academic_year_id: p.academic_year_id,
        term_id: p.term_id,
        tags: p.tags,
        visibility: p.visibility,
    };
    let doc = state
        .document_service
        .update_document(&ctx, id, input)
        .await?;
    Ok(ApiResponse::ok(doc_to_dto(doc)))
}

#[utoipa::path(delete, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path, description = "Document deleted")),
    responses((status = 204, description = "Deleted")), security(("bearer" = [])), tag = "Document")]
async fn delete_document(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
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

fn doc_requirement_to_dto(
    r: orbis_domain::entities::document_requirement::DocumentRequirement,
) -> DocumentRequirementDto {
    DocumentRequirementDto {
        id: r.id,
        org_id: r.org_id,
        target_type: r.target_type.to_string(),
        target_id: r.target_id,
        code: r.code,
        name_ar: r.name_ar,
        name_en: r.name_en,
        is_mandatory: r.is_mandatory,
        blocks_enrollment: r.blocks_enrollment,
        condition: r.condition.to_string(),
        created_at: r.created_at,
        updated_at: r.updated_at,
    }
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

#[derive(Deserialize, ToSchema)]
pub struct UpdateDocumentRequirementRequest {
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub code: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub is_mandatory: Option<bool>,
    pub blocks_enrollment: Option<bool>,
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

#[derive(Deserialize, IntoParams)]
pub struct EvaluateRequirementsQuery {
    pub grade_level_id: Option<Uuid>,
    pub is_national: Option<bool>,
    pub is_transfer: Option<bool>,
    pub is_special_needs: Option<bool>,
}

fn parse_requirement_target(
    value: &str,
    target_id: Option<Uuid>,
) -> Result<orbis_domain::entities::document_requirement::RequirementTargetType, ApiError> {
    use orbis_domain::entities::document_requirement::RequirementTargetType;

    match value {
        "global" if target_id.is_none() => Ok(RequirementTargetType::Global),
        "grade_level" if target_id.is_some() => Ok(RequirementTargetType::GradeLevel),
        "global" => Err(ApiError::bad_request(
            "Global document requirements cannot have a target ID",
        )),
        "grade_level" => Err(ApiError::bad_request(
            "Grade-level document requirements require a target ID",
        )),
        "stage" | "program" => Err(ApiError::bad_request(
            "Stage and program document targeting is not supported yet",
        )),
        _ => Err(ApiError::bad_request(
            "Unsupported document requirement target",
        )),
    }
}

fn parse_requirement_condition(
    value: &str,
) -> Result<orbis_domain::entities::document_requirement::DocumentCondition, ApiError> {
    use orbis_domain::entities::document_requirement::DocumentCondition;

    match value {
        "all_students" => Ok(DocumentCondition::AllStudents),
        "non_national_only" => Ok(DocumentCondition::NonNationalOnly),
        "transfer_students_only" => Ok(DocumentCondition::TransferStudentsOnly),
        "special_needs_only" => Ok(DocumentCondition::SpecialNeedsOnly),
        _ => Err(ApiError::bad_request(
            "Unsupported document requirement condition",
        )),
    }
}

// ── Document Requirement Handlers ───────────────────────────────

#[utoipa::path(post, path = "/api/v1/document-requirements", request_body = CreateDocumentRequirementRequest,
    responses((status = 201, body = DocumentRequirementDto)), security(("bearer" = [])), tag = "Document")]
async fn create_document_requirement(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateDocumentRequirementRequest>,
) -> Result<Json<ApiResponse<DocumentRequirementDto>>, ApiError> {
    let target_type = parse_requirement_target(&p.target_type, p.target_id)?;
    let condition = parse_requirement_condition(p.condition.as_deref().unwrap_or("all_students"))?;

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

    let req = state
        .document_service
        .create_document_requirement(&ctx, ctx.tenant_id, input)
        .await?;
    Ok(ApiResponse::ok(doc_requirement_to_dto(req)))
}

#[utoipa::path(get, path = "/api/v1/document-requirements",
    responses((status = 200, body = Vec<DocumentRequirementDto>)), security(("bearer" = [])), tag = "Document")]
async fn list_document_requirements(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentRequirementDto>>>, ApiError> {
    let reqs = state
        .document_service
        .list_document_requirements_by_org(ctx.tenant_id)
        .await?;
    Ok(ApiResponse::ok(
        reqs.into_iter().map(doc_requirement_to_dto).collect(),
    ))
}

#[utoipa::path(get, path = "/api/v1/document-requirements/{id}", params(("id" = Uuid, Path, description = "Requirement ID")),
    responses((status = 200, body = DocumentRequirementDto)), security(("bearer" = [])), tag = "Document")]
async fn get_document_requirement(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentRequirementDto>>, ApiError> {
    let req = state
        .document_service
        .get_document_requirement(id)
        .await?
        .ok_or_else(|| ApiError::not_found("Document requirement not found"))?;
    Ok(ApiResponse::ok(doc_requirement_to_dto(req)))
}

#[utoipa::path(patch, path = "/api/v1/document-requirements/{id}", params(("id" = Uuid, Path, description = "Requirement ID")),
    request_body = UpdateDocumentRequirementRequest,
    responses((status = 200, body = DocumentRequirementDto)), security(("bearer" = [])), tag = "Document")]
async fn update_document_requirement(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateDocumentRequirementRequest>,
) -> Result<Json<ApiResponse<DocumentRequirementDto>>, ApiError> {
    let target_type = p
        .target_type
        .as_deref()
        .map(|value| parse_requirement_target(value, p.target_id))
        .transpose()?;
    let condition = p
        .condition
        .as_deref()
        .map(parse_requirement_condition)
        .transpose()?;
    let target_id = match p.target_type.as_deref() {
        Some("global") => Some(None),
        _ => p.target_id.map(Some),
    };

    let input = UpdateDocumentRequirementInput {
        target_type,
        target_id,
        code: p.code,
        name_ar: p.name_ar,
        name_en: p.name_en,
        is_mandatory: p.is_mandatory,
        blocks_enrollment: p.blocks_enrollment,
        condition,
    };

    let req = state
        .document_service
        .update_document_requirement(&ctx, id, input)
        .await?;
    Ok(ApiResponse::ok(doc_requirement_to_dto(req)))
}

#[utoipa::path(delete, path = "/api/v1/document-requirements/{id}", params(("id" = Uuid, Path, description = "Requirement ID")),
    responses((status = 204, description = "Deleted")), security(("bearer" = [])), tag = "Document")]
async fn delete_document_requirement(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    state
        .document_service
        .delete_document_requirement(&ctx, id)
        .await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/v1/students/{id}/document-requirements", params(("id" = Uuid, Path, description = "Student ID"), EvaluateRequirementsQuery),
    responses((status = 200, body = Vec<StudentDocumentFulfillmentDto>)), security(("bearer" = [])), tag = "Document")]
async fn evaluate_student_requirements(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
    Query(q): Query<EvaluateRequirementsQuery>,
) -> Result<Json<ApiResponse<Vec<StudentDocumentFulfillmentDto>>>, ApiError> {
    let fulfillments = state
        .document_service
        .evaluate_student_requirements(
            ctx.tenant_id,
            student_id,
            q.grade_level_id,
            q.is_national.unwrap_or(true),
            q.is_transfer.unwrap_or(false),
            q.is_special_needs.unwrap_or(false),
        )
        .await?;

    let dtos = fulfillments
        .into_iter()
        .map(|f| StudentDocumentFulfillmentDto {
            requirement: doc_requirement_to_dto(f.requirement),
            is_fulfilled: f.is_fulfilled,
            is_applicable: f.is_applicable,
            document_id: f.uploaded_document.as_ref().map(|d| d.id),
            document_name: f.uploaded_document.as_ref().map(|d| d.name.clone()),
            file_path: f.uploaded_document.as_ref().map(|d| d.file_path.clone()),
            uploaded_at: f.uploaded_document.as_ref().map(|d| d.created_at),
        })
        .collect();

    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/documents",
            post(upload_document).get(list_documents),
        )
        .route(
            "/api/v1/documents/:id",
            get(get_document)
                .patch(update_document)
                .delete(delete_document),
        )
        .route(
            "/api/v1/document-requirements",
            post(create_document_requirement).get(list_document_requirements),
        )
        .route(
            "/api/v1/document-requirements/:id",
            get(get_document_requirement)
                .patch(update_document_requirement)
                .delete(delete_document_requirement),
        )
        .route(
            "/api/v1/students/:id/document-requirements",
            get(evaluate_student_requirements),
        )
}
