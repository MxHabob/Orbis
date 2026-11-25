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
    async fn get_counts(
        &self,
        org_id: Uuid,
        units: Option<&[Uuid]>,
        include_finance: bool,
    ) -> Result<DashboardCounts, DomainError> {
        self.get_counts_for_period(
            org_id,
            units,
            include_finance,
            self.current_period_id(org_id).await?,
        )
        .await
    }
}

impl DashboardRepoImpl {
    async fn get_counts_for_period(
        &self,
        org_id: Uuid,
        units: Option<&[Uuid]>,
        include_finance: bool,
        period_id: Option<Uuid>,
    ) -> Result<DashboardCounts, DomainError> {
        let mut values = vec![org_id.into(), period_id.into(), include_finance.into()];
        let unit_predicate = match units {
            Some([]) => "FALSE".to_owned(),
            Some(ids) => {
                let parameters = (4..4 + ids.len())
                    .map(|index| format!("${index}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                values.extend(ids.iter().copied().map(sea_orm::Value::from));
                format!("org_unit_id IN ({parameters})")
            }
            None => "TRUE".to_owned(),
        };
        let teacher_predicate = if units.is_some() {
            "AND user_id IN (SELECT user_id FROM scoped_employees)"
        } else {
            ""
        };

        let sql = format!(
            r#"
            WITH scoped_students AS (
                SELECT id, status FROM students
                WHERE org_id = $1 AND is_deleted = FALSE AND ({unit_predicate})
            ), scoped_employees AS (
                SELECT user_id FROM employees
                WHERE org_id = $1 AND is_deleted = FALSE AND ({unit_predicate})
            ), scoped_sections AS (
                SELECT id, is_active, section_type FROM sections
                WHERE org_id = $1 AND is_deleted = FALSE AND ({unit_predicate})
            ), scoped_enrollments AS (
                SELECT e.student_id, e.period_id, e.status, s.status AS student_status
                FROM enrollments e JOIN scoped_students s ON s.id = e.student_id
                WHERE e.org_id = $1
                    AND (e.section_id IS NULL OR e.section_id IN (SELECT id FROM scoped_sections) OR {organization_wide})
            )
            SELECT
                (SELECT COUNT(*) FROM scoped_students) AS total_students,
                (SELECT COUNT(*) FROM scoped_students WHERE status = 'active') AS eligible_students,
                (SELECT COUNT(DISTINCT student_id) FROM scoped_enrollments
                    WHERE period_id = $2 AND status IN ('active', 'pending_approval') AND student_status = 'active') AS enrolled_students,
                (SELECT COUNT(*) FROM scoped_employees) AS total_employees,
                (SELECT COUNT(*) FROM scoped_sections WHERE is_active = TRUE) AS active_sections,
                (SELECT COUNT(*) FROM scoped_sections WHERE section_type = 'class') AS total_classes,
                (SELECT COUNT(DISTINCT user_id) FROM user_role_assignments
                    WHERE org_id = $1 AND LOWER(role_id) LIKE '%teacher%' {teacher_predicate}) AS total_teachers,
                (SELECT COUNT(*) FROM enrollments WHERE org_id = $1 AND {organization_wide}) +
                    (SELECT COUNT(*) FROM scoped_enrollments WHERE NOT {organization_wide}) AS total_enrollments,
                (SELECT COUNT(*) FROM subjects WHERE org_id = $1) AS total_subjects,
                (SELECT name FROM academic_calendars WHERE org_id = $1
                    ORDER BY CASE WHEN status = 'active' THEN 0 ELSE 1 END, created_at DESC, id
                    LIMIT 1) AS active_academic_calendar,
                CAST(CASE WHEN $3 THEN COALESCE((SELECT SUM(amount)
                    FROM payments WHERE org_id = $1 AND ({unit_predicate})), 0) ELSE 0 END AS DOUBLE PRECISION) AS total_payments_amount,
                CAST(CASE WHEN $3 THEN COALESCE((SELECT SUM(net_amount)
                    FROM student_fees WHERE org_id = $1 AND status != 'paid'
                    AND ({organization_wide} OR student_id IN (SELECT id FROM scoped_students))), 0)
                    ELSE 0 END AS DOUBLE PRECISION) AS pending_fees_amount
        "#,
            organization_wide = if units.is_none() { "TRUE" } else { "FALSE" }
        );

        let row = self
            .db
            .query_one(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                sql,
                values,
            ))
            .await
            .map_err(dashboard_query_error)?
            .ok_or_else(|| DomainError::Internal("Dashboard aggregate returned no row".into()))?;
        let count = |column| {
            let value = row
                .try_get::<i64>("", column)
                .map_err(dashboard_query_error)?;
            u64::try_from(value).map_err(|error| DomainError::Internal(error.to_string()))
        };
        let enrolled_students = count("enrolled_students")?;
        let eligible_students = count("eligible_students")?;
        let enrollment_rate = if eligible_students == 0 {
            0.0
        } else {
            enrolled_students as f64 / eligible_students as f64 * 100.0
        };

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
}

fn dashboard_query_error(error: sea_orm::DbErr) -> DomainError {
    DomainError::Internal(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        migration::{Migrator, MigratorTrait},
        models::{org_unit, organization, role_permission, section, user_role_assignment},
    };
    use chrono::Utc;
    use sea_orm::{ActiveModelTrait, Database, Set};

    #[tokio::test]
    async fn enrollment_metrics_count_unique_active_students_in_current_period() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE students (id TEXT NOT NULL, org_id TEXT NOT NULL, status TEXT NOT NULL, is_deleted BOOLEAN NOT NULL, org_unit_id TEXT)",
        )
        .await
        .unwrap();
        db.execute_unprepared(
            "CREATE TABLE enrollments (org_id TEXT NOT NULL, period_id TEXT NOT NULL, student_id TEXT NOT NULL, status TEXT NOT NULL, section_id TEXT)",
        )
        .await
        .unwrap();
        db.execute_unprepared(
            "CREATE TABLE employees (org_id TEXT, user_id TEXT, is_deleted BOOLEAN, org_unit_id TEXT);
             CREATE TABLE sections (org_id TEXT, id TEXT, is_active BOOLEAN, is_deleted BOOLEAN, section_type TEXT, org_unit_id TEXT);
             CREATE TABLE user_role_assignments (org_id TEXT, user_id TEXT, role_id TEXT);
             CREATE TABLE subjects (org_id TEXT);
             CREATE TABLE academic_calendars (org_id TEXT, id TEXT, name TEXT, status TEXT, created_at TEXT);
             CREATE TABLE payments (org_id TEXT, amount REAL, org_unit_id TEXT);
             CREATE TABLE student_fees (org_id TEXT, student_id TEXT, status TEXT, net_amount REAL);",
        ).await.unwrap();

        let org_id = Uuid::new_v4();
        let current_period_id = Uuid::new_v4();
        let other_period_id = Uuid::new_v4();
        let active_student = Uuid::new_v4();
        let other_active_student = Uuid::new_v4();
        let inactive_student = Uuid::new_v4();
        let deleted_student = Uuid::new_v4();
        let branch_a = Uuid::new_v4();
        let branch_b = Uuid::new_v4();

        for (student_id, status, is_deleted, unit) in [
            (active_student, "active", false, branch_a),
            (other_active_student, "active", false, branch_b),
            (inactive_student, "inactive", false, branch_a),
            (deleted_student, "active", true, branch_a),
        ] {
            db.execute(Statement::from_sql_and_values(
                db.get_database_backend(),
                "INSERT INTO students (id, org_id, status, is_deleted, org_unit_id) VALUES ($1, $2, $3, $4, $5)",
                [
                    student_id.into(),
                    org_id.into(),
                    status.into(),
                    is_deleted.into(),
                    unit.into(),
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
        let counts = repo
            .get_counts_for_period(org_id, None, false, Some(current_period_id))
            .await
            .unwrap();

        assert_eq!(counts.enrolled_students, 1);
        assert_eq!(counts.enrollment_rate, 50.0);
        let branch_counts = repo
            .get_counts_for_period(org_id, Some(&[branch_a]), false, Some(current_period_id))
            .await
            .unwrap();
        assert_eq!(branch_counts.total_students, 2);
        assert_eq!(branch_counts.total_enrollments, 3);
        assert_eq!(branch_counts.enrolled_students, 1);
        assert_eq!(branch_counts.enrollment_rate, 100.0);
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

        let branch_a = Uuid::new_v4();
        let branch_b = Uuid::new_v4();
        for (id, name) in [(branch_a, "A"), (branch_b, "B")] {
            org_unit::ActiveModel {
                id: Set(id),
                org_id: Set(org_id),
                path: Set(id.to_string()),
                depth: Set(0),
                name: Set(name.into()),
                unit_type: Set("branch".into()),
                data_isolation: Set("isolated".into()),
                children_count: Set(0),
                descendant_count: Set(0),
                is_active: Set(true),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            }
            .insert(&db)
            .await
            .unwrap();
        }
        for (section_type, is_deleted, unit) in [
            ("class", false, branch_a),
            ("class", true, branch_a),
            ("course_section", false, branch_b),
        ] {
            section::ActiveModel {
                id: Set(Uuid::new_v4()),
                org_id: Set(org_id),
                org_unit_id: Set(Some(unit)),
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
        let counts = repo.get_counts(org_id, None, true).await.unwrap();
        assert_eq!(counts.total_classes, 1);
        assert_eq!(counts.active_sections, 2);
        assert_eq!(counts.active_academic_calendar, None);
        assert_eq!(counts.total_payments_amount, 0.0);
        assert_eq!(counts.pending_fees_amount, 0.0);
        let restricted = repo
            .get_counts(org_id, Some(&[Uuid::new_v4()]), false)
            .await
            .unwrap();
        assert_eq!(restricted.total_classes, 0);
        assert_eq!(restricted.active_sections, 0);
        assert_eq!(restricted.total_payments_amount, 0.0);

        use crate::{
            adapters::iam::db_permission_checker::DbPermissionChecker,
            event_bus::MemoryEventBus,
            repositories::{
                org_repo::OrgRepoImpl, org_unit_repo::OrgUnitRepoImpl,
                settings_repo::SettingsRepoImpl,
            },
        };
        use orbis_app::{
            context::{ActionSource, RequestContext},
            services::organization_service::{OrganizationService, UpdateOrgUnitRequest},
        };
        use orbis_domain::iam::actor::Actor;
        use std::sync::Arc;

        let user_id = Uuid::new_v4();
        user_role_assignment::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            org_id: Set(org_id),
            role_id: Set("branch_manager".into()),
            scope_type: Set("org_unit".into()),
            scope_id: Set(Some(branch_a)),
            assigned_by: Set(None),
            created_at: Set(now),
        }
        .insert(&db)
        .await
        .unwrap();
        for (resource, action) in [("dashboard", "read"), ("org_unit", "write")] {
            role_permission::ActiveModel {
                id: Set(Uuid::new_v4()),
                org_id: Set(org_id),
                role_id: Set("branch_manager".into()),
                resource: Set(resource.into()),
                action: Set(action.into()),
                condition: Set(None),
                created_at: Set(now),
            }
            .insert(&db)
            .await
            .unwrap();
        }
        let service = OrganizationService::new(
            Arc::new(OrgRepoImpl { db: db.clone() }),
            Arc::new(OrgUnitRepoImpl { db: db.clone() }),
            Arc::new(SettingsRepoImpl { db: db.clone() }),
            Arc::new(repo),
            Arc::new(MemoryEventBus::new()),
            Arc::new(DbPermissionChecker::new(db.clone())),
        );
        let ctx = RequestContext {
            tenant_id: org_id,
            actor: Actor::User(user_id),
            request_id: "branch-test".into(),
            source: ActionSource::Api,
            org_unit_path: None,
        };
        let access = service.get_dashboard_access(&ctx).await.unwrap();
        assert!(!access.organization_wide);
        assert_eq!(
            access.units.iter().map(|unit| unit.id).collect::<Vec<_>>(),
            vec![branch_a]
        );
        assert_eq!(
            service
                .get_dashboard_counts(&ctx, None)
                .await
                .unwrap()
                .active_sections,
            1
        );
        assert!(matches!(
            service.get_dashboard_counts(&ctx, Some(branch_b)).await,
            Err(DomainError::Forbidden(_))
        ));
        assert!(
            service
                .get_org_unit(&ctx, branch_b)
                .await
                .unwrap()
                .is_none()
        );
        assert!(matches!(
            service
                .update_org_unit(
                    &ctx,
                    branch_b,
                    UpdateOrgUnitRequest {
                        name: Some("Rejected".into()),
                        ..Default::default()
                    }
                )
                .await,
            Err(DomainError::NotFound(_))
        ));
        assert_eq!(
            service
                .update_org_unit(
                    &ctx,
                    branch_a,
                    UpdateOrgUnitRequest {
                        name: Some("Allowed".into()),
                        ..Default::default()
                    }
                )
                .await
                .unwrap()
                .name,
            "Allowed"
        );

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
