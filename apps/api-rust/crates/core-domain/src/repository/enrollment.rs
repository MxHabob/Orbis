use crate::entities::enrollment;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder, QuerySelect};

impl super::Repository {
    pub async fn get_enrollments(
        &self,
        org_id: &str,
        params: crate::params::EnrollmentParams,
    ) -> Result<Vec<enrollment::Model>, DbErr> {
        let mut query = enrollment::Entity::find()
            .filter(enrollment::Column::OrgId.eq(org_id));

        if let Some(ref class_id) = params.class_id {
            if !class_id.trim().is_empty() {
                query = query.filter(enrollment::Column::ClassId.eq(class_id));
            }
        }

        if let Some(ref cycle_id) = params.cycle_id {
            if !cycle_id.trim().is_empty() {
                query = query.filter(enrollment::Column::CycleId.eq(cycle_id));
            }
        }

        if let Some(ref student_id) = params.student_id {
            if !student_id.trim().is_empty() {
                query = query.filter(enrollment::Column::StudentId.eq(student_id));
            }
        }

        if let Some(ref status) = params.status {
            if !status.trim().is_empty() {
                query = query.filter(enrollment::Column::Status.eq(status));
            }
        }

        query = query.order_by_desc(enrollment::Column::CreatedAt);

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        query.all(&self.db).await
    }

    pub async fn get_enrollment_by_id(&self, id: &str) -> Result<Option<enrollment::Model>, DbErr> {
        enrollment::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn update_enrollment(&self, id: &str, updates: EnrollmentUpdate) -> Result<enrollment::Model, DbErr> {
        let e = enrollment::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("enrollment not found".to_string()))?;

        let mut active: enrollment::ActiveModel = e.into();

        if let Some(status) = updates.status {
            active.status = Set(status);
        }
        if let Some(class_id) = updates.class_id {
            active.class_id = Set(class_id);
        }
        if let Some(withdrawal_date) = updates.withdrawal_date {
            active.withdrawal_date = Set(Some(withdrawal_date));
        }
        if let Some(withdrawal_reason) = updates.withdrawal_reason {
            active.withdrawal_reason = Set(Some(withdrawal_reason));
        }
        if let Some(final_grade) = updates.final_grade {
            active.final_grade = Set(Some(final_grade));
        }
        if let Some(final_score) = updates.final_score {
            active.final_score = Set(Some(final_score));
        }
        if let Some(final_rank) = updates.final_rank {
            active.final_rank = Set(Some(final_rank));
        }
        if let Some(is_promoted) = updates.is_promoted {
            active.is_promoted = Set(Some(is_promoted));
        }
        if let Some(promotion_decision) = updates.promotion_decision {
            active.promotion_decision = Set(Some(promotion_decision));
        }
        if let Some(promotion_notes) = updates.promotion_notes {
            active.promotion_notes = Set(Some(promotion_notes));
        }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    pub async fn get_enrollments_by_class(&self, class_id: &str) -> Result<Vec<enrollment::Model>, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .filter(enrollment::Column::Status.eq("active"))
            .all(&self.db)
            .await
    }

    pub async fn get_enrollments_by_cycle(&self, org_id: &str, cycle_id: &str) -> Result<Vec<enrollment::Model>, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .filter(enrollment::Column::CycleId.eq(cycle_id))
            .filter(enrollment::Column::Status.eq("active"))
            .all(&self.db)
            .await
    }

    pub async fn get_enrollment_count_by_class(&self, class_id: &str) -> Result<u64, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .filter(enrollment::Column::Status.eq("active"))
            .count(&self.db)
            .await
    }

    pub async fn get_enrollment_count(&self, org_id: &str) -> Result<u64, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .filter(enrollment::Column::Status.eq("active"))
            .count(&self.db)
            .await
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnrollmentUpdate {
    pub status: Option<String>,
    pub class_id: Option<String>,
    pub withdrawal_date: Option<String>,
    pub withdrawal_reason: Option<String>,
    pub final_grade: Option<String>,
    pub final_score: Option<i32>,
    pub final_rank: Option<i32>,
    pub is_promoted: Option<i32>,
    pub promotion_decision: Option<String>,
    pub promotion_notes: Option<String>,
}

