//! Tauri IPC commands — Exam (Sessions, Schedules, Seating, Secrets, Proctors).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};
use orbis_app::usecases::exam::{
    exam_session::CreateExamSessionUseCase,
    exam_schedule::ScheduleExamUseCase,
    seating_number::AssignSeatingNumberUseCase,
    secret_number::AssignSecretNumberUseCase,
    exam_proctor::AssignProctorUseCase,
};
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct ExamSessionDto {
    pub id: String, pub org_id: String, pub academic_period_id: String,
    pub name: String,
    pub start_date: String, pub end_date: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateExamSessionInput {
    pub academic_period_id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize)]
pub struct ExamScheduleDto {
    pub id: String, pub exam_session_id: String, pub class_subject_id: String,
    pub exam_date: String, pub start_time: String, pub end_time: String,
}

#[derive(Deserialize)]
pub struct ScheduleExamInput {
    pub exam_session_id: String,
    pub class_subject_id: String,
    pub exam_date: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize)]
pub struct SeatingDto {
    pub id: String,
    pub exam_session_id: String,
    pub student_id: String,
    pub seating_number: String,
    pub room_id: Option<String>,
}

#[derive(Deserialize)]
pub struct AssignSeatingInput {
    pub exam_session_id: String,
    pub student_id: String,
    pub seating_number: String,
    pub room_id: Option<String>,
}

#[derive(Serialize)]
pub struct SecretDto {
    pub id: String,
    pub exam_schedule_id: String,
    pub student_id: String,
    pub secret_number: String,
}

#[derive(Deserialize)]
pub struct AssignSecretInput {
    pub exam_schedule_id: String,
    pub student_id: String,
    pub secret_number: String,
}

#[derive(Serialize)]
pub struct ProctorDto {
    pub id: String,
    pub exam_schedule_id: String,
    pub room_id: String,
    pub teacher_id: String,
    pub is_chief: bool,
}

#[derive(Deserialize)]
pub struct AssignProctorInput {
    pub exam_schedule_id: String,
    pub room_id: String,
    pub teacher_id: String,
    pub is_chief: bool,
}

#[tauri::command]
pub async fn exam_create_session(request: CreateExamSessionInput, state: State<'_, AppState>) -> Result<ApiResponse<ExamSessionDto>, String> {
    let ctx = make_ctx(&state);
    let uc = CreateExamSessionUseCase {
        session_repo: state.exam_session_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let period_id = Uuid::parse_str(&request.academic_period_id).map_err(|_| "Invalid academic_period_id")?;
    let start_date = NaiveDate::parse_from_str(&request.start_date, "%Y-%m-%d").map_err(|_| "Invalid start_date (expected YYYY-MM-DD)")?;
    let end_date = NaiveDate::parse_from_str(&request.end_date, "%Y-%m-%d").map_err(|_| "Invalid end_date (expected YYYY-MM-DD)")?;

    let s = uc.execute(&ctx, ctx.tenant_id, period_id, request.name, start_date, end_date)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(ExamSessionDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), academic_period_id: s.academic_period_id.to_string(),
        name: s.name,
        start_date: s.start_date.to_string(), end_date: s.end_date.to_string(),
        status: s.status,
    }))
}

#[tauri::command]
pub async fn exam_list_sessions(period_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ExamSessionDto>>, String> {
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let items = state.exam_session_repo.find_by_period(pid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| ExamSessionDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), academic_period_id: s.academic_period_id.to_string(),
        name: s.name,
        start_date: s.start_date.to_string(), end_date: s.end_date.to_string(),
        status: s.status,
    }).collect()))
}

#[tauri::command]
pub async fn exam_get_session(id: String, state: State<'_, AppState>) -> Result<ApiResponse<ExamSessionDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state.exam_session_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Session not found")?;
    Ok(ok(ExamSessionDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), academic_period_id: s.academic_period_id.to_string(),
        name: s.name,
        start_date: s.start_date.to_string(), end_date: s.end_date.to_string(),
        status: s.status,
    }))
}

#[tauri::command]
pub async fn exam_schedule_exam(request: ScheduleExamInput, state: State<'_, AppState>) -> Result<ApiResponse<ExamScheduleDto>, String> {
    let ctx = make_ctx(&state);
    let uc = ScheduleExamUseCase {
        schedule_repo: state.exam_schedule_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let session_id = Uuid::parse_str(&request.exam_session_id).map_err(|_| "Invalid exam_session_id")?;
    let class_subject_id = Uuid::parse_str(&request.class_subject_id).map_err(|_| "Invalid class_subject_id")?;
    let exam_date = NaiveDate::parse_from_str(&request.exam_date, "%Y-%m-%d").map_err(|_| "Invalid exam_date (expected YYYY-MM-DD)")?;
    let start_time = NaiveTime::parse_from_str(&request.start_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&request.start_time, "%H:%M"))
        .map_err(|_| "Invalid start_time")?;
    let end_time = NaiveTime::parse_from_str(&request.end_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&request.end_time, "%H:%M"))
        .map_err(|_| "Invalid end_time")?;

    let s = uc.execute(&ctx, session_id, class_subject_id, exam_date, start_time, end_time)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(ExamScheduleDto {
        id: s.id.to_string(),
        exam_session_id: s.exam_session_id.to_string(),
        class_subject_id: s.class_subject_id.to_string(),
        exam_date: s.exam_date.to_string(),
        start_time: s.start_time.to_string(),
        end_time: s.end_time.to_string(),
    }))
}

#[tauri::command]
pub async fn exam_list_schedules(session_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ExamScheduleDto>>, String> {
    let sid = Uuid::parse_str(&session_id).map_err(|_| "Invalid session ID")?;
    let items = state.exam_schedule_repo.find_by_session(sid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| ExamScheduleDto {
        id: s.id.to_string(), exam_session_id: s.exam_session_id.to_string(),
        class_subject_id: s.class_subject_id.to_string(),
        exam_date: s.exam_date.to_string(),
        start_time: s.start_time.to_string(), end_time: s.end_time.to_string(),
    }).collect()))
}

#[tauri::command]
pub async fn exam_assign_seating(request: AssignSeatingInput, state: State<'_, AppState>) -> Result<ApiResponse<SeatingDto>, String> {
    let ctx = make_ctx(&state);
    let uc = AssignSeatingNumberUseCase {
        seating_repo: state.seating_number_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let session_id = Uuid::parse_str(&request.exam_session_id).map_err(|_| "Invalid exam_session_id")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid student_id")?;
    let room_id = request.room_id.and_then(|r| Uuid::parse_str(&r).ok());

    let s = uc.execute(&ctx, session_id, student_id, request.seating_number, room_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(SeatingDto {
        id: s.id.to_string(),
        exam_session_id: s.exam_session_id.to_string(),
        student_id: s.student_id.to_string(),
        seating_number: s.seating_number,
        room_id: s.room_id.map(|r| r.to_string()),
    }))
}

#[tauri::command]
pub async fn exam_assign_secret(request: AssignSecretInput, state: State<'_, AppState>) -> Result<ApiResponse<SecretDto>, String> {
    let ctx = make_ctx(&state);
    let uc = AssignSecretNumberUseCase {
        secret_repo: state.secret_number_repo.clone(),
    };
    let schedule_id = Uuid::parse_str(&request.exam_schedule_id).map_err(|_| "Invalid exam_schedule_id")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid student_id")?;

    let s = uc.execute(&ctx, schedule_id, student_id, request.secret_number)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(SecretDto {
        id: s.id.to_string(),
        exam_schedule_id: s.exam_schedule_id.to_string(),
        student_id: s.student_id.to_string(),
        secret_number: s.secret_number,
    }))
}

#[tauri::command]
pub async fn exam_assign_proctor(request: AssignProctorInput, state: State<'_, AppState>) -> Result<ApiResponse<ProctorDto>, String> {
    let ctx = make_ctx(&state);
    let uc = AssignProctorUseCase {
        proctor_repo: state.exam_proctor_repo.clone(),
    };
    let schedule_id = Uuid::parse_str(&request.exam_schedule_id).map_err(|_| "Invalid exam_schedule_id")?;
    let room_id = Uuid::parse_str(&request.room_id).map_err(|_| "Invalid room_id")?;
    let teacher_id = Uuid::parse_str(&request.teacher_id).map_err(|_| "Invalid teacher_id")?;

    let pr = uc.execute(&ctx, schedule_id, room_id, teacher_id, request.is_chief)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(ProctorDto {
        id: pr.id.to_string(),
        exam_schedule_id: pr.exam_schedule_id.to_string(),
        room_id: pr.room_id.to_string(),
        teacher_id: pr.teacher_id.to_string(),
        is_chief: pr.is_chief,
    }))
}
