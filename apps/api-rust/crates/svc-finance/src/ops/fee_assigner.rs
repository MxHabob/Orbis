//! Concrete `FeeAssigner` implementation — assigns mandatory fees on enrollment.

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use tracing::info;

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
    ) -> Result<Vec<String>, AppError> {
        use crate::entities::{fee_template, student_fee};

        // Find mandatory active fee templates for this org
        let templates = fee_template::Entity::find()
            .filter(fee_template::Column::OrgId.eq(org_id))
            .filter(fee_template::Column::IsMandatory.eq(1))
            .filter(fee_template::Column::IsActive.eq(1))
            .all(&self.db)
            .await?;

        if templates.is_empty() {
            return Ok(vec![]);
        }

        let mut assigned_ids = Vec::new();

        for template in &templates {
            // Check if template applies to this class (via applicable_levels)
            if let Some(ref levels) = template.applicable_levels {
                if !levels.is_empty() && levels != "[]" {
                    let levels_list: Vec<String> = serde_json::from_str(levels)
                        .unwrap_or_default();
                    if !levels_list.is_empty() && !levels_list.contains(&class_id.to_string()) {
                        continue;
                    }
                }
            }

            let fee_id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now().naive_utc().to_string();

            let new_fee = student_fee::ActiveModel {
                id: Set(fee_id.clone()),
                org_id: Set(org_id.to_string()),
                period_id: Set(String::new()), // Will be set by caller
                enrollment_id: Set(enrollment_id.to_string()),
                fee_template_id: Set(template.id.clone()),
                original_amount: Set(template.default_amount),
                discount_amount: Set(0.0),
                net_amount: Set(template.default_amount),
                status: Set("pending".to_string()),
                due_date: Set(now.clone()),
                discount_details: Set(None),
                parent_fee_id: Set(None),
                installment_number: Set(None),
                notes: Set(None),
                created_at: Set(now.clone()),
                updated_at: Set(now),
            };

            new_fee.insert(&self.db).await?;
            assigned_ids.push(fee_id);
        }

        info!(
            org_id = org_id,
            enrollment_id = enrollment_id,
            count = assigned_ids.len(),
            "Assigned mandatory fees"
        );

        Ok(assigned_ids)
    }
}
