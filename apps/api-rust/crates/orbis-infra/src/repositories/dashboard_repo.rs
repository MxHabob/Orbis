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
        // Student records are the source of truth; role assignments may be absent.
        let total_students = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM students WHERE org_id = $1 AND is_deleted = FALSE",
                org_id,
            )
            .await?;

        let (enrolled_students, enrollment_rate) = self
            .enrollment_metrics(org_id, self.current_period_id(org_id).await?)
            .await?;

        let total_employees = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM employees WHERE org_id = $1 AND is_deleted = FALSE",
                org_id,
            )
            .await?;

        let active_sections = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM sections WHERE org_id = $1 AND is_active = TRUE AND is_deleted = FALSE",
                org_id,
            )
            .await?;

        // Count teachers from user_role_assignments
        let total_teachers = self.count_query(
            "SELECT COUNT(DISTINCT user_id) as cnt FROM user_role_assignments WHERE org_id = $1 AND (LOWER(role_id) LIKE '%teacher%')",
            org_id,
        ).await?;

        // Classes are stored as class-typed sections in the unified schema.
        let total_classes = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM sections WHERE org_id = $1 AND section_type = 'class' AND is_deleted = FALSE",
                org_id,
            )
            .await?;

        // Keep the all-period enrollment-record count for API consumers.
        let total_enrollments = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM enrollments WHERE org_id = $1",
                org_id,
            )
            .await?;

        // Count subjects
        let total_subjects = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM subjects WHERE org_id = $1",
                org_id,
            )
            .await?;

        // Active academic year (status = 'active')
        let active_year = {
            let result = self.db.query_one(
                Statement::from_sql_and_values(
                    self.db.get_database_backend(),
                    "SELECT name FROM academic_calendars WHERE org_id = $1 AND status = 'active' LIMIT 1",
                    [org_id.into()],
                ),
            ).await.map_err(|e| DomainError::Internal(e.to_string()))?;

            match result {
                Some(row) => row.try_get::<String>("", "name").ok(),
                None => {
                    // Fallback to most recent academic year if none is marked 'active'
                    let fallback = self.db.query_one(
                        Statement::from_sql_and_values(
                            self.db.get_database_backend(),
                            "SELECT name FROM academic_calendars WHERE org_id = $1 ORDER BY created_at DESC LIMIT 1",
                            [org_id.into()],
                        ),
                    ).await.map_err(|e| DomainError::Internal(e.to_string()))?;
                    fallback.and_then(|row| row.try_get::<String>("", "name").ok())
                }
            }
        };

        // Sum of payment amounts
        let total_payments_amount = self
            .sum_query(
                "SELECT CAST(COALESCE(SUM(amount), 0) AS DOUBLE PRECISION) as total FROM payments WHERE org_id = $1",
                org_id,
            )
            .await?;

        // Sum of pending fees
        let pending_fees_amount = self.sum_query(
            "SELECT CAST(COALESCE(SUM(net_amount), 0) AS DOUBLE PRECISION) as total FROM student_fees WHERE org_id = $1 AND status != 'paid'",
            org_id,
        ).await?;

        Ok(DashboardCounts {
            total_students,
            enrolled_students,
            enrollment_rate,
            total_employees,
            active_sections,
            total_teachers,
            total_classes,
            total_enrollments,
            total_subjects,
            active_academic_calendar: active_year,
            total_payments_amount,
            pending_fees_amount,
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

    async fn sum_query(&self, sql: &str, org_id: Uuid) -> Result<f64, DomainError> {
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
                let total: f64 = row
                    .try_get("", "total")
                    .map_err(|e| DomainError::Internal(e.to_string()))?;
                Ok(total)
            }
            None => Err(DomainError::Internal(
                "Dashboard sum query returned no row".into(),
            )),
        }
    }
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
    async fn dashboard_sum_query_decodes_empty_and_integer_sums_as_f64() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE payments (org_id TEXT NOT NULL, amount INTEGER NOT NULL)",
        )
        .await
        .unwrap();
        let repo = DashboardRepoImpl { db };
        let org_id = Uuid::new_v4();
        let query = "SELECT CAST(COALESCE(SUM(amount), 0) AS DOUBLE PRECISION) as total FROM payments WHERE org_id = $1";

        assert_eq!(repo.sum_query(query, org_id).await.unwrap(), 0.0);

        repo.db
            .execute(Statement::from_sql_and_values(
                repo.db.get_database_backend(),
                "INSERT INTO payments (org_id, amount) VALUES ($1, $2)",
                [org_id.into(), 125.into()],
            ))
            .await
            .unwrap();

        assert_eq!(repo.sum_query(query, org_id).await.unwrap(), 125.0);
    }

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

        let repo = DashboardRepoImpl { db };
        let total_classes = repo
            .count_query(
                "SELECT COUNT(*) as cnt FROM sections WHERE org_id = $1 AND section_type = 'class' AND is_deleted = FALSE",
                org_id,
            )
            .await
            .unwrap();
        let active_sections = repo
            .count_query(
                "SELECT COUNT(*) as cnt FROM sections WHERE org_id = $1 AND is_active = TRUE AND is_deleted = FALSE",
                org_id,
            )
            .await
            .unwrap();

        assert_eq!(total_classes, 1);
        assert_eq!(active_sections, 2);
    }
}
