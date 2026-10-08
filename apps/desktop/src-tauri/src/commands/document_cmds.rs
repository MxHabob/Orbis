//! Tauri IPC commands — Document & Requirements Management.

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_app::ports::storage::StorageAdapterResolver;
use orbis_app::services::{
    CreateDocumentInput, UpdateDocumentInput, UpdateDocumentRequirementInput,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
pub fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Serialize)]
pub struct DocumentDto {
    pub id: String,
    pub org_id: String,
    pub org_unit_id: Option<String>,
    pub folder_id: Option<String>,
    pub uploaded_by: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub tags: Option<String>,
    pub visibility: String,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub checksum_sha256: Option<String>,
    pub storage_provider: String,
    pub file_path: String,
    pub storage_key: Option<String>,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub is_archived: bool,
    pub archived_at: Option<String>,
    pub archived_by: Option<String>,
    pub archive_reason: Option<String>,
    pub is_legal_hold: bool,
    pub deleted_at: Option<String>,
    pub deleted_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct UploadDocumentInput {
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub org_unit_id: Option<String>,
    pub folder_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
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

#[derive(Deserialize)]
pub struct UpdateDocumentTauriInput {
    pub name: Option<String>,
    pub folder_id: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
}

pub fn doc_dto(d: orbis_domain::entities::document::Document) -> DocumentDto {
    DocumentDto {
        id: d.id.to_string(),
        org_id: d.org_id.to_string(),
        org_unit_id: d.org_unit_id.map(|u| u.to_string()),
        folder_id: d.folder_id.map(|f| f.to_string()),
        uploaded_by: d.uploaded_by.map(|u| u.to_string()),
        entity_type: d.entity_type,
        entity_id: d.entity_id.map(|e| e.to_string()),
        name: d.name,
        description: d.description,
        category: d.category,
        asset_type: d.asset_type,
        academic_year_id: d.academic_year_id.map(|y| y.to_string()),
        term_id: d.term_id.map(|t| t.to_string()),
        tags: d.tags,
        visibility: d.visibility,
        file_size_bytes: d.file_size_bytes,
        mime_type: d.mime_type,
        extension: d.extension,
        checksum_sha256: d.checksum_sha256,
        storage_provider: d.storage_provider,
        file_path: d.file_path,
        storage_key: d.storage_key,
        url: d.url,
        thumbnail_path: d.thumbnail_path,
        preview_path: d.preview_path,
        is_archived: d.is_archived,
        archived_at: d.archived_at.map(|t| t.to_rfc3339()),
        archived_by: d.archived_by.map(|u| u.to_string()),
        archive_reason: d.archive_reason,
        is_legal_hold: d.is_legal_hold,
        deleted_at: d.deleted_at.map(|t| t.to_rfc3339()),
        deleted_by: d.deleted_by.map(|u| u.to_string()),
        created_at: d.created_at.to_rfc3339(),
        updated_at: d.updated_at.to_rfc3339(),
    }
}

#[tauri::command]
pub async fn document_upload(
    request: UploadDocumentInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let storage_provider = state
        .storage_resolver
        .resolve(ctx.tenant_id)
        .await
        .map_err(|error| error.to_string())?
        .provider_id()
        .to_string();
    let entity_id = request.entity_id.and_then(|e| Uuid::parse_str(&e).ok());
    let org_unit_id = request.org_unit_id.and_then(|e| Uuid::parse_str(&e).ok());
    let folder_id = request.folder_id.and_then(|f| Uuid::parse_str(&f).ok());
    let academic_year_id = request
        .academic_year_id
        .and_then(|y| Uuid::parse_str(&y).ok());
    let term_id = request.term_id.and_then(|t| Uuid::parse_str(&t).ok());
    let uploaded_by = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
        _ => None,
    };

    let input = CreateDocumentInput {
        org_unit_id,
        folder_id,
        uploaded_by,
        entity_type: request.entity_type,
        entity_id,
        name: request.name,
        description: request.description,
        category: request.category.unwrap_or_else(|| "general".to_string()),
        asset_type: request.asset_type.unwrap_or_else(|| "file".to_string()),
        academic_year_id,
        term_id,
        tags: request.tags,
        visibility: request.visibility.unwrap_or_else(|| "internal".to_string()),
        file_size_bytes: request.file_size_bytes,
        mime_type: request.mime_type,
        extension: request.extension,
        checksum_sha256: request.checksum_sha256,
        storage_provider,
        file_path: request.file_path.clone(),
        storage_key: request.storage_key.or(Some(request.file_path)),
        url: request.url,
        thumbnail_path: request.thumbnail_path,
        preview_path: request.preview_path,
    };

    let doc = state
        .document_service
        .create_document(&ctx, ctx.tenant_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn document_update(
    id: String,
    request: UpdateDocumentTauriInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let folder_id = request.folder_id.and_then(|f| Uuid::parse_str(&f).ok());
    let academic_year_id = request
        .academic_year_id
        .and_then(|y| Uuid::parse_str(&y).ok());
    let term_id = request.term_id.and_then(|t| Uuid::parse_str(&t).ok());

    let input = UpdateDocumentInput {
        name: request.name,
        folder_id,
        description: request.description,
        category: request.category,
        asset_type: request.asset_type,
        academic_year_id,
        term_id,
        tags: request.tags,
        visibility: request.visibility,
    };

    let doc = state
        .document_service
        .update_document(&ctx, uid, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn document_list(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state
        .document_service
        .list_documents_by_org(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(doc_dto).collect()))
}

#[tauri::command]
pub async fn document_list_by_entity(
    entity_type: String,
    entity_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let eid = Uuid::parse_str(&entity_id).map_err(|_| "Invalid Entity ID")?;
    let items = state
        .document_service
        .list_documents_by_entity(&entity_type, eid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(doc_dto).collect()))
}

#[tauri::command]
pub async fn document_get(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let doc = state
        .document_service
        .get_document(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Document not found".to_string())?;
    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn document_delete(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state
        .document_service
        .delete_document(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[tauri::command]
pub async fn document_archive(
    id: String,
    reason: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let doc = state
        .file_storage_service
        .archive_document(&ctx, uid, reason)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn document_restore(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let doc = state
        .file_storage_service
        .restore_document(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn document_trash(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state
        .file_storage_service
        .trash_document(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[tauri::command]
pub async fn document_purge(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state
        .file_storage_service
        .purge_document(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[tauri::command]
pub async fn document_list_archived(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let ctx = make_ctx(&state);
    let docs = state
        .document_repo
        .find_archived(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(docs.into_iter().map(doc_dto).collect()))
}

#[tauri::command]
pub async fn document_list_trash(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let ctx = make_ctx(&state);
    let docs = state
        .document_repo
        .find_trash(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(docs.into_iter().map(doc_dto).collect()))
}

// ── Document Requirements ──────────────────────────────────────

#[derive(Serialize)]
pub struct DocumentRequirementDto {
    pub id: String,
    pub org_id: String,
    pub target_type: String,
    pub target_id: Option<String>,
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    pub is_mandatory: bool,
    pub blocks_enrollment: bool,
    pub condition: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn doc_req_dto(
    r: orbis_domain::entities::document_requirement::DocumentRequirement,
) -> DocumentRequirementDto {
    DocumentRequirementDto {
        id: r.id.to_string(),
        org_id: r.org_id.to_string(),
        target_type: r.target_type.to_string(),
        target_id: r.target_id.map(|t| t.to_string()),
        code: r.code,
        name_ar: r.name_ar,
        name_en: r.name_en,
        is_mandatory: r.is_mandatory,
        blocks_enrollment: r.blocks_enrollment,
        condition: r.condition.to_string(),
        created_at: r.created_at.to_rfc3339(),
        updated_at: r.updated_at.to_rfc3339(),
    }
}

#[derive(Deserialize)]
pub struct CreateRequirementInput {
    pub target_type: String,
    pub target_id: Option<String>,
    pub code: String,
    pub name_ar: String,
    pub name_en: Option<String>,
    pub is_mandatory: bool,
    pub blocks_enrollment: bool,
    pub condition: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRequirementInput {
    pub target_type: Option<String>,
    pub target_id: Option<Option<String>>,
    pub code: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub is_mandatory: Option<bool>,
    pub blocks_enrollment: Option<bool>,
    pub condition: Option<String>,
}

#[tauri::command]
pub async fn document_create_requirement(
    request: CreateRequirementInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentRequirementDto>, String> {
    let ctx = make_ctx(&state);
    let target_type = parse_target_type(&request.target_type);
    let target_id = request.target_id.and_then(|t| Uuid::parse_str(&t).ok());
    let condition = parse_condition(request.condition.as_deref());

    let input = orbis_app::services::CreateDocumentRequirementInput {
        target_type,
        target_id,
        code: request.code,
        name_ar: request.name_ar,
        name_en: request.name_en.unwrap_or_default(),
        is_mandatory: request.is_mandatory,
        blocks_enrollment: request.blocks_enrollment,
        condition,
    };

    let req = state
        .document_service
        .create_document_requirement(&ctx, ctx.tenant_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_req_dto(req)))
}

#[tauri::command]
pub async fn document_get_requirement(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentRequirementDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let req = state
        .document_service
        .get_document_requirement(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Requirement not found".to_string())?;
    Ok(ok(doc_req_dto(req)))
}

#[tauri::command]
pub async fn document_update_requirement(
    id: String,
    request: UpdateRequirementInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentRequirementDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let target_type = request.target_type.map(|t| parse_target_type(&t));
    let target_id = match request.target_id {
        Some(Some(t)) => Some(Uuid::parse_str(&t).ok()),
        Some(None) => Some(None),
        None => None,
    };
    let condition = request.condition.map(|c| parse_condition(Some(&c)));

    let input = UpdateDocumentRequirementInput {
        target_type,
        target_id,
        code: request.code,
        name_ar: request.name_ar,
        name_en: request.name_en,
        is_mandatory: request.is_mandatory,
        blocks_enrollment: request.blocks_enrollment,
        condition,
    };

    let req = state
        .document_service
        .update_document_requirement(&ctx, uid, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_req_dto(req)))
}

#[tauri::command]
pub async fn document_list_requirements(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DocumentRequirementDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state
        .document_service
        .list_document_requirements_by_org(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(doc_req_dto).collect()))
}

#[tauri::command]
pub async fn document_delete_requirement(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state
        .document_service
        .delete_document_requirement(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[derive(Serialize)]
pub struct StudentDocumentFulfillmentDto {
    pub requirement: DocumentRequirementDto,
    pub is_fulfilled: bool,
    pub uploaded_document: Option<DocumentDto>,
    pub is_applicable: bool,
}

#[tauri::command]
pub async fn document_evaluate_student_requirements(
    student_id: String,
    grade_level_id: Option<String>,
    is_national: Option<bool>,
    is_special_needs: Option<bool>,
    is_transfer: Option<bool>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<StudentDocumentFulfillmentDto>>, String> {
    let ctx = make_ctx(&state);
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid Student ID")?;
    let gid = grade_level_id.and_then(|g| Uuid::parse_str(&g).ok());

    let results = state
        .document_service
        .evaluate_student_requirements(
            ctx.tenant_id,
            sid,
            gid,
            is_national.unwrap_or(true),
            is_transfer.unwrap_or(false),
            is_special_needs.unwrap_or(false),
        )
        .await
        .map_err(|e| e.to_string())?;

    let dtos = results
        .into_iter()
        .map(|f| StudentDocumentFulfillmentDto {
            requirement: doc_req_dto(f.requirement),
            is_fulfilled: f.is_fulfilled,
            uploaded_document: f.uploaded_document.map(doc_dto),
            is_applicable: f.is_applicable,
        })
        .collect();

    Ok(ok(dtos))
}

fn parse_target_type(
    s: &str,
) -> orbis_domain::entities::document_requirement::RequirementTargetType {
    match s.to_lowercase().as_str() {
        "stage" => orbis_domain::entities::document_requirement::RequirementTargetType::Stage,
        "grade_level" | "gradelevel" => {
            orbis_domain::entities::document_requirement::RequirementTargetType::GradeLevel
        }
        "program" => orbis_domain::entities::document_requirement::RequirementTargetType::Program,
        _ => orbis_domain::entities::document_requirement::RequirementTargetType::Global,
    }
}

fn parse_condition(
    s: Option<&str>,
) -> orbis_domain::entities::document_requirement::DocumentCondition {
    match s {
        Some("non_national_only") => {
            orbis_domain::entities::document_requirement::DocumentCondition::NonNationalOnly
        }
        Some("transfer_students_only") => {
            orbis_domain::entities::document_requirement::DocumentCondition::TransferStudentsOnly
        }
        Some("special_needs_only") => {
            orbis_domain::entities::document_requirement::DocumentCondition::SpecialNeedsOnly
        }
        _ => orbis_domain::entities::document_requirement::DocumentCondition::AllStudents,
    }
}
