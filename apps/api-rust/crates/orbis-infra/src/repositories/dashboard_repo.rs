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

        // Count classes
        let total_classes = self
            .count_query(
                "SELECT COUNT(*) as cnt FROM classes WHERE org_id = $1",
                org_id,
            )
            .await?;

        // Count enrollments
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
                "SELECT COALESCE(SUM(amount), 0) as total FROM payments WHERE org_id = $1",
                org_id,
            )
            .await?;

        // Sum of pending fees
        let pending_fees_amount = self.sum_query(
            "SELECT COALESCE(SUM(net_amount), 0) as total FROM student_fees WHERE org_id = $1 AND status != 'paid'",
            org_id,
        ).await?;

        Ok(DashboardCounts {
            total_students,
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
