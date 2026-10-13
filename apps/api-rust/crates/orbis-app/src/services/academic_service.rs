use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    SectionRepository, SectionSubjectRepository, PrerequisiteRepository, GradeRepository,
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
    SectionFilter, SectionSubjectFilter, PaginatedList, SubjectFilter,
};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

pub struct AcademicService {
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub section_repo: Arc<dyn SectionRepository>,
    pub section_subject_repo: Arc<dyn SectionSubjectRepository>,
    pub prereq_repo: Arc<dyn PrerequisiteRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSubjectInput {
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub subject_type: Option<String>,
    pub credit_hours: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    pub is_core: Option<bool>,
    pub is_elective: Option<bool>,
    pub is_lab_required: Option<bool>,
    pub max_students_per_section: Option<u32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateSubjectInput {
    pub unit_id: Option<Uuid>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub subject_type: Option<String>,
    pub credit_hours: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    pub is_core: Option<bool>,
    pub is_elective: Option<bool>,
    pub is_lab_required: Option<bool>,
    pub max_students_per_section: Option<u32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSectionInput {
    pub unit_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub section_type: Option<String>,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub track: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateSectionInput {
    pub unit_id: Option<Uuid>,
    pub section_type: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub track: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssignSectionSubjectInput {
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub substitute_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateSectionSubjectInput {
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub substitute_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: Option<String>,
    pub is_active: Option<bool>,
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
        section_repo: Arc<dyn SectionRepository>,
        section_subject_repo: Arc<dyn SectionSubjectRepository>,
        prereq_repo: Arc<dyn PrerequisiteRepository>,
        grade_repo: Arc<dyn GradeRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            subject_repo,
            section_repo,
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
        input: CreateSubjectInput,
    ) -> Result<Subject, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SubjectWrite)
            .await?;

        let normalized_code = input.code.trim();
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

        let is_core = input.is_core.unwrap_or(true);
        let is_elective = input.is_elective.unwrap_or(!is_core);
        let now = Utc::now();
        let subject_id = Uuid::new_v4();

        let subject = Subject {
            id: subject_id,
            org_id,
            unit_id: input.unit_id,
            code: normalized_code.to_string(),
            name: input.name,
            name_ar: input.name_ar,
            description: input.description,
            is_core,
            is_elective,
            credit_units: input.credit_hours,
            workload_units: input.workload_units,
            theory_hours: input.theory_hours,
            practical_hours: input.practical_hours,
            is_lab_required: input.is_lab_required.unwrap_or(false),
            max_students_per_section: input.max_students_per_section,
            color: input.color,
            icon: input.icon,
            sort_order: input.sort_order,
            subject_type: input.subject_type.unwrap_or_else(|| "subject".to_string()),
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

    /// Update subject
    pub async fn update_subject(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        input: UpdateSubjectInput,
    ) -> Result<Subject, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SubjectWrite)
            .await?;

        let mut subject = self
            .subject_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Subject {} not found", id)))?;

        if let Some(code) = input.code {
            let normalized_code = code.trim();
            if normalized_code.is_empty() {
                return Err(DomainError::Validation("Subject code cannot be empty".into()));
            }
            if !subject.code.eq_ignore_ascii_case(normalized_code) {
                if self
                    .subject_repo
                    .find_by_org(subject.org_id)
                    .await?
                    .iter()
                    .any(|s| s.id != id && s.code.eq_ignore_ascii_case(normalized_code))
                {
                    return Err(DomainError::Conflict(format!(
                        "A subject with code '{}' already exists",
                        normalized_code
                    )));
                }
                subject.code = normalized_code.to_string();
            }
        }

        if let Some(unit_id) = input.unit_id {
            subject.unit_id = Some(unit_id);
        }
        if let Some(name) = input.name {
            subject.name = name;
        }
        if let Some(name_ar) = input.name_ar {
            subject.name_ar = Some(name_ar);
        }
        if let Some(desc) = input.description {
            subject.description = Some(desc);
        }
        if let Some(st) = input.subject_type {
            subject.subject_type = st;
        }
        if let Some(ch) = input.credit_hours {
            subject.credit_units = Some(ch);
        }
        if let Some(wu) = input.workload_units {
            subject.workload_units = Some(wu);
        }
        if let Some(th) = input.theory_hours {
            subject.theory_hours = Some(th);
        }
        if let Some(ph) = input.practical_hours {
            subject.practical_hours = Some(ph);
        }
        if let Some(core) = input.is_core {
            subject.is_core = core;
        }
        if let Some(elec) = input.is_elective {
            subject.is_elective = elec;
        }
        if let Some(lab) = input.is_lab_required {
            subject.is_lab_required = lab;
        }
        if let Some(max_s) = input.max_students_per_section {
            subject.max_students_per_section = Some(max_s);
        }
        if let Some(color) = input.color {
            subject.color = Some(color);
        }
        if let Some(icon) = input.icon {
            subject.icon = Some(icon);
        }
        if let Some(sort_order) = input.sort_order {
            subject.sort_order = Some(sort_order);
        }
        if let Some(active) = input.is_active {
            subject.is_active = active;
        }

        subject.updated_at = Utc::now();
        self.subject_repo.save(&subject).await?;

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

    /// Create Section (Class)
    pub async fn create_section(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: CreateSectionInput,
    ) -> Result<Section, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SectionWrite)
            .await?;

        let normalized_name = input.name.trim().to_lowercase();
        let normalized_name_ar = input.name_ar.as_deref().unwrap_or_default().trim().to_lowercase();
        if normalized_name.is_empty() {
            return Err(DomainError::Validation(
                "Section name is required".into(),
            ));
        }
        if input.capacity.is_some_and(|value| value <= 0) {
            return Err(DomainError::Validation(
                "Section capacity must be greater than zero".into(),
            ));
        }
        if self
            .section_repo
            .find_by_org(org_id)
            .await?
            .iter()
            .any(|s| {
                s.org_unit_id == input.unit_id
                    && s.level.as_deref().map(|l| l.trim().to_lowercase()) == input.level.as_deref().map(|l| l.trim().to_lowercase())
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
                "A Section named '{}' already exists for this level",
                input.name.trim()
            )));
        }

        let now = Utc::now();
        let section_id = Uuid::new_v4();
        let code = input.code.unwrap_or_else(|| input.name.trim().to_string());

        let section = Section {
            id: section_id,
            org_id,
            org_unit_id: input.unit_id,
            section_type: input.section_type.unwrap_or_else(|| "class".to_string()),
            code: code.trim().to_string(),
            name: input.name.trim().to_string(),
            name_ar: input.name_ar,
            level: input.level.map(|l| l.trim().to_string()),
            track: input.track,
            capacity: Some(input.capacity.unwrap_or(30)),
            color: input.color,
            is_active: true,
            is_deleted: false,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        };

        self.section_repo.save(&section).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::SectionCreated {
                    section_id,
                    org_id,
                    period_id: input.period_id.unwrap_or_default(),
                    timestamp: now,
                },
            )
            .await;

        Ok(section)
    }

    /// Update Section (Class)
    pub async fn update_section(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        input: UpdateSectionInput,
    ) -> Result<Section, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SectionWrite)
            .await?;

        let mut section = self
            .section_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Section {} not found", id)))?;

        if let Some(unit_id) = input.unit_id {
            section.org_unit_id = Some(unit_id);
        }
        if let Some(st) = input.section_type {
            section.section_type = st;
        }
        if let Some(code) = input.code {
            section.code = code;
        }
        if let Some(name) = input.name {
            section.name = name;
        }
        if let Some(name_ar) = input.name_ar {
            section.name_ar = Some(name_ar);
        }
        if let Some(level) = input.level {
            section.level = Some(level);
        }
        if let Some(track) = input.track {
            section.track = Some(track);
        }
        if let Some(capacity) = input.capacity {
            if capacity <= 0 {
                return Err(DomainError::Validation(
                    "Section capacity must be greater than zero".into(),
                ));
            }
            section.capacity = Some(capacity);
        }
        if let Some(color) = input.color {
            section.color = Some(color);
        }
        if let Some(active) = input.is_active {
            section.is_active = active;
        }

        section.updated_at = Utc::now();
        self.section_repo.save(&section).await?;
        Ok(section)
    }

    /// List classes
    pub async fn list_sections(
        &self,
        org_id: Uuid,
        filter: SectionFilter,
    ) -> Result<PaginatedList<Section>, DomainError> {
        self.section_repo.list(org_id, filter).await
    }

    /// Get Section by ID
    pub async fn get_section(&self, id: Uuid) -> Result<Option<Section>, DomainError> {
        self.section_repo.find_by_id(id).await
    }

    /// Assign teaching of a subject to a Section (SectionSubject)
    pub async fn assign_section_subject(
        &self,
        ctx: &RequestContext,
        input: AssignSectionSubjectInput,
    ) -> Result<SectionSubject, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SectionWrite)
            .await?;

        let now = Utc::now();
        let cs_id = Uuid::new_v4();

        let cs = SectionSubject {
            id: cs_id,
            org_id: ctx.tenant_id,
            section_id: input.section_id,
            subject_id: input.subject_id,
            period_id: input.period_id,
            instructor_id: input.instructor_id,
            assistant_instructor_id: input.assistant_instructor_id,
            substitute_instructor_id: input.substitute_instructor_id,
            scheme_id: input.scheme_id,
            weekly_hours: input.weekly_hours,
            theory_hours: input.theory_hours,
            practical_hours: input.practical_hours,
            assessment_config: input.assessment_config,
            grade_groups: input.grade_groups,
            capacity: input.capacity,
            status: input.status.unwrap_or_else(|| "active".to_string()),
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
                    section_id: input.section_id,
                    subject_id: input.subject_id,
                    timestamp: now,
                },
            )
            .await;

        Ok(cs)
    }

    /// Get SectionSubject by ID
    pub async fn get_section_subject(&self, id: Uuid) -> Result<Option<SectionSubject>, DomainError> {
        self.section_subject_repo.find_by_id(id).await
    }

    /// Update SectionSubject
    pub async fn update_section_subject(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        input: UpdateSectionSubjectInput,
    ) -> Result<SectionSubject, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SectionWrite)
            .await?;

        let mut cs = self
            .section_subject_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("SectionSubject {} not found", id)))?;

        if let Some(ins) = input.instructor_id {
            cs.instructor_id = Some(ins);
        }
        if let Some(asst) = input.assistant_instructor_id {
            cs.assistant_instructor_id = Some(asst);
        }
        if let Some(sub) = input.substitute_instructor_id {
            cs.substitute_instructor_id = Some(sub);
        }
        if let Some(scheme) = input.scheme_id {
            cs.scheme_id = Some(scheme);
        }
        if let Some(wh) = input.weekly_hours {
            cs.weekly_hours = Some(wh);
        }
        if let Some(th) = input.theory_hours {
            cs.theory_hours = Some(th);
        }
        if let Some(ph) = input.practical_hours {
            cs.practical_hours = Some(ph);
        }
        if let Some(cfg) = input.assessment_config {
            cs.assessment_config = Some(cfg);
        }
        if let Some(gg) = input.grade_groups {
            cs.grade_groups = Some(gg);
        }
        if let Some(cap) = input.capacity {
            cs.capacity = Some(cap);
        }
        if let Some(status) = input.status {
            cs.status = status;
        }
        if let Some(active) = input.is_active {
            cs.is_active = active;
        }

        cs.updated_at = Some(Utc::now());
        self.section_subject_repo.save(&cs).await?;
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
