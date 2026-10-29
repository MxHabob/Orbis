//! Report & Template routes — Template CRUD, Preview, Validation, Report Generation, Batch Print.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::header,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_domain::entities::ai::{AiCustomReportRequest, AiCustomReportResponse};
use orbis_domain::entities::report::*;
use orbis_domain::iam::permission::Permission;

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
    pub is_default: Option<bool>,
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
    pub output_size_bytes: Option<i64>,
    pub error_message: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct CancelReportResponse {
    pub job_id: Uuid,
    pub cancel_requested: bool,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BatchPrintRequest {
    pub template_id: Option<Uuid>,
    pub document_json: Option<String>,
    pub contexts: Option<Vec<serde_json::Value>>,
    pub title: Option<String>,
    pub report_code: Option<String>,
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
    responses((status = 200, body = Vec<TemplateDto>)),
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
    responses((status = 201, body = TemplateDto)),
    security(("bearer" = []))
)]
pub async fn create_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;

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

    state
        .report_service
        .save_template(&template)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(
        to_template_dto(template),
        ctx.request_id,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/templates/{id}",
    tag = "Report",
    responses((status = 200, body = TemplateDto)),
    security(("bearer" = []))
)]
pub async fn get_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let t = state
        .report_service
        .get_template(id)
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
    responses((status = 200, body = TemplateDto)),
    security(("bearer" = []))
)]
pub async fn update_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTemplateRequest>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let mut template = state
        .report_template_repo
        .find_by_id(id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Template not found"))?;

    if template.org_id != ctx.tenant_id {
        return Err(ApiError::forbidden(
            "Template belongs to another organization",
        ));
    }

    if let Some(name) = req.name {
        template.name = name;
    }
    if let Some(name_ar) = req.name_ar {
        template.name_ar = Some(name_ar);
    }
    if let Some(desc) = req.description {
        template.description = Some(desc);
    }
    if let Some(desc_ar) = req.description_ar {
        template.description_ar = Some(desc_ar);
    }
    if let Some(doc) = req.document_json {
        template.document_json = doc;
    }
    if let Some(ps) = req.page_size {
        template.page_size = ps;
    }
    if let Some(ori) = req.orientation {
        template.orientation = ori;
    }
    if let Some(mar) = req.margins_json {
        template.margins_json = mar;
    }
    template.updated_at = Utc::now();

    state
        .report_service
        .save_template(&template)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    if req.is_default == Some(true) {
        template = state
            .report_service
            .set_default_template(&ctx, id)
            .await
            .map_err(|e| ApiError::bad_request(e.to_string()))?;
    }

    Ok(ApiResponse::ok_with_id(
        to_template_dto(template),
        ctx.request_id,
    ))
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
    state
        .report_service
        .delete_template(&ctx, id)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/templates/{id}/publish",
    tag = "Report",
    responses((status = 200, body = TemplateDto)),
    security(("bearer" = []))
)]
pub async fn publish_template(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let template = state
        .report_service
        .publish_template(&ctx, id)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(
        to_template_dto(template),
        ctx.request_id,
    ))
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
    state
        .report_service
        .archive_template(&ctx, id)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v1/templates/{id}/versions",
    tag = "Report",
    responses((status = 200, body = Vec<TemplateVersionDto>)),
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
    responses((status = 200, body = TemplateDto)),
    security(("bearer" = []))
)]
pub async fn revert_template_version(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
    Json(req): Json<RevertVersionRequest>,
) -> Result<Json<ApiResponse<TemplateDto>>, ApiError> {
    let ctx = auth.0;
    let template = state
        .report_service
        .revert_template_version(&ctx, id, req.version)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(
        to_template_dto(template),
        ctx.request_id,
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/templates/preview",
    tag = "Report",
    request_body = PreviewTemplateRequest,
    responses((status = 200, body = Vec<u8>)),
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
    use docforge_engine::render::pdf_renderer::{PdfRenderConfig, render_to_pdf};

    let doc: TemplateDocument = serde_json::from_str(&req.document_json)
        .map_err(|e| ApiError::bad_request(format!("Invalid template JSON: {}", e)))?;

    let fonts = FontCache::with_system_fonts();
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
    responses((status = 200, body = ValidationResultDto)),
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
    responses((status = 200, description = "List report definitions", body = Vec<ReportDefinitionDto>)),
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
    responses((status = 200, description = "Get report definition", body = ReportDefinitionDto)),
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
    responses((status = 200, description = "List template bindings", body = Vec<TemplateBindingDto>)),
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
    responses((status = 201, description = "Create template binding", body = TemplateBindingDto)),
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

fn coerce_report_parameter_value(value: serde_json::Value) -> Result<ParamValue, ApiError> {
    match value {
        serde_json::Value::Null => Err(ApiError::bad_request(
            "Report parameter value cannot be null",
        )),
        serde_json::Value::String(s) => Ok(ParamValue::Text(s)),
        serde_json::Value::Bool(b) => Ok(ParamValue::Boolean(b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(ParamValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(ParamValue::Decimal(f))
            } else {
                Err(ApiError::bad_request(
                    "Unsupported numeric report parameter value",
                ))
            }
        }
        serde_json::Value::Array(items) => {
            let mut values = Vec::with_capacity(items.len());
            for item in items {
                let value = match item {
                    serde_json::Value::String(s) => s,
                    _ => {
                        return Err(ApiError::bad_request(
                            "Report parameter id list must contain only strings",
                        ));
                    }
                };
                values.push(value);
            }
            Ok(ParamValue::IdList(values))
        }
        serde_json::Value::Object(mut obj) => {
            if let Some(kind) = obj.get("type").and_then(|value| value.as_str()) {
                match kind {
                    "date_range" | "dateRange" => {
                        let from = obj
                            .remove("from")
                            .and_then(|value| value.as_str().map(str::to_owned))
                            .ok_or_else(|| {
                                ApiError::bad_request(
                                    "date_range parameter requires a 'from' string",
                                )
                            })?;
                        let to = obj
                            .remove("to")
                            .and_then(|value| value.as_str().map(str::to_owned))
                            .ok_or_else(|| {
                                ApiError::bad_request("date_range parameter requires a 'to' string")
                            })?;
                        Ok(ParamValue::DateRange { from, to })
                    }
                    _ => Ok(ParamValue::Text(serde_json::to_string(&obj).map_err(
                        |error| {
                            ApiError::bad_request(format!(
                                "Failed to coerce report parameter object: {error}"
                            ))
                        },
                    )?)),
                }
            } else if obj.contains_key("from") || obj.contains_key("to") {
                let from = obj
                    .remove("from")
                    .and_then(|value| value.as_str().map(str::to_owned))
                    .unwrap_or_default();
                let to = obj
                    .remove("to")
                    .and_then(|value| value.as_str().map(str::to_owned))
                    .unwrap_or_default();
                Ok(ParamValue::DateRange { from, to })
            } else {
                Ok(ParamValue::Text(serde_json::to_string(&obj).map_err(
                    |error| {
                        ApiError::bad_request(format!(
                            "Failed to coerce report parameter object: {error}"
                        ))
                    },
                )?))
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/reports/generate",
    tag = "Report",
    request_body = GenerateReportRequest,
    responses((status = 200, description = "Generate report", body = GenerateReportResponse)),
    security(("bearer" = []))
)]
pub async fn generate_report(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<GenerateReportRequest>,
) -> Result<Json<ApiResponse<GenerateReportResponse>>, ApiError> {
    let ctx = auth.0;

    let params: HashMap<String, ParamValue> = req
        .parameters
        .map(|value| match value {
            serde_json::Value::Object(object) => object
                .into_iter()
                .map(|(key, value)| coerce_report_parameter_value(value).map(|value| (key, value)))
                .collect::<Result<HashMap<_, _>, _>>(),
            _ => Err(ApiError::bad_request("Report parameters must be an object")),
        })
        .unwrap_or_else(|| Ok(HashMap::new()))?;

    let output_format = req
        .format
        .map(|f| OutputFormat::from_str(&f))
        .unwrap_or(OutputFormat::Pdf);

    let locale = req
        .locale
        .map(|l| ReportLocale::from_str(&l))
        .unwrap_or(ReportLocale::Ar);

    let input = orbis_app::services::GenerateReportInput {
        report_code: req.report_code,
        parameters: params,
        output_format,
        locale,
        title: None,
        selected_columns: None,
        template_id: None,
    };

    check_report_permissions(&state, &ctx, &input.report_code).await?;
    let job = state
        .report_service
        .enqueue_report(&ctx, input)
        .await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;

    Ok(ApiResponse::ok_with_id(
        GenerateReportResponse {
            job_id: job.id,
            status: job.status.to_string(),
        },
        ctx.request_id,
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/reports/custom",
    tag = "Report",
    request_body = AiCustomReportRequest,
    responses((status = 200, description = "Create a confirmed custom report job", body = AiCustomReportResponse)),
    security(("bearer" = []))
)]
pub async fn create_custom_report(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(request): Json<AiCustomReportRequest>,
) -> Result<Json<ApiResponse<AiCustomReportResponse>>, ApiError> {
    let ctx = auth.0;
    let response = state
        .ai_service
        .create_custom_report(&ctx, request)
        .await
        .map_err(ApiError::from)?;
    Ok(ApiResponse::ok_with_id(response, ctx.request_id))
}

#[utoipa::path(
    get,
    path = "/api/v1/reports/jobs/{id}/status",
    tag = "Report",
    responses((status = 200, description = "Get report job status", body = ReportJobStatusResponse)),
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
    ensure_report_job_owner(&ctx, &job)?;
    check_report_permissions(&state, &ctx, &job.report_code).await?;

    Ok(ApiResponse::ok_with_id(
        ReportJobStatusResponse {
            job_id: job.id,
            status: job.status.to_string(),
            output_size_bytes: job.output_size_bytes,
            error_message: job.error_message,
        },
        ctx.request_id,
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/reports/jobs/{id}/cancel",
    tag = "Report",
    responses((status = 200, description = "Request report cancellation", body = CancelReportResponse)),
    security(("bearer" = []))
)]
pub async fn cancel_report_job(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<CancelReportResponse>>, ApiError> {
    let ctx = auth.0;
    let job = state
        .report_job_repo
        .find_by_id(id)
        .await
        .map_err(|error| ApiError::internal(error.to_string()))?
        .ok_or_else(|| ApiError::not_found("Report job not found"))?;
    ensure_report_job_owner(&ctx, &job)?;
    check_report_permissions(&state, &ctx, &job.report_code).await?;
    if !matches!(&job.status, JobStatus::Pending | JobStatus::Processing) {
        return Err(ApiError::bad_request(
            "Only pending or processing reports can be cancelled",
        ));
    }
    let cancel_requested = state
        .report_service
        .cancel_report(id)
        .await
        .map_err(|error| ApiError::internal(error.to_string()))?;
    if !cancel_requested {
        return Err(ApiError::bad_request(
            "The report worker is no longer active; refresh the report status",
        ));
    }

    Ok(ApiResponse::ok_with_id(
        CancelReportResponse {
            job_id: id,
            cancel_requested: true,
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
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let ctx = auth.0;
    let job = state
        .report_job_repo
        .find_by_id(id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Report job not found"))?;
    ensure_report_job_owner(&ctx, &job)?;
    check_report_permissions(&state, &ctx, &job.report_code).await?;

    if job.status.to_string() != "completed" {
        return Err(ApiError::bad_request(
            "Report generation is not completed yet",
        ));
    }

    let file_path = job
        .output_path
        .ok_or_else(|| ApiError::not_found("Report file path missing"))?;

    if report_download_expired(job.created_at, Utc::now()) {
        let extension = std::path::Path::new(&file_path)
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default();
        let staging_path =
            std::path::Path::new(&file_path).with_extension(format!("{extension}.part"));
        for path in [std::path::Path::new(&file_path), staging_path.as_path()] {
            match tokio::fs::remove_file(path).await {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(ApiError::internal(error.to_string())),
            }
        }
        state
            .report_job_repo
            .update_status(
                job.id,
                &JobStatus::Failed,
                None,
                None,
                Some("Report download has expired".into()),
            )
            .await
            .map_err(|error| ApiError::internal(error.to_string()))?;
        return Err(ApiError::not_found("Report download has expired"));
    }

    let bytes = tokio::fs::read(&file_path)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to read report file: {}", e)))?;

    Ok((
        [(header::CONTENT_TYPE, job.output_format.mime_type())],
        bytes,
    ))
}

async fn check_report_permissions(
    state: &AppState,
    ctx: &orbis_app::context::RequestContext,
    report_code: &str,
) -> Result<(), ApiError> {
    if let Some(user_id) = ctx.actor.id() {
        if state
            .permission_checker
            .get_org_unit_scope(user_id, ctx.tenant_id)
            .await?
            .is_some()
        {
            return Err(ApiError::forbidden(
                "Report export is unavailable for users restricted to organizational units",
            ));
        }
    }
    state
        .permission_checker
        .check(ctx, Permission::ExportData)
        .await?;
    let definition = state
        .report_service
        .get_report_by_code(ctx.tenant_id, report_code)
        .await
        .map_err(|error| ApiError::internal(error.to_string()))?
        .ok_or_else(|| ApiError::not_found("Report definition not found"))?;
    if matches!(definition.domain, ReportDomain::Finance) {
        state
            .permission_checker
            .check(ctx, Permission::FinanceRead)
            .await?;
    }
    Ok(())
}

fn ensure_report_job_owner(
    ctx: &orbis_app::context::RequestContext,
    job: &ReportJob,
) -> Result<(), ApiError> {
    if job.org_id != ctx.tenant_id || ctx.actor.id() != Some(job.requested_by) {
        return Err(ApiError::not_found("Report job not found"));
    }
    Ok(())
}

fn report_download_expired(created_at: chrono::DateTime<Utc>, now: chrono::DateTime<Utc>) -> bool {
    now - created_at > chrono::Duration::hours(24)
}

#[utoipa::path(
    post,
    path = "/api/v1/reports/batch-print",
    tag = "Report",
    request_body = BatchPrintRequest,
    responses((status = 200, description = "Multi-up PDF", content(("application/pdf" = Vec<u8>)))),
    security(("bearer" = []))
)]
pub async fn batch_print(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<BatchPrintRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let ctx = auth.0;
    let config: docforge_engine::layout::multi_up::MultiUpConfig =
        serde_json::from_value(req.batch_config)
            .map_err(|e| ApiError::bad_request(format!("Invalid batch config: {}", e)))?;

    let locale = req
        .locale
        .as_deref()
        .map(ReportLocale::from_str)
        .unwrap_or_default();
    let mut title = req.title.clone();
    let mut contexts = req.contexts;
    let mut document_json = req.document_json;

    if let Some(report_code) = req.report_code.as_deref() {
        check_report_permissions(&state, &ctx, report_code).await?;
        let definition = state
            .report_definition_repo
            .find_by_code(ctx.tenant_id, report_code)
            .await
            .map_err(|error| ApiError::internal(error.to_string()))?
            .ok_or_else(|| ApiError::not_found("Report definition not found"))?;
        let org_context = state
            .report_org_context_resolver
            .resolve(ctx.tenant_id)
            .await
            .map_err(|error| ApiError::internal(error.to_string()))?;
        let parameters: HashMap<String, ParamValue> = req
            .parameters
            .map(serde_json::from_value)
            .transpose()
            .map_err(|error| ApiError::bad_request(format!("Invalid report parameters: {error}")))?
            .unwrap_or_default();
        let provider = state
            .report_provider_registry
            .get_provider(&definition.data_provider_key)
            .ok_or_else(|| {
                ApiError::bad_request(format!(
                    "No data provider for key: {}",
                    definition.data_provider_key
                ))
            })?;
        let dataset = provider
            .fetch_data(ctx.tenant_id, &org_context, &parameters, &locale)
            .await
            .map_err(|error| ApiError::internal(error.to_string()))?;

        title = Some(if locale.is_rtl() {
            dataset.report_name_ar.clone()
        } else {
            dataset.report_name.clone()
        });
        if contexts.is_none() {
            let org_context = serde_json::json!({
                "id": org_context.id,
                "name": org_context.name,
                "name_ar": org_context.name_ar,
                "currency": org_context.currency_code,
                "logo_url": org_context.logo_url,
                "primary_color": org_context.primary_color,
            });
            contexts = Some(
                dataset
                    .rows
                    .into_iter()
                    .map(|row| {
                        let mut context = row.as_object().cloned().unwrap_or_default();
                        context.insert("row".into(), row);
                        context.insert("$org".into(), org_context.clone());
                        serde_json::Value::Object(context)
                    })
                    .collect(),
            );
        }

        if req.template_id.is_none() {
            let locale_key = if locale.is_rtl() { "ar" } else { "en" };
            let binding = state
                .report_template_repo
                .find_binding(definition.id, locale_key, &OutputFormat::Pdf)
                .await
                .map_err(|error| ApiError::internal(error.to_string()))?
                .ok_or_else(|| {
                    ApiError::bad_request("No published PDF template is bound to this report")
                })?;
            let template = state
                .report_template_repo
                .find_by_id(binding.template_id)
                .await
                .map_err(|error| ApiError::internal(error.to_string()))?
                .ok_or_else(|| ApiError::not_found("Bound report template not found"))?;
            validate_batch_template(&template, ctx.tenant_id)?;
            title = Some(template.name.clone());
            document_json = Some(published_template_document(&state, &template).await?);
        }
    }

    if let Some(template_id) = req.template_id {
        let template = state
            .report_template_repo
            .find_by_id(template_id)
            .await
            .map_err(|error| ApiError::internal(error.to_string()))?
            .ok_or_else(|| ApiError::not_found("Print template not found"))?;
        validate_batch_template(&template, ctx.tenant_id)?;
        title = Some(template.name.clone());
        document_json = Some(published_template_document(&state, &template).await?);
    }

    let document_json = document_json
        .ok_or_else(|| ApiError::bad_request("A template or report code is required"))?;
    let contexts = contexts
        .filter(|contexts| !contexts.is_empty())
        .ok_or_else(|| ApiError::bad_request("At least one data context is required"))?;
    let org_context = state
        .report_org_context_resolver
        .resolve(ctx.tenant_id)
        .await
        .map_err(|error| ApiError::internal(error.to_string()))?;
    let org_context = serde_json::json!({
        "id": org_context.id,
        "name": org_context.name,
        "name_ar": org_context.name_ar,
        "currency": org_context.currency_code,
        "logo_url": org_context.logo_url,
        "primary_color": org_context.primary_color,
    });
    let mut contexts = contexts;
    for (index, context) in contexts.iter_mut().enumerate() {
        let context = context.as_object_mut().ok_or_else(|| {
            ApiError::bad_request(format!(
                "Data context at index {index} must be a JSON object"
            ))
        })?;
        context.entry("$org").or_insert_with(|| org_context.clone());
    }
    let pdf = docforge_engine::render::multi_up_renderer::render_multi_up(
        &document_json,
        &contexts,
        &config,
        title,
    )
    .map_err(|error| ApiError::bad_request(error.to_string()))?;

    Ok(([(header::CONTENT_TYPE, "application/pdf")], pdf.bytes))
}

fn validate_batch_template(template: &ReportTemplate, org_id: Uuid) -> Result<(), ApiError> {
    if template.org_id != org_id || template.deleted_at.is_some() {
        return Err(ApiError::not_found("Print template not found"));
    }
    if template.status != TemplateStatus::Published {
        return Err(ApiError::bad_request(
            "Only published templates can be batch printed",
        ));
    }
    Ok(())
}

async fn published_template_document(
    state: &AppState,
    template: &ReportTemplate,
) -> Result<String, ApiError> {
    if let Some(version) = template.published_version {
        return state
            .report_template_repo
            .find_version(template.id, version)
            .await
            .map_err(|error| ApiError::internal(error.to_string()))?
            .map(|version| version.document_json)
            .ok_or_else(|| ApiError::not_found("Published template version not found"));
    }
    Ok(template.document_json.clone())
}

#[derive(Serialize, ToSchema)]
pub struct ProviderSchemaResponse {
    pub provider_key: String,
    pub name: String,
    pub name_ar: String,
    pub fields: Vec<ProviderSchemaField>,
}

#[derive(Serialize, ToSchema)]
pub struct ProviderSchemaField {
    pub key: String,
    pub label: String,
    pub label_ar: String,
    pub data_type: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/reports/schema/{key}",
    tag = "Report",
    params(("key" = String, Path, description = "Data provider key (e.g. academic.term_result, exam.seating_plan)")),
    responses((status = 200, description = "Get data provider schema for template discovery", body = ProviderSchemaResponse)),
    security(("bearer" = []))
)]
pub async fn get_provider_schema(
    _auth: AuthContext,
    Path(key): Path<String>,
) -> Result<Json<ApiResponse<ProviderSchemaResponse>>, ApiError> {
    let (name, name_ar, fields) = match key.as_str() {
        "academic.term_result" => (
            "Term Student Certificate",
            "شهادة نتائج الفصل الدراسي",
            vec![
                ProviderSchemaField {
                    key: "student_name".into(),
                    label: "Student Name".into(),
                    label_ar: "اسم الطالب".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "subject_name".into(),
                    label: "Subject".into(),
                    label_ar: "المادة".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "score".into(),
                    label: "Score".into(),
                    label_ar: "الدرجة".into(),
                    data_type: "decimal".into(),
                },
                ProviderSchemaField {
                    key: "max_score".into(),
                    label: "Max Score".into(),
                    label_ar: "الدرجة العظمى".into(),
                    data_type: "decimal".into(),
                },
                ProviderSchemaField {
                    key: "percentage".into(),
                    label: "Percentage".into(),
                    label_ar: "النسبة المئوية".into(),
                    data_type: "percentage".into(),
                },
                ProviderSchemaField {
                    key: "status".into(),
                    label: "Status".into(),
                    label_ar: "الحالة".into(),
                    data_type: "status".into(),
                },
            ],
        ),
        "exam.seating_plan" => (
            "Exam Seating Plan",
            "مخطط جلوس الطلاب في الاختبارات",
            vec![
                ProviderSchemaField {
                    key: "seating_number".into(),
                    label: "Seat #".into(),
                    label_ar: "رقم الجلوس".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "student_name".into(),
                    label: "Student Name".into(),
                    label_ar: "اسم الطالب".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "room_name".into(),
                    label: "Room".into(),
                    label_ar: "القاعة".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "session_name".into(),
                    label: "Session".into(),
                    label_ar: "جلسة الاختبار".into(),
                    data_type: "text".into(),
                },
            ],
        ),
        "finance.payment_receipt" => (
            "Payment Receipt",
            "سند قبض مالي",
            vec![
                ProviderSchemaField {
                    key: "receipt_number".into(),
                    label: "Receipt #".into(),
                    label_ar: "رقم السند".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "student_name".into(),
                    label: "Student Name".into(),
                    label_ar: "اسم الطالب".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "fee_name".into(),
                    label: "Fee".into(),
                    label_ar: "نوع الرسم".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "amount".into(),
                    label: "Amount".into(),
                    label_ar: "المبلغ".into(),
                    data_type: "currency".into(),
                },
                ProviderSchemaField {
                    key: "payment_method".into(),
                    label: "Payment Method".into(),
                    label_ar: "طريقة الدفع".into(),
                    data_type: "text".into(),
                },
            ],
        ),
        _ => (
            "Standard Document Data",
            "بيانات المستند",
            vec![
                ProviderSchemaField {
                    key: "student_name".into(),
                    label: "Student Name".into(),
                    label_ar: "اسم الطالب".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "class_name".into(),
                    label: "Class".into(),
                    label_ar: "الصف".into(),
                    data_type: "text".into(),
                },
                ProviderSchemaField {
                    key: "status".into(),
                    label: "Status".into(),
                    label_ar: "الحالة".into(),
                    data_type: "status".into(),
                },
            ],
        ),
    };

    Ok(ApiResponse::ok(ProviderSchemaResponse {
        provider_key: key,
        name: name.to_string(),
        name_ar: name_ar.to_string(),
        fields,
    }))
}

#[derive(Serialize, ToSchema)]
pub struct AuditLogDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub metadata: Option<String>,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, IntoParams)]
pub struct AuditLogFilter {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

#[utoipa::path(
    get,
    path = "/api/v1/reports/audit-logs",
    tag = "Report",
    params(AuditLogFilter),
    responses((status = 200, description = "List report and system audit logs", body = Vec<AuditLogDto>)),
    security(("bearer" = []))
)]
pub async fn list_audit_logs(
    State(state): State<AppState>,
    auth: AuthContext,
    Query(filter): Query<AuditLogFilter>,
) -> Result<Json<ApiResponse<Vec<AuditLogDto>>>, ApiError> {
    let entries = match (filter.entity_type.as_deref(), filter.entity_id) {
        (Some(entity_type), Some(entity_id)) => {
            state
                .report_audit_repo
                .list_by_entity(
                    auth.tenant_id,
                    entity_type,
                    entity_id,
                    filter.limit,
                    filter.offset,
                )
                .await
        }
        (None, None) => {
            state
                .report_audit_repo
                .list_by_org(auth.tenant_id, filter.limit, filter.offset)
                .await
        }
        _ => {
            return Err(ApiError::bad_request(
                "entity_type and entity_id must be provided together",
            ));
        }
    }
    .map_err(|e| ApiError::internal(e.to_string()))?;

    let dtos = entries
        .into_iter()
        .map(|e| AuditLogDto {
            id: e.id,
            org_id: e.org_id,
            user_id: Some(e.actor_id),
            action: e.action,
            entity_type: e.entity_type,
            entity_id: Some(e.entity_id),
            ip_address: None,
            metadata: e
                .details
                .as_ref()
                .map(|d| serde_json::to_string_pretty(d).unwrap_or_default()),
            details: e.details,
            created_at: e.created_at,
        })
        .collect();

    Ok(ApiResponse::ok(dtos))
}

// ═══════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/templates",
            get(list_templates).post(create_template),
        )
        .route(
            "/api/v1/templates/:id",
            get(get_template)
                .put(update_template)
                .delete(delete_template),
        )
        .route("/api/v1/templates/:id/publish", post(publish_template))
        .route("/api/v1/templates/:id/archive", post(archive_template))
        .route("/api/v1/templates/:id/versions", get(get_template_versions))
        .route(
            "/api/v1/templates/:id/revert",
            post(revert_template_version),
        )
        .route("/api/v1/templates/preview", post(preview_template))
        .route("/api/v1/templates/validate", post(validate_template))
        .route("/api/v1/report-definitions", get(list_report_definitions))
        .route(
            "/api/v1/report-definitions/:code",
            get(get_report_definition),
        )
        .route(
            "/api/v1/report-definitions/:definition_id/bindings",
            get(list_template_bindings).post(create_template_binding),
        )
        .route("/api/v1/reports/generate", post(generate_report))
        .route("/api/v1/reports/custom", post(create_custom_report))
        .route(
            "/api/v1/reports/jobs/:id/status",
            get(get_report_job_status),
        )
        .route("/api/v1/reports/jobs/:id/cancel", post(cancel_report_job))
        .route(
            "/api/v1/reports/jobs/:id/download",
            get(download_report_file),
        )
        .route("/api/v1/reports/batch-print", post(batch_print))
        .route("/api/v1/reports/schema/:key", get(get_provider_schema))
        .route("/api/v1/reports/audit-logs", get(list_audit_logs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbis_app::context::ActionSource;
    use orbis_domain::iam::actor::Actor;

    #[test]
    fn report_download_expires_after_twenty_four_hours() {
        let created_at = Utc::now();
        assert!(!report_download_expired(
            created_at,
            created_at + chrono::Duration::hours(24)
        ));
        assert!(report_download_expired(
            created_at,
            created_at + chrono::Duration::hours(24) + chrono::Duration::seconds(1)
        ));
    }

    #[test]
    fn report_job_is_hidden_from_other_users_and_organizations() {
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let job = ReportJob {
            id: Uuid::new_v4(),
            org_id,
            report_definition_id: Uuid::new_v4(),
            template_id: None,
            requested_by: user_id,
            report_code: "rep.finance.income-statement".into(),
            parameters: serde_json::json!({}),
            output_format: OutputFormat::Xlsx,
            locale: ReportLocale::Ar,
            status: JobStatus::Pending,
            output_path: None,
            output_size_bytes: None,
            error_message: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        };
        let context = |tenant_id, actor_id| orbis_app::context::RequestContext {
            tenant_id,
            actor: Actor::User(actor_id),
            request_id: "test".into(),
            source: ActionSource::Api,
            org_unit_path: None,
        };

        assert!(ensure_report_job_owner(&context(org_id, user_id), &job).is_ok());
        assert!(ensure_report_job_owner(&context(org_id, Uuid::new_v4()), &job).is_err());
        assert!(ensure_report_job_owner(&context(Uuid::new_v4(), user_id), &job).is_err());
    }
}
