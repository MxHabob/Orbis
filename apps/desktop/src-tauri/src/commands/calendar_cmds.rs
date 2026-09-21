//! Tauri IPC commands — Calendar (Academic Years, Periods, Templates).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct AcademicYearDto {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateYearInput {
    pub name: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}

#[derive(Serialize)]
pub struct AcademicPeriodDto {
    pub id: String,
    pub year_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub period_type: String,
    pub status: String,
    pub is_locked: bool,
    pub is_enrollment_open: bool,
    pub is_grade_entry_open: bool,
}

#[derive(Deserialize)]
pub struct CreatePeriodInput {
    #[serde(rename = "yearId")]
    pub year_id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub name: String,
    #[serde(rename = "periodType")]
    pub period_type: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}

#[derive(Serialize)]
pub struct AcademicContextResponse {
    pub current_year: Option<AcademicYearDto>,
    pub all_years: Vec<AcademicYearDto>,
    pub periods: Vec<AcademicPeriodDto>,
}

#[derive(Serialize)]
pub struct PeriodTemplateDto {
    pub id: String,
    pub name: String,
}

fn year_dto(y: orbis_domain::entities::academic_year::AcademicYear) -> AcademicYearDto {
    let is_active = y.status == "active";
    AcademicYearDto {
        id: y.id.to_string(),
        org_id: y.org_id.to_string(),
        name: y.name,
        name_ar: y.name_ar,
        start_date: y.start_date.to_rfc3339(),
        end_date: y.end_date.to_rfc3339(),
        is_active,
        status: y.status,
    }
}

fn period_dto(p: orbis_domain::entities::academic_period::AcademicPeriod) -> AcademicPeriodDto {
    let status_str = match p.state {
        orbis_domain::entities::academic_period::AcademicPeriodState::Closed => "closed",
        orbis_domain::entities::academic_period::AcademicPeriodState::Archived => "archived",
        _ => if p.is_locked { "locked" } else { "active" },
    };
    AcademicPeriodDto {
        id: p.id.to_string(),
        year_id: p.year_id.to_string(),
        parent_id: p.parent_id.map(|u| u.to_string()),
        name: p.name,
        name_ar: p.name_ar,
        code: p.code,
        start_date: p.start_date.to_rfc3339(),
        end_date: p.end_date.to_rfc3339(),
        period_type: p.period_type,
        status: status_str.to_string(),
        is_locked: p.is_locked,
        is_enrollment_open: p.is_enrollment_open,
        is_grade_entry_open: p.is_grade_entry_open,
    }
}

#[tauri::command]
pub async fn calendar_create_year(request: CreateYearInput, state: State<'_, AppState>) -> Result<ApiResponse<AcademicYearDto>, String> {
    let ctx = make_ctx(&state);
    let start_date = DateTime::parse_from_rfc3339(&request.start_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid start_date format")?;
    let end_date = DateTime::parse_from_rfc3339(&request.end_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid end_date format")?;

    let y = state.calendar_service.create_year(&ctx, ctx.tenant_id, request.name, start_date, end_date)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(year_dto(y)))
}

#[tauri::command]
pub async fn calendar_list_years(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AcademicYearDto>>, String> {
    let ctx = make_ctx(&state);
    let years = state.calendar_service.list_years(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(years.into_iter().map(year_dto).collect()))
}

#[tauri::command]
pub async fn calendar_get_year(id: String, state: State<'_, AppState>) -> Result<ApiResponse<Option<AcademicYearDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let year = state.calendar_service.get_year(uid).await.map_err(|e| e.to_string())?;
    Ok(ok(year.map(year_dto)))
}

#[tauri::command]
pub async fn calendar_create_period(request: CreatePeriodInput, state: State<'_, AppState>) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let year_id = Uuid::parse_str(&request.year_id).map_err(|_| "Invalid yearId")?;
    let parent_id = request.parent_id.and_then(|p| Uuid::parse_str(&p).ok());
    let start_date = DateTime::parse_from_rfc3339(&request.start_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid start_date format")?;
    let end_date = DateTime::parse_from_rfc3339(&request.end_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid end_date format")?;

    let p = state.calendar_service.create_period(&ctx, ctx.tenant_id, year_id, parent_id, request.name, request.period_type, start_date, end_date)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(period_dto(p)))
}

#[tauri::command]
pub async fn calendar_list_periods(year_id: Option<String>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<AcademicPeriodDto>>, String> {
    let ctx = make_ctx(&state);
    let periods = if let Some(yid) = year_id.and_then(|s| Uuid::parse_str(&s).ok()) {
        state.calendar_service.list_periods_by_year(yid).await
    } else {
        state.calendar_service.list_periods_by_org(ctx.tenant_id).await
    }.map_err(|e| e.to_string())?;
    Ok(ok(periods.into_iter().map(period_dto).collect()))
}

#[tauri::command]
pub async fn calendar_get_period(id: String, state: State<'_, AppState>) -> Result<ApiResponse<Option<AcademicPeriodDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state.calendar_service.get_period(uid).await.map_err(|e| e.to_string())?;
    Ok(ok(period.map(period_dto)))
}

#[tauri::command]
pub async fn calendar_get_context(state: State<'_, AppState>) -> Result<ApiResponse<AcademicContextResponse>, String> {
    let ctx = make_ctx(&state);
    let years = state.calendar_service.list_years(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    let current = years.iter().find(|y| y.status == "active").cloned();
    let periods = if let Some(ref cur) = current {
        state.calendar_service.list_periods_by_year(cur.id).await.map_err(|e| e.to_string())?
    } else { vec![] };
    Ok(ok(AcademicContextResponse {
        current_year: current.map(year_dto),
        all_years: years.into_iter().map(year_dto).collect(),
        periods: periods.into_iter().map(period_dto).collect(),
    }))
}

#[tauri::command]
pub async fn calendar_lock_period(id: String, state: State<'_, AppState>) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state.calendar_service.lock_period(&ctx, uid).await.map_err(|e| e.to_string())?;
    Ok(ok(period_dto(period)))
}

#[tauri::command]
pub async fn calendar_unlock_period(id: String, state: State<'_, AppState>) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state.calendar_service.unlock_period(&ctx, uid).await.map_err(|e| e.to_string())?;
    Ok(ok(period_dto(period)))
}

#[tauri::command]
pub async fn calendar_close_period(id: String, state: State<'_, AppState>) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state.calendar_service.close_period(&ctx, uid).await.map_err(|e| e.to_string())?;
    Ok(ok(period_dto(period)))
}

#[tauri::command]
pub async fn calendar_list_period_templates() -> Result<ApiResponse<Vec<PeriodTemplateDto>>, String> {
    Ok(ok(vec![
        PeriodTemplateDto { id: "semesters".to_string(), name: "Two Semesters (Fall/Spring)".to_string() },
        PeriodTemplateDto { id: "trimesters".to_string(), name: "Three Trimesters".to_string() },
        PeriodTemplateDto { id: "quarters".to_string(), name: "Four Quarters".to_string() },
    ]))
}
