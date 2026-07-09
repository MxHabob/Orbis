use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::entities::academic_cycle;
use crate::repository::academic_cycle::CycleUpdate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateCycleRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub cycle_type: String,
    pub parent_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    pub grading_system: Option<String>,
    pub passing_grade: Option<i32>,
    pub max_grade: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateCycleRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub exam_start_date: Option<String>,
    pub exam_end_date: Option<String>,
    pub grading_system: Option<String>,
    pub passing_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub is_enrollment_open: Option<i32>,
}

impl super::Service {
    pub async fn get_cycles(
        &self,
        org_id: &str,
        params: crate::params::CycleParams,
    ) -> Result<Vec<academic_cycle::Model>, String> {
        self.repo.get_cycles(org_id, params).await
            .map_err(|e| e.to_string())
    }

    pub async fn get_cycle(&self, cycle_id: &str) -> Result<academic_cycle::Model, String> {
        self.repo.get_cycle_by_id(cycle_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "academic cycle not found".to_string())
    }

    pub async fn create_cycle(&self, org_id: &str, req: CreateCycleRequest) -> Result<academic_cycle::Model, String> {
        let valid_types = ["year", "semester", "trimester", "quarter", "monthly", "custom"];
        if !valid_types.contains(&req.cycle_type.as_str()) {
            return Err(format!("invalid cycle type: {}. Valid: {:?}", req.cycle_type, valid_types));
        }

        let cycle = academic_cycle::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            cycle_type: req.cycle_type,
            parent_id: req.parent_id,
            start_date: req.start_date,
            end_date: req.end_date,
            hijri_start_date: req.hijri_start_date,
            hijri_end_date: req.hijri_end_date,
            exam_start_date: None,
            exam_end_date: None,
            weight: Some(100),
            grading_system: req.grading_system.unwrap_or_else(|| "percentage".to_string()),
            passing_grade: req.passing_grade.or(Some(50)),
            max_grade: req.max_grade.or(Some(100)),
            is_current: 0,
            is_locked: 0,
            is_enrollment_open: 1,
            is_grade_entry_open: 0,
            grade_entry_start: None,
            grade_entry_end: None,
            locked_at: None,
            locked_by: None,
            settings: Some("{}".to_string()),
            sort_order: Some(0),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        self.repo.create_academic_cycle(cycle).await
            .map_err(|e| e.to_string())
    }

    pub async fn update_cycle(&self, cycle_id: &str, req: UpdateCycleRequest) -> Result<academic_cycle::Model, String> {
        // Check if cycle is locked - prevent updates on locked cycles (except unlock)
        let existing = self.repo.get_cycle_by_id(cycle_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "الدورة الأكاديمية غير موجودة".to_string())?;

        if existing.is_locked == 1 {
            return Err("لا يمكن تعديل دورة مقفلة. يجب فتح القفل أولاً.".to_string());
        }

        let updates = CycleUpdate {
            name: req.name,
            name_ar: req.name_ar,
            start_date: req.start_date,
            end_date: req.end_date,
            exam_start_date: req.exam_start_date,
            exam_end_date: req.exam_end_date,
            grading_system: req.grading_system,
            passing_grade: req.passing_grade,
            max_grade: req.max_grade,
            is_enrollment_open: req.is_enrollment_open,
            ..Default::default()
        };

        self.repo.update_cycle(cycle_id, updates).await
            .map_err(|e| e.to_string())
    }

    pub async fn activate_cycle(&self, org_id: &str, cycle_id: &str) -> Result<academic_cycle::Model, String> {
        // Deactivate all cycles first
        self.repo.deactivate_all_cycles(org_id).await
            .map_err(|e| e.to_string())?;

        // Activate the target
        let updates = CycleUpdate {
            is_current: Some(1),
            ..Default::default()
        };

        self.repo.update_cycle(cycle_id, updates).await
            .map_err(|e| e.to_string())
    }

    /// Lock cycle: closes grade entry + enrollment, records who locked and when
    pub async fn lock_cycle(&self, cycle_id: &str, locked_by: &str) -> Result<academic_cycle::Model, String> {
        let updates = CycleUpdate {
            is_locked: Some(1),
            is_enrollment_open: Some(0),
            is_grade_entry_open: Some(0),
            locked_at: Some(Utc::now().to_rfc3339()),
            locked_by: Some(locked_by.to_string()),
            ..Default::default()
        };

        self.repo.update_cycle(cycle_id, updates).await
            .map_err(|e| e.to_string())
    }

    /// Unlock cycle: only admin/super_admin should call this
    pub async fn unlock_cycle(&self, cycle_id: &str) -> Result<academic_cycle::Model, String> {
        let existing = self.repo.get_cycle_by_id(cycle_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "الدورة الأكاديمية غير موجودة".to_string())?;

        if existing.is_locked == 0 {
            return Err("الدورة غير مقفلة أصلاً".to_string());
        }

        let updates = CycleUpdate {
            is_locked: Some(0),
            locked_at: Some("".to_string()),
            locked_by: Some("".to_string()),
            ..Default::default()
        };

        self.repo.update_cycle(cycle_id, updates).await
            .map_err(|e| e.to_string())
    }

    /// Open grade entry period with optional date range
    pub async fn open_grade_entry(&self, cycle_id: &str, start: Option<String>, end: Option<String>) -> Result<academic_cycle::Model, String> {
        let existing = self.repo.get_cycle_by_id(cycle_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "الدورة الأكاديمية غير موجودة".to_string())?;

        if existing.is_locked == 1 {
            return Err("لا يمكن فتح فترة الإدخال لدورة مقفلة. يجب فتح القفل أولاً.".to_string());
        }

        let mut updates = CycleUpdate {
            is_grade_entry_open: Some(1),
            ..Default::default()
        };

        if let Some(s) = start {
            updates.grade_entry_start = Some(s);
        }
        if let Some(e) = end {
            updates.grade_entry_end = Some(e);
        }

        self.repo.update_cycle(cycle_id, updates).await
            .map_err(|e| e.to_string())
    }

    /// Close grade entry period
    pub async fn close_grade_entry(&self, cycle_id: &str) -> Result<academic_cycle::Model, String> {
        let updates = CycleUpdate {
            is_grade_entry_open: Some(0),
            ..Default::default()
        };

        self.repo.update_cycle(cycle_id, updates).await
            .map_err(|e| e.to_string())
    }
}

