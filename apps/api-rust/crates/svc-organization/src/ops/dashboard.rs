//! Dashboard stats operations.

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

pub async fn get_stats(
    ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    use crate::entities::{academic_year, organization};
    use svc_academic::entities::{class, enrollment};
    use svc_iam::entities::user_role;

    // Get current year name
    let org = organization::Entity::find_by_id(&ctx.org_id).one(&ctx.db).await?;
    let mut current_period = "N/A".to_string();
    if let Some(org) = org {
        if let Some(period_id) = org.current_period_id {
            if let Some(year) = academic_year::Entity::find_by_id(period_id).one(&ctx.db).await? {
                current_period = year.name;
            }
        }
    }

    // Count active student enrollments
    let students_count = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .filter(enrollment::Column::Status.eq("active"))
        .count(&ctx.db)
        .await?;

    // Count classes
    let classes_count = class::Entity::find()
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .count(&ctx.db)
        .await?;

    // Count unique users assigned with role 'teacher'
    let teachers_count = user_role::Entity::find()
        .filter(user_role::Column::OrgId.eq(&ctx.org_id))
        .filter(user_role::Column::Role.eq("teacher"))
        .count(&ctx.db)
        .await?;

    use sea_orm::{Statement, ConnectionTrait, FromQueryResult};

    #[derive(FromQueryResult)]
    struct TaskData {
        status: String,
        deadline_at: Option<String>,
        last_attempt_at: Option<String>,
    }

    let query = format!(
        "SELECT status, deadline_at, last_attempt_at FROM autopilot_tasks WHERE org_id = '{}' AND status IN ('executing', 'pending', 'ready', 'suggested')",
        ctx.org_id
    );

    let mut stuck_count = 0;
    let mut overdue_count = 0;

    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();

    if let Ok(tasks) = TaskData::find_by_statement(Statement::from_string(ctx.db.get_database_backend(), query)).all(&ctx.db).await {
        for t in tasks {
            if t.status == "executing" {
                let stuck = t.last_attempt_at.as_ref()
                    .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
                    .map(|started| (now - started.with_timezone(&chrono::Utc)).num_minutes() > 10)
                    .unwrap_or(true);
                if stuck { stuck_count += 1; }
            } else {
                let overdue = t.deadline_at.as_ref()
                    .map(|d| d.as_str() < now_str.as_str())
                    .unwrap_or(false);
                if overdue { overdue_count += 1; }
            }
        }
    }

    let mut ai_insights = Vec::new();
    if stuck_count > 0 {
        ai_insights.push(serde_json::json!({
            "key": "critical",
            "count": stuck_count,
            "href": format!("/{}/settings", ctx.org_id)
        }));
    } else if overdue_count > 0 {
        ai_insights.push(serde_json::json!({
            "key": "warning",
            "count": overdue_count,
            "href": format!("/{}/settings", ctx.org_id)
        }));
    }

    Ok(serde_json::json!({
        "total_students": students_count,
        "total_classes": classes_count,
        "total_teachers": teachers_count,
        "total_staff": teachers_count,
        "attendance_overview": {
            "present_rate": 0,
            "late_rate": 0,
            "absent_rate": 0
        },
        "behavior_summary": {
            "net_points": 0,
            "critical_count": 0,
            "total_records": 0
        },
        "enrollment_rate": 0,
        "total_enrollments": students_count,
        "active_period": current_period,
        "ai_insights": ai_insights
    }))
}
