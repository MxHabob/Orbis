use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Exam Services Helper ─────────────────────────────────────────────────────

struct ExamServices {
    session: Arc<domain_exam::services::exam_session::ExamSessionService>,
    ordering: Arc<domain_exam::services::student_ordering::StudentOrderingService>,
    numbers: domain_exam::services::exam_numbers::ExamNumbersService,
    logistics: domain_exam::services::exam_logistics::ExamLogisticsService,
    scheduler: domain_exam::services::exam_auto_schedule::ExamAutoScheduleService,
}

fn get_exam_services(db: sea_orm::DatabaseConnection) -> ExamServices {
    let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(db));
    let session = Arc::new(domain_exam::services::exam_session::ExamSessionService::new(cmd.clone(), query.clone()));
    let ordering = Arc::new(domain_exam::services::student_ordering::StudentOrderingService::new(cmd.clone(), query.clone()));
    let solver = Arc::new(infra_scheduler::NoOpScheduleSolver);
    
    ExamServices {
        session: session.clone(),
        ordering: ordering.clone(),
        numbers: domain_exam::services::exam_numbers::ExamNumbersService::new(cmd.clone(), query.clone(), ordering.clone()),
        logistics: domain_exam::services::exam_logistics::ExamLogisticsService::new(cmd.clone(), query.clone(), session.clone(), ordering),
        scheduler: domain_exam::services::exam_auto_schedule::ExamAutoScheduleService::new(cmd, query, session, solver),
    }
}

// ── Exam Sessions ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_exam_sessions(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: Option<String>,
) -> CmdResult<Vec<domain_exam::entities::exam_session::Model>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.get_exam_sessions(&org_id, cycle_id.as_deref())
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_exam_session(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<domain_exam::entities::exam_session::Model> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.get_exam_session(&org_id, &session_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_exam_session(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
    request: domain_exam::services::exam_session::CreateExamSessionRequest,
) -> CmdResult<domain_exam::entities::exam_session::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.create_exam_session(&org_id, &user_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_exam_session(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    request: domain_exam::services::exam_session::UpdateExamSessionRequest,
) -> CmdResult<domain_exam::entities::exam_session::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.update_exam_session(&org_id, &session_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Exam Subjects ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_exam_subjects(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<Vec<domain_exam::entities::exam_subject::Model>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.get_exam_subjects(&org_id, &session_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_exam_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_exam::services::exam_session::CreateExamSubjectRequest,
) -> CmdResult<domain_exam::entities::exam_subject::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.create_exam_subject(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Student Ordering ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_student_ordering(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    class_id: Option<String>,
) -> CmdResult<Vec<domain_exam::services::student_ordering::OrderedStudent>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.ordering.get_ordered_students(&org_id, &cycle_id, class_id.as_deref())
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn rebuild_student_ordering(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    strategy: String,
) -> CmdResult<domain_exam::services::student_ordering::RebuildOrderingResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.ordering.rebuild_ordering(&org_id, &cycle_id, &strategy)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn freeze_student_ordering(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    user_id: String,
) -> CmdResult<u64> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.ordering.freeze_ordering(&org_id, &cycle_id, &user_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Seating & Secret Numbers ─────────────────────────────────────────────────

#[tauri::command]
pub async fn generate_seating_numbers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    user_id: String,
) -> CmdResult<domain_exam::services::exam_numbers::GenerationResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.numbers.generate_seating_numbers(&org_id, &session_id, &user_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn generate_secret_numbers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    user_id: String,
) -> CmdResult<domain_exam::services::exam_numbers::GenerationResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.numbers.generate_secret_numbers(&org_id, &session_id, &user_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Exam Schedules ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_exam_schedules(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<Vec<domain_exam::entities::exam_schedule::Model>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.get_exam_schedules(&org_id, &session_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn start_exam_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
    request: domain_exam::services::exam_auto_schedule::AutoScheduleRequest,
) -> CmdResult<domain_exam::services::exam_auto_schedule::SolveJobResponse> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.scheduler.start_exam_solve(&org_id, &user_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_exam_solve_job(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<domain_exam::services::exam_auto_schedule::SolveJobResponse> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.scheduler.get_exam_solve_job(&org_id, &job_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn list_exam_solve_jobs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    exam_session_id: Option<String>,
) -> CmdResult<Vec<domain_exam::services::exam_auto_schedule::SolveJobResponse>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.scheduler.list_exam_solve_jobs(&org_id, exam_session_id.as_deref())
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn cancel_exam_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<domain_exam::services::exam_auto_schedule::SolveJobResponse> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.scheduler.cancel_exam_solve(&org_id, &job_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn apply_exam_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<domain_exam::services::exam_auto_schedule::SolveJobResponse> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.scheduler.apply_exam_solve(&org_id, &job_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Exam Logistics ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn allocate_students_to_halls(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    config: domain_exam::services::exam_logistics::AllocationConfig,
) -> CmdResult<domain_exam::services::exam_logistics::AllocationResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.logistics.allocate_students_to_halls(&org_id, &session_id, config)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn auto_assign_proctors(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    schedule_id: String,
    config: domain_exam::services::exam_logistics::ProctorAssignConfig,
) -> CmdResult<domain_exam::services::exam_logistics::ProctorAssignResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.logistics.auto_assign_proctors(&org_id, &session_id, &schedule_id, config)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_exam_halls(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<Vec<domain_exam::entities::exam_hall::Model>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.get_exam_halls(&org_id, &session_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_exam_hall(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_exam::services::exam_session::CreateExamHallRequest,
) -> CmdResult<domain_exam::entities::exam_hall::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.session.create_exam_hall(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_exam_proctors(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    session_id: String,
    schedule_id: Option<String>,
) -> CmdResult<Vec<domain_exam::entities::exam_proctor::Model>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    let params = domain_exam::params::ExamProctorParams {
        exam_session_id: Some(session_id.clone()),
        exam_schedule_id: schedule_id,
        teacher_id: None,
    };
    services.session.query.get_exam_proctors(&session_id, params)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_hall_students(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    hall_id: String,
) -> CmdResult<Vec<domain_exam::entities::exam_hall_allocation::Model>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.logistics.get_hall_students(&org_id, &session_id, &hall_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_seating_numbers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    class_id: Option<String>,
) -> CmdResult<Vec<domain_exam::services::exam_numbers::SeatingNumberView>> {
    let db = state.active_db().await;
    let services = get_exam_services(db);
    services.numbers.get_seating_numbers(&org_id, &session_id, class_id.as_deref())
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_exam_schedule(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    schedule_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let _ = domain_exam::entities::exam_schedule::Entity::delete_by_id(schedule_id)
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn update_exam_schedule_status(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    schedule_id: String,
    status: String,
) -> CmdResult<domain_exam::entities::exam_schedule::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let schedule = domain_exam::entities::exam_schedule::Entity::find_by_id(schedule_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Exam schedule not found".to_string()))?;
        
    let mut active: domain_exam::entities::exam_schedule::ActiveModel = schedule.into();
    active.status = Set(status);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}
