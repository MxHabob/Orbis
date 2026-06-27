use crate::entities::academic_cycle;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    pub async fn get_cycles(
        &self,
        org_id: &str,
        params: crate::params::CycleParams,
    ) -> Result<Vec<academic_cycle::Model>, DbErr> {
        let mut query = academic_cycle::Entity::find()
            .filter(academic_cycle::Column::OrgId.eq(org_id));

        if let Some(ref cycle_type) = params.cycle_type {
            if !cycle_type.trim().is_empty() {
                query = query.filter(academic_cycle::Column::CycleType.eq(cycle_type));
            }
        }

        if let Some(is_current) = params.is_current {
            query = query.filter(academic_cycle::Column::IsCurrent.eq(is_current));
        }

        query = query.order_by_desc(academic_cycle::Column::StartDate);

        query.all(&self.db).await
    }

    pub async fn get_cycle_by_id(&self, id: &str) -> Result<Option<academic_cycle::Model>, DbErr> {
        academic_cycle::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn update_cycle(&self, id: &str, updates: CycleUpdate) -> Result<academic_cycle::Model, DbErr> {
        let cycle = academic_cycle::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("academic cycle not found".to_string()))?;

        let mut active: academic_cycle::ActiveModel = cycle.into();

        if let Some(name) = updates.name {
            active.name = Set(name);
        }
        if let Some(name_ar) = updates.name_ar {
            active.name_ar = Set(Some(name_ar));
        }
        if let Some(start_date) = updates.start_date {
            active.start_date = Set(start_date);
        }
        if let Some(end_date) = updates.end_date {
            active.end_date = Set(end_date);
        }
        if let Some(exam_start_date) = updates.exam_start_date {
            active.exam_start_date = Set(Some(exam_start_date));
        }
        if let Some(exam_end_date) = updates.exam_end_date {
            active.exam_end_date = Set(Some(exam_end_date));
        }
        if let Some(grading_system) = updates.grading_system {
            active.grading_system = Set(grading_system);
        }
        if let Some(passing_grade) = updates.passing_grade {
            active.passing_grade = Set(Some(passing_grade));
        }
        if let Some(max_grade) = updates.max_grade {
            active.max_grade = Set(Some(max_grade));
        }
        if let Some(is_enrollment_open) = updates.is_enrollment_open {
            active.is_enrollment_open = Set(is_enrollment_open);
        }
        if let Some(is_locked) = updates.is_locked {
            active.is_locked = Set(is_locked);
        }
        if let Some(is_current) = updates.is_current {
            active.is_current = Set(is_current);
        }
        if let Some(is_grade_entry_open) = updates.is_grade_entry_open {
            active.is_grade_entry_open = Set(is_grade_entry_open);
        }
        if let Some(grade_entry_start) = updates.grade_entry_start {
            active.grade_entry_start = Set(Some(grade_entry_start));
        }
        if let Some(grade_entry_end) = updates.grade_entry_end {
            active.grade_entry_end = Set(Some(grade_entry_end));
        }
        if let Some(locked_at) = updates.locked_at {
            active.locked_at = Set(Some(locked_at));
        }
        if let Some(locked_by) = updates.locked_by {
            active.locked_by = Set(Some(locked_by));
        }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    pub async fn deactivate_all_cycles(&self, org_id: &str) -> Result<u64, DbErr> {
        let result = academic_cycle::Entity::update_many()
            .col_expr(academic_cycle::Column::IsCurrent, Expr::value(0))
            .filter(academic_cycle::Column::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn create_academic_cycle(&self, cycle: academic_cycle::Model) -> Result<academic_cycle::Model, DbErr> {
        let active = academic_cycle::ActiveModel {
            id: Set(cycle.id),
            org_id: Set(cycle.org_id),
            name: Set(cycle.name),
            name_ar: Set(cycle.name_ar),
            code: Set(cycle.code),
            cycle_type: Set(cycle.cycle_type),
            parent_id: Set(cycle.parent_id),
            start_date: Set(cycle.start_date),
            end_date: Set(cycle.end_date),
            hijri_start_date: Set(cycle.hijri_start_date),
            hijri_end_date: Set(cycle.hijri_end_date),
            exam_start_date: Set(cycle.exam_start_date),
            exam_end_date: Set(cycle.exam_end_date),
            weight: Set(cycle.weight),
            grading_system: Set(cycle.grading_system),
            passing_grade: Set(cycle.passing_grade),
            max_grade: Set(cycle.max_grade),
            is_current: Set(cycle.is_current),
            is_locked: Set(cycle.is_locked),
            is_enrollment_open: Set(cycle.is_enrollment_open),
            is_grade_entry_open: Set(cycle.is_grade_entry_open),
            grade_entry_start: Set(cycle.grade_entry_start),
            grade_entry_end: Set(cycle.grade_entry_end),
            locked_at: Set(cycle.locked_at),
            locked_by: Set(cycle.locked_by),
            settings: Set(cycle.settings),
            sort_order: Set(cycle.sort_order),
            created_at: Set(cycle.created_at),
            updated_at: Set(cycle.updated_at),
        };
        active.insert(&self.db).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct CycleUpdate {
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
    pub is_locked: Option<i32>,
    pub is_current: Option<i32>,
    pub is_grade_entry_open: Option<i32>,
    pub grade_entry_start: Option<String>,
    pub grade_entry_end: Option<String>,
    pub locked_at: Option<String>,
    pub locked_by: Option<String>,
}

