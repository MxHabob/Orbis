use axum::serve;
use orbis_server::app_router;
use orbis_server::state::AppState;
use orbis_infra::repositories::{org_repo::OrgRepoImpl, user_repo::UserRepoImpl, person_repo::PersonRepoImpl, session_repo::SessionRepoImpl};
use orbis_infra::event_bus::MemoryEventBus;
use orbis_infra::security::{jwt::JwtTokenService, bcrypt::BcryptHasher};
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Use an in-memory SQLite DB for now, or real DB if provided
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    
    // Connect to database
    let db = Database::connect(&db_url).await?;

    // Run Migrations
    use orbis_infra::migration::Migrator;
    use sea_orm_migration::MigratorTrait;
    Migrator::up(&db, None).await?;

    let org_repo = Arc::new(OrgRepoImpl { db: db.clone() });
    let org_unit_repo = Arc::new(orbis_infra::repositories::org_unit_repo::OrgUnitRepoImpl { db: db.clone() });
    let user_repo = Arc::new(UserRepoImpl { db: db.clone() });
    let person_repo = Arc::new(PersonRepoImpl { db: db.clone() });
    let session_repo = Arc::new(SessionRepoImpl { db: db.clone() });
    let calendar_repo = Arc::new(orbis_infra::repositories::calendar_repos::CalendarRepoImpl { db: db.clone() });
    let academic_repo = Arc::new(orbis_infra::repositories::academic_repos::AcademicRepoImpl { db: db.clone() });
    let student_repo = Arc::new(orbis_infra::repositories::student_repos::StudentRepoImpl { db: db.clone() });
    let scheduling_repo = Arc::new(orbis_infra::repositories::scheduling_repos::SchedulingRepoImpl { db: db.clone() });
    let assessment_repo = Arc::new(orbis_infra::repositories::assessment_repos::AssessmentRepoImpl { db: db.clone() });
    let exam_repo = Arc::new(orbis_infra::repositories::exam_repos::ExamRepoImpl { db: db.clone() });
    let finance_repo = Arc::new(orbis_infra::repositories::finance_repos::FinanceRepoImpl { db: db.clone() });
    let document_repo = Arc::new(orbis_infra::repositories::document_repos::DocumentRepoImpl { db: db.clone() });
    let integration_repo = Arc::new(orbis_infra::repositories::integration_repos::IntegrationRepoImpl { db: db.clone() });
    let ai_repo = Arc::new(orbis_infra::repositories::ai_repos::AiRepoImpl { db: db.clone() });
    let workflow_repo = Arc::new(orbis_infra::repositories::workflow_repos::WorkflowRepoImpl { db: db.clone() });
    let report_repo = Arc::new(orbis_infra::repositories::report_repos::ReportRepoImpl { db: db.clone() });
    
    let autopilot_engine = Arc::new(orbis_app::autopilot::engine::AutopilotEngine {
        policy_repo: Arc::new(orbis_infra::repositories::policy_repo::PgPolicyRepo { db: db.clone() }),
    });

    let event_bus = Arc::new(MemoryEventBus {
        workflow_engine: Some(autopilot_engine),
    });
    let token_service = Arc::new(JwtTokenService::new("super-secret-key-for-dev".to_string()));
    let password_hasher = Arc::new(BcryptHasher);

    let integration_resolver = Arc::new(orbis_infra::adapters::resolver::IntegrationResolver::new(integration_repo.clone()));

    // Build AppState
    let state = AppState {
        db,
        org_repo,
        org_unit_repo,
        user_repo,
        person_repo,
        session_repo,
        year_repo: calendar_repo.clone(),
        period_repo: calendar_repo.clone(),
        period_template_repo: calendar_repo,
        subject_repo: academic_repo.clone(),
        class_repo: academic_repo.clone(),
        class_subject_repo: academic_repo,
        enrollment_repo: student_repo.clone(),
        student_guardian_repo: student_repo,
        room_repo: scheduling_repo.clone(),
        time_slot_repo: scheduling_repo.clone(),
        lesson_repo: scheduling_repo,
        assessment_scheme_repo: assessment_repo.clone(),
        grade_repo: assessment_repo.clone(),
        attendance_repo: assessment_repo.clone(),
        behavior_record_repo: assessment_repo,
        exam_session_repo: exam_repo.clone(),
        exam_schedule_repo: exam_repo.clone(),
        seating_number_repo: exam_repo.clone(),
        secret_number_repo: exam_repo.clone(),
        exam_proctor_repo: exam_repo,
        account_repo: finance_repo.clone(),
        journal_entry_repo: finance_repo.clone(),
        fee_template_repo: finance_repo.clone(),
        student_fee_repo: finance_repo.clone(),
        payment_repo: finance_repo,
        document_repo,
        integration_repo,
        ai_repo,
        workflow_repo,
        report_repo,
        event_bus,
        token_service,
        password_hasher,
        integration_resolver,
    };

    // Build Axum Router
    let app = app_router(state);

    // Start Server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");
    
    serve(listener, app).await?;

    Ok(())
}
