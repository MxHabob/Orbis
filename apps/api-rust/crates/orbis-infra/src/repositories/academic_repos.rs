use async_trait::async_trait;
use orbis_app::ports::repositories::{ClassRepository, SectionSubjectRepository, SubjectRepository};
use orbis_domain::entities::section::Section as DomainSection;
use orbis_domain::entities::section_subject::SectionSubject as DomainSectionSubject;
use orbis_domain::entities::subject::Subject as DomainSubject;
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use uuid::Uuid;

use crate::models::{
    class::{self, Entity as SectionEntity},
    section_subject::{self, Entity as SectionSubjectEntity},
    subject::{self, Entity as SubjectEntity},
};

pub struct AcademicRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl SubjectRepository for AcademicRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainSubject>, DomainError> {
        let model = SubjectEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_subject_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainSubject>, DomainError> {
        let models = SubjectEntity::find()
            .filter(subject::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_subject_model_to_domain)
            .collect())
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::SubjectFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<DomainSubject>, DomainError> {
        let mut query = SubjectEntity::find().filter(subject::Column::OrgId.eq(org_id));
        if let Some(is_active) = filter.is_active {
            query = query.filter(subject::Column::IsActive.eq(is_active));
        }
        if let Some(ref s) = filter.search {
            if !s.is_empty() {
                let pat = format!("%{}%", s);
                query = query.filter(
                    subject::Column::Name
                        .like(&pat)
                        .or(subject::Column::Code.like(&pat)),
                );
            }
        }
        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let items = rows.into_iter().map(map_subject_model_to_domain).collect();
        Ok(orbis_domain::queries::filters::PaginatedList::new(
            items,
            total_count,
            page,
            page_size,
        ))
    }

    async fn save(&self, subject: &DomainSubject) -> Result<(), DomainError> {
        let active = subject::ActiveModel {
            id: Set(subject.id),
            org_id: Set(subject.org_id),
            unit_id: Set(subject.org_unit_id),
            code: Set(subject.code.clone()),
            name: Set(subject.name.clone()),
            name_ar: Set(subject.name_ar.clone()),
            description: Set(subject.description.clone()),
            is_core: Set(subject.is_core),
            is_elective: Set(subject.is_elective),
            credit_hours: Set(subject.credit_units),
            theory_hours: Set(subject.theory_hours),
            practical_hours: Set(subject.practical_hours),
            is_lab_required: Set(subject.is_lab_required),
            max_students_per_section: Set(subject.max_students_per_section.map(|v| v as i32)),
            color: Set(subject.color.clone()),
            icon: Set(subject.icon.clone()),
            sort_order: Set(subject.sort_order),
            is_active: Set(subject.is_active),
            created_at: Set(subject.created_at),
            updated_at: Set(subject.updated_at),
        };

        if SubjectEntity::find_by_id(subject.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ClassRepository for AcademicRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainSection>, DomainError> {
        let model = SectionEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_class_model_to_domain))
    }

    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<DomainSection>, DomainError> {
        let models = SectionEntity::find()
            .filter(class::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_class_model_to_domain).collect())
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainSection>, DomainError> {
        let models = SectionEntity::find()
            .filter(class::Column::OrgId.eq(org_id))
            .filter(class::Column::IsDeleted.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_class_model_to_domain).collect())
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::ClassFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<DomainSection>, DomainError> {
        let mut query = SectionEntity::find()
            .filter(class::Column::OrgId.eq(org_id))
            .filter(class::Column::IsDeleted.eq(false));
        if let Some(period_id) = filter.period_id {
            query = query.filter(class::Column::PeriodId.eq(period_id));
        }
        if let Some(ref lvl) = filter.level {
            query = query.filter(class::Column::Level.eq(lvl.clone()));
        }
        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let items = rows.into_iter().map(map_class_model_to_domain).collect();
        Ok(orbis_domain::queries::filters::PaginatedList::new(
            items,
            total_count,
            page,
            page_size,
        ))
    }

    async fn save(&self, class: &DomainSection) -> Result<(), DomainError> {
        let active = class::ActiveModel {
            id: Set(class.id),
            org_id: Set(class.org_id),
            unit_id: Set(class.org_unit_id),
            period_id: Set(class.period_id),
            name: Set(class.name.clone()),
            name_ar: Set(class.name_ar.clone()),
            level: Set(class.level.clone()),
            division: Set(class.division.clone()),
            homeroom_teacher_id: Set(class.homeroom_instructor_id),
            capacity: Set(class.capacity),
            current_students: Set(class.current_students),
            room_id: Set(class.room_id),
            color: Set(class.color.clone()),
            is_active: Set(class.is_active),
            is_deleted: Set(class.is_deleted),
            deleted_at: Set(class.deleted_at),
            created_at: Set(class.created_at),
            updated_at: Set(class.updated_at),
        };

        if SectionEntity::find_by_id(class.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl SectionSubjectRepository for AcademicRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainSectionSubject>, DomainError> {
        let model = SectionSubjectEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_section_subject_model_to_domain))
    }

    async fn find_by_class(&self, section_id: Uuid) -> Result<Vec<DomainSectionSubject>, DomainError> {
        let models = SectionSubjectEntity::find()
            .filter(section_subject::Column::ClassId.eq(class_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_section_subject_model_to_domain)
            .collect())
    }

    async fn find_by_class_and_period(
        &self,
        section_id: Uuid,
        period_id: Uuid,
    ) -> Result<Vec<DomainSectionSubject>, DomainError> {
        let models = SectionSubjectEntity::find()
            .filter(section_subject::Column::ClassId.eq(class_id))
            .filter(section_subject::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_section_subject_model_to_domain)
            .collect())
    }

    async fn find_by_teacher(
        &self,
        teacher_id: Uuid,
        period_id: Uuid,
    ) -> Result<Vec<DomainSectionSubject>, DomainError> {
        let models = SectionSubjectEntity::find()
            .filter(
                section_subject::Column::PrimaryTeacherId
                    .eq(teacher_id)
                    .or(section_subject::Column::AssistantTeacherId.eq(teacher_id))
                    .or(section_subject::Column::SubstituteTeacherId.eq(teacher_id)),
            )
            .filter(section_subject::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_section_subject_model_to_domain)
            .collect())
    }

    async fn list(
        &self,
        _org_id: Uuid,
        filter: orbis_domain::queries::filters::SectionSubjectFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<DomainSectionSubject>, DomainError>
    {
        let mut query = SectionSubjectEntity::find();
        if let Some(class_id) = filter.section_id {
            query = query.filter(section_subject::Column::ClassId.eq(class_id));
        }
        if let Some(subject_id) = filter.subject_id {
            query = query.filter(section_subject::Column::SubjectId.eq(subject_id));
        }
        if let Some(period_id) = filter.period_id {
            query = query.filter(section_subject::Column::PeriodId.eq(period_id));
        }
        if let Some(teacher_id) = filter.teacher_id {
            query = query.filter(
                section_subject::Column::PrimaryTeacherId
                    .eq(teacher_id)
                    .or(section_subject::Column::AssistantTeacherId.eq(teacher_id))
                    .or(section_subject::Column::SubstituteTeacherId.eq(teacher_id)),
            );
        }
        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let items = rows
            .into_iter()
            .map(map_section_subject_model_to_domain)
            .collect();
        Ok(orbis_domain::queries::filters::PaginatedList::new(
            items,
            total_count,
            page,
            page_size,
        ))
    }

    async fn save(&self, section_subject: &DomainSectionSubject) -> Result<(), DomainError> {
        let active = section_subject::ActiveModel {
            id: Set(section_subject.id),
            section_id: Set(section_subject.section_id),
            subject_id: Set(section_subject.subject_id),
            period_id: Set(section_subject.period_id),
            instructor_id: Set(section_subject.instructor_id),
            assistant_instructor_id: Set(section_subject.assistant_instructor_id),
            substitute_instructor_id: Set(section_subject.substitute_instructor_id),
            grade_groups: Set(section_subject.grade_groups.clone()),
            weekly_hours: Set(section_subject.weekly_hours),
            theory_hours: Set(section_subject.theory_hours),
            practical_hours: Set(section_subject.practical_hours),
            assessment_config: Set(section_subject.assessment_config.clone()),
            scheme_id: Set(section_subject.scheme_id),
            is_active: Set(section_subject.is_active),
            created_at: Set(section_subject.created_at),
            updated_at: Set(section_subject.updated_at),
        };

        if SectionSubjectEntity::find_by_id(section_subject.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

fn map_subject_model_to_domain(m: subject::Model) -> DomainSubject {
    DomainSubject {
        id: m.id,
        org_id: m.org_id,
        unit_id: m.org_unit_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        description: m.description,
        is_core: m.is_core,
        is_elective: m.is_elective,
        credit_hours: m.credit_units,
        theory_hours: m.theory_hours,
        practical_hours: m.practical_hours,
        is_lab_required: m.is_lab_required,
        max_students_per_section: m.max_students_per_section.map(|v| v as u32),
        color: m.color,
        icon: m.icon,
        sort_order: m.sort_order,
        is_active: m.is_active,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_class_model_to_domain(m: class::Model) -> DomainSection {
    DomainSection {
        id: m.id,
        org_id: m.org_id,
        unit_id: m.org_unit_id,
        period_id: m.period_id,
        name: m.name,
        name_ar: m.name_ar,
        level: m.level,
        division: m.division,
        homeroom_teacher_id: m.homeroom_instructor_id,
        capacity: m.capacity,
        current_students: m.current_students,
        room_id: m.room_id,
        color: m.color,
        is_active: m.is_active,
        is_deleted: m.is_deleted,
        deleted_at: m.deleted_at,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_section_subject_model_to_domain(m: section_subject::Model) -> DomainSectionSubject {
    DomainSectionSubject {
        id: m.id,
        section_id: m.section_id,
        subject_id: m.subject_id,
        period_id: m.period_id,
        instructor_id: m.instructor_id,
        assistant_instructor_id: m.assistant_instructor_id,
        substitute_instructor_id: m.substitute_instructor_id,
        grade_groups: m.grade_groups,
        weekly_hours: m.weekly_hours,
        theory_hours: m.theory_hours,
        practical_hours: m.practical_hours,
        assessment_config: m.assessment_config,
        scheme_id: m.scheme_id,
        is_active: m.is_active,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl orbis_app::ports::repositories::PrerequisiteRepository for AcademicRepoImpl {
    async fn find_by_subject(
        &self,
        subject_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::prerequisite::Prerequisite>, DomainError>
    {
        let models = crate::models::prerequisite::Entity::find()
            .filter(crate::models::prerequisite::Column::SubjectId.eq(subject_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(|m| orbis_domain::entities::prerequisite::Prerequisite {
            id: m.id,
            org_id: m.org_id,
            subject_id: m.subject_id,
            prerequisite_subject_id: m.prerequisite_subject_id,
            requirement_type: match m.requirement_type.as_str() {
                "recommended" => orbis_domain::entities::prerequisite::PrerequisiteType::Recommended,
                "corequisite" => orbis_domain::entities::prerequisite::PrerequisiteType::Corequisite,
                _ => orbis_domain::entities::prerequisite::PrerequisiteType::Enforce,
            },
            min_grade: m.min_grade,
            min_score: m.min_score,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }).collect())
    }

    async fn find_by_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::prerequisite::Prerequisite>, DomainError>
    {
        let models = crate::models::prerequisite::Entity::find()
            .filter(crate::models::prerequisite::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(|m| orbis_domain::entities::prerequisite::Prerequisite {
            id: m.id,
            org_id: m.org_id,
            subject_id: m.subject_id,
            prerequisite_subject_id: m.prerequisite_subject_id,
            requirement_type: match m.requirement_type.as_str() {
                "recommended" => orbis_domain::entities::prerequisite::PrerequisiteType::Recommended,
                "corequisite" => orbis_domain::entities::prerequisite::PrerequisiteType::Corequisite,
                _ => orbis_domain::entities::prerequisite::PrerequisiteType::Enforce,
            },
            min_grade: m.min_grade,
            min_score: m.min_score,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }).collect())
    }

    async fn save(
        &self,
        prereq: &orbis_domain::entities::prerequisite::Prerequisite,
    ) -> Result<(), DomainError> {
        let active = crate::models::prerequisite::ActiveModel {
            id: Set(prereq.id),
            org_id: Set(prereq.org_id),
            subject_id: Set(prereq.subject_id),
            prerequisite_subject_id: Set(prereq.prerequisite_subject_id),
            requirement_type: Set(prereq.requirement_type.to_string()),
            min_grade: Set(prereq.min_grade.clone()),
            min_score: Set(prereq.min_score),
            created_at: Set(prereq.created_at),
            updated_at: Set(prereq.updated_at),
        };

        if crate::models::prerequisite::Entity::find_by_id(prereq.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        crate::models::prerequisite::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
