use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    ClassRepository, SectionSubjectRepository, PrerequisiteRepository, GradeRepository,
    SubjectRepository,
};
use chrono::Utc;
use orbis_domain::entities::section::Section;
use orbis_domain::entities::section_subject::SectionSubject;
use orbis_domain::entities::prerequisite::PrerequisiteType;
use orbis_domain::entities::subject::Subject;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{
    ClassFilter, SectionSubjectFilter, PaginatedList, SubjectFilter,
};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

pub struct AcademicService {
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub class_repo: Arc<dyn ClassRepository>,
    pub section_subject_repo: Arc<dyn SectionSubjectRepository>,
    pub prereq_repo: Arc<dyn PrerequisiteRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrerequisiteCheckResult {
    pub subject_id: Uuid,
    pub prerequisite_subject_id: Uuid,
    pub requirement_type: PrerequisiteType,
    pub is_satisfied: bool,
    pub min_score_required: Option<Decimal>,
    pub current_score: Option<Decimal>,
    pub failure_reason: Option<String>,
}

impl AcademicService {
    pub fn new(
        subject_repo: Arc<dyn SubjectRepository>,
        class_repo: Arc<dyn ClassRepository>,
        section_subject_repo: Arc<dyn SectionSubjectRepository>,
        prereq_repo: Arc<dyn PrerequisiteRepository>,
        grade_repo: Arc<dyn GradeRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            subject_repo,
            class_repo,
            section_subject_repo,
            prereq_repo,
            grade_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Create subject
    pub async fn create_subject(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        unit_id: Option<Uuid>,
        code: String,
        name: String,
        name_ar: Option<String>,
        credit_hours: Option<Decimal>,
        is_core: bool,
    ) -> Result<Subject, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SubjectWrite)
            .await?;

        let normalized_code = code.trim();
        if normalized_code.is_empty() {
            return Err(DomainError::Validation("Subject code is required".into()));
        }
        if self
            .subject_repo
            .find_by_org(org_id)
            .await?
            .iter()
            .any(|subject| subject.code.eq_ignore_ascii_case(normalized_code))
        {
            return Err(DomainError::Conflict(format!(
                "A subject with code '{}' already exists",
                normalized_code
            )));
        }

        let now = Utc::now();
        let subject_id = Uuid::new_v4();

        let subject = Subject {
            id: subject_id,
            org_id,
            unit_id,
            code: normalized_code.to_string(),
            name,
            name_ar,
            description: None,
            is_core,
            is_elective: !is_core,
            credit_units: credit_hours,
            workload_units: None,
            theory_hours: None,
            practical_hours: None,
            is_lab_required: false,
            max_students_per_section: None,
            color: None,
            icon: None,
            sort_order: None,
            subject_type: "subject".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.subject_repo.save(&subject).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::SubjectCreated {
                    subject_id,
                    org_id,
                    timestamp: now,
                },
            )
            .await;

        Ok(subject)
    }

    /// List subjects
    pub async fn list_subjects(
        &self,
        org_id: Uuid,
        filter: SubjectFilter,
    ) -> Result<PaginatedList<Subject>, DomainError> {
        self.subject_repo.list(org_id, filter).await
    }

    /// Get subject by ID
    pub async fn get_subject(&self, id: Uuid) -> Result<Option<Subject>, DomainError> {
        self.subject_repo.find_by_id(id).await
    }

    /// Create Section
    pub async fn create_class(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        unit_id: Uuid,
        period_id: Uuid,
        name: String,
        name_ar: Option<String>,
        level: String,
        capacity: Option<i32>,
    ) -> Result<Section, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ClassWrite)
            .await?;

        let normalized_name = name.trim().to_lowercase();
        let normalized_level = level.trim().to_lowercase();
        let normalized_name_ar = name_ar.as_deref().unwrap_or_default().trim().to_lowercase();
        if normalized_name.is_empty() || normalized_level.is_empty() {
            return Err(DomainError::Validation(
                "Section name and level are required".into(),
            ));
        }
        if capacity.is_some_and(|value| value <= 0) {
            return Err(DomainError::Validation(
                "Section capacity must be greater than zero".into(),
            ));
        }
        if self
            .class_repo
            .find_by_org(org_id)
            .await?
            .iter()
            .any(|s| {
                s.org_unit_id == Some(unit_id)
                    && s.level.as_deref().map(|l| l.trim().to_lowercase()) == Some(normalized_level.clone())
                    && (s.name.trim().to_lowercase() == normalized_name
                        || (!normalized_name_ar.is_empty()
                            && s
                                .name_ar
                                .as_deref()
                                .unwrap_or_default()
                                .trim()
                                .to_lowercase()
                                == normalized_name_ar))
            })
        {
            return Err(DomainError::Conflict(format!(
                "A Section named '{}' already exists for this level and period",
                name.trim()
            )));
        }

        let now = Utc::now();
        let section_id = Uuid::new_v4();

        let section = Section {
            id: section_id,
            org_id,
            org_unit_id: Some(unit_id),
            section_type: "class".to_string(),
            code: name.trim().to_string(),
            name: name.trim().to_string(),
            name_ar,
            level: Some(level.trim().to_string()),
            track: None,
            capacity: Some(capacity.unwrap_or(30)),
            color: None,
            is_active: true,
            is_deleted: false,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        };

        self.class_repo.save(&section).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::ClassCreated {
                    section_id,
                    org_id,
                    period_id,
                    timestamp: now,
                },
            )
            .await;

        Ok(section)
    }

    /// List classes
    pub async fn list_classes(
        &self,
        org_id: Uuid,
        filter: ClassFilter,
    ) -> Result<PaginatedList<Section>, DomainError> {
        self.class_repo.list(org_id, filter).await
    }

    /// Get Section by ID
    pub async fn get_class(&self, id: Uuid) -> Result<Option<Section>, DomainError> {
        self.class_repo.find_by_id(id).await
    }

    /// Assign teaching of a subject to a Section (SectionSubject)
    pub async fn assign_section_subject(
        &self,
        ctx: &RequestContext,
        section_id: Uuid,
        subject_id: Uuid,
        period_id: Uuid,
        instructor_id: Uuid,
        weekly_hours: Option<i32>,
    ) -> Result<SectionSubject, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ClassWrite)
            .await?;

        let now = Utc::now();
        let cs_id = Uuid::new_v4();

        let cs = SectionSubject {
            id: cs_id,
            org_id: ctx.tenant_id,
            section_id,
            subject_id,
            period_id,
            instructor_id: Some(instructor_id),
            assistant_instructor_id: None,
            substitute_instructor_id: None,
            scheme_id: None,
            weekly_hours,
            theory_hours: None,
            practical_hours: None,
            assessment_config: None,
            grade_groups: None,
            capacity: None,
            status: "active".to_string(),
            is_active: true,
            created_at: Some(now),
            updated_at: Some(now),
        };

        self.section_subject_repo.save(&cs).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::SectionSubjectAssigned {
                    section_subject_id: cs_id,
                    section_id,
                    subject_id,
                    timestamp: now,
                },
            )
            .await;

        Ok(cs)
    }

    /// List Section subjects
    pub async fn list_section_subjects(
        &self,
        org_id: Uuid,
        filter: SectionSubjectFilter,
    ) -> Result<PaginatedList<SectionSubject>, DomainError> {
        self.section_subject_repo.list(org_id, filter).await
    }

    /// Check prerequisites for a student before enrolling in a subject
    pub async fn check_prerequisites(
        &self,
        student_id: Uuid,
        subject_id: Uuid,
    ) -> Result<Vec<PrerequisiteCheckResult>, DomainError> {
        let prereqs = self.prereq_repo.find_by_subject(subject_id).await?;
        let student_grades = self.grade_repo.find_by_student(student_id).await?;

        let mut results = Vec::new();

        for prereq in prereqs {
            // Find grade for the prerequisite subject by resolving SectionSubject -> Subject
            let mut matching_grade = None;
            for g in &student_grades {
                if let Some(cs) = self
                    .section_subject_repo
                    .find_by_id(g.section_subject_id)
                    .await?
                {
                    if cs.subject_id == prereq.prerequisite_subject_id {
                        matching_grade = Some(g);
                        break;
                    }
                }
            }

            let (is_satisfied, current_score, failure_reason) = match matching_grade {
                Some(g) => {
                    if let Some(min_s) = prereq.min_score {
                        if g.score >= min_s {
                            (true, Some(g.score), None)
                        } else {
                            (
                                false,
                                Some(g.score),
                                Some(format!(
                                    "Score {} is below required minimum {}",
                                    g.score, min_s
                                )),
                            )
                        }
                    } else {
                        (true, Some(g.score), None)
                    }
                }
                None => {
                    let rt = prereq.requirement_type.as_str();
                    if rt == "mandatory" || rt == "enforce" {
                        (
                            false,
                            None,
                            Some("Prerequisite course has not been taken".into()),
                        )
                    } else if rt == "recommended" {
                        (true, None, Some("Recommended course not completed".into()))
                    } else {
                        // corequisite or unknown
                        (true, None, None)
                    }
                }
            };

            results.push(PrerequisiteCheckResult {
                subject_id,
                prerequisite_subject_id: prereq.prerequisite_subject_id,
                requirement_type: prereq.requirement_type,
                is_satisfied,
                min_score_required: prereq.min_score,
                current_score,
                failure_reason,
            });
        }

        Ok(results)
    }
}
