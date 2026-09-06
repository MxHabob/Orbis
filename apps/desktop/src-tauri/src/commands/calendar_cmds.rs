//! Tauri IPC commands — Calendar (Academic Years, Periods, Templates).

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct AcademicYearDto {
    pub id: String, pub org_id: String,
    pub name: String, pub name_ar: Option<String>,
    pub start_date: String, pub end_date: String,
    pub is_active: bool, pub status: String,
}

#[derive(Serialize)]
pub struct AcademicPeriodDto {
    pub id: String, pub year_id: String, pub name: String,
    pub start_date: String, pub end_date: String,
    pub period_type: String, pub status: String,
}

#[derive(Serialize)]
pub struct AcademicContextResponse {
    pub current_year: Option<AcademicYearDto>,
    pub all_years: Vec<AcademicYearDto>,
    pub periods: Vec<AcademicPeriodDto>,
}

#[derive(Serialize)]
pub struct PeriodTemplateDto { pub id: String, pub name: String }

fn year_dto(y: orbis_domain::entities::academic_year::AcademicYear) -> AcademicYearDto {
    let is_active = y.status == "active";
    AcademicYearDto {
        id: y.id.to_string(), org_id: y.org_id.to_string(),
        name: y.name, name_ar: y.name_ar,
        start_date: y.start_date.to_string(), end_date: y.end_date.to_string(),
        is_active, status: y.status,
    }
}

fn period_dto(p: orbis_domain::entities::academic_period::AcademicPeriod) -> AcademicPeriodDto {
    AcademicPeriodDto {
        id: p.id.to_string(), year_id: p.year_id.to_string(), name: p.name,
        start_date: p.start_date.to_string(), end_date: p.end_date.to_string(),
        period_type: p.period_type, status: p.status,
    }
}

#[tauri::command]
pub async fn calendar_list_years(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AcademicYearDto>>, String> {
    let ctx = make_ctx(&state);
    let years = state.year_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(years.into_iter().map(year_dto).collect()))
}

#[tauri::command]
pub async fn calendar_get_year(id: String, state: State<'_, AppState>) -> Result<ApiResponse<Option<AcademicYearDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let year = state.year_repo.find_by_id(uid).await.map_err(|e| e.to_string())?;
    Ok(ok(year.map(year_dto)))
}

#[tauri::command]
pub async fn calendar_list_periods(year_id: Option<String>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<AcademicPeriodDto>>, String> {
    let ctx = make_ctx(&state);
    let periods = if let Some(yid) = year_id.and_then(|s| Uuid::parse_str(&s).ok()) {
        state.period_repo.find_by_year(yid).await
    } else {
        state.period_repo.find_by_org(ctx.tenant_id).await
    }.map_err(|e| e.to_string())?;
    Ok(ok(periods.into_iter().map(period_dto).collect()))
}

#[tauri::command]
pub async fn calendar_get_period(id: String, state: State<'_, AppState>) -> Result<ApiResponse<Option<AcademicPeriodDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state.period_repo.find_by_id(uid).await.map_err(|e| e.to_string())?;
    Ok(ok(period.map(period_dto)))
}

#[tauri::command]
pub async fn calendar_get_context(state: State<'_, AppState>) -> Result<ApiResponse<AcademicContextResponse>, String> {
    let ctx = make_ctx(&state);
    let years = state.year_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    let current = state.year_repo.find_active(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    let periods = if let Some(ref cur) = current {
        state.period_repo.find_by_year(cur.id).await.map_err(|e| e.to_string())?
    } else { vec![] };
    Ok(ok(AcademicContextResponse {
        current_year: current.map(year_dto),
        all_years: years.into_iter().map(year_dto).collect(),
        periods: periods.into_iter().map(period_dto).collect(),
    }))
}

#[tauri::command]
pub async fn calendar_list_period_templates() -> Result<ApiResponse<Vec<PeriodTemplateDto>>, String> {
    Ok(ok(vec![
        PeriodTemplateDto { id: "semesters".to_string(), name: "Two Semesters (Fall/Spring)".to_string() },
        PeriodTemplateDto { id: "trimesters".to_string(), name: "Three Trimesters".to_string() },
        PeriodTemplateDto { id: "quarters".to_string(), name: "Four Quarters".to_string() },
    ]))
}
