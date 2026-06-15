use crate::entities::{assessment, class_subject};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait};

impl super::Repository {
    pub async fn get_assessments(
        &self,
        class_subject_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<assessment::Model>, DbErr> {
        assessment::Entity::find()
            .filter(assessment::Column::ClassSubjectId.eq(class_subject_id))
            .filter(assessment::Column::CycleId.eq(cycle_id))
            .all(&self.db)
            .await
    }

    pub async fn get_assessment_by_id(&self, id: &str) -> Result<Option<assessment::Model>, DbErr> {
        assessment::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn insert_assessment(&self, a: assessment::Model) -> Result<assessment::Model, DbErr> {
        let active = assessment::ActiveModel {
            id: Set(a.id),
            org_id: Set(a.org_id),
            cycle_id: Set(a.cycle_id),
            class_subject_id: Set(a.class_subject_id),
            title: Set(a.title),
            title_ar: Set(a.title_ar),
            description: Set(a.description),
            item_type: Set(a.item_type),
            max_score: Set(a.max_score),
            passing_score: Set(a.passing_score),
            weight: Set(a.weight),
            grade_group_name: Set(a.grade_group_name),
            assessment_date: Set(a.assessment_date),
            due_date: Set(a.due_date),
            duration_minutes: Set(a.duration_minutes),
            is_published: Set(a.is_published),
            published_at: Set(a.published_at),
            published_by: Set(a.published_by),
            is_locked: Set(a.is_locked),
            locked_at: Set(a.locked_at),
            locked_by: Set(a.locked_by),
            grading_method: Set(a.grading_method),
            rubric_id: Set(a.rubric_id),
            instructions: Set(a.instructions),
            instructions_ar: Set(a.instructions_ar),
            attachments: Set(a.attachments),
            created_by: Set(a.created_by),
            created_at: Set(a.created_at),
            updated_at: Set(a.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn find_class_subject_for_assessment(
        &self,
        class_id: &str,
        subject_id: &str,
    ) -> Result<Option<class_subject::Model>, DbErr> {
        class_subject::Entity::find()
            .filter(class_subject::Column::ClassId.eq(class_id))
            .filter(class_subject::Column::SubjectId.eq(subject_id))
            .filter(class_subject::Column::IsActive.eq(1))
            .one(&self.db)
            .await
    }
}
