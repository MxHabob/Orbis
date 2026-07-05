//! Promotion Policy Service — per-organization promotion configuration.
//!
//! Manages the `promotion_policies` table which controls:
//! - Scoring thresholds (passing score, min attendance)
//! - Graduation rules (final level, auto-graduate)
//! - Fee carry-over mode (4 modes: ignore, carry_all, carry_selected, require_approval)
//! - Data carry-over (guardians, time slots, class subjects)
//! - Auto-create cycles in new year

use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};

use crate::entities::promotion_policy;

// ── Request DTOs ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePromotionPolicyRequest {
    pub passing_score: Option<i32>,
    pub min_attendance_rate: Option<f64>,
    pub auto_graduate_final_level: Option<i32>,
    pub final_level: Option<String>,
    pub exclude_withdrawn: Option<i32>,
    pub fee_carry_over_mode: Option<String>,
    pub fee_carry_over_categories: Option<String>,
    pub carry_over_guardians: Option<i32>,
    pub carry_over_time_slots: Option<i32>,
    pub carry_over_class_subjects: Option<i32>,
    pub auto_create_cycles: Option<i32>,
    pub cycle_template: Option<String>,
}

// ── Service ────────────────────────────────────────────────────────

impl super::Service {
    /// Get the promotion policy for an organization. Creates a default if none exists.
    pub async fn get_promotion_policy(
        &self,
        org_id: &str,
    ) -> Result<promotion_policy::Model, String> {
        let existing = promotion_policy::Entity::find()
            .filter(promotion_policy::Column::OrgId.eq(org_id))
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?;

        match existing {
            Some(policy) => Ok(policy),
            None => {
                let now = Utc::now().to_rfc3339();
                let default = promotion_policy::Model {
                    id: Uuid::new_v4().to_string(),
                    org_id: org_id.to_string(),
                    passing_score: 50,
                    min_attendance_rate: 75.0,
                    auto_graduate_final_level: 1,
                    final_level: None,
                    exclude_withdrawn: 1,
                    fee_carry_over_mode: "ignore".to_string(),
                    fee_carry_over_categories: None,
                    carry_over_guardians: 1,
                    carry_over_time_slots: 1,
                    carry_over_class_subjects: 1,
                    auto_create_cycles: 1,
                    cycle_template: None,
                    created_at: now.clone(),
                    updated_at: now,
                };

                let active: promotion_policy::ActiveModel = default.into();
                active
                    .insert(&self.repo.db)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }

    /// Update the promotion policy for an organization.
    pub async fn update_promotion_policy(
        &self,
        org_id: &str,
        req: UpdatePromotionPolicyRequest,
    ) -> Result<promotion_policy::Model, String> {
        // Ensure policy exists
        let policy = self.get_promotion_policy(org_id).await?;

        let mut active: promotion_policy::ActiveModel = policy.into();
        let now = Utc::now().to_rfc3339();

        // Validate fee mode
        if let Some(ref mode) = req.fee_carry_over_mode {
            let valid_modes = ["ignore", "carry_all", "carry_selected", "require_approval"];
            if !valid_modes.contains(&mode.as_str()) {
                return Err(format!(
                    "Invalid fee_carry_over_mode: '{}'. Valid: {:?}",
                    mode, valid_modes
                ));
            }
        }

        if let Some(v) = req.passing_score { active.passing_score = Set(v); }
        if let Some(v) = req.min_attendance_rate { active.min_attendance_rate = Set(v); }
        if let Some(v) = req.auto_graduate_final_level { active.auto_graduate_final_level = Set(v); }
        if let Some(v) = req.final_level { active.final_level = Set(Some(v)); }
        if let Some(v) = req.exclude_withdrawn { active.exclude_withdrawn = Set(v); }
        if let Some(v) = req.fee_carry_over_mode { active.fee_carry_over_mode = Set(v); }
        if let Some(v) = req.fee_carry_over_categories { active.fee_carry_over_categories = Set(Some(v)); }
        if let Some(v) = req.carry_over_guardians { active.carry_over_guardians = Set(v); }
        if let Some(v) = req.carry_over_time_slots { active.carry_over_time_slots = Set(v); }
        if let Some(v) = req.carry_over_class_subjects { active.carry_over_class_subjects = Set(v); }
        if let Some(v) = req.auto_create_cycles { active.auto_create_cycles = Set(v); }
        if let Some(v) = req.cycle_template { active.cycle_template = Set(Some(v)); }
        active.updated_at = Set(now);

        active
            .update(&self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }
}
