//! Infrastructure implementation of DashboardRepository.
//!
//! Uses raw SQL aggregation queries for performance — counting across
//! multiple tables in a single pass per table.

use async_trait::async_trait;
use orbis_app::ports::repositories::{DashboardCounts, DashboardRepository};
use orbis_domain::errors::DomainError;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use uuid::Uuid;

pub struct DashboardRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl DashboardRepository for DashboardRepoImpl {
    async fn get_counts(&self, org_id: Uuid) -> Result<DashboardCounts, DomainError> {
        let row = self.db.query_one(Statement::from_sql_and_values(
            self.db.get_database_backend(),
            r#"
                SELECT
                    (SELECT COUNT(*) FROM students WHERE org_id = $1 AND is_deleted = FALSE) AS total_students,
                    (SELECT COUNT(*) FROM employees WHERE org_id = $1 AND is_deleted = FALSE) AS total_employees,
                    sections.active_sections,
                    sections.total_classes,
                    (SELECT COUNT(DISTINCT user_id) FROM user_role_assignments
                        WHERE org_id = $1 AND LOWER(role_id) LIKE '%teacher%') AS total_teachers,
                    (SELECT COUNT(*) FROM enrollments WHERE org_id = $1) AS total_enrollments,
                    (SELECT COUNT(*) FROM subjects WHERE org_id = $1) AS total_subjects,
                    (SELECT name FROM academic_calendars WHERE org_id = $1
                        ORDER BY CASE WHEN status = 'active' THEN 0 ELSE 1 END, created_at DESC, id
                        LIMIT 1) AS active_academic_calendar,
                    (SELECT CAST(COALESCE(SUM(amount), 0) AS DOUBLE PRECISION)
                        FROM payments WHERE org_id = $1) AS total_payments_amount,
                    (SELECT CAST(COALESCE(SUM(net_amount), 0) AS DOUBLE PRECISION)
                        FROM student_fees WHERE org_id = $1 AND status != 'paid') AS pending_fees_amount
                FROM (
                    SELECT COUNT(CASE WHEN is_active = TRUE THEN 1 END) AS active_sections,
                           COUNT(CASE WHEN section_type = 'class' THEN 1 END) AS total_classes
                    FROM sections
                    WHERE org_id = $1 AND is_deleted = FALSE
                ) AS sections
            "#,
            [org_id.into()],
        )).await.map_err(|error| DomainError::Internal(error.to_string()))?
            .ok_or_else(|| DomainError::Internal("Dashboard aggregate returned no row".into()))?;
        let count = |column| {
            let value = row
                .try_get::<i64>("", column)
                .map_err(dashboard_query_error)?;
            u64::try_from(value).map_err(|error| DomainError::Internal(error.to_string()))
        };
        let (enrolled_students, enrollment_rate) = self
            .enrollment_metrics(org_id, self.current_period_id(org_id).await?)
            .await?;

        Ok(DashboardCounts {
            total_students: count("total_students")?,
            enrolled_students,
            enrollment_rate,
            total_employees: count("total_employees")?,
            active_sections: count("active_sections")?,
            total_teachers: count("total_teachers")?,
            total_classes: count("total_classes")?,
            total_enrollments: count("total_enrollments")?,
            total_subjects: count("total_subjects")?,
            active_academic_calendar: row
                .try_get("", "active_academic_calendar")
                .map_err(dashboard_query_error)?,
            total_payments_amount: row
                .try_get("", "total_payments_amount")
                .map_err(dashboard_query_error)?,
            pending_fees_amount: row
                .try_get("", "pending_fees_amount")
                .map_err(dashboard_query_error)?,
        })
    }
}

impl DashboardRepoImpl {
    async fn current_period_id(&self, org_id: Uuid) -> Result<Option<Uuid>, DomainError> {
        let calendar = self
            .db
            .query_one(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                "SELECT id, name, current_period_id FROM academic_calendars WHERE org_id = $1 AND status = 'active' ORDER BY created_at DESC LIMIT 1",
                [org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let calendar = match calendar {
            Some(calendar) => calendar,
            None => {
                match self
                    .db
                    .query_one(Statement::from_sql_and_values(
                        self.db.get_database_backend(),
                        "SELECT id, name, current_period_id FROM academic_calendars WHERE org_id = $1 ORDER BY created_at DESC LIMIT 1",
                        [org_id.into()],
                    ))
                    .await
                    .map_err(|e| DomainError::Internal(e.to_string()))?
                {
                    Some(calendar) => calendar,
                    None => return Ok(None),
                }
            }
        };

        let current_period_id = calendar
            .try_get::<Option<Uuid>>("", "current_period_id")
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        if current_period_id.is_some() {
            return Ok(current_period_id);
        }

        let calendar_id: Uuid = calendar
            .try_get("", "id")
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let period = self
            .db
            .query_one(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                "SELECT id FROM academic_periods WHERE org_id = $1 AND calendar_id = $2 AND status = 'active' ORDER BY sort_order ASC, created_at ASC LIMIT 1",
                [org_id.into(), calendar_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        period
            .map(|row| row.try_get("", "id"))
            .transpose()
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn enrollment_metrics(
        &self,
        org_id: Uuid,
        current_period_id: Option<Uuid>,
    ) -> Result<(u64, f64), DomainError> {
        let eligible_students = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM students WHERE org_id = $1 AND is_deleted = FALSE AND status = 'active'",
                org_id,
            )
            .await?;

        let enrolled_students = match current_period_id {
            Some(period_id) => {
                let result = self
                    .db
                    .query_one(Statement::from_sql_and_values(
                        self.db.get_database_backend(),
                        "SELECT COUNT(DISTINCT e.student_id) as cnt FROM enrollments e INNER JOIN students s ON s.id = e.student_id AND s.org_id = e.org_id WHERE e.org_id = $1 AND e.period_id = $2 AND e.status IN ('active', 'pending_approval') AND s.is_deleted = FALSE AND s.status = 'active'",
                        [org_id.into(), period_id.into()],
                    ))
                    .await
                    .map_err(|e| DomainError::Internal(e.to_string()))?;

                match result {
                    Some(row) => {
                        let count: i64 = row
                            .try_get("", "cnt")
                            .map_err(|e| DomainError::Internal(e.to_string()))?;
                        u64::try_from(count).map_err(|e| {
                            DomainError::Internal(format!("Invalid enrollment count: {e}"))
                        })?
                    }
                    None => {
                        return Err(DomainError::Internal(
                            "Enrollment count query returned no row".into(),
                        ));
                    }
                }
            }
            None => 0,
        };

        let enrollment_rate = if eligible_students == 0 {
            0.0
        } else {
            enrolled_students as f64 / eligible_students as f64 * 100.0
        };

        Ok((enrolled_students, enrollment_rate))
    }

    async fn count_query(&self, sql: &str, org_id: Uuid) -> Result<u64, DomainError> {
        let result = self
            .db
            .query_one(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                sql,
                [org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        match result {
            Some(row) => {
                let cnt: i64 = row
                    .try_get("", "cnt")
                    .map_err(|e| DomainError::Internal(e.to_string()))?;
                u64::try_from(cnt)
                    .map_err(|e| DomainError::Internal(format!("Invalid dashboard count: {e}")))
            }
            None => Err(DomainError::Internal(
                "Dashboard count query returned no row".into(),
            )),
        }
    }
}

fn dashboard_query_error(error: sea_orm::DbErr) -> DomainError {
    DomainError::Internal(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        migration::{Migrator, MigratorTrait},
        models::{organization, section},
    };
    use chrono::Utc;
    use sea_orm::{ActiveModelTrait, Database, Set};

    #[tokio::test]
    async fn enrollment_metrics_count_unique_active_students_in_current_period() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE students (id TEXT NOT NULL, org_id TEXT NOT NULL, status TEXT NOT NULL, is_deleted BOOLEAN NOT NULL)",
        )
        .await
        .unwrap();
        db.execute_unprepared(
            "CREATE TABLE enrollments (org_id TEXT NOT NULL, period_id TEXT NOT NULL, student_id TEXT NOT NULL, status TEXT NOT NULL)",
        )
        .await
        .unwrap();

        let org_id = Uuid::new_v4();
        let current_period_id = Uuid::new_v4();
        let other_period_id = Uuid::new_v4();
        let active_student = Uuid::new_v4();
        let other_active_student = Uuid::new_v4();
        let inactive_student = Uuid::new_v4();
        let deleted_student = Uuid::new_v4();

        for (student_id, status, is_deleted) in [
            (active_student, "active", false),
            (other_active_student, "active", false),
            (inactive_student, "inactive", false),
            (deleted_student, "active", true),
        ] {
            db.execute(Statement::from_sql_and_values(
                db.get_database_backend(),
                "INSERT INTO students (id, org_id, status, is_deleted) VALUES ($1, $2, $3, $4)",
                [
                    student_id.into(),
                    org_id.into(),
                    status.into(),
                    is_deleted.into(),
                ],
            ))
            .await
            .unwrap();
        }

        for (student_id, period_id, status) in [
            (active_student, current_period_id, "active"),
            (active_student, current_period_id, "pending_approval"),
            (other_active_student, other_period_id, "active"),
            (inactive_student, current_period_id, "active"),
            (deleted_student, current_period_id, "active"),
        ] {
            db.execute(Statement::from_sql_and_values(
                db.get_database_backend(),
                "INSERT INTO enrollments (org_id, period_id, student_id, status) VALUES ($1, $2, $3, $4)",
                [
                    org_id.into(),
                    period_id.into(),
                    student_id.into(),
                    status.into(),
                ],
            ))
            .await
            .unwrap();
        }

        let repo = DashboardRepoImpl { db };
        let (enrolled_students, enrollment_rate) = repo
            .enrollment_metrics(org_id, Some(current_period_id))
            .await
            .unwrap();

        assert_eq!(enrolled_students, 1);
        assert_eq!(enrollment_rate, 50.0);
    }

    #[tokio::test]
    async fn dashboard_counts_classes_from_unified_sections_table() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        check_dashboard_and_autopilot_on_migrated_database(db).await;
    }

    #[tokio::test]
    #[ignore = "requires a disposable PostgreSQL database"]
    async fn postgres_migrations_dashboard_and_autopilot() {
        let url = std::env::var("ORBIS_TEST_POSTGRES_URL").unwrap();
        let db = Database::connect(url).await.unwrap();
        check_dashboard_and_autopilot_on_migrated_database(db).await;
    }

    async fn check_dashboard_and_autopilot_on_migrated_database(db: DatabaseConnection) {
        use crate::autopilot::task_repo::PgAutopilotTaskRepo;
        use orbis_app::ports::repositories::AutopilotTaskRepository;
        use orbis_domain::autopilot::task::{AutopilotTask, RiskLevel, TaskStatus, TaskType};

        Migrator::up(&db, None).await.unwrap();

        let org_id = Uuid::new_v4();
        let now = Utc::now();
        organization::ActiveModel {
            id: Set(org_id),
            name: Set("Test organization".to_owned()),
            name_ar: Set(None),
            slug: Set(format!("test-{}", org_id)),
            code: Set(None),
            institution_type: Set("school".to_owned()),
            timezone: Set("UTC".to_owned()),
            locale: Set("en".to_owned()),
            date_format: Set(None),
            format_locale: Set(None),
            time_format: Set(None),
            is_hijri_enabled: Set(false),
            base_currency_code: Set(None),
            settings: Set(None),
            features: Set(None),
            logo: Set(None),
            favicon: Set(None),
            primary_color: Set(None),
            is_active: Set(true),
            is_verified: Set(false),
            is_deleted: Set(false),
            deleted_at: Set(None),
            deleted_by: Set(None),
            current_period_id: Set(None),
            metadata: Set(None),
            created_by: Set(None),
            updated_by: Set(None),
            version: Set(1),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(&db)
        .await
        .unwrap();

        for (section_type, is_deleted) in
            [("class", false), ("class", true), ("course_section", false)]
        {
            section::ActiveModel {
                id: Set(Uuid::new_v4()),
                org_id: Set(org_id),
                org_unit_id: Set(None),
                section_type: Set(section_type.to_owned()),
                code: Set(Uuid::new_v4().to_string()),
                name: Set("Test section".to_owned()),
                name_ar: Set(None),
                level: Set(None),
                track: Set(None),
                capacity: Set(None),
                color: Set(None),
                is_active: Set(true),
                is_deleted: Set(is_deleted),
                deleted_at: Set(None),
                created_at: Set(now),
                updated_at: Set(now),
            }
            .insert(&db)
            .await
            .unwrap();
        }

        let repo = DashboardRepoImpl { db: db.clone() };
        let counts = repo.get_counts(org_id).await.unwrap();
        assert_eq!(counts.total_classes, 1);
        assert_eq!(counts.active_sections, 2);
        assert_eq!(counts.active_academic_calendar, None);
        assert_eq!(counts.total_payments_amount, 0.0);
        assert_eq!(counts.pending_fees_amount, 0.0);

        let task_repo = PgAutopilotTaskRepo::new(db);
        let mut task = AutopilotTask::new(
            org_id,
            TaskType::PeriodActivate,
            Some(Uuid::new_v4()),
            None,
            RiskLevel::Medium,
            None,
            now,
        );
        task.idempotency_key = Some("policy:period_activate:test".into());
        let mut duplicate = task.clone();
        duplicate.id = Uuid::new_v4();
        let (first, second) = tokio::join!(
            task_repo.schedule_task(&task),
            task_repo.schedule_task(&duplicate),
        );
        let scheduled = first.unwrap();
        assert_eq!(scheduled.id, second.unwrap().id);
        assert!(
            task_repo
                .claim_task(scheduled.id, org_id, TaskStatus::Pending, 0)
                .await
                .unwrap()
        );

        let mut dismissal = scheduled;
        dismissal.status = TaskStatus::Cancelled;
        assert!(matches!(
            task_repo
                .update_task_state(&dismissal, TaskStatus::Pending)
                .await,
            Err(DomainError::Conflict(_))
        ));
        assert_eq!(
            task_repo
                .find_by_id(dismissal.id)
                .await
                .unwrap()
                .unwrap()
                .status,
            TaskStatus::Executing,
        );
        assert_eq!(
            task_repo.count_by_status(org_id).await.unwrap(),
            vec![(TaskStatus::Executing, 1)]
        );
    }
}
