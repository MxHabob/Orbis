use tauri::State;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use core_domain::{
    entities::academic_year,
    repository::academic_year::AcademicYearRepo,
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

/// Response type for academic year list.
#[derive(Debug, Serialize, Deserialize)]
pub struct AcademicYearInfo {
    #[serde(flatten)]
    pub year: academic_year::Model,
    pub is_active_in_app: bool,
}

/// Request to create a new academic year.
#[derive(Debug, Deserialize)]
pub struct CreateAcademicYearRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    pub notes: Option<String>,
}

/// List all academic years for the organization.
#[tauri::command]
pub async fn list_academic_years(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<AcademicYearInfo>> {
    let registry_db = state.provider.registry();
    let current_year_id = state.provider.current_year_id().await;

    let years = AcademicYearRepo::get_all(registry_db, &org_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    let infos = years
        .into_iter()
        .map(|y| {
            let is_active = y.id == current_year_id;
            AcademicYearInfo {
                year: y,
                is_active_in_app: is_active,
            }
        })
        .collect();

    Ok(infos)
}

/// Get the currently active academic year.
#[tauri::command]
pub async fn get_current_academic_year(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Option<academic_year::Model>> {
    let year_id = state.provider.current_year_id().await;
    if year_id.is_empty() {
        return Ok(None);
    }

    let registry_db = state.provider.registry();
    AcademicYearRepo::get_by_id(registry_db, &year_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

/// Create a new academic year.
/// This creates a fresh database file with the full schema.
#[tauri::command]
pub async fn create_academic_year(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateAcademicYearRequest,
) -> CmdResult<academic_year::Model> {
    let registry_db = state.provider.registry();

    // Validate: no duplicate year names
    let exists = AcademicYearRepo::name_exists(registry_db, &org_id, &request.name)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;
    if exists {
        return Err(CommandError::from(format!(
            "Academic year '{}' already exists",
            request.name
        )));
    }

    // Generate database filename
    let db_filename =
        core_domain::database::registry::year_db_filename(&request.name);

    // Create the year database file with schema
    let _db = state
        .provider
        .create_year_db(&db_filename)
        .await
        .map_err(CommandError::from)?;

    // Register in the registry
    let now = chrono::Utc::now().to_rfc3339();
    let year = academic_year::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id: org_id.clone(),
        name: request.name,
        name_ar: request.name_ar,
        start_date: request.start_date,
        end_date: request.end_date,
        hijri_start_date: request.hijri_start_date,
        hijri_end_date: request.hijri_end_date,
        db_filename,
        status: "active".to_string(),
        is_current: 0,
        archived_at: None,
        archived_by: None,
        promotion_source_year_id: None,
        db_size_bytes: None,
        student_count: Some(0),
        teacher_count: Some(0),
        class_count: Some(0),
        notes: request.notes,
        created_at: now.clone(),
        updated_at: now,
    };

    AcademicYearRepo::create(registry_db, year)
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

/// Switch to a different academic year.
/// This changes the active database connection for all subsequent commands.
#[tauri::command]
pub async fn switch_academic_year(
    state: State<'_, Arc<AppState>>,
    year_id: String,
) -> CmdResult<()> {
    state
        .provider
        .switch_year(&year_id)
        .await
        .map_err(CommandError::from)
}

/// Archive an academic year (make it read-only).
#[tauri::command]
pub async fn archive_academic_year(
    state: State<'_, Arc<AppState>>,
    year_id: String,
    user_id: String,
) -> CmdResult<academic_year::Model> {
    let registry_db = state.provider.registry();

    // Cannot archive the current active year
    let year = AcademicYearRepo::get_by_id(registry_db, &year_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?
        .ok_or_else(|| CommandError::from("Academic year not found".to_string()))?;

    if year.is_current == 1 {
        return Err(CommandError::from(
            "Cannot archive the currently active year. Switch to another year first.".to_string(),
        ));
    }

    AcademicYearRepo::update_status(registry_db, &year_id, "archived", Some(&user_id))
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

/// Check if the current year is archived (read-only mode).
#[tauri::command]
pub async fn is_current_year_archived(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<bool> {
    Ok(state.provider.is_current_archived().await)
}

/// Set a year as the current active year in the registry.
/// This does NOT switch the database connection (use switch_academic_year for that).
#[tauri::command]
pub async fn set_active_academic_year(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    year_id: String,
) -> CmdResult<()> {
    let registry_db = state.provider.registry();

    // Verify the year exists and is active (not archived)
    let year = AcademicYearRepo::get_by_id(registry_db, &year_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?
        .ok_or_else(|| CommandError::from("Academic year not found".to_string()))?;

    if year.status == "archived" {
        return Err(CommandError::from(
            "Cannot set an archived year as the active year".to_string(),
        ));
    }

    AcademicYearRepo::set_current(registry_db, &org_id, &year_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    // Also switch the database connection
    state
        .provider
        .switch_year(&year_id)
        .await
        .map_err(CommandError::from)
}
