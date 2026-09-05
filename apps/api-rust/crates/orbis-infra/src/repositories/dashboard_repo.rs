//! Infrastructure implementation of DashboardRepository.
//!
//! Uses raw SQL aggregation queries for performance — counting across
//! multiple tables in a single pass per table.

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbBackend, Statement, FromQueryResult, JsonValue};
use orbis_app::ports::repositories::{DashboardRepository, DashboardCounts};
use orbis_domain::errors::DomainError;
use uuid::Uuid;

pub struct DashboardRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl DashboardRepository for DashboardRepoImpl {
    async fn get_counts(&self, org_id: Uuid) -> Result<DashboardCounts, DomainError> {
        let org_str = org_id.to_string();

        // Count students (users with role = Student)
        let total_students = self.count_query(
            "SELECT COUNT(*) as cnt FROM users WHERE org_id = $1 AND role = 'Student'",
            &org_str,
        ).await?;

        // Count teachers
        let total_teachers = self.count_query(
            "SELECT COUNT(*) as cnt FROM users WHERE org_id = $1 AND role = 'Teacher'",
            &org_str,
        ).await?;

        // Count classes
        let total_classes = self.count_query(
            "SELECT COUNT(*) as cnt FROM classes WHERE org_id = $1",
            &org_str,
        ).await?;

        // Count enrollments
        let total_enrollments = self.count_query(
            "SELECT COUNT(*) as cnt FROM enrollments WHERE org_id = $1",
            &org_str,
        ).await?;

        // Count subjects
        let total_subjects = self.count_query(
            "SELECT COUNT(*) as cnt FROM subjects WHERE org_id = $1",
            &org_str,
        ).await?;

        // Active academic year
        let active_year = {
            let result = sea_orm::DatabaseConnection::query_one(
                &self.db,
                Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    "SELECT name FROM academic_years WHERE org_id = $1 AND is_active = true LIMIT 1",
                    [org_str.clone().into()],
                ),
            ).await.map_err(|e| DomainError::Internal(e.to_string()))?;

            result.and_then(|row| {
                row.try_get::<String>("", "name").ok()
            })
        };

        // Sum of payment amounts
        let total_payments_amount = self.sum_query(
            "SELECT COALESCE(SUM(amount), 0) as total FROM payments WHERE org_id = $1 AND status = 'completed'",
            &org_str,
        ).await?;

        // Sum of pending fees
        let pending_fees_amount = self.sum_query(
            "SELECT COALESCE(SUM(net_amount - paid_amount), 0) as total FROM student_fees WHERE org_id = $1 AND status != 'paid'",
            &org_str,
        ).await?;

        Ok(DashboardCounts {
            total_students,
            total_teachers,
            total_classes,
            total_enrollments,
            total_subjects,
            active_academic_year: active_year,
            total_payments_amount,
            pending_fees_amount,
        })
    }
}

impl DashboardRepoImpl {
    async fn count_query(&self, sql: &str, org_id: &str) -> Result<u64, DomainError> {
        let result = sea_orm::DatabaseConnection::query_one(
            &self.db,
            Statement::from_sql_and_values(
                DbBackend::Postgres,
                sql,
                [org_id.into()],
            ),
        ).await.map_err(|e| DomainError::Internal(e.to_string()))?;

        match result {
            Some(row) => {
                let cnt: i64 = row.try_get("", "cnt").unwrap_or(0);
                Ok(cnt as u64)
            }
            None => Ok(0),
        }
    }

    async fn sum_query(&self, sql: &str, org_id: &str) -> Result<f64, DomainError> {
        let result = sea_orm::DatabaseConnection::query_one(
            &self.db,
            Statement::from_sql_and_values(
                DbBackend::Postgres,
                sql,
                [org_id.into()],
            ),
        ).await.map_err(|e| DomainError::Internal(e.to_string()))?;

        match result {
            Some(row) => {
                let total: f64 = row.try_get("", "total").unwrap_or(0.0);
                Ok(total)
            }
            None => Ok(0.0),
        }
    }
}
