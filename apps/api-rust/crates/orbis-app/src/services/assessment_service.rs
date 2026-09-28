use orbis_domain::entities::assessment_scheme::AssessmentScheme;
use orbis_domain::entities::behavior_record::BehaviorRecord;
use orbis_domain::entities::grade::Grade;
use orbis_domain::entities::attendance::Attendance;
use orbis_domain::entities::gpa_record::CumulativeGpaRecord;
use orbis_domain::entities::student::AcademicStanding;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::{GradeFilter, AttendanceFilter, PaginatedList};
use orbis_domain::iam::permission::Permission;
use crate::context::RequestContext;
use crate::ports::repositories::{AssessmentSchemeRepository, GradeRepository, AttendanceRepository, BehaviorRecordRepository, GpaRecordRepository, ClassSubjectRepository, SubjectRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::services::calculation_engine::{CalcEngine, AstNode};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use rust_decimal::Decimal;

pub struct BatchGradeItem {
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub class_subject_id: Uuid,
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
    pub class_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
}

pub struct AssessmentService {
    pub scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
    pub attendance_repo: Arc<dyn AttendanceRepository>,
    pub behavior_repo: Arc<dyn BehaviorRecordRepository>,
    pub gpa_repo: Arc<dyn GpaRecordRepository>,
    pub class_subject_repo: Arc<dyn ClassSubjectRepository>,
    pub subject_repo: Arc<dyn SubjectRepository>,
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
        class_subject_repo: Arc<dyn ClassSubjectRepository>,
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
            class_subject_repo,
            subject_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Record or update a student grade
    pub async fn record_grade(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        class_subject_id: Uuid,
        component_id: Option<Uuid>,
        score: Decimal,
        max_score: Decimal,
        is_final: bool,
    ) -> Result<Grade, DomainError> {
        self.permission_checker.check(ctx, Permission::GradeWrite).await?;

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
            class_subject_id,
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

        let _ = self.event_bus.publish(ctx, &DomainEvent::GradeRecorded {
            grade_id,
            student_id,
            class_subject_id,
            score,
            timestamp: now,
        }).await;

        Ok(grade)
    }

    /// Batch record or update student grades
    pub async fn batch_record_grades(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        items: Vec<BatchGradeItem>,
    ) -> Result<Vec<Grade>, DomainError> {
        self.permission_checker.check(ctx, Permission::GradeWrite).await?;

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
                class_subject_id: item.class_subject_id,
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
            let _ = self.event_bus.publish(ctx, &DomainEvent::GradeRecorded {
                grade_id: g.id,
                student_id: g.student_id,
                class_subject_id: g.class_subject_id,
                score: g.score,
                timestamp: now,
            }).await;
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
        self.permission_checker.check(ctx, Permission::GradeWrite).await?;

        let grade = self.grade_repo.find_by_id(grade_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Grade {} not found", grade_id)))?;

        if grade.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.grade_repo.delete(grade_id).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::GradeDeleted {
            grade_id,
            timestamp: Utc::now(),
        }).await;

        Ok(())
    }

    /// Calculate final grade from assessment scheme formula
    pub async fn calculate_scheme_grades(
        &self,
        _ctx: &RequestContext,
        scheme_id: Uuid,
        student_id: Uuid,
        class_subject_id: Uuid,
    ) -> Result<Decimal, DomainError> {
        let scheme = self.scheme_repo.find_by_id(scheme_id).await?
            .ok_or_else(|| DomainError::NotFound("Assessment Scheme not found".into()))?;

        let formula_val = scheme.formula
            .ok_or_else(|| DomainError::Validation("Scheme has no calculation formula".into()))?;

        let formula: AstNode = serde_json::from_value(formula_val)
            .map_err(|e| DomainError::Validation(format!("Invalid formula format: {}", e)))?;

        let grades = self.grade_repo.find_by_student_and_subject(student_id, class_subject_id).await?;

        let mut engine = CalcEngine::new();
        for grade in grades {
            let var_name = grade.component_id.map(|cid| cid.to_string()).unwrap_or_else(|| grade.class_subject_id.to_string());
            engine.set_variable(&var_name, grade.score);
        }

        let max_score_decimal = Decimal::from_f64_retain(scheme.max_score).unwrap_or(Decimal::from(100));
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
        self.permission_checker.check(ctx, Permission::GradeAnalytics).await?;

        let term_grades = self.grade_repo.find_by_student_and_period(student_id, period_id).await?;
        let past_records = self.gpa_repo.find_by_student(student_id).await?;

        let mut term_attempted = Decimal::ZERO;
        let mut term_earned = Decimal::ZERO;
        let mut term_points = Decimal::ZERO;

        for g in &term_grades {
            // Resolve actual subject credit hours
            let credit = if let Some(cs) = self.class_subject_repo.find_by_id(g.class_subject_id).await? {
                if let Some(subj) = self.subject_repo.find_by_id(cs.subject_id).await? {
                    subj.credit_hours.unwrap_or(Decimal::from(3))
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

        let standing = if cumulative_gpa >= Decimal::from_str_exact("3.5").unwrap_or(Decimal::from(3)) {
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

        let _ = self.event_bus.publish(ctx, &DomainEvent::GpaCalculated {
            student_id,
            period_id,
            gpa: term_gpa,
            cgpa: cumulative_gpa,
            timestamp: now,
        }).await;

        Ok(record)
    }

    /// Mark attendance record
    pub async fn mark_attendance(
        &self,
        ctx: &RequestContext,
        _org_id: Uuid,
        _period_id: Uuid,
        student_id: Uuid,
        class_id: Option<Uuid>,
        class_subject_id: Option<Uuid>,
        date: NaiveDate,
        status: String,
        notes: Option<String>,
    ) -> Result<Attendance, DomainError> {
        self.permission_checker.check(ctx, Permission::AttendanceWrite).await?;

        let now = Utc::now();
        let att_id = Uuid::new_v4();

        let attendance = Attendance {
            id: att_id,
            student_id,
            class_id,
            class_subject_id,
            scheduled_lesson_id: None,
            attendance_date: date,
            status: status.clone(),
            notes,
            marked_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.attendance_repo.save(&attendance).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::AttendanceMarked {
            attendance_id: att_id,
            student_id,
            class_id,
            class_subject_id,
            scheduled_lesson_id: None,
            status,
            timestamp: now,
        }).await;

        Ok(attendance)
    }

    /// Batch mark attendance records for a class on a date
    pub async fn batch_mark_attendance(
        &self,
        ctx: &RequestContext,
        _org_id: Uuid,
        class_id: Uuid,
        date: NaiveDate,
        items: Vec<BatchAttendanceItem>,
    ) -> Result<Vec<Attendance>, DomainError> {
        self.permission_checker.check(ctx, Permission::AttendanceWrite).await?;

        let now = Utc::now();
        let mut attendances = Vec::with_capacity(items.len());

        for item in items {
            let att_id = Uuid::new_v4();
            attendances.push(Attendance {
                id: att_id,
                student_id: item.student_id,
                class_id: Some(class_id),
                class_subject_id: item.class_subject_id,
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
            let _ = self.event_bus.publish(ctx, &DomainEvent::AttendanceMarked {
                attendance_id: a.id,
                student_id: a.student_id,
                class_id: a.class_id,
                class_subject_id: a.class_subject_id,
                scheduled_lesson_id: a.scheduled_lesson_id,
                status: a.status.clone(),
                timestamp: now,
            }).await;
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
        self.permission_checker.check(ctx, Permission::AttendanceWrite).await?;

        self.attendance_repo.delete(attendance_id).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::AttendanceDeleted {
            attendance_id,
            timestamp: Utc::now(),
        }).await;

        Ok(())
    }

    /// List student attendance records
    pub async fn list_student_attendance(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<Attendance>, DomainError> {
        self.permission_checker.check(ctx, Permission::AttendanceRead).await?;
        self.attendance_repo.find_by_student(student_id).await
    }

    /// Create assessment scheme
    pub async fn create_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        max_score: f64,
        formula: Option<serde_json::Value>,
    ) -> Result<AssessmentScheme, DomainError> {
        self.permission_checker.check(ctx, Permission::AssessmentWrite).await?;

        let now = Utc::now();
        let scheme_id = Uuid::new_v4();
        let scheme = AssessmentScheme {
            id: scheme_id,
            org_id,
            name: name.clone(),
            description: None,
            max_score,
            formula,
            created_at: now,
            updated_at: now,
        };

        self.scheme_repo.save(&scheme).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::AssessmentSchemeCreated {
            scheme_id,
            org_id,
            name,
            timestamp: now,
        }).await;

        Ok(scheme)
    }

    /// Update assessment scheme
    pub async fn update_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        scheme_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        max_score: Option<f64>,
        formula: Option<serde_json::Value>,
    ) -> Result<AssessmentScheme, DomainError> {
        self.permission_checker.check(ctx, Permission::AssessmentWrite).await?;

        let mut scheme = self.scheme_repo.find_by_id(scheme_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Assessment Scheme {} not found", scheme_id)))?;

        if scheme.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if let Some(n) = name {
            scheme.name = n;
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
        scheme.updated_at = Utc::now();

        self.scheme_repo.save(&scheme).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::AssessmentSchemeUpdated {
            scheme_id,
            org_id,
            timestamp: scheme.updated_at,
        }).await;

        Ok(scheme)
    }

    /// Delete assessment scheme
    pub async fn delete_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        scheme_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::AssessmentDelete).await?;

        let scheme = self.scheme_repo.find_by_id(scheme_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Assessment Scheme {} not found", scheme_id)))?;

        if scheme.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.scheme_repo.delete(scheme_id).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::AssessmentSchemeDeleted {
            scheme_id,
            org_id,
            timestamp: Utc::now(),
        }).await;

        Ok(())
    }

    /// List assessment schemes
    pub async fn list_assessment_schemes(&self, org_id: Uuid) -> Result<Vec<AssessmentScheme>, DomainError> {
        self.scheme_repo.find_by_org(org_id).await
    }

    /// List grades
    pub async fn list_grades(&self, org_id: Uuid, filter: GradeFilter) -> Result<PaginatedList<Grade>, DomainError> {
        self.grade_repo.list(org_id, filter).await
    }

    /// List attendance
    pub async fn list_attendance(&self, org_id: Uuid, filter: AttendanceFilter) -> Result<PaginatedList<Attendance>, DomainError> {
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
        self.permission_checker.check(ctx, Permission::BehaviorWrite).await?;
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

        let _ = self.event_bus.publish(ctx, &DomainEvent::BehaviorLogged {
            record_id,
            student_id,
            record_type,
            points,
            timestamp: now,
        }).await;

        Ok(record)
    }

    /// Delete behavior record
    pub async fn delete_behavior(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        behavior_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::BehaviorDelete).await?;

        let rec = self.behavior_repo.find_by_id(behavior_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Behavior record {} not found", behavior_id)))?;

        if rec.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.behavior_repo.delete(behavior_id).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::BehaviorDeleted {
            record_id: behavior_id,
            timestamp: Utc::now(),
        }).await;

        Ok(())
    }

    /// List behavior records for a student
    pub async fn list_student_behavior_records(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<BehaviorRecord>, DomainError> {
        self.permission_checker.check(ctx, Permission::BehaviorRead).await?;
        self.behavior_repo.find_by_student(student_id).await
    }

    /// List all behavior records for an organization
    pub async fn list_behavior_records(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Vec<BehaviorRecord>, DomainError> {
        self.permission_checker.check(ctx, Permission::BehaviorRead).await?;
        self.behavior_repo.find_by_org(org_id).await
    }
}
