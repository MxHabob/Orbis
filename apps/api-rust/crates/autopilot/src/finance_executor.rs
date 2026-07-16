use chrono::Utc;
use sea_orm::{DatabaseConnection, DbBackend, Statement, ConnectionTrait};
use tracing::{info, warn};
use uuid::Uuid;
use serde_json::json;

use shared_kernel::errors::AppError;
use crate::academic_evaluator::{ProposedTask, Risk};
use platform_policy::entities::academic_policy::Model as AcademicPolicy;

pub struct FinanceEvaluator;

impl FinanceEvaluator {
    /// Evaluates financial state and proposes tasks (e.g. fee reminders)
    pub async fn evaluate(
        db: &DatabaseConnection,
        org_id: &str,
        _policy: &AcademicPolicy,
    ) -> Result<Vec<ProposedTask>, AppError> {
        let mut tasks = Vec::new();
        let now_date = Utc::now().naive_utc().date();

        // Find overdue fees that don't have a recent reminder task
        let sql = r#"
            SELECT sf.id, sf.enrollment_id, sf.due_date, sf.net_amount
            FROM student_fees sf
            WHERE sf.org_id = 
              AND sf.status = 'active'
              AND sf.due_date < 
              AND NOT EXISTS (
                  SELECT 1 FROM autopilot_tasks at 
                  WHERE at.target_id = sf.id 
                    AND at.task_type = 'finance.fee_reminder'
                    AND at.created_at > datetime('now', '-7 days')
              )
            LIMIT 50
        "#;

        let rows = db.query_all(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                org_id.into(),
                now_date.format("%Y-%m-%d").to_string().into(),
            ],
        )).await?;

        for row in rows {
            let fee_id: String = row.try_get_by_index(0)?;
            let enrollment_id: String = row.try_get_by_index(1)?;
            let due_date_str: String = row.try_get_by_index(2)?;
            let amount: f64 = row.try_get_by_index(3)?;

            let params = json!({
                "enrollment_id": enrollment_id,
                "amount": amount,
                "due_date": due_date_str
            }).to_string();

            tasks.push(ProposedTask::new_with_params(
                "finance.fee_reminder",
                &fee_id,
                "student_fee",
                format!("Fee {} is overdue since {}", fee_id, due_date_str),
                Risk::Medium,
                Some(params),
            ));
        }

        Ok(tasks)
    }
}

pub struct FinanceExecutor;

impl FinanceExecutor {
    /// Executes financial tasks
    pub async fn execute_task(
        db: &DatabaseConnection,
        task_type: &str,
        target_id: &str,
        _params: Option<&str>,
    ) -> Result<(), AppError> {
        match task_type {
            "finance.fee_reminder" => Self::send_fee_reminder(db, target_id).await,
            _ => {
                warn!("Unknown finance task type: {}", task_type);
                Ok(())
            }
        }
    }

    async fn send_fee_reminder(
        db: &DatabaseConnection,
        fee_id: &str,
    ) -> Result<(), AppError> {
        info!("Executing fee reminder for fee {}", fee_id);
        
        // 1. Fetch fee details and student info
        let sql = r#"
            SELECT sf.org_id, e.student_id, e.period_id, sf.net_amount
            FROM student_fees sf
            JOIN enrollments e ON sf.enrollment_id = e.id
            WHERE sf.id = 
        "#;

        let row = db.query_one(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![fee_id.into()],
        )).await?;

        if let Some(row) = row {
            let org_id: String = row.try_get_by_index(0)?;
            let student_id: String = row.try_get_by_index(1)?;
            let amount: f64 = row.try_get_by_index(3)?;

            // 2. Insert notification
            let notif_id = format!("notif_{}", Uuid::new_v4().to_string().replace("-", "")).chars().take(24).collect::<String>();
            let now = Utc::now().to_rfc3339();

            let insert_sql = r#"
                INSERT INTO app_notifications
                    (id, org_id, user_id, title, message, is_read, type_code, created_at)
                VALUES
                    (, , , , , 0, 'fee_reminder', )
            "#;

            let title = "رسالة تذكير بسداد رسوم مستحقة";
            let message = format!("نود تذكيركم بسداد رسوم مستحقة بقيمة {} للطالب.", amount);

            db.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                insert_sql,
                vec![
                    notif_id.into(),
                    org_id.into(),
                    student_id.into(), // In a real system, this would be the parent/guardian
                    title.into(),
                    message.into(),
                    now.into(),
                ],
            )).await?;
            
            info!("Fee reminder notification sent for fee {}", fee_id);
            Ok(())
        } else {
            Err(AppError::not_found("student_fee", fee_id))
        }
    }
}
