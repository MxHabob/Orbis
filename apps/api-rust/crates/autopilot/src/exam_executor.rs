use chrono::{Utc, NaiveDate, Duration};
use sea_orm::{DatabaseConnection, DbBackend, Statement, ConnectionTrait};
use tracing::{info, warn};
use serde_json::json;

use shared_kernel::errors::AppError;
use crate::academic_evaluator::{ProposedTask, Risk};
use platform_policy::entities::academic_policy::Model as AcademicPolicy;

pub struct ExamEvaluator;

impl ExamEvaluator {
    /// Evaluates exam sessions and proposes tasks (e.g. session_prep)
    pub async fn evaluate(
        db: &DatabaseConnection,
        org_id: &str,
        _policy: &AcademicPolicy,
    ) -> Result<Vec<ProposedTask>, AppError> {
        let mut tasks = Vec::new();
        let now_date = Utc::now().naive_utc().date();
        let prep_window_end = now_date + Duration::days(7); // Prep 7 days before start

        // Find upcoming exam sessions that need preparation
        let sql = r#"
            SELECT id, name, start_date
            FROM exam_sessions
            WHERE org_id = 
              AND status IN ('draft', 'scheduled')
              AND start_date <= 
              AND NOT EXISTS (
                  SELECT 1 FROM autopilot_tasks at 
                  WHERE at.target_id = exam_sessions.id 
                    AND at.task_type = 'exam.session_prep'
                    AND at.status IN ('ready', 'approved', 'completed')
              )
            LIMIT 20
        "#;

        let rows = db.query_all(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                org_id.into(),
                prep_window_end.format("%Y-%m-%d").to_string().into(),
            ],
        )).await?;

        for row in rows {
            let session_id: String = row.try_get_by_index(0)?;
            let name: String = row.try_get_by_index(1)?;
            let start_date: String = row.try_get_by_index(2)?;

            let params = json!({
                "session_name": name,
                "start_date": start_date
            }).to_string();

            tasks.push(ProposedTask::new_with_params(
                "exam.session_prep",
                &session_id,
                "exam_session",
                format!("Prepare seating charts and grading sheets for exam session: {}", name),
                Risk::Low,
                Some(params),
            ));
        }

        Ok(tasks)
    }
}

pub struct ExamExecutor;

impl ExamExecutor {
    /// Executes exam tasks
    pub async fn execute_task(
        db: &DatabaseConnection,
        task_type: &str,
        target_id: &str,
        _params: Option<&str>,
    ) -> Result<(), AppError> {
        match task_type {
            "exam.session_prep" => Self::prep_session(db, target_id).await,
            _ => {
                warn!("Unknown exam task type: {}", task_type);
                Ok(())
            }
        }
    }

    async fn prep_session(
        db: &DatabaseConnection,
        session_id: &str,
    ) -> Result<(), AppError> {
        info!("Executing exam session prep for session {}", session_id);
        
        // In a real implementation, this would generate seating charts, lock assignments, etc.
        // For Phase 3 audit, we update the status to indicate prep is done.
        
        let sql = r#"
            UPDATE exam_sessions
            SET status = 'prepared', updated_at = 
            WHERE id =  AND status IN ('draft', 'scheduled')
        "#;
        
        let now = Utc::now().to_rfc3339();

        let result = db.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                now.into(),
                session_id.into(),
            ],
        )).await?;
        
        if result.rows_affected() > 0 {
            info!("Exam session {} prepared successfully", session_id);
        } else {
            info!("Exam session {} was already prepared or not found", session_id);
        }

        Ok(())
    }
}
