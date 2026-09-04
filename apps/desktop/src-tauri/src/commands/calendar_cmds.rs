use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::calendar::{
    academic_year::CreateAcademicYearUseCase,
    academic_period::CreateAcademicPeriodUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[tauri::command]
pub async fn create_academic_year(
    org_id: String,
    name: String,
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_start = DateTime::parse_from_rfc3339(&start_date).map_err(|_| "Invalid start date")?.with_timezone(&Utc);
    let parsed_end = DateTime::parse_from_rfc3339(&end_date).map_err(|_| "Invalid end date")?.with_timezone(&Utc);

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateAcademicYearUseCase {
        year_repo: state.year_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, name, parsed_start, parsed_end).await {
        Ok(year) => Ok(year.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn create_academic_period(
    org_id: String,
    year_id: String,
    parent_id: Option<String>,
    name: String,
    period_type: String,
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_year_id = Uuid::parse_str(&year_id).map_err(|_| "Invalid year ID")?;
    let parsed_parent_id = parent_id.map(|id| Uuid::parse_str(&id).unwrap());
    
    let parsed_start = DateTime::parse_from_rfc3339(&start_date).map_err(|_| "Invalid start date")?.with_timezone(&Utc);
    let parsed_end = DateTime::parse_from_rfc3339(&end_date).map_err(|_| "Invalid end date")?.with_timezone(&Utc);

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateAcademicPeriodUseCase {
        period_repo: state.period_repo.clone(),
        year_repo: state.year_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, parsed_year_id, parsed_parent_id, name, period_type, parsed_start, parsed_end).await {
        Ok(period) => Ok(period.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
