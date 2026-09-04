use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::exam::{
    exam_session::CreateExamSessionUseCase,
    exam_schedule::ScheduleExamUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};

#[tauri::command]
pub async fn create_exam_session(
    org_id: String,
    academic_period_id: String,
    name: String,
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_academic_period_id = Uuid::parse_str(&academic_period_id).map_err(|_| "Invalid period ID")?;
    let parsed_start_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").map_err(|_| "Invalid start date")?;
    let parsed_end_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").map_err(|_| "Invalid end date")?;

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateExamSessionUseCase {
        session_repo: state.exam_session_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, parsed_academic_period_id, name, parsed_start_date, parsed_end_date).await {
        Ok(session) => Ok(session.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn schedule_exam(
    exam_session_id: String,
    class_subject_id: String,
    exam_date: String,
    start_time: String,
    end_time: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_exam_session_id = Uuid::parse_str(&exam_session_id).map_err(|_| "Invalid exam session ID")?;
    let parsed_class_subject_id = Uuid::parse_str(&class_subject_id).map_err(|_| "Invalid class subject ID")?;
    
    let parsed_exam_date = NaiveDate::parse_from_str(&exam_date, "%Y-%m-%d").map_err(|_| "Invalid date format")?;
    let parsed_start_time = NaiveTime::parse_from_str(&start_time, "%H:%M:%S").map_err(|_| "Invalid start time")?;
    let parsed_end_time = NaiveTime::parse_from_str(&end_time, "%H:%M:%S").map_err(|_| "Invalid end time")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = ScheduleExamUseCase {
        schedule_repo: state.exam_schedule_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_exam_session_id, parsed_class_subject_id, parsed_exam_date, parsed_start_time, parsed_end_time).await {
        Ok(schedule) => Ok(schedule.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn assign_seating_number(
    exam_session_id: String,
    student_id: String,
    seating_number: String,
    room_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_exam_session_id = Uuid::parse_str(&exam_session_id).map_err(|_| "Invalid session ID")?;
    let parsed_student_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let parsed_room_id = match room_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|_| "Invalid room ID")?),
        None => None,
    };

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = orbis_app::usecases::exam::seating_number::AssignSeatingNumberUseCase {
        seating_repo: state.seating_number_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_exam_session_id, parsed_student_id, seating_number, parsed_room_id).await {
        Ok(seating) => Ok(seating.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn assign_secret_number(
    exam_schedule_id: String,
    student_id: String,
    secret_number: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_exam_schedule_id = Uuid::parse_str(&exam_schedule_id).map_err(|_| "Invalid schedule ID")?;
    let parsed_student_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = orbis_app::usecases::exam::secret_number::AssignSecretNumberUseCase {
        secret_repo: state.secret_number_repo.clone(),
    };

    match usecase.execute(&ctx, parsed_exam_schedule_id, parsed_student_id, secret_number).await {
        Ok(secret) => Ok(secret.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn assign_proctor(
    exam_schedule_id: String,
    room_id: String,
    teacher_id: String,
    is_chief: bool,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_exam_schedule_id = Uuid::parse_str(&exam_schedule_id).map_err(|_| "Invalid schedule ID")?;
    let parsed_room_id = Uuid::parse_str(&room_id).map_err(|_| "Invalid room ID")?;
    let parsed_teacher_id = Uuid::parse_str(&teacher_id).map_err(|_| "Invalid teacher ID")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = orbis_app::usecases::exam::exam_proctor::AssignProctorUseCase {
        proctor_repo: state.exam_proctor_repo.clone(),
    };

    match usecase.execute(&ctx, parsed_exam_schedule_id, parsed_room_id, parsed_teacher_id, is_chief).await {
        Ok(proctor) => Ok(proctor.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
