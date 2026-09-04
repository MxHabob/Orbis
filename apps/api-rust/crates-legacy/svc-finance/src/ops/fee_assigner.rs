//! Concrete `FeeAssigner` implementation — assigns mandatory fees on enrollment.

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use shared_kernel::errors::AppError;
use shared_kernel::traits::FeeAssigner;

/// Concrete `FeeAssigner` backed by `student_fees` and `fee_templates` tables.
pub struct FinanceFeeAssigner {
    db: DatabaseConnection,
}

impl FinanceFeeAssigner {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl FeeAssigner for FinanceFeeAssigner {
    async fn assign_mandatory_fees(
        &self,
        org_id: &str,
        enrollment_id: &str,
        class_id: &str,
        period_id: &str,
    ) -> Result<Vec<String>, AppError> {
        infra_persistence::fee_assignment::assign_mandatory_fees(
            &self.db,
            org_id,
            enrollment_id,
            class_id,
            period_id,
        )
        .await
    }
}

// Re-export for callers that participate in transactions (same infra implementation).
pub use infra_persistence::fee_assignment::assign_mandatory_fees;
