use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Response Types ──────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AcademicCycleSummary {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub cycle_type: String,
    pub start_date: String,
    pub end_date: String,
    pub is_current: bool,
    pub is_locked: bool,
    pub phase: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AcademicCycleContext {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub cycle_type: String,
    pub start_date: String,
    pub end_date: String,
    pub is_enrollment_open: bool,
    pub is_grade_entry_open: bool,
    pub is_locked: bool,
    pub phase: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AcademicContextResponse {
    pub year_id: String,
    pub year_name: String,
    pub year_name_ar: Option<String>,
    pub year_status: String,
    pub is_read_only: bool,
    pub active_cycle: Option<AcademicCycleContext>,
    pub all_cycles: Vec<AcademicCycleSummary>,
}

// ── Helper to determine cycle phase ──────────────────────────────────────────

fn determine_phase(cycle: &domain_lifecycle::entities::academic_cycle::Model) -> String {
    if cycle.is_locked == 1 {
        return "locked".to_string();
    }
    if cycle.is_grade_entry_open == 1 {
        return "grading".to_string();
    }
    
    let today = chrono::Local::now().date_naive().to_string();
    
    if let (Some(start), Some(end)) = (&cycle.exam_start_date, &cycle.exam_end_date) {
        if today >= *start && today <= *end {
            return "exams".to_string();
        }
    }
    
    if today >= cycle.start_date && today <= cycle.end_date {
        return "classes".to_string();
    }
    
    if cycle.is_enrollment_open == 1 {
        return "enrollment".to_string();
    }
    
    "planning".to_string()
}

// ── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_academic_context(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<AcademicContextResponse> {
    let year_id = state.provider.current_year_id().await;
    let is_read_only = state.provider.is_current_archived().await;
    let registry_db = state.provider.registry();
    let db = state.active_db().await;

    // 1. Fetch current year info from registry DB
    let year = if !year_id.is_empty() {
        domain_lifecycle::repository::academic_year::AcademicYearRepo::get_by_id(registry_db, &year_id)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?
    } else {
        None
    };

    let (year_name, year_name_ar, year_status) = match &year {
        Some(y) => (y.name.clone(), y.name_ar.clone(), y.status.clone()),
        None => ("Default Year".to_string(), None, "active".to_string()),
    };

    // 2. Fetch all cycles in the active year database
    let cycles = domain_lifecycle::entities::academic_cycle::Entity::find()
        .filter(domain_lifecycle::entities::academic_cycle::Column::OrgId.eq(&org_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    // 3. Resolve active cycle (is_current = 1, or fall back to the first active cycle)
    let active_cycle_model = cycles.iter()
        .find(|c| c.is_current == 1)
        .cloned();

    let active_cycle = active_cycle_model.map(|c| {
        let phase = determine_phase(&c);
        AcademicCycleContext {
            id: c.id,
            name: c.name,
            name_ar: c.name_ar,
            cycle_type: c.cycle_type,
            start_date: c.start_date,
            end_date: c.end_date,
            is_enrollment_open: c.is_enrollment_open == 1,
            is_grade_entry_open: c.is_grade_entry_open == 1,
            is_locked: c.is_locked == 1,
            phase,
        }
    });

    let all_cycles = cycles.into_iter().map(|c| {
        let phase = determine_phase(&c);
        AcademicCycleSummary {
            id: c.id,
            name: c.name,
            name_ar: c.name_ar,
            cycle_type: c.cycle_type,
            start_date: c.start_date,
            end_date: c.end_date,
            is_current: c.is_current == 1,
            is_locked: c.is_locked == 1,
            phase,
        }
    }).collect();

    Ok(AcademicContextResponse {
        year_id,
        year_name,
        year_name_ar,
        year_status,
        is_read_only,
        active_cycle,
        all_cycles,
    })
}

#[tauri::command]
pub async fn resolve_active_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Option<serde_json::Value>> {
    let db = state.active_db().await;

    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find()
        .filter(domain_lifecycle::entities::academic_cycle::Column::OrgId.eq(&org_id))
        .filter(domain_lifecycle::entities::academic_cycle::Column::IsCurrent.eq(1))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    match cycle {
        Some(c) => {
            let phase = determine_phase(&c);
            let ctx = AcademicCycleContext {
                id: c.id,
                name: c.name,
                name_ar: c.name_ar,
                cycle_type: c.cycle_type,
                start_date: c.start_date,
                end_date: c.end_date,
                is_enrollment_open: c.is_enrollment_open == 1,
                is_grade_entry_open: c.is_grade_entry_open == 1,
                is_locked: c.is_locked == 1,
                phase,
            };
            let val = serde_json::to_value(&ctx).map_err(|e| CommandError::new(e.to_string()))?;
            Ok(Some(val))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn is_year_writable(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<bool> {
    Ok(!state.provider.is_current_archived().await)
}
