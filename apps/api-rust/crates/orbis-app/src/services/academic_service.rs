use orbis_domain::entities::subject::Subject;
use orbis_domain::entities::class::Class;
use orbis_domain::entities::class_subject::ClassSubject;
use orbis_domain::entities::course_prerequisite::PrerequisiteType;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::{SubjectFilter, ClassFilter, ClassSubjectFilter, PaginatedList};
use orbis_domain::iam::permission::Permission;
use crate::context::RequestContext;
use crate::ports::repositories::{SubjectRepository, ClassRepository, ClassSubjectRepository, CoursePrerequisiteRepository, GradeRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

pub struct AcademicService {
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub class_repo: Arc<dyn ClassRepository>,
    pub class_subject_repo: Arc<dyn ClassSubjectRepository>,
    pub prereq_repo: Arc<dyn CoursePrerequisiteRepository>,
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
        class_subject_repo: Arc<dyn ClassSubjectRepository>,
        prereq_repo: Arc<dyn CoursePrerequisiteRepository>,
        grade_repo: Arc<dyn GradeRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            subject_repo,
            class_repo,
            class_subject_repo,
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
        credit_hours: Option<Decimal>,
        is_core: bool,
    ) -> Result<Subject, DomainError> {
        self.permission_checker.check(ctx, Permission::SubjectWrite).await?;

        let now = Utc::now();
        let subject_id = Uuid::new_v4();

        let subject = Subject {
            id: subject_id,
            org_id,
            unit_id,
            code,
            name,
            name_ar: None,
            description: None,
            is_core,
            is_elective: !is_core,
            credit_hours,
            theory_hours: None,
            practical_hours: None,
            is_lab_required: false,
            max_students_per_class: None,
            color: None,
            icon: None,
            sort_order: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.subject_repo.save(&subject).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::SubjectCreated {
            subject_id,
            org_id,
            timestamp: now,
        }).await;

        Ok(subject)
    }

    /// List subjects
    pub async fn list_subjects(&self, org_id: Uuid, filter: SubjectFilter) -> Result<PaginatedList<Subject>, DomainError> {
        self.subject_repo.list(org_id, filter).await
    }

    /// Get subject by ID
    pub async fn get_subject(&self, id: Uuid) -> Result<Option<Subject>, DomainError> {
        self.subject_repo.find_by_id(id).await
    }

    /// Create class
    pub async fn create_class(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        unit_id: Uuid,
        period_id: Uuid,
        name: String,
        level: String,
    ) -> Result<Class, DomainError> {
        self.permission_checker.check(ctx, Permission::ClassWrite).await?;

        let now = Utc::now();
        let class_id = Uuid::new_v4();

        let class = Class {
            id: class_id,
            org_id,
            unit_id,
            period_id,
            name,
            name_ar: None,
            level,
            division: None,
            homeroom_teacher_id: None,
            capacity: Some(30),
            current_students: Some(0),
            room_id: None,
            color: None,
            is_active: true,
            is_deleted: false,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        };

        self.class_repo.save(&class).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::ClassCreated {
            class_id,
            org_id,
            period_id,
            timestamp: now,
        }).await;

        Ok(class)
    }

    /// List classes
    pub async fn list_classes(&self, org_id: Uuid, filter: ClassFilter) -> Result<PaginatedList<Class>, DomainError> {
        self.class_repo.list(org_id, filter).await
    }

    /// Get class by ID
    pub async fn get_class(&self, id: Uuid) -> Result<Option<Class>, DomainError> {
        self.class_repo.find_by_id(id).await
    }

    /// Assign teaching of a subject to a class (ClassSubject)
    pub async fn assign_class_subject(
        &self,
        ctx: &RequestContext,
        class_id: Uuid,
        subject_id: Uuid,
        period_id: Uuid,
        primary_teacher_id: Uuid,
        weekly_hours: Option<i32>,
    ) -> Result<ClassSubject, DomainError> {
        self.permission_checker.check(ctx, Permission::ClassWrite).await?;

        let now = Utc::now();
        let cs_id = Uuid::new_v4();

        let cs = ClassSubject {
            id: cs_id,
            class_id,
            subject_id,
            period_id,
            primary_teacher_id,
            assistant_teacher_id: None,
            substitute_teacher_id: None,
            grade_groups: None,
            weekly_hours,
            theory_hours: None,
            practical_hours: None,
            assessment_config: None,
            scheme_id: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.class_subject_repo.save(&cs).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::ClassSubjectAssigned {
            class_subject_id: cs_id,
            class_id,
            subject_id,
            timestamp: now,
        }).await;

        Ok(cs)
    }

    /// List class subjects
    pub async fn list_class_subjects(&self, org_id: Uuid, filter: ClassSubjectFilter) -> Result<PaginatedList<ClassSubject>, DomainError> {
        self.class_subject_repo.list(org_id, filter).await
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
            // Find grade for the prerequisite subject by resolving ClassSubject -> Subject
            let mut matching_grade = None;
            for g in &student_grades {
                if let Some(cs) = self.class_subject_repo.find_by_id(g.class_subject_id).await? {
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
                            (false, Some(g.score), Some(format!("Score {} is below required minimum {}", g.score, min_s)))
                        }
                    } else {
                        (true, Some(g.score), None)
                    }
                }
                None => {
                    match prereq.requirement_type {
                        PrerequisiteType::Enforce => (false, None, Some("Prerequisite course has not been taken".into())),
                        PrerequisiteType::Recommended => (true, None, Some("Recommended course not completed".into())),
                        PrerequisiteType::Corequisite => (true, None, None),
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
