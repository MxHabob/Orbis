use async_trait::async_trait;
use orbis_app::ports::repositories::{SubjectRepository, ClassRepository, ClassSubjectRepository};
use orbis_domain::entities::subject::Subject as DomainSubject;
use orbis_domain::entities::class::Class as DomainClass;
use orbis_domain::entities::class_subject::ClassSubject as DomainClassSubject;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{
    subject::{self, Entity as SubjectEntity},
    class::{self, Entity as ClassEntity},
    class_subject::{self, Entity as ClassSubjectEntity},
};

pub struct AcademicRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl SubjectRepository for AcademicRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainSubject>, DomainError> {
        let model = SubjectEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainSubject {
            id: m.id, org_id: m.org_id, unit_id: m.unit_id, code: m.code,
            name: m.name, name_ar: m.name_ar, name_en: m.name_en,
            description: m.description, short_description: m.short_description,
            is_core: m.is_core, is_elective: m.is_elective, credit_hours: m.credit_hours,
            requires_lab: m.requires_lab, requires_practical: m.requires_practical,
            max_students_per_class: m.max_students_per_class, default_grade_groups: m.default_grade_groups,
            prerequisites: m.prerequisites, learning_outcomes: m.learning_outcomes,
            color: m.color, icon: m.icon, cover_image: m.cover_image,
            sort_order: m.sort_order, is_active: m.is_active,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainSubject>, DomainError> {
        let models = SubjectEntity::find().filter(subject::Column::OrgId.eq(org_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainSubject {
            id: m.id, org_id: m.org_id, unit_id: m.unit_id, code: m.code,
            name: m.name, name_ar: m.name_ar, name_en: m.name_en,
            description: m.description, short_description: m.short_description,
            is_core: m.is_core, is_elective: m.is_elective, credit_hours: m.credit_hours,
            requires_lab: m.requires_lab, requires_practical: m.requires_practical,
            max_students_per_class: m.max_students_per_class, default_grade_groups: m.default_grade_groups,
            prerequisites: m.prerequisites, learning_outcomes: m.learning_outcomes,
            color: m.color, icon: m.icon, cover_image: m.cover_image,
            sort_order: m.sort_order, is_active: m.is_active,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, subject: &DomainSubject) -> Result<(), DomainError> {
        let active = subject::ActiveModel {
            id: Set(subject.id), org_id: Set(subject.org_id), unit_id: Set(subject.unit_id),
            code: Set(subject.code.clone()), name: Set(subject.name.clone()),
            name_ar: Set(subject.name_ar.clone()), name_en: Set(subject.name_en.clone()),
            description: Set(subject.description.clone()), short_description: Set(subject.short_description.clone()),
            is_core: Set(subject.is_core), is_elective: Set(subject.is_elective),
            credit_hours: Set(subject.credit_hours), requires_lab: Set(subject.requires_lab),
            requires_practical: Set(subject.requires_practical), max_students_per_class: Set(subject.max_students_per_class),
            default_grade_groups: Set(subject.default_grade_groups.clone()), prerequisites: Set(subject.prerequisites.clone()),
            learning_outcomes: Set(subject.learning_outcomes.clone()), color: Set(subject.color.clone()),
            icon: Set(subject.icon.clone()), cover_image: Set(subject.cover_image.clone()),
            sort_order: Set(subject.sort_order), is_active: Set(subject.is_active),
            created_at: Set(subject.created_at), updated_at: Set(subject.updated_at),
        };

        if SubjectEntity::find_by_id(subject.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ClassRepository for AcademicRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainClass>, DomainError> {
        let model = ClassEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainClass {
            id: m.id, org_id: m.org_id, unit_id: m.unit_id, period_id: m.period_id,
            name: m.name, name_ar: m.name_ar, level: m.level, division: m.division,
            homeroom_teacher_id: m.homeroom_teacher_id, capacity: m.capacity,
            current_students: m.current_students, room_id: m.room_id, color: m.color,
            is_active: m.is_active, is_deleted: m.is_deleted, deleted_at: m.deleted_at,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<DomainClass>, DomainError> {
        let models = ClassEntity::find().filter(class::Column::PeriodId.eq(period_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainClass {
            id: m.id, org_id: m.org_id, unit_id: m.unit_id, period_id: m.period_id,
            name: m.name, name_ar: m.name_ar, level: m.level, division: m.division,
            homeroom_teacher_id: m.homeroom_teacher_id, capacity: m.capacity,
            current_students: m.current_students, room_id: m.room_id, color: m.color,
            is_active: m.is_active, is_deleted: m.is_deleted, deleted_at: m.deleted_at,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, class: &DomainClass) -> Result<(), DomainError> {
        let active = class::ActiveModel {
            id: Set(class.id), org_id: Set(class.org_id), unit_id: Set(class.unit_id),
            period_id: Set(class.period_id), name: Set(class.name.clone()),
            name_ar: Set(class.name_ar.clone()), level: Set(class.level.clone()),
            division: Set(class.division.clone()), homeroom_teacher_id: Set(class.homeroom_teacher_id),
            capacity: Set(class.capacity), current_students: Set(class.current_students),
            room_id: Set(class.room_id), color: Set(class.color.clone()),
            is_active: Set(class.is_active), is_deleted: Set(class.is_deleted),
            deleted_at: Set(class.deleted_at), created_at: Set(class.created_at),
            updated_at: Set(class.updated_at),
        };

        if ClassEntity::find_by_id(class.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ClassSubjectRepository for AcademicRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainClassSubject>, DomainError> {
        let model = ClassSubjectEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainClassSubject {
            id: m.id, class_id: m.class_id, subject_id: m.subject_id, period_id: m.period_id,
            primary_teacher_id: m.primary_teacher_id, assistant_teacher_id: m.assistant_teacher_id,
            substitute_teacher_id: m.substitute_teacher_id, grade_groups: m.grade_groups,
            weekly_hours: m.weekly_hours, theory_hours: m.theory_hours,
            practical_hours: m.practical_hours, assessment_config: m.assessment_config,
            scheme_id: m.scheme_id, is_active: m.is_active,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<DomainClassSubject>, DomainError> {
        let models = ClassSubjectEntity::find().filter(class_subject::Column::ClassId.eq(class_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainClassSubject {
            id: m.id, class_id: m.class_id, subject_id: m.subject_id, period_id: m.period_id,
            primary_teacher_id: m.primary_teacher_id, assistant_teacher_id: m.assistant_teacher_id,
            substitute_teacher_id: m.substitute_teacher_id, grade_groups: m.grade_groups,
            weekly_hours: m.weekly_hours, theory_hours: m.theory_hours,
            practical_hours: m.practical_hours, assessment_config: m.assessment_config,
            scheme_id: m.scheme_id, is_active: m.is_active,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, class_subject: &DomainClassSubject) -> Result<(), DomainError> {
        let active = class_subject::ActiveModel {
            id: Set(class_subject.id), class_id: Set(class_subject.class_id),
            subject_id: Set(class_subject.subject_id), period_id: Set(class_subject.period_id),
            primary_teacher_id: Set(class_subject.primary_teacher_id),
            assistant_teacher_id: Set(class_subject.assistant_teacher_id),
            substitute_teacher_id: Set(class_subject.substitute_teacher_id),
            grade_groups: Set(class_subject.grade_groups.clone()),
            weekly_hours: Set(class_subject.weekly_hours),
            theory_hours: Set(class_subject.theory_hours),
            practical_hours: Set(class_subject.practical_hours),
            assessment_config: Set(class_subject.assessment_config.clone()),
            scheme_id: Set(class_subject.scheme_id),
            is_active: Set(class_subject.is_active),
            created_at: Set(class_subject.created_at),
            updated_at: Set(class_subject.updated_at),
        };

        if ClassSubjectEntity::find_by_id(class_subject.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
