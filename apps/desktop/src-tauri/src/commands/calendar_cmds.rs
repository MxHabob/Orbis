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
    pub id: String, pub org_id: String, pub code: String,
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
    AcademicYearDto {
        id: y.id.to_string(), org_id: y.org_id.to_string(), code: y.code,
        name: y.name, name_ar: y.name_ar,
        start_date: y.start_date.to_rfc3339(), end_date: y.end_date.to_rfc3339(),
        is_active: y.is_active, status: y.status,
    }
}

fn period_dto(p: orbis_domain::entities::academic_period::AcademicPeriod) -> AcademicPeriodDto {
    AcademicPeriodDto {
        id: p.id.to_string(), year_id: p.year_id.to_string(), name: p.name,
        start_date: p.start_date.to_rfc3339(), end_date: p.end_date.to_rfc3339(),
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
pub async fn calendar_get_context(state: State<'_, AppState>) -> Result<ApiResponse<AcademicContextResponse>, String> {
    let ctx = make_ctx(&state);
    let all = state.year_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    let current = state.year_repo.find_current(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    let periods = state.period_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(AcademicContextResponse {
        current_year: current.map(year_dto),
        all_years: all.into_iter().map(year_dto).collect(),
        periods: periods.into_iter().map(period_dto).collect(),
    }))
}

#[tauri::command]
pub async fn calendar_get_year(id: String, state: State<'_, AppState>) -> Result<ApiResponse<AcademicYearDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let y = state.year_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Year not found")?;
    Ok(ok(year_dto(y)))
}

#[tauri::command]
pub async fn calendar_list_periods(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AcademicPeriodDto>>, String> {
    let ctx = make_ctx(&state);
    let periods = state.period_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(periods.into_iter().map(period_dto).collect()))
}

#[tauri::command]
pub async fn calendar_get_period(id: String, state: State<'_, AppState>) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let p = state.period_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Period not found")?;
    Ok(ok(period_dto(p)))
}

#[tauri::command]
pub async fn calendar_list_period_templates(state: State<'_, AppState>) -> Result<ApiResponse<Vec<PeriodTemplateDto>>, String> {
    let ctx = make_ctx(&state);
    let templates = state.period_template_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(templates.into_iter().map(|t| PeriodTemplateDto { id: t.id.to_string(), name: t.name }).collect()))
}
