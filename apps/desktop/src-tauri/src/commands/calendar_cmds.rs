//! Tauri IPC commands — Calendar (Academic Years, Periods, Templates).

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Serialize)]
pub struct AcademicCalendarDto {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub status: String,
    pub current_period_id: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateYearInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize)]
pub struct UpdateYearInput {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub notes: Option<String>,
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
    pub period_kind: String,
    pub status: String,
    pub is_locked: bool,
    pub is_enrollment_open: bool,
    pub is_grade_entry_open: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreatePeriodInput {
    pub year_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub period_type: String,
    pub period_kind: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize)]
pub struct UpdatePeriodInput {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub period_kind: Option<String>,
}

#[derive(Serialize)]
pub struct AcademicContextResponse {
    pub current_year: Option<AcademicCalendarDto>,
    pub all_years: Vec<AcademicCalendarDto>,
    pub periods: Vec<AcademicPeriodDto>,
}

#[derive(Serialize)]
pub struct PeriodTypeDto {
    pub id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub sort_order: i32,
}

fn year_dto(y: orbis_domain::entities::academic_calendar::AcademicCalendar) -> AcademicCalendarDto {
    let is_active = y.status == "active";
    AcademicCalendarDto {
        id: y.id.to_string(),
        org_id: y.org_id.to_string(),
        name: y.name,
        name_ar: y.name_ar,
        start_date: y.start_date.to_rfc3339(),
        end_date: y.end_date.to_rfc3339(),
        is_active,
        status: y.status.to_string(),
        current_period_id: y.current_period_id.map(|u| u.to_string()),
        notes: y.notes,
        created_at: y.created_at.to_rfc3339(),
        updated_at: y.updated_at.to_rfc3339(),
    }
}

fn period_dto(p: orbis_domain::entities::academic_period::AcademicPeriod) -> AcademicPeriodDto {
    let status_str = match p.state {
        orbis_domain::entities::academic_period::AcademicPeriodState::Closed => "closed",
        orbis_domain::entities::academic_period::AcademicPeriodState::Archived => "archived",
        _ => {
            if p.is_locked {
                "locked"
            } else {
                "active"
            }
        }
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
        period_type: p.period_type.to_string(),
        period_kind: p.period_kind.to_string(),
        status: status_str.to_string(),
        is_locked: p.is_locked,
        is_enrollment_open: p.is_enrollment_open,
        is_grade_entry_open: p.is_grade_entry_open,
        created_at: p.created_at.to_rfc3339(),
        updated_at: p.updated_at.to_rfc3339(),
    }
}

#[tauri::command]
pub async fn calendar_create_year(
    request: CreateYearInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicCalendarDto>, String> {
    let ctx = make_ctx(&state);
    let start_date = DateTime::parse_from_rfc3339(&request.start_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid start_date format")?;
    let end_date = DateTime::parse_from_rfc3339(&request.end_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid end_date format")?;

    let y = state
        .calendar_service
        .create_year(
            &ctx,
            ctx.tenant_id,
            request.name,
            request.name_ar,
            start_date,
            end_date,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(year_dto(y)))
}

#[tauri::command]
pub async fn calendar_update_year(
    id: String,
    request: UpdateYearInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicCalendarDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let start_date = match request.start_date {
        Some(ref s) => Some(
            DateTime::parse_from_rfc3339(s)
                .map(|d| d.with_timezone(&Utc))
                .map_err(|_| "Invalid start_date format")?,
        ),
        None => None,
    };
    let end_date = match request.end_date {
        Some(ref s) => Some(
            DateTime::parse_from_rfc3339(s)
                .map(|d| d.with_timezone(&Utc))
                .map_err(|_| "Invalid end_date format")?,
        ),
        None => None,
    };

    let y = state
        .calendar_service
        .update_year(
            &ctx,
            uid,
            request.name,
            request.name_ar,
            start_date,
            end_date,
            request.notes,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(year_dto(y)))
}

#[tauri::command]
pub async fn calendar_activate_year(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicCalendarDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let y = state
        .calendar_service
        .activate_year(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(year_dto(y)))
}

#[tauri::command]
pub async fn calendar_close_year(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicCalendarDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let y = state
        .calendar_service
        .close_year(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(year_dto(y)))
}

#[tauri::command]
pub async fn calendar_archive_year(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicCalendarDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let y = state
        .calendar_service
        .archive_year(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(year_dto(y)))
}

#[tauri::command]
pub async fn calendar_delete_year(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .calendar_service
        .delete_year(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(true))
}

#[tauri::command]
pub async fn calendar_list_years(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AcademicCalendarDto>>, String> {
    let ctx = make_ctx(&state);
    let years = state
        .calendar_service
        .list_years(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(years.into_iter().map(year_dto).collect()))
}

#[tauri::command]
pub async fn calendar_get_year(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Option<AcademicCalendarDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let year = state
        .calendar_service
        .get_year(uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(year.map(year_dto)))
}

#[tauri::command]
pub async fn calendar_create_period(
    request: CreatePeriodInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let year_id = Uuid::parse_str(&request.year_id).map_err(|_| "Invalid yearId")?;
    let parent_id = request.parent_id.and_then(|p| Uuid::parse_str(&p).ok());
    let start_date = DateTime::parse_from_rfc3339(&request.start_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid start_date format")?;
    let end_date = DateTime::parse_from_rfc3339(&request.end_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| "Invalid end_date format")?;

    let p = state
        .calendar_service
        .create_period(
            &ctx,
            ctx.tenant_id,
            year_id,
            parent_id,
            request.name,
            request.name_ar,
            request.code,
            request.period_type,
            request.period_kind,
            start_date,
            end_date,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(period_dto(p)))
}

#[tauri::command]
pub async fn calendar_update_period(
    id: String,
    request: UpdatePeriodInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let start_date = match request.start_date {
        Some(ref s) => Some(
            DateTime::parse_from_rfc3339(s)
                .map(|d| d.with_timezone(&Utc))
                .map_err(|_| "Invalid start_date format")?,
        ),
        None => None,
    };
    let end_date = match request.end_date {
        Some(ref s) => Some(
            DateTime::parse_from_rfc3339(s)
                .map(|d| d.with_timezone(&Utc))
                .map_err(|_| "Invalid end_date format")?,
        ),
        None => None,
    };

    let p = state
        .calendar_service
        .update_period(
            &ctx,
            uid,
            request.name,
            request.name_ar,
            request.code,
            start_date,
            end_date,
            request.period_kind,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(period_dto(p)))
}

#[tauri::command]
pub async fn calendar_delete_period(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .calendar_service
        .delete_period(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(true))
}

#[tauri::command]
pub async fn calendar_list_periods(
    year_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AcademicPeriodDto>>, String> {
    let ctx = make_ctx(&state);
    let periods = if let Some(yid) = year_id.and_then(|s| Uuid::parse_str(&s).ok()) {
        state.calendar_service.list_periods_by_year(yid).await
    } else {
        state
            .calendar_service
            .list_periods_by_org(ctx.tenant_id)
            .await
    }
    .map_err(|e| e.to_string())?;
    Ok(ok(periods.into_iter().map(period_dto).collect()))
}

#[tauri::command]
pub async fn calendar_get_period(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Option<AcademicPeriodDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state
        .calendar_service
        .get_period(uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(period.map(period_dto)))
}

#[tauri::command]
pub async fn calendar_get_context(
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicContextResponse>, String> {
    let ctx = make_ctx(&state);
    let years = state
        .calendar_service
        .list_years(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    let current = years
        .iter()
        .find(|y| y.status == "active")
        .cloned()
        .or_else(|| years.first().cloned());
    let periods = if let Some(ref cur) = current {
        state
            .calendar_service
            .list_periods_by_year(cur.id)
            .await
            .map_err(|e| e.to_string())?
    } else {
        vec![]
    };
    Ok(ok(AcademicContextResponse {
        current_year: current.map(year_dto),
        all_years: years.into_iter().map(year_dto).collect(),
        periods: periods.into_iter().map(period_dto).collect(),
    }))
}

#[tauri::command]
pub async fn calendar_lock_period(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state
        .calendar_service
        .lock_period(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(period_dto(period)))
}

#[tauri::command]
pub async fn calendar_unlock_period(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state
        .calendar_service
        .unlock_period(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(period_dto(period)))
}

#[tauri::command]
pub async fn calendar_close_period(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AcademicPeriodDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let period = state
        .calendar_service
        .close_period(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(period_dto(period)))
}

#[tauri::command]
pub async fn calendar_list_period_types(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<PeriodTypeDto>>, String> {
    let ctx = make_ctx(&state);
    let templates = state
        .period_type_repo
        .find_by_org(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    let dtos = templates
        .into_iter()
        .map(|t| PeriodTypeDto {
            id: t.id.to_string(),
            org_id: t.org_id.to_string(),
            code: t.code,
            name: t.name,
            name_ar: t.name_ar,
            sort_order: t.sort_order,
        })
        .collect();
    Ok(ok(dtos))
}

#[derive(Serialize)]
pub struct OfficialCalendarTemplateDto {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub country: String,
    pub country_name: String,
    pub country_name_ar: String,
    pub hijri_year: i32,
    pub gregorian_years: String,
    pub decision_number: Option<String>,
    pub decision_date: Option<String>,
    pub authority: String,
    pub authority_ar: String,
}

#[derive(Deserialize)]
pub struct ApplyCalendarTemplateRequest {
    pub template_code: String,
}

#[tauri::command]
pub async fn calendar_list_official_templates(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<OfficialCalendarTemplateDto>>, String> {
    let templates = state.calendar_service.list_official_templates();
    let dtos = templates
        .into_iter()
        .map(|t| OfficialCalendarTemplateDto {
            code: t.code,
            name: t.name,
            name_ar: t.name_ar,
            country: t.country,
            country_name: t.country_name,
            country_name_ar: t.country_name_ar,
            hijri_year: t.hijri_year,
            gregorian_years: t.gregorian_years,
            decision_number: t.decision_number,
            decision_date: t.decision_date,
            authority: t.authority,
            authority_ar: t.authority_ar,
        })
        .collect();
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn calendar_apply_template(
    year_id: String,
    request: ApplyCalendarTemplateRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AcademicPeriodDto>>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&year_id).map_err(|e| e.to_string())?;
    let periods = state
        .calendar_service
        .apply_official_calendar_template(&ctx, uid, &request.template_code)
        .await
        .map_err(|e| e.to_string())?;
    let dtos = periods.into_iter().map(period_dto).collect();
    Ok(ok(dtos))
}

pub use orbis_app::services::calendar_service::CalendarDefaultsDto;

#[tauri::command]
pub async fn calendar_get_defaults(
    org_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<CalendarDefaultsDto>, String> {
    let ctx = make_ctx(&state);
    let target_org_id = if let Some(ref id_str) = org_id {
        Uuid::parse_str(id_str).map_err(|e| format!("Invalid org_id UUID: {e}"))?
    } else if ctx.tenant_id != Uuid::nil() {
        ctx.tenant_id
    } else {
        let orgs = state.org_repo.list_all().await.map_err(|e| e.to_string())?;
        orgs.first()
            .map(|o| o.id)
            .ok_or_else(|| "No organization exists to onboard".to_string())?
    };

    let org = state
        .org_repo
        .find_by_id(target_org_id)
        .await
        .map_err(|e| e.to_string())?;
    let (inst_type, country_code) = if let Some(o) = org {
        let it = o
            .metadata
            .as_ref()
            .and_then(|m| m.get("institution_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("school")
            .to_string();
        let cc = o
            .metadata
            .as_ref()
            .and_then(|m| m.get("country_code"))
            .and_then(|v| v.as_str())
            .unwrap_or("YE")
            .to_string();
        (it, cc)
    } else {
        ("school".to_string(), "YE".to_string())
    };

    let dto = state
        .calendar_service
        .get_calendar_defaults(target_org_id, &inst_type, &country_code);

    Ok(ok(dto))
}


