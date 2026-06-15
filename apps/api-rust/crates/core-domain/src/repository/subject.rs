use crate::entities::subject;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    pub async fn get_subjects(&self, org_id: &str) -> Result<Vec<subject::Model>, DbErr> {
        subject::Entity::find()
            .filter(subject::Column::OrgId.eq(org_id))
            .filter(subject::Column::IsActive.eq(1))
            .order_by_asc(subject::Column::SortOrder)
            .order_by_asc(subject::Column::Name)
            .all(&self.db)
            .await
    }

    pub async fn get_subject_by_id(&self, id: &str) -> Result<Option<subject::Model>, DbErr> {
        subject::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn insert_subject(&self, s: subject::Model) -> Result<subject::Model, DbErr> {
        let active = subject::ActiveModel {
            id: Set(s.id),
            org_id: Set(s.org_id),
            unit_id: Set(s.unit_id),
            code: Set(s.code),
            name: Set(s.name),
            name_ar: Set(s.name_ar),
            name_en: Set(s.name_en),
            description: Set(s.description),
            short_description: Set(s.short_description),
            is_core: Set(s.is_core),
            is_elective: Set(s.is_elective),
            credit_hours: Set(s.credit_hours),
            requires_lab: Set(s.requires_lab),
            requires_practical: Set(s.requires_practical),
            max_students_per_class: Set(s.max_students_per_class),
            default_grade_groups: Set(s.default_grade_groups),
            prerequisites: Set(s.prerequisites),
            learning_outcomes: Set(s.learning_outcomes),
            color: Set(s.color),
            icon: Set(s.icon),
            cover_image: Set(s.cover_image),
            sort_order: Set(s.sort_order),
            is_active: Set(s.is_active),
            created_at: Set(s.created_at),
            updated_at: Set(s.updated_at),
        };
        active.insert(&self.db).await
    }
}
