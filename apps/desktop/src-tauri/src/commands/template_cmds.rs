//! Tauri IPC commands — Template Management, Preview & Validation.
//!
//! These commands expose template lifecycle operations to the Svelte frontend
//! through the new hexagonal architecture:
//!   - CRUD (create, read, update, delete)
//!   - Lifecycle (publish, archive)
//!   - Versioning (list versions, revert)
//!   - Bindings (link templates to report definitions)
//!   - Preview (local rendering via docforge-engine)
//!   - Validation (schema/expression checking)
//!   - Report Definitions (read-only browsing)

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use orbis_domain::entities::report::*;
use uuid::Uuid;
use chrono::Utc;

use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

// ═══════════════════════════════════════════════════════════════
// DTOs
// ═══════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct TemplateDto {
    pub id: String,
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
    pub master_template_id: Option<String>,
    pub code: Option<String>,
    pub tags: Option<String>,
    pub is_system: bool,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}

fn template_dto(t: ReportTemplate) -> TemplateDto {
    TemplateDto {
        id: t.id.to_string(),
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
        master_template_id: t.master_template_id.map(|id| id.to_string()),
        code: t.code,
        tags: t.tags,
        is_system: t.is_system,
        is_default: t.is_default,
        created_at: t.created_at.to_rfc3339(),
        updated_at: t.updated_at.to_rfc3339(),
    }
}

#[derive(Deserialize)]
pub struct CreateTemplateInput {
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
    pub code: Option<String>,
    pub tags: Option<String>,
    pub master_template_id: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTemplateInput {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub page_size: Option<String>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    pub document_json: Option<String>,
}

#[derive(Serialize)]
pub struct VersionDto {
    pub id: String,
    pub template_id: String,
    pub version: i32,
    pub change_summary: Option<String>,
    pub created_by: String,
    pub created_at: String,
}

fn version_dto(v: ReportTemplateVersion) -> VersionDto {
    VersionDto {
        id: v.id.to_string(),
        template_id: v.template_id.to_string(),
        version: v.version,
        change_summary: v.change_summary,
        created_by: v.created_by.to_string(),
        created_at: v.created_at.to_rfc3339(),
    }
}

#[derive(Serialize)]
pub struct ReportDefinitionDto {
    pub id: String,
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub domain: String,
    pub category: String,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub supported_formats: Vec<String>,
    pub parameters: Vec<serde_json::Value>,
    pub is_system: bool,
    pub is_active: bool,
}

fn definition_dto(d: ReportDefinition) -> ReportDefinitionDto {
    ReportDefinitionDto {
        id: d.id.to_string(),
        code: d.code,
        name: d.name,
        name_ar: d.name_ar,
        domain: d.domain.to_string(),
        category: d.category,
        description: d.description,
        description_ar: d.description_ar,
        supported_formats: d.supported_formats.iter().map(|f| f.to_string()).collect(),
        parameters: d.parameters.iter()
            .filter_map(|p| serde_json::to_value(p).ok())
            .collect(),
        is_system: d.is_system,
        is_active: d.is_active,
    }
}

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

#[derive(Serialize)]
pub struct ValidationIssue {
    pub node_id: Option<String>,
    pub path: String,
    pub message: String,
    pub severity: String,
}

#[derive(Serialize)]
pub struct BindingDto {
    pub id: String,
    pub report_definition_id: String,
    pub template_id: String,
    pub locale: String,
    pub format: String,
    pub is_default: bool,
    pub created_at: String,
}

fn binding_dto(b: ReportTemplateBinding) -> BindingDto {
    BindingDto {
        id: b.id.to_string(),
        report_definition_id: b.report_definition_id.to_string(),
        template_id: b.template_id.to_string(),
        locale: b.locale,
        format: b.format.to_string(),
        is_default: b.is_default,
        created_at: b.created_at.to_rfc3339(),
    }
}

// ═══════════════════════════════════════════════════════════════
// ═══════════════════════════════════════════════════════════════
// TEMPLATE CRUD COMMANDS
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn template_create(
    input: CreateTemplateInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TemplateDto>, String> {
    let ctx = make_ctx(&state);

    let now = Utc::now();
    let template = ReportTemplate {
        id: Uuid::new_v4(),
        org_id: ctx.tenant_id,
        code: input.code,
        name: input.name,
        name_ar: input.name_ar,
        description: input.description,
        description_ar: input.description_ar,
        document_type: input.document_type,
        entity_type: input.entity_type,
        master_template_id: input.master_template_id
            .and_then(|id| Uuid::parse_str(&id).ok()),
        status: TemplateStatus::Draft,
        version: 1,
        published_version: None,
        published_at: None,
        published_by: None,
        page_size: input.page_size.unwrap_or_else(|| "A4".into()),
        page_width_mm: input.page_width_mm,
        page_height_mm: input.page_height_mm,
        orientation: input.orientation.unwrap_or_else(|| "portrait".into()),
        margins_json: input.margins_json.unwrap_or_else(|| r#"{"top":15,"bottom":15,"left":15,"right":15}"#.into()),
        document_json: input.document_json.unwrap_or_else(|| "{}".into()),
        locale: input.locale.unwrap_or_else(|| "ar".into()),
        tags: input.tags,
        is_system: false,
        is_default: false,
        created_by: ctx.actor.id().unwrap_or_default(),
        deleted_at: None,
        created_at: now,
        updated_at: now,
    };

    state.report_service.save_template(&template).await.map_err(|e| e.to_string())?;

    Ok(ok(template_dto(template)))
}

#[tauri::command]
pub async fn template_get(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TemplateDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid template ID")?;
    let template = state.report_template_repo.find_by_id(uid).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Template not found".to_string())?;
    Ok(ok(template_dto(template)))
}

#[tauri::command]
pub async fn template_list(
    document_type: Option<String>,
    entity_type: Option<String>,
    status: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<TemplateDto>>, String> {
    let ctx = make_ctx(&state);
    let mut templates = state.report_template_repo
        .list_by_org(
            ctx.tenant_id,
            status.as_deref(),
            document_type.as_deref(),
            entity_type.as_deref(),
        )
        .await
        .map_err(|e| e.to_string())?;

    if templates.is_empty() && document_type.is_none() && entity_type.is_none() && status.is_none() {
        let now = chrono::Utc::now().to_rfc3339();
        let _ = orbis_infra::adapters::bootstrap_adapter::seed_default_reports(&state.db, ctx.tenant_id, &now).await;
        if let Ok(reloaded) = state.report_template_repo.list_by_org(ctx.tenant_id, None, None, None).await {
            templates = reloaded;
        }
    }

    Ok(ok(templates.into_iter().map(template_dto).collect()))
}

#[tauri::command]
pub async fn template_update(
    id: String,
    input: UpdateTemplateInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TemplateDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid template ID")?;
    let mut template = state.report_template_repo.find_by_id(uid).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Template not found".to_string())?;

    if template.org_id != ctx.tenant_id {
        return Err("Template belongs to another organization".to_string());
    }

    if let Some(name) = input.name { template.name = name; }
    if let Some(name_ar) = input.name_ar { template.name_ar = Some(name_ar); }
    if let Some(desc) = input.description { template.description = Some(desc); }
    if let Some(desc_ar) = input.description_ar { template.description_ar = Some(desc_ar); }
    if let Some(doc) = input.document_json { template.document_json = doc; }
    if let Some(ps) = input.page_size { template.page_size = ps; }
    if let Some(o) = input.orientation { template.orientation = o; }
    if let Some(m) = input.margins_json { template.margins_json = m; }
    template.updated_at = Utc::now();

    state.report_service.save_template(&template).await.map_err(|e| e.to_string())?;

    Ok(ok(template_dto(template)))
}

#[tauri::command]
pub async fn template_delete(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid template ID")?;
    state.report_service.delete_template(&ctx, uid).await
        .map_err(|e| e.to_string())
}

// ═══════════════════════════════════════════════════════════════
// TEMPLATE LIFECYCLE COMMANDS
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn template_publish(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TemplateDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid template ID")?;
    let template = state.report_service.publish_template(&ctx, uid).await
        .map_err(|e| e.to_string())?;
    Ok(ok(template_dto(template)))
}

#[tauri::command]
pub async fn template_archive(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid template ID")?;
    state.report_service.archive_template(&ctx, uid).await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════
// TEMPLATE VERSIONING COMMANDS
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn template_get_versions(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<VersionDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid template ID")?;
    let versions = state.report_template_repo.find_versions(uid).await.map_err(|e| e.to_string())?;
    Ok(ok(versions.into_iter().map(version_dto).collect()))
}

#[tauri::command]
pub async fn template_revert_version(
    id: String,
    target_version: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TemplateDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid template ID")?;
    let template = state.report_service.revert_template_version(&ctx, uid, target_version).await
        .map_err(|e| e.to_string())?;
    Ok(ok(template_dto(template)))
}

// ═══════════════════════════════════════════════════════════════
// TEMPLATE BINDING COMMANDS
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn template_list_bindings(
    definition_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<BindingDto>>, String> {
    let uid = Uuid::parse_str(&definition_id).map_err(|_| "Invalid definition ID")?;
    let bindings = state.report_template_repo
        .find_bindings_by_definition(uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(bindings.into_iter().map(binding_dto).collect()))
}

#[tauri::command]
pub async fn template_create_binding(
    definition_id: String,
    template_id: String,
    locale: String,
    format: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<BindingDto>, String> {
    let def_id = Uuid::parse_str(&definition_id).map_err(|_| "Invalid definition ID")?;
    let tmpl_id = Uuid::parse_str(&template_id).map_err(|_| "Invalid template ID")?;

    let binding = ReportTemplateBinding {
        id: Uuid::new_v4(),
        report_definition_id: def_id,
        template_id: tmpl_id,
        locale,
        format: OutputFormat::from_str(&format),
        is_default: true,
        created_at: Utc::now(),
    };

    state.report_template_repo.save_binding(&binding).await
        .map_err(|e| e.to_string())?;

    Ok(ok(binding_dto(binding)))
}

// ═══════════════════════════════════════════════════════════════
// TEMPLATE PREVIEW (Local DocForge rendering)
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn template_preview(
    document_json: String,
    context_data: serde_json::Value,
    _locale: Option<String>,
    _page_index: Option<u32>,
    _state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    use docforge_engine::model::document::TemplateDocument;
    use docforge_engine::layout::engine::LayoutEngine;
    use docforge_engine::render::font_cache::FontCache;
    use docforge_engine::render::pdf_renderer::{render_to_pdf, PdfRenderConfig};

    // 1. Parse the template document
    let doc: TemplateDocument = serde_json::from_str(&document_json)
        .map_err(|e| format!("Invalid template document JSON: {}", e))?;

    // 2. Create font cache — using system default Arabic font
    let fonts = FontCache::new("Noto Sans Arabic");

    // 3. Run layout engine
    let layout_engine = LayoutEngine::new(&fonts);
    let layout = layout_engine.layout(&doc, &context_data)
        .map_err(|e| format!("Layout computation failed: {}", e))?;

    // 4. Render to PDF
    let config = PdfRenderConfig {
        title: Some(doc.meta.name.clone()),
        ..Default::default()
    };

    let pdf_output = render_to_pdf(&layout, &fonts, &config)
        .map_err(|e| format!("PDF rendering failed: {}", e))?;

    Ok(pdf_output.bytes)
}

// ═══════════════════════════════════════════════════════════════
// TEMPLATE VALIDATION
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn template_validate(
    document_json: String,
) -> Result<ApiResponse<ValidationResult>, String> {
    // Attempt to parse the document as a TemplateDocument
    let parse_result: Result<docforge_engine::model::document::TemplateDocument, _> =
        serde_json::from_str(&document_json);

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    match parse_result {
        Ok(doc) => {
            // Check for basic structural issues
            if doc.meta.name.is_empty() {
                warnings.push(ValidationIssue {
                    node_id: None,
                    path: "meta.name".into(),
                    message: "Template name is empty".into(),
                    severity: "warning".into(),
                });
            }

            // Validate page setup
            if doc.meta.page_setup.margins.top < 0.0
                || doc.meta.page_setup.margins.bottom < 0.0
                || doc.meta.page_setup.margins.left < 0.0
                || doc.meta.page_setup.margins.right < 0.0
            {
                errors.push(ValidationIssue {
                    node_id: None,
                    path: "meta.page_setup.margins".into(),
                    message: "Page margins cannot be negative".into(),
                    severity: "error".into(),
                });
            }

            // Validate expression syntax in text nodes
            validate_node_expressions(&doc.root, &mut errors, &mut warnings, "root");
        }
        Err(e) => {
            errors.push(ValidationIssue {
                node_id: None,
                path: "".into(),
                message: format!("Invalid template document: {}", e),
                severity: "error".into(),
            });
        }
    }

    Ok(ok(ValidationResult {
        valid: errors.is_empty(),
        errors,
        warnings,
    }))
}

/// Recursively validate expression syntax in text nodes.
fn validate_node_expressions(
    node: &docforge_engine::model::nodes::TemplateNode,
    errors: &mut Vec<ValidationIssue>,
    warnings: &mut Vec<ValidationIssue>,
    path: &str,
) {
    use docforge_engine::model::nodes::TemplateNode;

    match node {
        TemplateNode::Text(text_node) => {
            // Check if the text content has balanced expression brackets
            let content = &text_node.props.content;
            let open_count = content.matches("{{").count();
            let close_count = content.matches("}}").count();
            if open_count != close_count {
                errors.push(ValidationIssue {
                    node_id: Some(text_node.id.clone()),
                    path: format!("{}.content", path),
                    message: format!(
                        "Unbalanced expression brackets: {} opening vs {} closing",
                        open_count, close_count
                    ),
                    severity: "error".into(),
                });
            }
        }
        TemplateNode::Page(page) => {
            for (i, child) in page.children.iter().enumerate() {
                validate_node_expressions(child, errors, warnings, &format!("{}.children[{}]", path, i));
            }
        }
        TemplateNode::Frame(frame) => {
            for (i, child) in frame.children.iter().enumerate() {
                validate_node_expressions(child, errors, warnings, &format!("{}.children[{}]", path, i));
            }
        }
        TemplateNode::Grid(grid) => {
            for (i, child) in grid.children.iter().enumerate() {
                validate_node_expressions(child, errors, warnings, &format!("{}.children[{}]", path, i));
            }
        }
        TemplateNode::Repeater(repeater) => {
            for (i, child) in repeater.children.iter().enumerate() {
                validate_node_expressions(child, errors, warnings, &format!("{}.children[{}]", path, i));
            }
        }
        TemplateNode::Conditional(cond) => {
            for (i, child) in cond.children.iter().enumerate() {
                validate_node_expressions(child, errors, warnings, &format!("{}.then[{}]", path, i));
            }
            if let Some(ref else_nodes) = cond.else_children {
                for (i, child) in else_nodes.iter().enumerate() {
                    validate_node_expressions(child, errors, warnings, &format!("{}.else[{}]", path, i));
                }
            }
        }
        TemplateNode::Header(header) => {
            for (i, child) in header.children.iter().enumerate() {
                validate_node_expressions(child, errors, warnings, &format!("{}.children[{}]", path, i));
            }
        }
        TemplateNode::Footer(footer) => {
            for (i, child) in footer.children.iter().enumerate() {
                validate_node_expressions(child, errors, warnings, &format!("{}.children[{}]", path, i));
            }
        }
        // Content-only nodes or nodes without children — nothing to recurse into
        _ => {}
    }
}

// ═══════════════════════════════════════════════════════════════
// REPORT DEFINITION COMMANDS (read-only browsing)
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn report_list_definitions(
    domain: Option<String>,
    category: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<ReportDefinitionDto>>, String> {
    let ctx = make_ctx(&state);
    let mut defs = state.report_definition_repo
        .list_by_org(ctx.tenant_id, domain.as_deref(), category.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    if defs.is_empty() && domain.is_none() && category.is_none() {
        let now = chrono::Utc::now().to_rfc3339();
        let _ = orbis_infra::adapters::bootstrap_adapter::seed_default_reports(&state.db, ctx.tenant_id, &now).await;
        if let Ok(reloaded) = state.report_definition_repo.list_by_org(ctx.tenant_id, None, None).await {
            defs = reloaded;
        }
    }

    Ok(ok(defs.into_iter().map(definition_dto).collect()))
}

#[tauri::command]
pub async fn report_get_definition(
    code: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<ReportDefinitionDto>, String> {
    let ctx = make_ctx(&state);
    let def = state.report_definition_repo
        .find_by_code(ctx.tenant_id, &code)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Report definition not found: {}", code))?;
    Ok(ok(definition_dto(def)))
}
