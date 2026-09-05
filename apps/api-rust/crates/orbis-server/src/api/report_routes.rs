//! Report API routes — comprehensive REST API for the reporting system.

use axum::{
    extract::{State, Path, Query},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::state::AppState;
use orbis_domain::entities::report::*;
use orbis_app::usecases::report::generate_report::{GenerateReportUseCase, GenerateReportInput};
use orbis_app::usecases::report::manage_template::ManageTemplateUseCase;
use orbis_app::usecases::report::list_reports::ListReportsUseCase;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct GenerateReportRequest {
    pub report_code: String,
    pub parameters: HashMap<String, ParamValue>,
    #[serde(default)]
    pub output_format: String,
    #[serde(default)]
    pub locale: String,
}

#[derive(Serialize)]
pub struct ReportJobResponse {
    pub job_id: Uuid,
    pub status: String,
    pub report_code: String,
    pub output_format: String,
    pub output_size_bytes: Option<i64>,
    pub error_message: Option<String>,
}

#[derive(Deserialize)]
pub struct ListReportsQuery {
    pub domain: Option<String>,
    pub category: Option<String>,
}

#[derive(Deserialize)]
pub struct ListTemplatesQuery {
    pub status: Option<String>,
    pub document_type: Option<String>,
    pub entity_type: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub document_type: String,
    pub entity_type: String,
    pub master_template_id: Option<Uuid>,
    #[serde(default = "default_page_size")]
    pub page_size: String,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    #[serde(default = "default_margins")]
    pub margins_json: String,
    #[serde(default = "default_document")]
    pub document_json: String,
    #[serde(default = "default_locale")]
    pub locale: String,
}

fn default_page_size() -> String { "A4".into() }
fn default_orientation() -> String { "portrait".into() }
fn default_margins() -> String { r#"{"top":10,"bottom":10,"left":10,"right":10}"#.into() }
fn default_document() -> String { "{}".into() }
fn default_locale() -> String { "ar".into() }

#[derive(Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub document_json: Option<String>,
    pub page_size: Option<String>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
}

#[derive(Deserialize)]
pub struct RevertVersionRequest {
    pub target_version: i32,
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        // Report definitions
        .route("/api/reports/definitions", get(list_definitions))
        .route("/api/reports/definitions/{code}", get(get_definition))
        // Report generation
        .route("/api/reports/generate", post(generate_report))
        // Batch / multi-up printing
        .route("/api/reports/batch-print", post(batch_print))
        .route("/api/reports/compute-layout", post(compute_layout))
        // Jobs
        .route("/api/reports/jobs/{id}", get(get_job_status))
        .route("/api/reports/jobs/{id}/download", get(download_report))
        // Templates
        .route("/api/reports/templates", get(list_templates).post(create_template))
        .route("/api/reports/templates/{id}", get(get_template).put(update_template).delete(delete_template))
        .route("/api/reports/templates/{id}/publish", post(publish_template))
        .route("/api/reports/templates/{id}/archive", post(archive_template))
        .route("/api/reports/templates/{id}/versions", get(get_versions))
        .route("/api/reports/templates/{id}/revert", post(revert_version))
        // CLI-style commands
        .route("/api/reports/cmd/seed-definitions", post(cmd_seed_definitions))
        .route("/api/reports/cmd/list-providers", get(cmd_list_providers))
}

// ── Helper ──────────────────────────────────────────────────────

fn make_ctx(org_id: Option<Uuid>) -> orbis_app::context::RequestContext {
    // In production, this would be extracted from the JWT/session middleware.
    // For now, uses the provided org_id or a default.
    orbis_app::context::RequestContext {
        tenant_id: org_id.unwrap_or_default(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    }
}

#[derive(Deserialize)]
pub struct OrgIdQuery {
    pub org_id: Option<Uuid>,
}

// ── Handlers ────────────────────────────────────────────────────

async fn list_definitions(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Query(q): Query<ListReportsQuery>,
) -> Result<Json<Vec<ReportDefinition>>, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let usecase = ListReportsUseCase {
        definition_repo: state.report_definition_repo.clone(),
    };
    let results = usecase
        .execute(&ctx, q.domain.as_deref(), q.category.as_deref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(results))
}

async fn get_definition(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Path(code): Path<String>,
) -> Result<Json<ReportDefinition>, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let def = state.report_definition_repo
        .find_by_code(ctx.tenant_id, &code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Report definition not found: {}", code)))?;
    Ok(Json(def))
}

async fn generate_report(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Json(payload): Json<GenerateReportRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);

    let usecase = GenerateReportUseCase {
        definition_repo: state.report_definition_repo.clone(),
        template_repo: state.report_template_repo.clone(),
        job_repo: state.report_job_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
        provider_registry: state.report_provider_registry.clone(),
        renderer_registry: state.report_renderer_registry.clone(),
        org_context_resolver: state.report_org_context_resolver.clone(),
    };

    let format = if payload.output_format.is_empty() {
        OutputFormat::DataOnly
    } else {
        OutputFormat::from_str(&payload.output_format)
    };

    let locale = if payload.locale.is_empty() {
        ReportLocale::Ar
    } else {
        ReportLocale::from_str(&payload.locale)
    };

    let input = GenerateReportInput {
        report_code: payload.report_code,
        parameters: payload.parameters,
        output_format: format.clone(),
        locale,
    };

    let (_job, export) = usecase
        .execute(&ctx, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // For data_only/json, return JSON directly
    if format == OutputFormat::DataOnly || format == OutputFormat::Json {
        return Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, export.mime_type.clone())],
            export.data,
        ).into_response());
    }

    // For file formats, return with content-disposition
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, export.mime_type.clone()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", export.filename)),
        ],
        export.data,
    ).into_response())
}

async fn get_job_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportJobResponse>, (StatusCode, String)> {
    let job = state.report_job_repo
        .find_by_id(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Job not found".to_string()))?;

    Ok(Json(ReportJobResponse {
        job_id: job.id,
        status: job.status.to_string(),
        report_code: job.report_code,
        output_format: job.output_format.to_string(),
        output_size_bytes: job.output_size_bytes,
        error_message: job.error_message,
    }))
}

async fn download_report(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    // File-based download would read from output_path.
    // For now the generate endpoint returns data directly.
    Err((StatusCode::NOT_IMPLEMENTED, "File-based download not yet implemented. Use /generate endpoint.".into()))
}

// ── Template CRUD ───────────────────────────────────────────────

async fn list_templates(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Query(q): Query<ListTemplatesQuery>,
) -> Result<Json<Vec<ReportTemplate>>, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let templates = state.report_template_repo
        .list_by_org(ctx.tenant_id, q.status.as_deref(), q.document_type.as_deref(), q.entity_type.as_deref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(templates))
}

async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportTemplate>, (StatusCode, String)> {
    let template = state.report_template_repo
        .find_by_id(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Template not found".to_string()))?;
    Ok(Json(template))
}

async fn create_template(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Json(payload): Json<CreateTemplateRequest>,
) -> Result<(StatusCode, Json<ReportTemplate>), (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };

    let template = ReportTemplate {
        id: Uuid::nil(), // set by use case
        org_id: Uuid::nil(), // set by use case
        code: None,
        name: payload.name,
        name_ar: payload.name_ar,
        description: payload.description,
        description_ar: payload.description_ar,
        document_type: payload.document_type,
        entity_type: payload.entity_type,
        master_template_id: payload.master_template_id,
        status: TemplateStatus::Draft,
        version: 1,
        published_version: None,
        published_at: None,
        published_by: None,
        page_size: payload.page_size,
        page_width_mm: None,
        page_height_mm: None,
        orientation: payload.orientation,
        margins_json: payload.margins_json,
        document_json: payload.document_json,
        locale: payload.locale,
        tags: None,
        is_system: false,
        is_default: false,
        created_by: Uuid::nil(), // set by use case
        deleted_at: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created = usecase
        .create(&ctx, template)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(created)))
}

async fn update_template(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTemplateRequest>,
) -> Result<Json<ReportTemplate>, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };

    let updated = usecase
        .update(
            &ctx, id,
            payload.name, payload.name_ar,
            payload.description, payload.description_ar,
            payload.document_json, payload.page_size,
            payload.orientation, payload.margins_json,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(updated))
}

async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.report_template_repo
        .soft_delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

async fn publish_template(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportTemplate>, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };
    let published = usecase
        .publish(&ctx, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(published))
}

async fn archive_template(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };
    usecase
        .archive(&ctx, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_versions(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<ReportTemplateVersion>>, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };
    let versions = usecase
        .get_versions(&ctx, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(versions))
}

async fn revert_version(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Path(id): Path<Uuid>,
    Json(payload): Json<RevertVersionRequest>,
) -> Result<Json<ReportTemplate>, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);
    let usecase = ManageTemplateUseCase {
        template_repo: state.report_template_repo.clone(),
        audit_repo: state.report_audit_repo.clone(),
    };
    let reverted = usecase
        .revert_to_version(&ctx, id, payload.target_version)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(reverted))
}

// ═══════════════════════════════════════════════════════════════
// BATCH / MULTI-UP PRINTING
// ═══════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct BatchPrintRequest {
    pub report_code: String,
    pub parameters: HashMap<String, ParamValue>,
    pub batch_config: BatchPrintConfig,
    #[serde(default)]
    pub locale: String,
}

#[derive(Serialize)]
pub struct ComputeLayoutResponse {
    pub columns: u32,
    pub rows: u32,
    pub items_per_sheet: u32,
    pub sheet_width_mm: f64,
    pub sheet_height_mm: f64,
    pub slots: Vec<orbis_domain::entities::report::SlotPosition>,
}

/// Batch print: fetch data, then render each row as a card/item
/// arranged in multi-up layout on physical sheets.
async fn batch_print(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
    Json(payload): Json<BatchPrintRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ctx = make_ctx(org_q.org_id);

    // 1. Resolve definition
    let definition = state.report_definition_repo
        .find_by_code(ctx.tenant_id, &payload.report_code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Report definition not found: {}", payload.report_code)))?;

    // 2. Resolve org context
    let org_context = state.report_org_context_resolver
        .resolve(ctx.tenant_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 3. Fetch data
    let locale = if payload.locale.is_empty() {
        ReportLocale::Ar
    } else {
        ReportLocale::from_str(&payload.locale)
    };

    let provider = state.report_provider_registry
        .get_provider(&definition.data_provider_key)
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, format!("No data provider for key: {}", definition.data_provider_key)))?;

    let dataset = provider
        .fetch_data(ctx.tenant_id, &org_context, &payload.parameters, &locale)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 4. Compute multi-up layout
    let layout = payload.batch_config.compute_layout();
    let is_rtl = matches!(locale, ReportLocale::Ar | ReportLocale::ArEn);
    let dir = if is_rtl { "rtl" } else { "ltr" };

    // 5. Render multi-up HTML
    let primary_color = org_context.primary_color.as_deref().unwrap_or("#6366f1");
    let mut html = format!(r#"<!DOCTYPE html>
<html dir="{dir}">
<head>
<meta charset="UTF-8">
<title>Batch Print — {report_code}</title>
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
        report_code = payload.report_code,
        w = layout.sheet_width_mm,
        h = layout.sheet_height_mm,
        color = primary_color,
    );

    // Paginate rows into sheets
    let pages = payload.batch_config.paginate(&dataset.rows);
    let visible_cols: Vec<&ColumnDef> = dataset.columns.iter().filter(|c| c.visible).collect();
    let org_name = if is_rtl { &org_context.name_ar } else { &org_context.name };

    for page_items in &pages {
        html.push_str(&format!(r#"<div class="sheet">"#));

        // Render each item at its slot position
        for (idx, row) in page_items.iter().enumerate() {
            if let Some(slot) = layout.slots.get(idx) {
                html.push_str(&format!(
                    r#"<div class="item" style="left:{x}mm; top:{y}mm; width:{w}mm; height:{h}mm;">"#,
                    x = slot.x_mm, y = slot.y_mm,
                    w = slot.width_mm, h = slot.height_mm,
                ));

                // Org name header
                html.push_str(&format!(r#"<div class="org-name">{}</div>"#, org_name));

                // Render visible fields as label-value pairs
                for col in &visible_cols {
                    let label = if is_rtl { &col.header_ar } else { &col.header };
                    let val = row.get(&col.key)
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

        // Cut lines
        if payload.batch_config.show_cut_lines {
            for r in 1..layout.rows {
                let y = layout.slots.first().map(|s| s.y_mm).unwrap_or(0.0)
                    + r as f64 * (payload.batch_config.item_height_mm + payload.batch_config.gap_mm)
                    - payload.batch_config.gap_mm / 2.0;
                html.push_str(&format!(r#"<div class="cut-line-h" style="top:{y}mm;"></div>"#, y = y));
            }
            for c in 1..layout.columns {
                let x = layout.slots.first().map(|s| s.x_mm).unwrap_or(0.0)
                    + c as f64 * (payload.batch_config.item_width_mm + payload.batch_config.gap_mm)
                    - payload.batch_config.gap_mm / 2.0;
                html.push_str(&format!(r#"<div class="cut-line-v" style="left:{x}mm;"></div>"#, x = x));
            }
        }

        html.push_str("</div>");
    }

    html.push_str("</body></html>");

    let bytes = html.into_bytes();
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8".to_string()),
            (header::CONTENT_DISPOSITION, format!("inline; filename=\"batch_{}.html\"", payload.report_code.replace('.', "_"))),
        ],
        bytes,
    ).into_response())
}

/// Compute multi-up layout without rendering — for frontend preview.
async fn compute_layout(
    Json(config): Json<BatchPrintConfig>,
) -> Json<ComputeLayoutResponse> {
    let layout = config.compute_layout();
    Json(ComputeLayoutResponse {
        columns: layout.columns,
        rows: layout.rows,
        items_per_sheet: layout.items_per_sheet,
        sheet_width_mm: layout.sheet_width_mm,
        sheet_height_mm: layout.sheet_height_mm,
        slots: layout.slots,
    })
}

// ═══════════════════════════════════════════════════════════════
// CLI / ADMINISTRATIVE COMMANDS
// ═══════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct SeedResult {
    pub seeded: u32,
    pub definitions: Vec<String>,
}

#[derive(Serialize)]
pub struct ProviderListResult {
    pub providers: Vec<String>,
}

/// Seed built-in report definitions for the given org.
async fn cmd_seed_definitions(
    State(state): State<AppState>,
    Query(org_q): Query<OrgIdQuery>,
) -> Result<Json<SeedResult>, (StatusCode, String)> {
    let org_id = org_q.org_id.unwrap_or_default();
    let now = chrono::Utc::now();

    let definitions = vec![
        ReportDefinition {
            id: Uuid::new_v4(),
            org_id,
            code: "finance.fee_collection".into(),
            name: "Fee Collection Report".into(),
            name_ar: "تقرير تحصيل الرسوم".into(),
            domain: ReportDomain::Finance,
            category: "fees".into(),
            description: Some("Summary of fee collection by student".into()),
            description_ar: Some("ملخص تحصيل الرسوم حسب الطالب".into()),
            parameters: vec![],
            required_permissions: vec!["report.finance.view".into()],
            supported_formats: vec![OutputFormat::DataOnly, OutputFormat::Json, OutputFormat::Csv, OutputFormat::Html, OutputFormat::Pdf],
            data_provider_key: "finance.fee_collection".into(),
            is_system: true,
            is_active: true,
            sort_order: 1,
            version: 1,
            created_at: now,
            updated_at: now,
        },
        ReportDefinition {
            id: Uuid::new_v4(),
            org_id,
            code: "academic.student_list".into(),
            name: "Student List".into(),
            name_ar: "قائمة الطلاب".into(),
            domain: ReportDomain::Academic,
            category: "students".into(),
            description: Some("List of enrolled students by class".into()),
            description_ar: Some("قائمة الطلاب المسجلين حسب الصف".into()),
            parameters: vec![],
            required_permissions: vec!["report.academic.view".into()],
            supported_formats: vec![OutputFormat::DataOnly, OutputFormat::Json, OutputFormat::Csv, OutputFormat::Html, OutputFormat::Pdf],
            data_provider_key: "academic.student_list".into(),
            is_system: true,
            is_active: true,
            sort_order: 2,
            version: 1,
            created_at: now,
            updated_at: now,
        },
    ];

    let mut seeded = 0u32;
    let mut names = Vec::new();
    for def in &definitions {
        // Check if already exists
        if state.report_definition_repo.find_by_code(org_id, &def.code).await.ok().flatten().is_some() {
            continue;
        }
        state.report_definition_repo
            .save(def)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        seeded += 1;
        names.push(def.code.clone());
    }

    Ok(Json(SeedResult { seeded, definitions: names }))
}

/// List all registered data providers.
async fn cmd_list_providers(
    State(state): State<AppState>,
) -> Json<ProviderListResult> {
    let keys = state.report_provider_registry.list_provider_keys();
    Json(ProviderListResult {
        providers: keys.into_iter().map(|s| s.to_string()).collect(),
    })
}
