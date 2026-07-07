//! Discount service — fee discount structures and scholarship definitions.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use shared_kernel::errors::AppError;
use crate::entities::discount_template;
use crate::repository::{FinanceCommandRepo, FinanceQueryRepo};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateDiscountTemplateRequest {
    pub name: String,
    pub code: Option<String>,
    pub item_type: String, // e.g. system, manual, scholarship
    pub calculation_method: String, // percentage, fixed
    pub value: i32,
    pub eligibility_rules: Option<String>,
    pub limits: Option<String>,
    pub requires_approval: i32,
    pub approver_roles: Option<String>,
    pub is_auto_apply: i32,
    pub is_stackable: i32,
    pub priority: Option<i32>,
    pub is_active: i32,
}

pub struct DiscountService {
    pub cmd: Arc<FinanceCommandRepo>,
    pub query: Arc<FinanceQueryRepo>,
}

impl DiscountService {
    pub fn new(cmd: Arc<FinanceCommandRepo>, query: Arc<FinanceQueryRepo>) -> Self {
        Self { cmd, query }
    }

    pub async fn get_active_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, AppError> {
        Ok(self.query.get_discount_templates(org_id, Some(1)).await?)
    }

    pub async fn get_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, AppError> {
        Ok(self.query.get_discount_templates(org_id, None).await?)
    }

    pub async fn create_discount_template(&self, org_id: &str, req: CreateDiscountTemplateRequest) -> Result<discount_template::Model, AppError> {
        let tpl = discount_template::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            code: req.code,
            item_type: req.item_type,
            calculation_method: req.calculation_method,
            value: req.value,
            eligibility_rules: req.eligibility_rules,
            limits: req.limits,
            requires_approval: req.requires_approval,
            approver_roles: req.approver_roles,
            is_auto_apply: req.is_auto_apply,
            is_stackable: req.is_stackable,
            priority: req.priority,
            is_active: req.is_active,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        Ok(self.cmd.create_discount_template(tpl).await?)
    }

    pub async fn update_discount_template(&self, org_id: &str, id: &str, req: CreateDiscountTemplateRequest) -> Result<discount_template::Model, AppError> {
        let tpl = self.query.get_discount_template_by_id(id).await?
            .ok_or_else(|| AppError::not_found("discount_template", id))?;

        if tpl.org_id != org_id {
            return Err(AppError::forbidden("discount_template", "update"));
        }

        let mut active: discount_template::ActiveModel = tpl.into();
        active.name = sea_orm::ActiveValue::Set(req.name);
        active.code = sea_orm::ActiveValue::Set(req.code);
        active.item_type = sea_orm::ActiveValue::Set(req.item_type);
        active.calculation_method = sea_orm::ActiveValue::Set(req.calculation_method);
        active.value = sea_orm::ActiveValue::Set(req.value);
        active.eligibility_rules = sea_orm::ActiveValue::Set(req.eligibility_rules);
        active.limits = sea_orm::ActiveValue::Set(req.limits);
        active.requires_approval = sea_orm::ActiveValue::Set(req.requires_approval);
        active.approver_roles = sea_orm::ActiveValue::Set(req.approver_roles);
        active.is_auto_apply = sea_orm::ActiveValue::Set(req.is_auto_apply);
        active.is_stackable = sea_orm::ActiveValue::Set(req.is_stackable);
        active.priority = sea_orm::ActiveValue::Set(req.priority);
        active.is_active = sea_orm::ActiveValue::Set(req.is_active);
        active.updated_at = sea_orm::ActiveValue::Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_discount_template(active.into_value().unwrap()).await?)
    }

    pub async fn delete_discount_template(&self, org_id: &str, id: &str) -> Result<u64, AppError> {
        Ok(self.cmd.delete_discount_template(org_id, id).await?)
    }
}
