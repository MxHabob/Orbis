//! Report & Template routes — Template CRUD, Preview, Validation, Report Generation, Batch Print.

use axum::{
    extract::{Path, Query, State},
    http::header,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::usecases::report::{
    generate_report::{GenerateReportInput, GenerateReportUseCase},
    manage_template::ManageTemplateUseCase,
};
use orbis_domain::entities::report::*;

// ═══════════════════════════════════════════════════════════════
// DTOs
// ═══════════════════════════════════════════════════════════════

#[derive(Serialize, ToSchema)]
pub struct TemplateDto {
    pub id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub document_type: String,
    pub entity_type: String,
    pub status: String,
    pub version: i32,
    pub published_version: Option<i32>,
    pub page_size: String,
    pub page_width_mm: Option<f64>,
    pub page_height_mm: Option<f64>,
    pub orientation: String,
    pub margins_json: String,
    pub document_json: String,
    pub locale: String,
    pub master_template_id: Option<Uuid>,
    pub code: Option<String>,
    pub tags: Option<String>,
    pub is_system: bool,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_template_dto(t: ReportTemplate) -> TemplateDto {
    TemplateDto {
        id: t.id,
        name: t.name,
        name_ar: t.name_ar,
        description: t.description,
        description_ar: t.description_ar,
        document_type: t.document_type,
        entity_type: t.entity_type,
        status: t.status.to_string(),
        version: t.version,
        published_version: t.published_version,
        page_size: t.page_size,
        page_width_mm: t.page_width_mm,
        page_height_mm: t.page_height_mm,
        orientation: t.orientation,
        margins_json: t.margins_json,
        document_json: t.document_json,
        locale: t.locale,
        master_template_id: t.master_template_id,
        code: t.code,
        tags: t.tags,
        is_system: t.is_system,
        is_default: t.is_default,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub document_type: String,
    pub entity_type: String,
    pub page_size: Option<String>,
    pub page_width_mm: Option<f64>,
    pub page_height_mm: Option<f64>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    pub document_json: Option<String>,
    pub locale: Option<String>,
    pub master_template_id: Option<Uuid>,
    pub code: Option<String>,
    pub tags: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub page_size: Option<String>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    pub document_json: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct TemplateVersionDto {
    pub id: Uuid,
    pub template_id: Uuid,
    pub version: i32,
    pub change_summary: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct TemplateBindingDto {
    pub id: Uuid,
    pub report_definition_id: Uuid,
    pub template_id: Uuid,
    pub locale: String,
    pub format: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTemplateBindingRequest {
    pub template_id: Uuid,
    pub locale: String,
    pub format: String,
    pub is_default: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct PreviewTemplateRequest {
    pub document_json: String,
    pub context_data: serde_json::Value,
    pub locale: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ValidateTemplateRequest {
    pub document_json: String,
}

#[derive(Serialize, ToSchema)]
pub struct ValidationResultDto {
    pub valid: bool,
    pub errors: Vec<ValidationIssueDto>,
    pub warnings: Vec<ValidationIssueDto>,
}

#[derive(Serialize, ToSchema)]
pub struct ValidationIssueDto {
    pub node_id: Option<String>,
    pub path: String,
    pub message: String,
    pub severity: String,
}

#[derive(Serialize, ToSchema)]
pub struct ReportDefinitionDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub domain: String,
    pub category: String,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub supported_formats: Vec<String>,
    pub is_system: bool,
    pub is_active: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct GenerateReportRequest {
    pub report_code: String,
    pub format: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub locale: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct GenerateReportResponse {
    pub job_id: Uuid,
    pub status: String,
}

#[derive(Serialize, ToSchema)]
pub struct ReportJobStatusResponse {
    pub job_id: Uuid,
    pub status: String,
    pub output_path: Option<String>,
    pub output_size_bytes: Option<i64>,
    pub error_message: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct BatchPrintRequest {
    pub report_code: String,
    pub parameters: Option<serde_json::Value>,
    pub batch_config: serde_json::Value,
    pub locale: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct RevertVersionRequest {
    pub version: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct ListTemplatesQuery {
    pub document_type: Option<String>,
    pub entity_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct ListReportDefinitionsQuery {
    pub domain: Option<String>,
    pub category: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════

#[utoipa::path(
    get,
    path = "/api/v1/templates",
    tag = "Report",
    params(ListTemplatesQuery),
    responses((status = 200, description = "List templates", body = ApiResponse<Vec<TemplateDto>>)),
    security(("bearer" = []))
)]
pub async fn list_templates(
    State(state): State<AppState>,
    auth: AuthContext,
    Query(query): Query<ListTemplatesQuery>,
) -> Result<Json<ApiResponse<Vec<TemplateDto>>>, ApiError> {
    let ctx = auth.0;
    let items = state
        .report_template_repo
        .list_by_org(
            ctx.tenant_id,
            query.status.as_deref(),
            query.document_type.as_deref(),
            query.entity_type.as_deref(),
        )
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let dtos: Vec<TemplateDto> = items.into_iter().map(to_template_dto).collect();
    Ok(ApiResponse::ok_with_id(dtos, ctx.request_id))
}

#[utoipa::path(
    post,
    path = "/api/v1/templates",
    tag = "Report",
    request_body = CreateTemplateRequest,
    responses((status = 201, description = "Create template", body = ApiResponse<TemplateDto>)),
    security(("bearer" = []))
)]
pub async fn create_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };

    let template = ReportTemplate {
        id: Uuid::new_v4(),
        org_id: ctx.tenant_id,
        name: req.name,
        name_ar: req.name_ar,
        description: req.description,
        description_ar: req.description_ar,
        document_type: req.document_type,
        entity_type: req.entity_type,
        status: TemplateStatus::Draft,
        version: 1,
        published_version: None,
        page_size: req.page_size.unwrap_or_else(|| "A4".into()),
        page_width_mm: req.page_width_mm,
        page_height_mm: req.page_height_mm,
        orientation: req.orientation.unwrap_or_else(|| "portrait".into()),
        margins_json: req.margins_json.unwrap_or_else(|| "{}".into()),
        document_json: req.document_json.unwrap_or_else(|| "{}".into()),
        locale: req.locale.unwrap_or_else(|| "ar".into()),
        master_template_id: req.master_template_id,
        code: req.code,
        tags: req.tags,
        is_system: false,
        is_default: false,
        created_by: ctx.actor.id().unwrap_or_default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
        published_at: None,
        published_by: None,
    };

    let created = usecase
        .create(&ctx, template)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(to_template_dto(created), ctx.request_id))
}

#[utoipa::path(
    get,
    path = "/api/v1/templates/{id}",
    tag = "Report",
    responses((status = 200, description = "Get template", body = ApiResponse<TemplateDto>)),
    security(("bearer" = []))
)]
pub async fn get_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let t = state
        .report_template_repo
        .find_by_id(id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Template not found"))?;

    Ok(ApiResponse::ok_with_id(to_template_dto(t), ctx.request_id))
}

#[utoipa::path(
    put,
    path = "/api/v1/templates/{id}",
    tag = "Report",
    request_body = UpdateTemplateRequest,
    responses((status = 200, description = "Update template", body = ApiResponse<TemplateDto>)),
    security(("bearer" = []))
)]
pub async fn update_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTemplateRequest>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };

    let template = usecase
        .update(
            &ctx,
            id,
            req.name,
            req.name_ar,
            req.description,
            req.description_ar,
            req.document_json,
            req.page_size,
            req.orientation,
            req.margins_json,
        )
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(to_template_dto(template), ctx.request_id))
}

#[utoipa::path(
    delete,
    path = "/api/v1/templates/{id}",
    tag = "Report",
    responses((status = 204, description = "Delete template")),
    security(("bearer" = []))
)]
pub async fn delete_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let ctx = auth.0;
    let t = state
        .report_template_repo
        .find_by_id(id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Template not found"))?;

    if t.org_id != ctx.tenant_id {
        return Err(ApiError::forbidden("Template belongs to another organization"));
    }

    state
        .report_template_repo
        .soft_delete(id)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/templates/{id}/publish",
    tag = "Report",
    responses((status = 200, description = "Publish template", body = ApiResponse<TemplateDto>)),
    security(("bearer" = []))
)]
pub async fn publish_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };

    let template = usecase
        .publish(&ctx, id)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(to_template_dto(template), ctx.request_id))
}

#[utoipa::path(
    post,
    path = "/api/v1/templates/{id}/archive",
    tag = "Report",
    responses((status = 204, description = "Archive template")),
    security(("bearer" = []))
)]
pub async fn archive_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let ctx = auth.0;
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };

    usecase
        .archive(&ctx, id)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v1/templates/{id}/versions",
    tag = "Report",
    responses((status = 200, description = "Get template versions", body = ApiResponse<Vec<TemplateVersionDto>>)),
    security(("bearer" = []))
)]
pub async fn get_template_versions(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<TemplateVersionDto>>>, ApiError> {
    let ctx = auth.0;
    let versions = state
        .report_template_repo
        .find_versions(id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let dtos = versions
        .into_iter()
        .map(|v| TemplateVersionDto {
            id: v.id,
            template_id: v.template_id,
            version: v.version,
            change_summary: v.change_summary,
            created_by: v.created_by.to_string(),
            created_at: v.created_at,
        })
        .collect();

    Ok(ApiResponse::ok_with_id(dtos, ctx.request_id))
}

#[utoipa::path(
    post,
    path = "/api/v1/templates/{id}/revert",
    tag = "Report",
    request_body = RevertVersionRequest,
    responses((status = 200, description = "Revert template version", body = ApiResponse<TemplateDto>)),
    security(("bearer" = []))
)]
pub async fn revert_template_version(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
    Json(req): Json<RevertVersionRequest>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };

    let template = usecase
        .revert_to_version(&ctx, id, req.version)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(to_template_dto(template), ctx.request_id))
}

#[utoipa::path(
    post,
    path = "/api/v1/templates/preview",
    tag = "Report",
    request_body = PreviewTemplateRequest,
    responses((status = 200, description = "Preview template as byte array", body = Vec<u8>)),
    security(("bearer" = []))
)]
pub async fn preview_template(
    State(_state): State<AppState>,
    _auth: AuthContext,
    Json(req): Json<PreviewTemplateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    use docforge_engine::layout::engine::LayoutEngine;
    use docforge_engine::model::document::TemplateDocument;
    use docforge_engine::render::font_cache::FontCache;
    use docforge_engine::render::pdf_renderer::{render_to_pdf, PdfRenderConfig};

    let doc: TemplateDocument = serde_json::from_str(&req.document_json)
        .map_err(|e| ApiError::bad_request(format!("Invalid template JSON: {}", e)))?;

    let fonts = FontCache::new("Noto Sans Arabic");
    let layout_engine = LayoutEngine::new(&fonts);
    let layout = layout_engine
        .layout(&doc, &req.context_data)
        .map_err(|e| ApiError::bad_request(format!("Layout error: {}", e)))?;

    let config = PdfRenderConfig {
        title: Some(doc.meta.name.clone()),
        ..Default::default()
    };

    let pdf_output = render_to_pdf(&layout, &fonts, &config)
        .map_err(|e| ApiError::internal(format!("PDF render error: {}", e)))?;

    Ok((
        [(header::CONTENT_TYPE, "application/pdf")],
        pdf_output.bytes,
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/templates/validate",
    tag = "Report",
    request_body = ValidateTemplateRequest,
    responses((status = 200, description = "Validate template", body = ApiResponse<ValidationResultDto>)),
    security(("bearer" = []))
)]
pub async fn validate_template(
    State(_state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<ValidateTemplateRequest>,
) -> Result<Json<ApiResponse<ValidationResultDto>>, ApiError> {
    let ctx = auth.0;
    let parse_result: Result<docforge_engine::model::document::TemplateDocument, _> =
        serde_json::from_str(&req.document_json);

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    match parse_result {
        Ok(doc) => {
            if doc.meta.name.is_empty() {
                warnings.push(ValidationIssueDto {
                    node_id: None,
                    path: "meta.name".into(),
                    message: "Template name is empty".into(),
                    severity: "warning".into(),
                });
            }
            if doc.meta.page_setup.margins.top < 0.0
                || doc.meta.page_setup.margins.bottom < 0.0
                || doc.meta.page_setup.margins.left < 0.0
                || doc.meta.page_setup.margins.right < 0.0
            {
                errors.push(ValidationIssueDto {
                    node_id: None,
                    path: "meta.page_setup.margins".into(),
                    message: "Page margins cannot be negative".into(),
                    severity: "error".into(),
                });
            }
        }
        Err(e) => {
            errors.push(ValidationIssueDto {
                node_id: None,
                path: "".into(),
                message: format!("Invalid template JSON: {}", e),
                severity: "error".into(),
            });
        }
    }

    Ok(ApiResponse::ok_with_id(
        ValidationResultDto {
            valid: errors.is_empty(),
            errors,
            warnings,
        },
        ctx.request_id,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/report-definitions",
    tag = "Report",
    params(ListReportDefinitionsQuery),
    responses((status = 200, description = "List report definitions", body = ApiResponse<Vec<ReportDefinitionDto>>)),
    security(("bearer" = []))
)]
pub async fn list_report_definitions(
    State(state): State<AppState>,
    auth: AuthContext,
    Query(query): Query<ListReportDefinitionsQuery>,
) -> Result<Json<ApiResponse<Vec<ReportDefinitionDto>>>, ApiError> {
    let ctx = auth.0;
    let items = state
        .report_definition_repo
        .list_by_org(
            ctx.tenant_id,
            query.domain.as_deref(),
            query.category.as_deref(),
        )
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let dtos = items
        .into_iter()
        .map(|d| ReportDefinitionDto {
            id: d.id,
            code: d.code,
            name: d.name,
            name_ar: d.name_ar,
            domain: d.domain.to_string(),
            category: d.category.to_string(),
            description: d.description,
            description_ar: d.description_ar,
            supported_formats: d.supported_formats.iter().map(|f| f.to_string()).collect(),
            is_system: d.is_system,
            is_active: d.is_active,
        })
        .collect();

    Ok(ApiResponse::ok_with_id(dtos, ctx.request_id))
}

#[utoipa::path(
    get,
    path = "/api/v1/report-definitions/{code}",
    tag = "Report",
    responses((status = 200, description = "Get report definition", body = ApiResponse<ReportDefinitionDto>)),
    security(("bearer" = []))
)]
pub async fn get_report_definition(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(code): Path<String>,
) -> Result<Json<ApiResponse<ReportDefinitionDto>>, ApiError> {
    let ctx = auth.0;
    let d = state
        .report_definition_repo
        .find_by_code(ctx.tenant_id, &code)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Report definition not found"))?;

    Ok(ApiResponse::ok_with_id(
        ReportDefinitionDto {
            id: d.id,
            code: d.code,
            name: d.name,
            name_ar: d.name_ar,
            domain: d.domain.to_string(),
            category: d.category.to_string(),
            description: d.description,
            description_ar: d.description_ar,
            supported_formats: d.supported_formats.iter().map(|f| f.to_string()).collect(),
            is_system: d.is_system,
            is_active: d.is_active,
        },
        ctx.request_id,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/report-definitions/{definition_id}/bindings",
    tag = "Report",
    responses((status = 200, description = "List template bindings", body = ApiResponse<Vec<TemplateBindingDto>>)),
    security(("bearer" = []))
)]
pub async fn list_template_bindings(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(definition_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<TemplateBindingDto>>>, ApiError> {
    let ctx = auth.0;
    let items = state
        .report_template_repo
        .find_bindings_by_definition(definition_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let dtos = items
        .into_iter()
        .map(|b| TemplateBindingDto {
            id: b.id,
            report_definition_id: b.report_definition_id,
            template_id: b.template_id,
            locale: b.locale,
            format: b.format.to_string(),
            is_default: b.is_default,
            created_at: b.created_at,
        })
        .collect();

    Ok(ApiResponse::ok_with_id(dtos, ctx.request_id))
}

#[utoipa::path(
    post,
    path = "/api/v1/report-definitions/{definition_id}/bindings",
    tag = "Report",
    request_body = CreateTemplateBindingRequest,
    responses((status = 201, description = "Create template binding", body = ApiResponse<TemplateBindingDto>)),
    security(("bearer" = []))
)]
pub async fn create_template_binding(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(definition_id): Path<Uuid>,
    Json(req): Json<CreateTemplateBindingRequest>,
) -> Result<Json<ApiResponse<TemplateBindingDto>>, ApiError> {
    let ctx = auth.0;
    let format = OutputFormat::from_str(&req.format);
    let binding = ReportTemplateBinding {
        id: Uuid::new_v4(),
        report_definition_id: definition_id,
        template_id: req.template_id,
        locale: req.locale,
        format,
        is_default: req.is_default.unwrap_or(false),
        created_at: Utc::now(),
    };

    state
        .report_template_repo
        .save_binding(&binding)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(
        TemplateBindingDto {
            id: binding.id,
            report_definition_id: binding.report_definition_id,
            template_id: binding.template_id,
            locale: binding.locale,
            format: binding.format.to_string(),
            is_default: binding.is_default,
            created_at: binding.created_at,
        },
        ctx.request_id,
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/reports/generate",
    tag = "Report",
    request_body = GenerateReportRequest,
    responses((status = 200, description = "Generate report", body = ApiResponse<GenerateReportResponse>)),
    security(("bearer" = []))
)]
pub async fn generate_report(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<GenerateReportRequest>,
) -> Result<Json<ApiResponse<GenerateReportResponse>>, ApiError> {
    let ctx = auth.0;
    let usecase = GenerateReportUseCase {
        definition_repo: state.report_definition_repo.clone(),
        template_repo: state.report_template_repo.clone(),
        job_repo: state.report_job_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
        provider_registry: state.report_provider_registry.clone(),
        renderer_registry: state.report_renderer_registry.clone(),
        org_context_resolver: state.report_org_context_resolver.clone(),
    };

    let params: HashMap<String, ParamValue> = req
        .parameters
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let output_format = req
        .format
        .map(|f| OutputFormat::from_str(&f))
        .unwrap_or(OutputFormat::Pdf);

    let locale = req
        .locale
        .map(|l| ReportLocale::from_str(&l))
        .unwrap_or(ReportLocale::Ar);

    let input = GenerateReportInput {
        report_code: req.report_code,
        parameters: params,
        output_format,
        locale,
    };

    let (mut job, export) = usecase
        .execute(&ctx, input)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    let temp_dir = std::env::temp_dir().join("orbis_reports");
    tokio::fs::create_dir_all(&temp_dir)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let file_path = temp_dir.join(format!("{}.{}", job.id, export.filename));
    tokio::fs::write(&file_path, &export.data)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    job.output_path = Some(file_path.to_string_lossy().to_string());
    job.output_size_bytes = Some(export.data.len() as i64);

    state
        .report_job_repo
        .save(&job)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(
        GenerateReportResponse {
            job_id: job.id,
            status: job.status.to_string(),
        },
        ctx.request_id,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/reports/jobs/{id}/status",
    tag = "Report",
    responses((status = 200, description = "Get report job status", body = ApiResponse<ReportJobStatusResponse>)),
    security(("bearer" = []))
)]
pub async fn get_report_job_status(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ReportJobStatusResponse>>, ApiError> {
    let ctx = auth.0;
    let job = state
        .report_job_repo
        .find_by_id(id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Report job not found"))?;

    Ok(ApiResponse::ok_with_id(
        ReportJobStatusResponse {
            job_id: job.id,
            status: job.status.to_string(),
            output_path: job.output_path,
            output_size_bytes: job.output_size_bytes,
            error_message: job.error_message,
        },
        ctx.request_id,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/reports/jobs/{id}/download",
    tag = "Report",
    responses((status = 200, description = "Download report file", body = Vec<u8>)),
    security(("bearer" = []))
)]
pub async fn download_report_file(
    State(state): State<AppState>,
    _auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let job = state
        .report_job_repo
        .find_by_id(id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Report job not found"))?;

    if job.status.to_string() != "completed" {
        return Err(ApiError::bad_request("Report generation is not completed yet"));
    }

    let file_path = job
        .output_path
        .ok_or_else(|| ApiError::not_found("Report file path missing"))?;

    let bytes = tokio::fs::read(&file_path)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to read report file: {}", e)))?;

    Ok((
        [(header::CONTENT_TYPE, "application/octet-stream")],
        bytes,
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/reports/batch-print",
    tag = "Report",
    request_body = BatchPrintRequest,
    responses((status = 200, description = "Batch print HTML", body = ApiResponse<String>)),
    security(("bearer" = []))
)]
pub async fn batch_print(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<BatchPrintRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let ctx = auth.0;
    let config: BatchPrintConfig = serde_json::from_value(req.batch_config)
        .map_err(|e| ApiError::bad_request(format!("Invalid batch config: {}", e)))?;

    let params: HashMap<String, ParamValue> = req
        .parameters
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let definition = state
        .report_definition_repo
        .find_by_code(ctx.tenant_id, &req.report_code)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Report definition not found"))?;

    let org_context = state
        .report_org_context_resolver
        .resolve(ctx.tenant_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let locale = req
        .locale
        .map(|l| ReportLocale::from_str(&l))
        .unwrap_or(ReportLocale::Ar);

    let provider = state
        .report_provider_registry
        .get_provider(&definition.data_provider_key)
        .ok_or_else(|| ApiError::bad_request(format!("No data provider for key: {}", definition.data_provider_key)))?;

    let dataset = provider
        .fetch_data(ctx.tenant_id, &org_context, &params, &locale)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let layout = config.compute_layout();
    let is_rtl = matches!(locale, ReportLocale::Ar | ReportLocale::ArEn);
    let dir = if is_rtl { "rtl" } else { "ltr" };
    let primary_color = org_context.primary_color.as_deref().unwrap_or("#6366f1");

    let mut html = format!(
        r#"<!DOCTYPE html>
<html dir="{dir}">
<head>
<meta charset="UTF-8">
<style>
@page {{ size: {w}mm {h}mm; margin: 0; }}
body {{ margin: 0; padding: 0; font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; }}
.sheet {{ width: {w}mm; height: {h}mm; position: relative; page-break-after: always; overflow: hidden; }}
.sheet:last-child {{ page-break-after: auto; }}
.item {{ position: absolute; overflow: hidden; border: 0.3px solid #e0e0e0; box-sizing: border-box;
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    padding: 3mm; font-size: 10px; text-align: center; direction: {dir}; }}
.item .org-name {{ font-weight: 700; color: {color}; font-size: 11px; margin-bottom: 2mm; }}
.item .field-label {{ color: #888; font-size: 8px; }}
.item .field-value {{ font-weight: 600; font-size: 10px; margin-bottom: 1mm; }}
.cut-line-h {{ position: absolute; border-top: 0.5px dashed #ccc; width: 100%; left: 0; }}
.cut-line-v {{ position: absolute; border-left: 0.5px dashed #ccc; height: 100%; top: 0; }}
@media print {{ body {{ -webkit-print-color-adjust: exact; print-color-adjust: exact; }} }}
</style>
</head>
<body>"#,
        dir = dir,
        w = layout.sheet_width_mm,
        h = layout.sheet_height_mm,
        color = primary_color,
    );

    let pages = config.paginate(&dataset.rows);
    let visible_cols: Vec<&ColumnDef> = dataset.columns.iter().filter(|c| c.visible).collect();
    let org_name = if is_rtl { &org_context.name_ar } else { &org_context.name };

    for page_items in &pages {
        html.push_str(r#"<div class="sheet">"#);

        for (idx, row) in page_items.iter().enumerate() {
            if let Some(slot) = layout.slots.get(idx) {
                html.push_str(&format!(
                    r#"<div class="item" style="left:{x}mm; top:{y}mm; width:{w}mm; height:{h}mm;">"#,
                    x = slot.x_mm,
                    y = slot.y_mm,
                    w = slot.width_mm,
                    h = slot.height_mm,
                ));

                html.push_str(&format!(r#"<div class="org-name">{}</div>"#, org_name));

                for col in &visible_cols {
                    let label = if is_rtl { &col.header_ar } else { &col.header };
                    let val = row
                        .get(&col.key)
                        .map(|v| match v {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Number(n) => n.to_string(),
                            _ => v.to_string(),
                        })
                        .unwrap_or_default();

                    if !val.is_empty() {
                        html.push_str(&format!(
                            r#"<div class="field-label">{}</div><div class="field-value">{}</div>"#,
                            label, val
                        ));
                    }
                }

                html.push_str("</div>");
            }
        }

        if config.show_cut_lines {
            for r in 1..layout.rows {
                let y = layout.slots.first().map(|s| s.y_mm).unwrap_or(0.0)
                    + r as f64 * (config.item_height_mm + config.gap_mm)
                    - config.gap_mm / 2.0;
                html.push_str(&format!(r#"<div class="cut-line-h" style="top:{y}mm;"></div>"#, y = y));
            }
            for c in 1..layout.columns {
                let x = layout.slots.first().map(|s| s.x_mm).unwrap_or(0.0)
                    + c as f64 * (config.item_width_mm + config.gap_mm)
                    - config.gap_mm / 2.0;
                html.push_str(&format!(r#"<div class="cut-line-v" style="left:{x}mm;"></div>"#, x = x));
            }
        }

        html.push_str("</div>");
    }

    html.push_str("</body></html>");
    Ok(ApiResponse::ok_with_id(html, ctx.request_id))
}

// ═══════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/templates", get(list_templates).post(create_template))
        .route("/api/v1/templates/:id", get(get_template).put(update_template).delete(delete_template))
        .route("/api/v1/templates/:id/publish", post(publish_template))
        .route("/api/v1/templates/:id/archive", post(archive_template))
        .route("/api/v1/templates/:id/versions", get(get_template_versions))
        .route("/api/v1/templates/:id/revert", post(revert_template_version))
        .route("/api/v1/templates/preview", post(preview_template))
        .route("/api/v1/templates/validate", post(validate_template))
        .route("/api/v1/report-definitions", get(list_report_definitions))
        .route("/api/v1/report-definitions/:code", get(get_report_definition))
        .route("/api/v1/report-definitions/:definition_id/bindings", get(list_template_bindings).post(create_template_binding))
        .route("/api/v1/reports/generate", post(generate_report))
        .route("/api/v1/reports/jobs/:id/status", get(get_report_job_status))
        .route("/api/v1/reports/jobs/:id/download", get(download_report_file))
        .route("/api/v1/reports/batch-print", post(batch_print))
}
