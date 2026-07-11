//! Student Fee operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::student_fee;

#[derive(serde::Deserialize)]
pub struct AssignStudentFeePayload {
    pub enrollment_id: String,
    pub fee_template_id: String,
    pub amount: f64,
    pub due_date: String,
    pub notes: Option<String>,
    pub cycle_id: String,
}

pub async fn list_by_enrollment(
    ctx: &ServiceContext,
    enrollment_id: &str,
) -> Result<Vec<student_fee::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(student_fee::Entity::find()
        .filter(student_fee::Column::OrgId.eq(&ctx.org_id))
        .filter(student_fee::Column::EnrollmentId.eq(enrollment_id))
        .order_by_asc(student_fee::Column::DueDate)
        .all(&ctx.db).await?)
}

pub async fn assign(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<student_fee::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let req: AssignStudentFeePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid assign fee payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = student_fee::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        cycle_id: req.cycle_id,
        enrollment_id: req.enrollment_id,
        fee_template_id: req.fee_template_id,
        original_amount: req.amount,
        discount_amount: 0.0,
        net_amount: req.amount,
        status: "pending".into(),
        due_date: req.due_date,
        discount_details: None,
        parent_fee_id: None,
        installment_number: None,
        notes: req.notes,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: student_fee::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;
    Ok(created)
}
