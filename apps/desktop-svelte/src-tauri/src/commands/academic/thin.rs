/// Thin Tauri command wrappers for the Academic domain.
///
/// Each command is a 1-line delegation to `svc_academic::ops::*`.
/// NO business logic lives here.

use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

use svc_academic::ops;
use svc_organization::ops as org_ops;

use std::collections::HashMap;
use docforge_engine::{
    render::font_cache::{FontCache, FontStyleKind},
    render::pdf_renderer::{render_to_pdf, PdfRenderConfig},
    layout::LayoutEngine,
    model::TemplateDocument,
};
use docforge_orbis::{
    datasource::DataSourceRegistry,
    template_service::TemplateService,
};

//// ── Periods (was Periods) ────────────────────────────────────────────

#[tauri::command]
pub async fn list_periods(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = org_ops::academic_period::list_hierarchy(&ctx).await.map_err(CommandError::from)?;
    Ok(result)
}

#[tauri::command]
pub async fn list_periods_by_year(
    state: State<'_, Arc<AppState>>,
    period_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = org_ops::academic_period::list_by_year(&ctx, &period_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_period(
    state: State<'_, Arc<AppState>>,
    period_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = org_ops::academic_period::get(&ctx, &period_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_period(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = org_ops::academic_period::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_period(
    state: State<'_, Arc<AppState>>,
    period_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = org_ops::academic_period::update(&ctx, &period_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_period(
    state: State<'_, Arc<AppState>>,
    period_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    org_ops::academic_period::delete(&ctx, &period_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn set_current_period(
    _state: State<'_, Arc<AppState>>,
    _period_id: String,
) -> CmdResult<()> {
    // set_current period is deprecated.
    // Current year is now set via organizations.current_period_id.
    Err(CommandError::new("Deprecated: set active year via switch_academic_year command."))
}

#[tauri::command]
pub async fn lock_period(
    state: State<'_, Arc<AppState>>,
    period_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    org_ops::academic_period::lock(&ctx, &period_id).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn unlock_period(
    state: State<'_, Arc<AppState>>,
    period_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    org_ops::academic_period::unlock(&ctx, &period_id).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn toggle_period_enrollment(
    state: State<'_, Arc<AppState>>,
    period_id: String,
    open: bool,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    org_ops::academic_period::toggle_enrollment(&ctx, &period_id, open).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn toggle_period_grade_entry(
    state: State<'_, Arc<AppState>>,
    period_id: String,
    open: bool,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    org_ops::academic_period::toggle_grade_entry(&ctx, &period_id, open).await.map(|_| ()).map_err(CommandError::from)
}

// ── Classes ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_classes(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::class::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_class(
    state: State<'_, Arc<AppState>>,
    class_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _class = svc_academic::entities::class::Entity::find_by_id(class_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Class not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::class::get(&ctx, &class_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_class(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::class::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_class(
    state: State<'_, Arc<AppState>>,
    class_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _class = svc_academic::entities::class::Entity::find_by_id(class_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Class not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::class::update(&ctx, &class_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_class(
    state: State<'_, Arc<AppState>>,
    class_id: String,
) -> CmdResult<()> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _class = svc_academic::entities::class::Entity::find_by_id(class_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Class not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::class::delete(&ctx, &class_id).await.map_err(CommandError::from)
}

// ── Subjects ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_subjects(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::subject::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_subject(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::subject::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_subject(
    state: State<'_, Arc<AppState>>,
    subject_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _subject = svc_academic::entities::subject::Entity::find_by_id(subject_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Subject not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::subject::get(&ctx, &subject_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_subject(
    state: State<'_, Arc<AppState>>,
    subject_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _subject = svc_academic::entities::subject::Entity::find_by_id(subject_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Subject not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::subject::update(&ctx, &subject_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_subject(
    state: State<'_, Arc<AppState>>,
    subject_id: String,
) -> CmdResult<()> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _subject = svc_academic::entities::subject::Entity::find_by_id(subject_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Subject not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::subject::delete(&ctx, &subject_id).await.map_err(CommandError::from)
}

// ── Enrollments ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_enrollments(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::enrollment::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn list_enrollments_paginated(
    state: State<'_, Arc<AppState>>,
    cursor: Option<String>,
    limit: Option<u64>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let request = shared_kernel::pagination::CursorRequest {
        cursor,
        limit: limit.unwrap_or(50),
    };
    let result = ops::enrollment::list_paginated(&ctx, request).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::enrollment::get(&ctx, &enrollment_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_enrollment(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::enrollment::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn transfer_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
    new_class_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::enrollment::transfer(&ctx, &enrollment_id, &new_class_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn withdraw_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::enrollment::withdraw(&ctx, &enrollment_id).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn check_duplicate_enrollment(
    state: State<'_, Arc<AppState>>,
    national_id: Option<String>,
    email: Option<String>,
    phone: Option<String>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::enrollment::check_duplicate(&ctx, national_id, email, phone).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

// ── Period Gate Commands ────────────────────────────────────────────

#[tauri::command]
pub async fn check_enrollment_eligibility(
    state: State<'_, Arc<AppState>>,
    period_id: String,
    class_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let verdict = ops::enrollment::check_eligibility(&ctx, &period_id, &class_id)
        .await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(verdict).unwrap())
}

#[tauri::command]
pub async fn confirm_enrollment_draft(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::enrollment::confirm_draft(&ctx, &enrollment_id)
        .await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn check_period_gate(
    state: State<'_, Arc<AppState>>,
    gate_type: String,
    period_id: String,
) -> CmdResult<serde_json::Value> {
    use shared_kernel::period_gate::GateType;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;

    let gate = match gate_type.as_str() {
        "enrollment" => GateType::Enrollment,
        "grade_entry" => GateType::GradeEntry,
        "attendance" => GateType::Attendance,
        "exam_entry" => GateType::ExamEntry,
        "assessment" => GateType::Assessment,
        _ => return Err(CommandError::from(
            shared_kernel::errors::AppError::validation(format!("Unknown gate type: {}", gate_type))
        )),
    };

    let verdict = ctx.period_gate
        .check(&ctx.db, &ctx.org_id, &period_id, gate)
        .await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(verdict).unwrap())
}

// ── Class Subjects ──────────────────────────────────────────────────

#[tauri::command]
pub async fn list_class_subjects(
    state: State<'_, Arc<AppState>>,
    class_id: String,
    period_id: Option<String>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::class_subject::list(&ctx, &class_id, period_id.as_deref()).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_class_subject(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::class_subject::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_class_subject(
    state: State<'_, Arc<AppState>>,
    class_subject_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::class_subject::update(&ctx, &class_subject_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_class_subject(
    state: State<'_, Arc<AppState>>,
    class_subject_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::class_subject::delete(&ctx, &class_subject_id).await.map_err(CommandError::from)
}

// ── Guardians ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_guardians_by_student(
    state: State<'_, Arc<AppState>>,
    student_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::guardian::list_by_student(&ctx, &student_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn search_guardians(
    state: State<'_, Arc<AppState>>,
    query: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::guardian::search(&ctx, &query).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_guardian_link(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::guardian::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_guardian_link(
    state: State<'_, Arc<AppState>>,
    link_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::guardian::update(&ctx, &link_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_guardian_link(
    state: State<'_, Arc<AppState>>,
    link_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::guardian::delete(&ctx, &link_id).await.map_err(CommandError::from)
}



// ── Documents ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_documents(
    state: State<'_, Arc<AppState>>,
    owner_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::document::list(&ctx, &owner_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_document(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::document::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_document(
    state: State<'_, Arc<AppState>>,
    document_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::document::delete(&ctx, &document_id).await.map_err(CommandError::from)
}

// ── Notifications ───────────────────────────────────────────────────

#[tauri::command]
pub async fn list_notifications(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::notification::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn mark_notification_read(
    state: State<'_, Arc<AppState>>,
    notification_id: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::notification::mark_read(&ctx, &notification_id).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn assign_teacher(
    state: State<'_, Arc<AppState>>,
    class_id: String,
    teacher_id: String,
) -> CmdResult<()> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _class = svc_academic::entities::class::Entity::find_by_id(class_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Class not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::class::assign_teacher(&ctx, &class_id, &teacher_id).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_teacher_subjects(
    state: State<'_, Arc<AppState>>,
    teacher_id: String,
    period_id: Option<String>,
) -> CmdResult<serde_json::Value> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    
    let resolved_period_id = match period_id {
        Some(cid) => cid,
        None => {
            let current = org_ops::academic_period::get_current(&ctx).await.map_err(CommandError::from)?;
            current.id
        }
    };
    
    let result = ops::class_subject::list_by_teacher(&ctx, &teacher_id, &resolved_period_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_current_period(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = org_ops::academic_period::get_current(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn toggle_grade_entry(
    state: State<'_, Arc<AppState>>,
    period_id: String,
    open: bool,
    _start: Option<String>,
    _end: Option<String>,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _period = svc_organization::entities::academic_period::Entity::find_by_id(period_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Period not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = org_ops::academic_period::toggle_grade_entry(&ctx, &period_id, open).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

/// Previews a document template from raw editor JSON content and custom data context.
#[tauri::command]
pub async fn preview_document_template(
    _state: State<'_, Arc<AppState>>,
    template_json: String,
    context_json: String,
) -> CmdResult<Vec<u8>> {
    // 1. Parse template JSON
    let doc: TemplateDocument = serde_json::from_str(&template_json)
        .map_err(|e| CommandError::new(format!("Failed to parse template JSON: {}", e)))?;

    // 2. Parse context JSON
    let context = serde_json::from_str(&context_json)
        .map_err(|e| CommandError::new(format!("Failed to parse context JSON: {}", e)))?;

    // 3. Setup font cache and bundle/load standard system fonts
    let mut fonts = FontCache::new("Rubik");
    let font_path = "C:\\Windows\\Fonts\\tahoma.ttf";
    if std::path::Path::new(font_path).exists() {
        fonts.load_font_file("Tahoma", 400, FontStyleKind::Normal, font_path).ok();
        fonts.load_font_file("Rubik", 400, FontStyleKind::Normal, font_path).ok();
    }
    
    // 4. Compute layout
    let engine = LayoutEngine::new(&fonts);
    let layout_result = engine.layout(&doc, &context)
        .map_err(|e| CommandError::new(format!("Layout calculation failed: {:?}", e)))?;

    // 5. Render to PDF
    let pdf_output = render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default())
        .map_err(|e| CommandError::new(format!("PDF rendering failed: {:?}", e)))?;

    Ok(pdf_output.bytes)
}

/// Renders a template from database using resolved real entity database records.
#[tauri::command]
pub async fn render_document(
    state: State<'_, Arc<AppState>>,
    template_id: String,
    entity_id: String,
) -> CmdResult<Vec<u8>> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let db = state.db();

    // 1. Fetch template record from DB
    let template = TemplateService::get_by_id(&db, &ctx.org_id, &template_id)
        .await
        .map_err(|e| CommandError::new(format!("Failed to fetch template: {}", e)))?;

    // 2. Parse template JSON
    let doc: TemplateDocument = serde_json::from_str(&template.document_json)
        .map_err(|e| CommandError::new(format!("Failed to parse template JSON: {}", e)))?;

    // 3. Resolve data context from appropriate DataSource
    let registry = DataSourceRegistry::new();
    let data_source = registry.get(&template.entity_type)
        .ok_or_else(|| CommandError::new(format!("Unsupported entity type data source: {}", template.entity_type)))?;

    let context = data_source.resolve(&db, &entity_id)
        .await
        .map_err(|e| CommandError::new(format!("Failed to resolve data source: {}", e)))?;

    // 4. Setup font cache
    let mut fonts = FontCache::new("Rubik");
    let font_path = "C:\\Windows\\Fonts\\tahoma.ttf";
    if std::path::Path::new(font_path).exists() {
        fonts.load_font_file("Tahoma", 400, FontStyleKind::Normal, font_path).ok();
        fonts.load_font_file("Rubik", 400, FontStyleKind::Normal, font_path).ok();
    }

    // 5. Compute layout
    let engine = LayoutEngine::new(&fonts);
    let layout_result = engine.layout(&doc, &context)
        .map_err(|e| CommandError::new(format!("Layout calculation failed: {:?}", e)))?;

    // 6. Render to PDF
    let pdf_output = render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default())
        .map_err(|e| CommandError::new(format!("PDF rendering failed: {:?}", e)))?;

    Ok(pdf_output.bytes)
}

/// Returns dynamic variables schemas for editor picker auto-completion.
#[tauri::command]
pub async fn get_datasource_schemas() -> CmdResult<HashMap<String, serde_json::Value>> {
    let registry = DataSourceRegistry::new();
    Ok(registry.list_schemas())
}

/// Evaluates a template string containing expressions against a JSON context.
#[tauri::command]
pub async fn evaluate_template_expressions(
    expression_template: String,
    context_json: String,
) -> CmdResult<String> {
    let context = serde_json::from_str(&context_json)
        .map_err(|e| CommandError::new(format!("Failed to parse context JSON: {}", e)))?;
    let val = docforge_engine::expr::evaluator::evaluate_expression(&expression_template, &context);
    Ok(val)
}

/// Starts a background batch printing job for multiple entity IDs using a template.
#[tauri::command]
pub async fn start_batch_print(
    state: State<'_, Arc<AppState>>,
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    app_handle: tauri::AppHandle,
    template_id: String,
    entity_ids: Vec<String>,
    output_dir: String,
) -> CmdResult<docforge_orbis::batch::BatchJobStatus> {
    // Generate a unique job ID
    let job_id = uuid::Uuid::new_v4().to_string();
    
    // Create the job status
    let job = batch_manager.create_job(job_id, entity_ids.len(), output_dir.clone());
    
    // Spawn the background print orchestrator
    let db = state.db();
    let job_clone = job.clone();
    
    let app_handle_clone = app_handle.clone();
    let callback = Arc::new(move |status| {
        use tauri::Emitter;
        let _ = app_handle_clone.emit("batch-job-progress", status);
    });

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    tokio::spawn(async move {
        docforge_orbis::batch::start_batch_print_job(
            callback,
            job_clone,
            db,
            ctx.org_id,
            template_id,
            entity_ids,
            output_dir,
        ).await;
    });

    let status = job.status.lock().unwrap().clone();
    Ok(status)
}

/// Pauses a batch print job.
#[tauri::command]
pub async fn pause_batch_print(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<bool> {
    Ok(batch_manager.pause_job(&job_id))
}

/// Resumes a paused batch print job.
#[tauri::command]
pub async fn resume_batch_print(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<bool> {
    Ok(batch_manager.resume_job(&job_id))
}

/// Cancels a running or paused batch print job.
#[tauri::command]
pub async fn cancel_batch_print(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<bool> {
    Ok(batch_manager.cancel_job(&job_id))
}

/// Returns the current progress or error status of a batch print job.
#[tauri::command]
pub async fn get_batch_print_status(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<docforge_orbis::batch::BatchJobStatus> {
    batch_manager.get_status(&job_id)
        .ok_or_else(|| CommandError::new(format!("Job not found: {}", job_id)))
}

/// Compute optimal multi-up layout for given config.
/// Returns layout info (columns, rows, items_per_sheet, slot positions).
/// Used by the frontend for instant WYSIWYG preview.
#[tauri::command]
pub async fn compute_multi_up_layout(
    config_json: String,
) -> CmdResult<serde_json::Value> {
    let config: docforge_engine::layout::multi_up::MultiUpConfig =
        serde_json::from_str(&config_json)
            .map_err(|e| CommandError::new(format!("Invalid multi-up config: {}", e)))?;

    let layout = config.compute_layout();
    let result = serde_json::to_value(&layout)
        .map_err(|e| CommandError::new(format!("Serialization error: {}", e)))?;

    Ok(result)
}

/// Generate a multi-up preview PDF containing only the first sheet.
/// Uses real entity data for WYSIWYG accuracy.
#[tauri::command]
pub async fn preview_multi_up(
    state: State<'_, Arc<AppState>>,
    template_id: String,
    entity_ids: Vec<String>,
    config_json: String,
) -> CmdResult<Vec<u8>> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let db = state.db();

    let config: docforge_engine::layout::multi_up::MultiUpConfig =
        serde_json::from_str(&config_json)
            .map_err(|e| CommandError::new(format!("Invalid multi-up config: {}", e)))?;

    let pdf_bytes = docforge_orbis::batch_multi_up::preview_multi_up(
        &db, &ctx.org_id, &template_id, &entity_ids, &config,
    )
    .await
    .map_err(|e| CommandError::new(e))?;

    Ok(pdf_bytes)
}

/// Start a background multi-up batch print job.
/// Emits "multi-up-batch-progress" events for frontend progress tracking.
#[tauri::command]
pub async fn start_multi_up_batch_print(
    state: State<'_, Arc<AppState>>,
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    app_handle: tauri::AppHandle,
    template_id: String,
    entity_ids: Vec<String>,
    config_json: String,
    output_dir: String,
) -> CmdResult<docforge_orbis::batch::BatchJobStatus> {
    let config: docforge_engine::layout::multi_up::MultiUpConfig =
        serde_json::from_str(&config_json)
            .map_err(|e| CommandError::new(format!("Invalid multi-up config: {}", e)))?;

    // Generate a unique job ID
    let job_id = uuid::Uuid::new_v4().to_string();

    // Create the job status
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let job = batch_manager.create_job(job_id.clone(), entity_ids.len(), output_dir.clone());

    // Build the multi-up job config
    let multi_up_config = docforge_orbis::batch_multi_up::MultiUpPrintJobConfig {
        job_id: job_id.clone(),
        template_id,
        org_id: ctx.org_id.clone(),
        entity_ids,
        multi_up: config,
        output_mode: docforge_orbis::batch_multi_up::MultiUpOutputMode::MergedPdf,
        output_path: output_dir,
    };

    // Progress callback emits Tauri events
    let app_handle_clone = app_handle.clone();
    let callback = Arc::new(move |status: docforge_orbis::batch::BatchJobStatus| {
        use tauri::Emitter;
        let _ = app_handle_clone.emit("multi-up-batch-progress", &status);
    });

    // Spawn the background orchestrator
    let db = state.db();
    let job_clone = job.clone();

    tokio::spawn(async move {
        docforge_orbis::batch_multi_up::start_multi_up_print_job(
            multi_up_config,
            job_clone,
            db,
            callback,
        ).await;
    });

    let status = job.status.lock().unwrap().clone();
    Ok(status)
}





