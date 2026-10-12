use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AcademicPeriodRepository, AcademicCalendarRepository, AssessmentSchemeRepository,
    AttendanceRepository, BehaviorRecordRepository, ClassRepository, SectionSubjectRepository,
    EnrollmentRepository, GpaRecordRepository, GradeRepository, SettingsRepository,
    StudentRepository, SubjectRepository,
};
use crate::services::calculation_engine::{
    AcademicCalculationEngine, AstNode, CalcEngine, apply_rounding,
};
use chrono::{NaiveDate, Utc};
use orbis_domain::entities::academic_period::PeriodKind;
use orbis_domain::entities::academic_policy::{
    AcademicPolicy, AcademicStatus, ComponentScore, PeriodAssessmentResult, PeriodComponentScores,
    StudentAnnualReportCard, StudentTermReportCard, SubjectTermResult,
};
use orbis_domain::entities::assessment_scheme::{
    AssessmentComponent, AssessmentLevel, AssessmentScheme, InstitutionScope,
};
use orbis_domain::entities::attendance::Attendance;
use orbis_domain::entities::behavior_record::BehaviorRecord;
use orbis_domain::entities::enrollment::EnrollmentStatus;
use orbis_domain::entities::gpa_record::CumulativeGpaRecord;
use orbis_domain::entities::grade::Grade;
use orbis_domain::entities::student::AcademicStanding;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{AttendanceFilter, GradeFilter, PaginatedList};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

pub struct BatchGradeItem {
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub section_subject_id: Uuid,
    pub component_id: Option<Uuid>,
    pub score: Decimal,
    pub max_score: Option<Decimal>,
    pub is_excused: bool,
    pub is_exempt: bool,
    pub notes: Option<String>,
}

pub struct BatchAttendanceItem {
    pub student_id: Uuid,
    pub status: String,
    pub notes: Option<String>,
    pub section_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
}

pub struct AssessmentService {
    pub scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
    pub attendance_repo: Arc<dyn AttendanceRepository>,
    pub behavior_repo: Arc<dyn BehaviorRecordRepository>,
    pub gpa_repo: Arc<dyn GpaRecordRepository>,
    pub section_subject_repo: Arc<dyn SectionSubjectRepository>,
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub period_repo: Option<Arc<dyn AcademicPeriodRepository>>,
    pub class_repo: Option<Arc<dyn ClassRepository>>,
    pub student_repo: Option<Arc<dyn StudentRepository>>,
    pub year_repo: Option<Arc<dyn AcademicCalendarRepository>>,
    pub enrollment_repo: Option<Arc<dyn EnrollmentRepository>>,
    pub settings_repo: Option<Arc<dyn SettingsRepository>>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl AssessmentService {
    pub fn new(
        scheme_repo: Arc<dyn AssessmentSchemeRepository>,
        grade_repo: Arc<dyn GradeRepository>,
        attendance_repo: Arc<dyn AttendanceRepository>,
        behavior_repo: Arc<dyn BehaviorRecordRepository>,
        gpa_repo: Arc<dyn GpaRecordRepository>,
        section_subject_repo: Arc<dyn SectionSubjectRepository>,
        subject_repo: Arc<dyn SubjectRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            scheme_repo,
            grade_repo,
            attendance_repo,
            behavior_repo,
            gpa_repo,
            section_subject_repo,
            subject_repo,
            period_repo: None,
            class_repo: None,
            student_repo: None,
            year_repo: None,
            enrollment_repo: None,
            settings_repo: None,
            event_bus,
            permission_checker,
        }
    }

    /// Builder method to supply additional repositories needed for calculated results & report cards
    pub fn with_academic_repos(
        mut self,
        period_repo: Option<Arc<dyn AcademicPeriodRepository>>,
        class_repo: Option<Arc<dyn ClassRepository>>,
        student_repo: Option<Arc<dyn StudentRepository>>,
        year_repo: Option<Arc<dyn AcademicCalendarRepository>>,
        enrollment_repo: Option<Arc<dyn EnrollmentRepository>>,
        settings_repo: Option<Arc<dyn SettingsRepository>>,
    ) -> Self {
        self.period_repo = period_repo;
        self.class_repo = class_repo;
        self.student_repo = student_repo;
        self.year_repo = year_repo;
        self.enrollment_repo = enrollment_repo;
        self.settings_repo = settings_repo;
        self
    }

    /// Helper to resolve the authoritative AcademicPolicy for an organization
    async fn resolve_policy(&self, org_id: Uuid, policy: Option<AcademicPolicy>) -> AcademicPolicy {
        if let Some(p) = policy {
            p
        } else if let Some(sr) = &self.settings_repo {
            sr.get_settings(org_id)
                .await
                .map(|s| s.academic.to_academic_policy(org_id))
                .unwrap_or_default()
        } else {
            AcademicPolicy::default()
        }
    }

    /// Record or update a student grade
    pub async fn record_grade(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        section_subject_id: Uuid,
        component_id: Option<Uuid>,
        score: Decimal,
        max_score: Decimal,
        is_final: bool,
    ) -> Result<Grade, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeWrite)
            .await?;

        if score < Decimal::ZERO || score > max_score {
            return Err(DomainError::Validation(format!(
                "Score {} is outside allowable range [0, {}]",
                score, max_score
            )));
        }

        let now = Utc::now();
        let grade_id = Uuid::new_v4();

        let grade = Grade {
            id: grade_id,
            org_id,
            period_id,
            student_id,
            enrollment_id: None,
            section_subject_id,
            component_id,
            score,
            max_score,
            weight: None,
            letter_grade: None,
            is_final,
            is_excused: false,
            is_exempt: false,
            change_reason: None,
            recorded_by: ctx.user_id(),
            updated_by: None,
            notes: None,
            created_at: now,
            updated_at: now,
        };

        self.grade_repo.save(&grade).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::GradeRecorded {
                    grade_id,
                    student_id,
                    section_subject_id,
                    score,
                    timestamp: now,
                },
            )
            .await;

        Ok(grade)
    }

    /// Batch record or update student grades
    pub async fn batch_record_grades(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        items: Vec<BatchGradeItem>,
    ) -> Result<Vec<Grade>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeWrite)
            .await?;

        let now = Utc::now();
        let mut grades = Vec::with_capacity(items.len());

        for item in items {
            let max_score = item.max_score.unwrap_or_else(|| Decimal::from(100));
            if item.score < Decimal::ZERO || item.score > max_score {
                return Err(DomainError::Validation(format!(
                    "Score {} is outside allowable range [0, {}] for student {}",
                    item.score, max_score, item.student_id
                )));
            }

            let grade_id = Uuid::new_v4();
            grades.push(Grade {
                id: grade_id,
                org_id,
                period_id: item.period_id,
                student_id: item.student_id,
                enrollment_id: item.enrollment_id,
                section_subject_id: item.section_subject_id,
                component_id: item.component_id,
                score: item.score,
                max_score,
                weight: None,
                letter_grade: None,
                is_final: false,
                is_excused: item.is_excused,
                is_exempt: item.is_exempt,
                change_reason: None,
                recorded_by: ctx.user_id(),
                updated_by: None,
                notes: item.notes,
                created_at: now,
                updated_at: now,
            });
        }

        self.grade_repo.save_batch(&grades).await?;

        for g in &grades {
            let _ = self
                .event_bus
                .publish(
                    ctx,
                    &DomainEvent::GradeRecorded {
                        grade_id: g.id,
                        student_id: g.student_id,
                        section_subject_id: g.section_subject_id,
                        score: g.score,
                        timestamp: now,
                    },
                )
                .await;
        }

        Ok(grades)
    }

    /// Delete a grade record
    pub async fn delete_grade(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        grade_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeWrite)
            .await?;

        let grade = self
            .grade_repo
            .find_by_id(grade_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Grade {} not found", grade_id)))?;

        if grade.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.grade_repo.delete(grade_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::GradeDeleted {
                    grade_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Calculate final grade from assessment scheme formula
    pub async fn calculate_scheme_grades(
        &self,
        _ctx: &RequestContext,
        scheme_id: Uuid,
        student_id: Uuid,
        section_subject_id: Uuid,
    ) -> Result<Decimal, DomainError> {
        let scheme = self
            .scheme_repo
            .find_by_id(scheme_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Assessment Scheme not found".into()))?;

        let formula_val = scheme
            .formula
            .ok_or_else(|| DomainError::Validation("Scheme has no calculation formula".into()))?;

        let formula: AstNode = serde_json::from_value(formula_val)
            .map_err(|e| DomainError::Validation(format!("Invalid formula format: {}", e)))?;

        let grades = self
            .grade_repo
            .find_by_student_and_subject(student_id, section_subject_id)
            .await?;

        let mut engine = CalcEngine::new();
        for grade in grades {
            let var_name = grade
                .component_id
                .map(|cid| cid.to_string())
                .unwrap_or_else(|| grade.section_subject_id.to_string());
            engine.set_variable(&var_name, grade.score);
        }

        let max_score_decimal = scheme.max_score;
        let final_score = engine.evaluate(&formula)?;
        let final_score = if final_score > max_score_decimal {
            max_score_decimal
        } else {
            final_score
        };

        Ok(final_score)
    }

    /// Calculate and record GPA for a student in a specific academic period
    pub async fn calculate_gpa(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
    ) -> Result<CumulativeGpaRecord, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeAnalytics)
            .await?;

        let term_grades = self
            .grade_repo
            .find_by_student_and_period(student_id, period_id)
            .await?;
        let past_records = self.gpa_repo.find_by_student(student_id).await?;

        let mut term_attempted = Decimal::ZERO;
        let mut term_earned = Decimal::ZERO;
        let mut term_points = Decimal::ZERO;

        for g in &term_grades {
            // Resolve actual subject credit hours
            let credit = if let Some(cs) = self
                .section_subject_repo
                .find_by_id(g.section_subject_id)
                .await?
            {
                if let Some(subj) = self.subject_repo.find_by_id(cs.subject_id).await? {
                    subj.credit_units.unwrap_or(Decimal::from(3))
                } else {
                    Decimal::from(3)
                }
            } else {
                Decimal::from(3)
            };

            term_attempted += credit;
            if g.score >= Decimal::from(60) {
                term_earned += credit;
            }
            let course_gpa_point = (g.score / Decimal::from(100)) * Decimal::from(4);
            term_points += course_gpa_point * credit;
        }

        let term_gpa = if term_attempted > Decimal::ZERO {
            (term_points / term_attempted).round_dp(2)
        } else {
            Decimal::ZERO
        };

        let mut cum_attempted = term_attempted;
        let mut cum_earned = term_earned;
        let mut cum_points = term_points;

        for rec in &past_records {
            if rec.period_id != period_id {
                cum_attempted += rec.term_attempted_credits;
                cum_earned += rec.term_earned_credits;
                cum_points += rec.term_points;
            }
        }

        let cumulative_gpa = if cum_attempted > Decimal::ZERO {
            (cum_points / cum_attempted).round_dp(2)
        } else {
            Decimal::ZERO
        };

        let standing = if cumulative_gpa
            >= Decimal::from_str_exact("3.5").unwrap_or(Decimal::from(3))
        {
            AcademicStanding::HonorList
        } else if cumulative_gpa >= Decimal::from(2) {
            AcademicStanding::GoodStanding
        } else if cumulative_gpa >= Decimal::from_str_exact("1.75").unwrap_or(Decimal::from(1)) {
            AcademicStanding::AcademicWarning1
        } else {
            AcademicStanding::Probation
        };

        let now = Utc::now();
        let record = CumulativeGpaRecord {
            id: Uuid::new_v4(),
            org_id,
            student_id,
            period_id,
            term_attempted_credits: term_attempted,
            term_earned_credits: term_earned,
            term_points,
            term_gpa,
            cumulative_attempted_credits: cum_attempted,
            cumulative_earned_credits: cum_earned,
            cumulative_points: cum_points,
            cumulative_gpa,
            standing,
            calculated_at: now,
        };

        self.gpa_repo.save(&record).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::GpaCalculated {
                    student_id,
                    period_id,
                    gpa: term_gpa,
                    cgpa: cumulative_gpa,
                    timestamp: now,
                },
            )
            .await;

        Ok(record)
    }

    /// Mark attendance record
    pub async fn mark_attendance(
        &self,
        ctx: &RequestContext,
        _org_id: Uuid,
        _period_id: Uuid,
        student_id: Uuid,
        section_id: Option<Uuid>,
        section_subject_id: Option<Uuid>,
        date: NaiveDate,
        status: String,
        notes: Option<String>,
    ) -> Result<Attendance, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceWrite)
            .await?;

        let now = Utc::now();
        let att_id = Uuid::new_v4();

        let attendance = Attendance {
            id: att_id,
            student_id,
            section_id,
            section_subject_id,
            scheduled_lesson_id: None,
            attendance_date: date,
            status: status.clone(),
            notes,
            marked_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.attendance_repo.save(&attendance).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AttendanceMarked {
                    attendance_id: att_id,
                    student_id,
                    section_id,
                    section_subject_id,
                    scheduled_lesson_id: None,
                    status,
                    timestamp: now,
                },
            )
            .await;

        Ok(attendance)
    }

    /// Batch mark attendance records for a class on a date
    pub async fn batch_mark_attendance(
        &self,
        ctx: &RequestContext,
        _org_id: Uuid,
        section_id: Uuid,
        date: NaiveDate,
        items: Vec<BatchAttendanceItem>,
    ) -> Result<Vec<Attendance>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceWrite)
            .await?;

        let now = Utc::now();
        let mut attendances = Vec::with_capacity(items.len());

        for item in items {
            let att_id = Uuid::new_v4();
            attendances.push(Attendance {
                id: att_id,
                student_id: item.student_id,
                section_id: Some(section_id),
                section_subject_id: item.section_subject_id,
                scheduled_lesson_id: item.scheduled_lesson_id,
                attendance_date: date,
                status: item.status,
                notes: item.notes,
                marked_by: ctx.user_id(),
                created_at: now,
                updated_at: now,
            });
        }

        self.attendance_repo.save_batch(&attendances).await?;

        for a in &attendances {
            let _ = self
                .event_bus
                .publish(
                    ctx,
                    &DomainEvent::AttendanceMarked {
                        attendance_id: a.id,
                        student_id: a.student_id,
                        section_id: a.section_id,
                        section_subject_id: a.section_subject_id,
                        scheduled_lesson_id: a.scheduled_lesson_id,
                        status: a.status.clone(),
                        timestamp: now,
                    },
                )
                .await;
        }

        Ok(attendances)
    }

    /// Delete an attendance record
    pub async fn delete_attendance(
        &self,
        ctx: &RequestContext,
        _org_id: Uuid,
        attendance_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceWrite)
            .await?;

        self.attendance_repo.delete(attendance_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AttendanceDeleted {
                    attendance_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// List student attendance records
    pub async fn list_student_attendance(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<Attendance>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceRead)
            .await?;
        self.attendance_repo.find_by_student(student_id).await
    }

    /// Create assessment scheme
    pub async fn create_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        name_ar: Option<String>,
        description: Option<String>,
        max_score: Decimal,
        formula: Option<serde_json::Value>,
        level: Option<AssessmentLevel>,
        institution_scope: Option<InstitutionScope>,
        components: Option<Vec<AssessmentComponent>>,
        is_default: bool,
    ) -> Result<AssessmentScheme, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AssessmentWrite)
            .await?;

        let now = Utc::now();
        let scheme_id = Uuid::new_v4();
        let scheme = AssessmentScheme {
            id: scheme_id,
            org_id,
            name: name.clone(),
            name_ar,
            description,
            max_score,
            formula,
            level: level.unwrap_or(AssessmentLevel::MonthlyPeriod),
            institution_scope: institution_scope.unwrap_or(InstitutionScope::All),
            components: components.unwrap_or_default(),
            is_default,
            is_system: false,
            created_at: now,
            updated_at: now,
        };

        self.scheme_repo.save(&scheme).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AssessmentSchemeCreated {
                    scheme_id,
                    org_id,
                    name,
                    timestamp: now,
                },
            )
            .await;

        Ok(scheme)
    }

    /// Update assessment scheme
    pub async fn update_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        scheme_id: Uuid,
        name: Option<String>,
        name_ar: Option<String>,
        description: Option<String>,
        max_score: Option<Decimal>,
        formula: Option<serde_json::Value>,
        level: Option<AssessmentLevel>,
        institution_scope: Option<InstitutionScope>,
        components: Option<Vec<AssessmentComponent>>,
        is_default: Option<bool>,
    ) -> Result<AssessmentScheme, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AssessmentWrite)
            .await?;

        let mut scheme = self
            .scheme_repo
            .find_by_id(scheme_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Assessment Scheme {} not found", scheme_id))
            })?;

        if scheme.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if let Some(n) = name {
            scheme.name = n;
        }
        if let Some(nar) = name_ar {
            scheme.name_ar = Some(nar);
        }
        if let Some(d) = description {
            scheme.description = Some(d);
        }
        if let Some(m) = max_score {
            scheme.max_score = m;
        }
        if let Some(f) = formula {
            scheme.formula = Some(f);
        }
        if let Some(lvl) = level {
            scheme.level = lvl;
        }
        if let Some(scope) = institution_scope {
            scheme.institution_scope = scope;
        }
        if let Some(comps) = components {
            scheme.components = comps;
        }
        if let Some(def) = is_default {
            scheme.is_default = def;
        }
        scheme.updated_at = Utc::now();

        self.scheme_repo.save(&scheme).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AssessmentSchemeUpdated {
                    scheme_id,
                    org_id,
                    timestamp: scheme.updated_at,
                },
            )
            .await;

        Ok(scheme)
    }

    /// Delete assessment scheme
    pub async fn delete_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        scheme_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::AssessmentDelete)
            .await?;

        let scheme = self
            .scheme_repo
            .find_by_id(scheme_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Assessment Scheme {} not found", scheme_id))
            })?;

        if scheme.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.scheme_repo.delete(scheme_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AssessmentSchemeDeleted {
                    scheme_id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// List assessment schemes
    pub async fn list_assessment_schemes(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<AssessmentScheme>, DomainError> {
        self.scheme_repo.find_by_org(org_id).await
    }

    /// List grades
    pub async fn list_grades(
        &self,
        org_id: Uuid,
        filter: GradeFilter,
    ) -> Result<PaginatedList<Grade>, DomainError> {
        self.grade_repo.list(org_id, filter).await
    }

    /// List attendance
    pub async fn list_attendance(
        &self,
        org_id: Uuid,
        filter: AttendanceFilter,
    ) -> Result<PaginatedList<Attendance>, DomainError> {
        self.attendance_repo.list(org_id, filter).await
    }

    /// Record behavior for a student
    pub async fn record_behavior(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        student_id: Uuid,
        record_type: String,
        description: String,
        points: i32,
    ) -> Result<BehaviorRecord, DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorWrite)
            .await?;
        let now = Utc::now();
        let record_id = Uuid::new_v4();
        let record = BehaviorRecord {
            id: record_id,
            student_id,
            org_id,
            reporter_id: ctx.user_id().unwrap_or(Uuid::nil()),
            record_type: record_type.clone(),
            description,
            points,
            created_at: now,
            updated_at: now,
        };
        self.behavior_repo.save(&record).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::BehaviorLogged {
                    record_id,
                    student_id,
                    record_type,
                    points,
                    timestamp: now,
                },
            )
            .await;

        Ok(record)
    }

    /// Delete behavior record
    pub async fn delete_behavior(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        behavior_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorDelete)
            .await?;

        let rec = self
            .behavior_repo
            .find_by_id(behavior_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Behavior record {} not found", behavior_id))
            })?;

        if rec.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.behavior_repo.delete(behavior_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::BehaviorDeleted {
                    record_id: behavior_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// List behavior records for a student
    pub async fn list_student_behavior_records(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<BehaviorRecord>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorRead)
            .await?;
        self.behavior_repo.find_by_student(student_id).await
    }

    /// List all behavior records for an organization
    pub async fn list_behavior_records(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Vec<BehaviorRecord>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorRead)
            .await?;
        self.behavior_repo.find_by_org(org_id).await
    }

    /// Calculate comprehensive student term report card
    pub async fn calculate_student_term_report_card(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        student_id: Uuid,
        period_id: Uuid,
        section_id: Option<Uuid>,
        policy: Option<AcademicPolicy>,
    ) -> Result<StudentTermReportCard, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        // 1. Resolve student details
        let (student_name, student_number) = if let Some(sr) = &self.student_repo {
            if let Some(detail) = sr.find_detail_by_id(student_id).await? {
                (
                    detail.person.full_name(),
                    Some(detail.student.student_number),
                )
            } else if let Some(st) = sr.find_by_id(student_id).await? {
                (
                    format!("Student {}", &st.id.to_string()[..8]),
                    Some(st.student_number),
                )
            } else {
                ("Student".to_string(), None)
            }
        } else {
            ("Student".to_string(), None)
        };

        // 2. Resolve period details and child periods
        let (period_name, child_periods) = if let Some(pr) = &self.period_repo {
            let p = pr.find_by_id(period_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;
            let all_year_periods = pr.find_by_year(p.year_id).await?;
            let mut children: Vec<_> = all_year_periods
                .into_iter()
                .filter(|cp| cp.parent_id == Some(period_id))
                .collect();
            children.sort_by_key(|cp| cp.sort_order);
            (p.name, children)
        } else {
            ("Term Period".to_string(), vec![])
        };

        // 3. Resolve class
        let resolved_section_id = if let Some(cid) = section_id {
            cid
        } else if let Some(er) = &self.enrollment_repo {
            let enrollments = er.find_by_student(student_id).await?;
            enrollments
                .into_iter()
                .find(|e| e.period_id == period_id || e.status == EnrollmentStatus::Active)
                .and_then(|e| e.section_id)
                .unwrap_or_else(Uuid::nil)
        } else {
            Uuid::nil()
        };

        let class_name = if let Some(cr) = &self.class_repo {
            if let Some(cls) = cr.find_by_id(resolved_section_id).await? {
                cls.name
            } else {
                "Class".to_string()
            }
        } else {
            "Class".to_string()
        };

        // 4. Resolve class subjects
        let section_subjects = if resolved_section_id != Uuid::nil() {
            let cs_list = self
                .section_subject_repo
                .find_by_class_and_period(resolved_section_id, period_id)
                .await?;
            if cs_list.is_empty() {
                self.section_subject_repo
                    .find_by_class(resolved_section_id)
                    .await?
            } else {
                cs_list
            }
        } else {
            vec![]
        };

        // 5. Calculate subject term results
        let mut subject_results = Vec::with_capacity(section_subjects.len());

        for cs in &section_subjects {
            let subject = self.subject_repo.find_by_id(cs.subject_id).await?;
            let (subject_code, subject_name, subject_name_ar, credit_units) = match &subject {
                Some(s) => (
                    s.code.clone(),
                    s.name.clone(),
                    s.name_ar.clone(),
                    s.credit_units.unwrap_or(Decimal::ONE),
                ),
                None => (
                    "SUBJ".to_string(),
                    "Subject".to_string(),
                    None,
                    Decimal::ONE,
                ),
            };

            let all_grades = self
                .grade_repo
                .find_by_student_and_subject(student_id, cs.id)
                .await?;

            let mut monthly_results = Vec::new();
            let mut term_final_exam = None;

            if !child_periods.is_empty() {
                for cp in &child_periods {
                    let is_exam = cp.period_kind == PeriodKind::Examination
                        || cp
                            .code
                            .as_deref()
                            .unwrap_or("")
                            .to_uppercase()
                            .contains("EXAM");

                    let period_grades: Vec<&Grade> =
                        all_grades.iter().filter(|g| g.period_id == cp.id).collect();

                    if is_exam {
                        if let Some(first_grade) = period_grades.first() {
                            term_final_exam = Some(first_grade.score);
                        }
                    } else {
                        // Monthly period
                        let components: Vec<ComponentScore> = if period_grades.is_empty() {
                            vec![]
                        } else {
                            period_grades
                                .iter()
                                .map(|g| ComponentScore {
                                    code: g
                                        .component_id
                                        .map(|cid| cid.to_string())
                                        .unwrap_or_else(|| "TOTAL".into()),
                                    name: g.notes.clone().unwrap_or_else(|| "Score".into()),
                                    name_ar: None,
                                    score: g.score,
                                    max_score: g.max_score,
                                })
                                .collect()
                        };

                        let mut breakdown = PeriodComponentScores {
                            components,
                            total: Decimal::ZERO,
                            total_max: Decimal::from(100),
                        };
                        breakdown.compute_total();

                        monthly_results.push(PeriodAssessmentResult {
                            period_id: cp.id,
                            period_code: cp.code.clone().unwrap_or_default(),
                            period_name: cp.name.clone(),
                            period_name_ar: cp.name_ar.clone(),
                            breakdown,
                        });
                    }
                }
            } else {
                // Flat/direct grades
                let mut direct_components = Vec::new();
                for g in &all_grades {
                    if g.is_final {
                        term_final_exam = Some(g.score);
                    } else {
                        direct_components.push(ComponentScore {
                            code: g
                                .component_id
                                .map(|cid| cid.to_string())
                                .unwrap_or_else(|| "TOTAL".into()),
                            name: g.notes.clone().unwrap_or_else(|| "Score".into()),
                            name_ar: None,
                            score: g.score,
                            max_score: g.max_score,
                        });
                    }
                }
                if !direct_components.is_empty() {
                    let mut breakdown = PeriodComponentScores {
                        components: direct_components,
                        total: Decimal::ZERO,
                        total_max: Decimal::from(100),
                    };
                    breakdown.compute_total();
                    monthly_results.push(PeriodAssessmentResult {
                        period_id,
                        period_code: "TERM".into(),
                        period_name: period_name.clone(),
                        period_name_ar: None,
                        breakdown,
                    });
                }
            }

            let res = AcademicCalculationEngine::calculate_subject_term_result(
                cs.subject_id,
                subject_code,
                subject_name,
                subject_name_ar,
                credit_units,
                monthly_results,
                term_final_exam,
                &policy,
            )?;
            subject_results.push(res);
        }

        // 6. Aggregate student term report card
        let total_score: Decimal = subject_results.iter().map(|s| s.term_total).sum();
        let max_possible_score = Decimal::from(subject_results.len() * 50);
        let raw_pct = if max_possible_score > Decimal::ZERO {
            (total_score / max_possible_score) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        let overall_percentage =
            apply_rounding(raw_pct, policy.rounding_mode, policy.decimal_places);

        let status = if overall_percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if overall_percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        Ok(StudentTermReportCard {
            student_id,
            student_name,
            student_number,
            section_id: resolved_section_id,
            class_name,
            period_id,
            period_name,
            subjects: subject_results,
            total_score,
            max_possible_score,
            overall_percentage,
            gpa: None,
            rank_in_class: None,
            rank_in_grade: None,
            total_students_in_class: 0,
            total_students_in_grade: 0,
            status,
            calculated_at: Utc::now(),
        })
    }

    /// Calculate comprehensive student annual report card
    pub async fn calculate_student_annual_report_card(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        student_id: Uuid,
        calendar_id: Uuid,
        section_id: Option<Uuid>,
        policy: Option<AcademicPolicy>,
    ) -> Result<StudentAnnualReportCard, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        let (year_name, term_periods) = if let Some(pr) = &self.period_repo {
            let all_periods = pr.find_by_year(calendar_id).await?;
            let mut terms: Vec<_> = all_periods
                .into_iter()
                .filter(|p| p.parent_id.is_none())
                .collect();
            terms.sort_by_key(|p| p.sort_order);

            let yr_name = if let Some(yr) = &self.year_repo {
                yr.find_by_id(calendar_id)
                    .await?
                    .map(|y| y.name)
                    .unwrap_or_else(|| "Academic Year".into())
            } else {
                "Academic Year".into()
            };
            (yr_name, terms)
        } else {
            ("Academic Year".into(), vec![])
        };

        let term_1_card = if let Some(t1) = term_periods.first() {
            Some(
                self.calculate_student_term_report_card(
                    ctx,
                    org_id,
                    student_id,
                    t1.id,
                    section_id,
                    Some(policy.clone()),
                )
                .await?,
            )
        } else {
            None
        };

        let term_2_card = if term_periods.len() > 1 {
            let t2 = &term_periods[1];
            Some(
                self.calculate_student_term_report_card(
                    ctx,
                    org_id,
                    student_id,
                    t2.id,
                    section_id,
                    Some(policy.clone()),
                )
                .await?,
            )
        } else {
            None
        };

        let student_name = term_1_card
            .as_ref()
            .map(|c| c.student_name.clone())
            .or_else(|| term_2_card.as_ref().map(|c| c.student_name.clone()))
            .unwrap_or_else(|| "Student".into());

        let student_number = term_1_card
            .as_ref()
            .and_then(|c| c.student_number.clone())
            .or_else(|| term_2_card.as_ref().and_then(|c| c.student_number.clone()));

        let resolved_section_id = section_id.unwrap_or_else(|| {
            term_1_card
                .as_ref()
                .map(|c| c.section_id)
                .or_else(|| term_2_card.as_ref().map(|c| c.section_id))
                .unwrap_or_else(Uuid::nil)
        });

        let class_name = term_1_card
            .as_ref()
            .map(|c| c.class_name.clone())
            .or_else(|| term_2_card.as_ref().map(|c| c.class_name.clone()))
            .unwrap_or_else(|| "Class".into());

        // Match subjects
        let mut subject_map: std::collections::BTreeMap<
            Uuid,
            (Option<SubjectTermResult>, Option<SubjectTermResult>),
        > = std::collections::BTreeMap::new();

        if let Some(t1) = &term_1_card {
            for s in &t1.subjects {
                subject_map.entry(s.subject_id).or_insert((None, None)).0 = Some(s.clone());
            }
        }
        if let Some(t2) = &term_2_card {
            for s in &t2.subjects {
                subject_map.entry(s.subject_id).or_insert((None, None)).1 = Some(s.clone());
            }
        }

        let mut annual_subjects = Vec::with_capacity(subject_map.len());
        for (subject_id, (t1_res, t2_res)) in subject_map {
            let (code, name, name_ar, credit) = if let Some(r) = t1_res.as_ref().or(t2_res.as_ref())
            {
                (
                    r.subject_code.clone(),
                    r.subject_name.clone(),
                    r.subject_name_ar.clone(),
                    r.credit_hours,
                )
            } else {
                ("SUBJ".into(), "Subject".into(), None, Decimal::ONE)
            };

            let annual_res = AcademicCalculationEngine::calculate_subject_annual_result(
                subject_id, code, name, name_ar, credit, t1_res, t2_res, &policy,
            )?;
            annual_subjects.push(annual_res);
        }

        let total_score: Decimal = annual_subjects.iter().map(|s| s.annual_total).sum();
        let max_possible_score = Decimal::from(annual_subjects.len() * 100);
        let raw_pct = if max_possible_score > Decimal::ZERO {
            (total_score / max_possible_score) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        let overall_percentage =
            apply_rounding(raw_pct, policy.rounding_mode, policy.decimal_places);

        let status = if overall_percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if overall_percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        Ok(StudentAnnualReportCard {
            student_id,
            student_name,
            student_number,
            section_id: resolved_section_id,
            class_name,
            calendar_id,
            academic_calendar_name: year_name,
            subjects: annual_subjects,
            total_score,
            max_possible_score,
            overall_percentage,
            cumulative_gpa: None,
            rank_in_class: None,
            rank_in_grade: None,
            total_students_in_class: 0,
            total_students_in_grade: 0,
            status,
            calculated_at: Utc::now(),
        })
    }

    /// Calculate class term results with rankings
    pub async fn calculate_class_term_report_cards(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        section_id: Uuid,
        period_id: Uuid,
        policy: Option<AcademicPolicy>,
    ) -> Result<Vec<StudentTermReportCard>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        let enrollments = if let Some(er) = &self.enrollment_repo {
            er.find_by_class(section_id).await?
        } else {
            vec![]
        };

        let mut cards = Vec::with_capacity(enrollments.len());
        for en in &enrollments {
            let card = self
                .calculate_student_term_report_card(
                    ctx,
                    org_id,
                    en.student_id,
                    period_id,
                    Some(section_id),
                    Some(policy.clone()),
                )
                .await?;
            cards.push(card);
        }

        let total_students = cards.len();
        if policy.ranking_enabled && !cards.is_empty() {
            let scores: Vec<(Uuid, Decimal)> = cards
                .iter()
                .map(|c| (c.student_id, c.overall_percentage))
                .collect();
            let rankings =
                AcademicCalculationEngine::compute_rankings_legacy(&scores, policy.tie_policy)?;
            let rank_map: std::collections::HashMap<Uuid, u32> = rankings.into_iter().collect();

            for c in &mut cards {
                if let Some(r) = rank_map.get(&c.student_id) {
                    if let Some(max_r) = policy.max_displayed_rank {
                        if *r <= max_r {
                            c.rank_in_class = Some(*r);
                        }
                    } else {
                        c.rank_in_class = Some(*r);
                    }
                }
                c.total_students_in_class = total_students;
            }
        }

        Ok(cards)
    }

    /// Calculate class annual results with rankings
    pub async fn calculate_class_annual_report_cards(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        section_id: Uuid,
        calendar_id: Uuid,
        policy: Option<AcademicPolicy>,
    ) -> Result<Vec<StudentAnnualReportCard>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        let enrollments = if let Some(er) = &self.enrollment_repo {
            er.find_by_class(section_id).await?
        } else {
            vec![]
        };

        let mut cards = Vec::with_capacity(enrollments.len());
        for en in &enrollments {
            let card = self
                .calculate_student_annual_report_card(
                    ctx,
                    org_id,
                    en.student_id,
                    calendar_id,
                    Some(section_id),
                    Some(policy.clone()),
                )
                .await?;
            cards.push(card);
        }

        let total_students = cards.len();
        if policy.ranking_enabled && !cards.is_empty() {
            let scores: Vec<(Uuid, Decimal)> = cards
                .iter()
                .map(|c| (c.student_id, c.overall_percentage))
                .collect();
            let rankings =
                AcademicCalculationEngine::compute_rankings_legacy(&scores, policy.tie_policy)?;
            let rank_map: std::collections::HashMap<Uuid, u32> = rankings.into_iter().collect();

            for c in &mut cards {
                if let Some(r) = rank_map.get(&c.student_id) {
                    if let Some(max_r) = policy.max_displayed_rank {
                        if *r <= max_r {
                            c.rank_in_class = Some(*r);
                        }
                    } else {
                        c.rank_in_class = Some(*r);
                    }
                }
                c.total_students_in_class = total_students;
            }
        }

        Ok(cards)
    }
}
