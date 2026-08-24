//! Grade service — student grade submission, bulk entry, semester compilation, and report cards.
//!
//! All grade calculations go through [`AssessmentEngine`] which loads
//! CalculationPolicy + GradeScale for the period, falling back to the
//! organisation's `academic_policies` defaults when none are configured.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, ActiveValue::Set};

use shared_kernel::errors::AppError;
use crate::entities::grade;
use crate::params::GradeParams;
use crate::repository::{AssessmentCommandRepo, AssessmentQueryRepo};
use crate::ops::assessment_engine::AssessmentEngine;

use svc_academic::entities::enrollment;
use svc_calendar::entities::academic_period;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateGradeRequest {
    pub enrollment_id: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub score: f64,
    pub max_score: f64,
    pub weight: Option<f64>,
    pub exam_type: String,
    pub semester: String,
    pub notes: Option<String>,
    pub is_final: Option<i32>,
    pub assessment_id: Option<String>,
    pub class_subject_id: Option<String>,
    pub component_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkGradeRequest {
    pub grades: Vec<CreateGradeRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateGradeRequest {
    pub score: f64,
    pub max_score: f64,
    pub notes: Option<String>,
    pub change_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkSubjectGradeEntry {
    pub period_id: String,
    pub class_id: String,
    pub subject_id: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub assessment_id: Option<String>,
    pub class_subject_id: Option<String>,
    pub exam_type: String,
    pub semester: String,
    pub max_score: f64,
    pub weight: Option<f64>,
    pub is_final: Option<i32>,
    pub students: Vec<StudentScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct StudentScore {
    pub enrollment_id: String,
    pub score: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkSubjectGradeResult {
    pub created: usize,
    pub updated: usize,
    pub failed: usize,
    pub errors: Vec<String>,
    pub grades: Vec<grade::Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GradeMatrixResponse {
    pub period_id: String,
    pub class_subject_id: String,
    pub students: Vec<StudentGradeRow>,
    pub assessment_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct StudentGradeRow {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub student_name_ar: Option<String>,
    pub grades: Vec<GradeCell>,
    pub total: f64,
    pub max_total: f64,
    pub percentage: f64,
    pub grade_letter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GradeCell {
    pub grade_id: Option<String>,
    pub assessment_id: Option<String>,
    pub exam_type: String,
    pub score: Option<f64>,
    pub max_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GradeReportCard {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub period_name: String,
    pub subjects: Vec<SubjectGradeSummary>,
    pub overall_percentage: f64,
    pub overall_gpa: f64,
    pub overall_letter: String,
    pub total_subjects: usize,
    pub passed_subjects: usize,
    pub failed_subjects: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SubjectGradeSummary {
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub final_score: f64,
    pub max_score: f64,
    pub percentage: f64,
    pub grade_letter: String,
    pub grade_points: f64,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpsertCellRequest {
    pub enrollment_id: String,
    pub period_id: String,
    pub assessment_id: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub exam_type: String,
    pub semester: String,
    pub score: f64,
    pub max_score: f64,
    pub class_subject_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpsertCellResult {
    pub grade_id: String,
    pub created: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GradeAnalytics {
    pub class_average: f64,
    pub class_median: f64,
    pub class_min: f64,
    pub class_max: f64,
    pub pass_count: usize,
    pub fail_count: usize,
    pub pass_rate: f64,
    pub grade_distribution: GradeDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GradeDistribution {
    pub excellent: usize,
    pub very_good: usize,
    pub good: usize,
    pub acceptable: usize,
    pub weak: usize,
    pub fail: usize,
}

pub struct GradeService {
    pub cmd: Arc<AssessmentCommandRepo>,
    pub query: Arc<AssessmentQueryRepo>,
}

impl GradeService {
    pub fn new(cmd: Arc<AssessmentCommandRepo>, query: Arc<AssessmentQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Build an [`AssessmentEngine`] for the given org + period.
    /// This loads CalculationPolicy + GradeScale from the DB,
    /// falling back to org-level `academic_policies` defaults.
    async fn engine_for(&self, org_id: &str, period_id: &str) -> Result<AssessmentEngine, AppError> {
        AssessmentEngine::for_period(&self.query.db, org_id, period_id).await
    }

    pub async fn check_period_open_for_grading(&self, period_id: &str) -> Result<(), AppError> {
        let period = academic_period::Entity::find_by_id(period_id.to_string())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_period", period_id))?;

        if period.is_locked == 1 {
            return Err(AppError::unauthorized("Academic period is locked"));
        }

        if period.is_grade_entry_open == 0 {
            return Err(AppError::unauthorized("Grade entry window is closed"));
        }

        if let (Some(ref start), Some(ref end)) = (&period.grade_entry_start, &period.grade_entry_end) {
            let now = Utc::now().to_rfc3339();
            if now < *start {
                return Err(AppError::unauthorized(format!("Grade entry has not started yet. Starts: {}", start)));
            }
            if now > *end {
                return Err(AppError::unauthorized(format!("Grade entry ended on: {}", end)));
            }
        }

        Ok(())
    }



    pub async fn submit_grade(
        &self,
        org_id: &str,
        
        req: CreateGradeRequest,
    ) -> Result<grade::Model, AppError> {
        let enrollment = enrollment::Entity::find_by_id(req.enrollment_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", &req.enrollment_id))?;

        self.check_period_open_for_grading(&enrollment.period_id).await?;

        if req.score < 0.0 || req.score > req.max_score {
            return Err(AppError::validation(format!("Score must be between 0 and {}", req.max_score)));
        }

        let engine = self.engine_for(org_id, &enrollment.period_id).await?;
        let mapped = engine.compute_and_map(req.score, req.max_score)?;

        let g = grade::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            period_id: enrollment.period_id,
            enrollment_id: req.enrollment_id,
            assessment_id: req.assessment_id,
            class_subject_id: req.class_subject_id,
            subject_name: req.subject_name,
            subject_name_ar: req.subject_name_ar,
            score: req.score,
            max_score: req.max_score,
            weight: req.weight.unwrap_or(1.0),
            grade_letter: Some(mapped.grade_letter),
            grade_points: mapped.grade_points,
            exam_type: req.exam_type,
            component_id: req.component_id,
            semester: req.semester,
            notes: req.notes,
            
            graded_by: "system".to_string(),
            graded_at: Utc::now().to_rfc3339(),
            is_final: req.is_final.unwrap_or(0),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        Ok(self.cmd.create_grade(g).await?)
    }

    pub async fn submit_grades_bulk(
        &self,
        org_id: &str,
        
        req: BulkGradeRequest,
    ) -> Result<Vec<grade::Model>, AppError> {
        let mut results = Vec::new();
        for grade_req in req.grades {
            let result = self.submit_grade(org_id, grade_req).await?;
            results.push(result);
        }
        Ok(results)
    }

    pub async fn bulk_subject_grade_entry(
        &self,
        org_id: &str,
        
        req: BulkSubjectGradeEntry,
    ) -> Result<BulkSubjectGradeResult, AppError> {
        self.check_period_open_for_grading(&req.period_id).await?;

        // Build engine once for the whole batch (same period)
        let engine = self.engine_for(org_id, &req.period_id).await?;

        let mut result = BulkSubjectGradeResult {
            created: 0,
            updated: 0,
            failed: 0,
            errors: Vec::new(),
            grades: Vec::new(),
        };

        for student in &req.students {
            if student.score < 0.0 || student.score > req.max_score {
                result.failed += 1;
                result.errors.push(format!("Student {}: Score outside of range", student.enrollment_id));
                continue;
            }

            let existing = if let Some(ref aid) = req.assessment_id {
                self.query.find_grade_by_enrollment_assessment(&student.enrollment_id, aid).await?
            } else {
                None
            };

            let mapped = match engine.compute_and_map(student.score, req.max_score) {
                Ok(m) => m,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Student {}: {}", student.enrollment_id, e));
                    continue;
                }
            };

            match existing {
                Some(existing_grade) => {
                    let mut active: grade::ActiveModel = existing_grade.into();
                    active.score = Set(student.score);
                    active.max_score = Set(req.max_score);
                    active.grade_letter = Set(Some(mapped.grade_letter));
                    active.grade_points = Set(mapped.grade_points);
                    active.notes = Set(student.notes.clone());
                    active.updated_at = Set(Utc::now().to_rfc3339());

                    let updated = self.cmd.update_grade(active).await?;
                    result.updated += 1;
                    result.grades.push(updated);
                }
                None => {
                    let enrollment = enrollment::Entity::find_by_id(student.enrollment_id.clone())
                        .one(&self.query.db)
                        .await?;

                    if enrollment.is_none() {
                        result.failed += 1;
                        result.errors.push(format!("Enrollment {} not found", student.enrollment_id));
                        continue;
                    }

                    let g = grade::Model {
                        id: Uuid::new_v4().to_string(),
                        org_id: org_id.to_string(),
                        period_id: req.period_id.clone(),
                        enrollment_id: student.enrollment_id.clone(),
                        assessment_id: req.assessment_id.clone(),
                        class_subject_id: req.class_subject_id.clone(),
                        subject_name: req.subject_name.clone(),
                        subject_name_ar: req.subject_name_ar.clone(),
                        score: student.score,
                        max_score: req.max_score,
                        weight: req.weight.unwrap_or(1.0),
                        grade_letter: Some(mapped.grade_letter.clone()),
                        grade_points: mapped.grade_points,
                        exam_type: req.exam_type.clone(),
                        component_id: None,
                        semester: req.semester.clone(),
                        notes: student.notes.clone(),
                        graded_by: "system".to_string(),
                        graded_at: Utc::now().to_rfc3339(),
                        is_final: req.is_final.unwrap_or(0),
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                    };

                    match self.cmd.create_grade(g).await {
                        Ok(created) => {
                            result.created += 1;
                            result.grades.push(created);
                        }
                        Err(e) => {
                            result.failed += 1;
                            result.errors.push(format!("Failed to create grade: {}", e));
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    pub async fn get_grades(&self, org_id: &str, params: GradeParams) -> Result<Vec<grade::Model>, AppError> {
        Ok(self.query.get_grades(org_id, params).await?)
    }

    pub async fn update_grade(
        &self,
        id: &str,
        org_id: &str,
        req: UpdateGradeRequest,
    ) -> Result<grade::Model, AppError> {
        if req.score < 0.0 || req.score > req.max_score {
            return Err(AppError::validation(format!("Score must be between 0 and {}", req.max_score)));
        }

        let existing = self.query.get_grade_by_id(id).await?
            .ok_or_else(|| AppError::not_found("grade", id))?;

        self.check_period_open_for_grading(&existing.period_id).await?;

        let engine = self.engine_for(org_id, &existing.period_id).await?;
        let mapped = engine.compute_and_map(req.score, req.max_score)?;

        let mut active: grade::ActiveModel = existing.into();
        active.score = Set(req.score);
        active.max_score = Set(req.max_score);
        active.grade_letter = Set(Some(mapped.grade_letter));
        active.grade_points = Set(mapped.grade_points);
        active.notes = Set(req.notes);
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_grade(active).await?)
    }

    pub async fn upsert_grade_cell(
        &self,
        org_id: &str,
        
        req: UpsertCellRequest,
    ) -> Result<UpsertCellResult, AppError> {
        if req.score < 0.0 || req.score > req.max_score {
            return Err(AppError::validation(format!("Score must be between 0 and {}", req.max_score)));
        }

        self.check_period_open_for_grading(&req.period_id).await?;

        let engine = self.engine_for(org_id, &req.period_id).await?;
        let mapped = engine.compute_and_map(req.score, req.max_score)?;

        let existing = self.query.find_grade_by_enrollment_assessment(&req.enrollment_id, &req.assessment_id).await?;

        match existing {
            Some(g) => {
                let mut active: grade::ActiveModel = g.into();
                active.score = Set(req.score);
                active.max_score = Set(req.max_score);
                active.grade_letter = Set(Some(mapped.grade_letter));
                active.grade_points = Set(mapped.grade_points);
                active.updated_at = Set(Utc::now().to_rfc3339());

                let updated = self.cmd.update_grade(active).await?;
                Ok(UpsertCellResult { grade_id: updated.id, created: false })
            }
            None => {
                let create_req = CreateGradeRequest {
                    enrollment_id: req.enrollment_id,
                    subject_name: req.subject_name,
                    subject_name_ar: req.subject_name_ar,
                    score: req.score,
                    max_score: req.max_score,
                    weight: Some(1.0),
                    exam_type: req.exam_type,
                    semester: req.semester,
                    notes: None,
                    is_final: Some(0),
                    assessment_id: Some(req.assessment_id),
                    class_subject_id: req.class_subject_id,
                    component_id: None,
                };
                let created = self.submit_grade(org_id, create_req).await?;
                Ok(UpsertCellResult { grade_id: created.id, created: true })
            }
        }
    }

    pub async fn compile_semester_grades(
        &self,
        org_id: &str,
        period_id: &str,
        class_id: &str,
        semester: &str,
    ) -> Result<Vec<grade::Model>, AppError> {
        use sea_orm::{QueryFilter, ColumnTrait, EntityTrait};
        
        let class_subjects = svc_academic::entities::class_subject::Entity::find()
            .filter(svc_academic::entities::class_subject::Column::ClassId.eq(class_id))
            .filter(svc_academic::entities::class_subject::Column::PeriodId.eq(period_id))
            .all(&self.query.db)
            .await?;

        let cs_ids: Vec<String> = class_subjects.into_iter().map(|cs| cs.id).collect();

        if cs_ids.is_empty() {
            return Ok(Vec::new());
        }

        let grades = grade::Entity::find()
            .filter(grade::Column::OrgId.eq(org_id))
            .filter(grade::Column::PeriodId.eq(period_id))
            .filter(grade::Column::Semester.eq(semester))
            .filter(grade::Column::ClassSubjectId.is_in(cs_ids))
            .all(&self.query.db)
            .await?;

        let mut compiled = Vec::new();
        for g in grades {
            let mut active: grade::ActiveModel = g.into();
            active.is_final = Set(1);
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            let updated = self.cmd.update_grade(active).await?;
            compiled.push(updated);
        }

        Ok(compiled)
    }

    pub async fn get_report_card(
        &self,
        org_id: &str,
        enrollment_id: &str,
        period_id: &str
    ) -> Result<GradeReportCard, AppError> {
        let enrollment = enrollment::Entity::find_by_id(enrollment_id.to_string())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", enrollment_id))?;

        let student = svc_iam::entities::user::Entity::find_by_id(enrollment.student_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("student", &enrollment.student_id))?;

        let period = academic_period::Entity::find_by_id(period_id.to_string())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_period", period_id))?;

        // Use the org-level passing grade (from academic_policies) instead of hardcoded 50
        let _passing_grade = AssessmentEngine::get_org_passing_grade(&self.query.db, org_id).await;
        let engine = self.engine_for(org_id, period_id).await?;

        use sea_orm::{QueryFilter, ColumnTrait};
        let grades = grade::Entity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .filter(grade::Column::PeriodId.eq(period_id))
            .all(&self.query.db)
            .await?;

        let mut subject_map: std::collections::HashMap<String, Vec<grade::Model>> = std::collections::HashMap::new();
        for g in grades {
            subject_map.entry(g.subject_name.clone()).or_default().push(g);
        }

        let mut subjects = Vec::new();
        let mut total_weight = 0.0f64;
        let mut weighted_points = 0.0f64;
        let mut passed_subjects = 0usize;
        let mut failed_subjects = 0usize;

        for (subject_name, subject_grades) in &subject_map {
            let sw: f64 = subject_grades.iter().map(|g| g.weight).sum();
            let weighted_sum: f64 = subject_grades.iter()
                .map(|g| (g.score / g.max_score) * g.weight)
                .sum();
            let percentage = if sw > 0.0 { (weighted_sum / sw) * 100.0 } else { 0.0 };

            let mapped = engine.map_score(percentage).unwrap_or_else(|_| crate::ops::assessment_engine::AssessmentResult {
                grade_letter: "N/A".to_string(),
                grade_points: Some(0.0),
                is_passing: false,
            });

            let passed = mapped.is_passing;
            let grade_points = mapped.grade_points.unwrap_or(0.0);

            if passed { passed_subjects += 1; } else { failed_subjects += 1; }

            total_weight += sw;
            weighted_points += grade_points * sw;

            subjects.push(SubjectGradeSummary {
                subject_name: subject_name.clone(),
                subject_name_ar: subject_grades.first().and_then(|g| g.subject_name_ar.clone()),
                final_score: if sw > 0.0 { (weighted_sum / sw) * 100.0 } else { 0.0 },
                max_score: 100.0,
                percentage,
                grade_letter: mapped.grade_letter,
                grade_points,
                passed,
            });
        }

        let overall_gpa = if total_weight > 0.0 { weighted_points / total_weight } else { 0.0 };
        let overall_percentage = if !subjects.is_empty() {
            subjects.iter().map(|s| s.percentage).sum::<f64>() / subjects.len() as f64
        } else {
            0.0
        };

        let overall_mapped = engine.map_score(overall_percentage).unwrap_or_else(|_| crate::ops::assessment_engine::AssessmentResult {
            grade_letter: "N/A".to_string(),
            grade_points: Some(0.0),
            is_passing: false,
        });
        subjects.sort_by(|a, b| a.subject_name.cmp(&b.subject_name));

        Ok(GradeReportCard {
            enrollment_id: enrollment_id.to_string(),
            student_id: student.id,
            student_name: student.full_name,
            period_name: period.name,
            subjects,
            overall_percentage: (overall_percentage * 100.0).round() / 100.0,
            overall_gpa: (overall_gpa * 100.0).round() / 100.0,
            overall_letter: overall_mapped.grade_letter,
            total_subjects: subject_map.len(),
            passed_subjects,
            failed_subjects,
        })
    }

    pub async fn get_analytics(
        &self,
        org_id: &str,
        period_id: &str,
        class_subject_id: &str,
    ) -> Result<GradeAnalytics, AppError> {
        use sea_orm::{QueryFilter, ColumnTrait};
        let grades = grade::Entity::find()
            .filter(grade::Column::PeriodId.eq(period_id))
            .filter(grade::Column::ClassSubjectId.eq(class_subject_id))
            .all(&self.query.db)
            .await?;

        if grades.is_empty() {
            return Ok(GradeAnalytics {
                class_average: 0.0,
                class_median: 0.0,
                class_min: 0.0,
                class_max: 0.0,
                pass_count: 0,
                fail_count: 0,
                pass_rate: 0.0,
                grade_distribution: GradeDistribution {
                    excellent: 0,
                    very_good: 0,
                    good: 0,
                    acceptable: 0,
                    weak: 0,
                    fail: 0,
                },
            });
        }

        let engine = self.engine_for(org_id, period_id).await?;
        let passing_grade = AssessmentEngine::get_org_passing_grade(&self.query.db, org_id).await;

        let mut percentages: Vec<f64> = grades.iter()
            .map(|g| (g.score / g.max_score) * 100.0)
            .collect();
        percentages.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let count = percentages.len();
        let sum: f64 = percentages.iter().sum();
        let average = sum / count as f64;

        let median = if count % 2 == 0 {
            (percentages[count / 2 - 1] + percentages[count / 2]) / 2.0
        } else {
            percentages[count / 2]
        };

        let min = percentages.first().copied().unwrap_or(0.0);
        let max = percentages.last().copied().unwrap_or(0.0);

        let pass_count = percentages.iter().filter(|&&p| p >= passing_grade).count();
        let fail_count = count - pass_count;
        let pass_rate = (pass_count as f64 / count as f64) * 100.0;

        // Build distribution using the engine's GradeMapper
        let mut dist = GradeDistribution {
            excellent: 0,
            very_good: 0,
            good: 0,
            acceptable: 0,
            weak: 0,
            fail: 0,
        };
        for &pct in &percentages {
            if let Ok(mapped) = engine.map_score(pct) {
                // Map the label to distribution buckets
                let label = mapped.grade_letter.as_str();
                match label {
                    "ممتاز" | "A+" | "A" => dist.excellent += 1,
                    "جيد جداً" | "B+" | "B" => dist.very_good += 1,
                    "جيد" | "C+" | "C" => dist.good += 1,
                    "مقبول" | "D+" | "D" => dist.acceptable += 1,
                    "ضعيف" => dist.weak += 1,
                    _ => dist.fail += 1,
                }
            } else {
                dist.fail += 1;
            }
        }

        Ok(GradeAnalytics {
            class_average: (average * 100.0).round() / 100.0,
            class_median: (median * 100.0).round() / 100.0,
            class_min: (min * 100.0).round() / 100.0,
            class_max: (max * 100.0).round() / 100.0,
            pass_count,
            fail_count,
            pass_rate: (pass_rate * 100.0).round() / 100.0,
            grade_distribution: dist,
        })
    }

    pub async fn get_grade_matrix(
        &self,
        org_id: &str,
        period_id: &str,
        class_subject_id: &str,
    ) -> Result<GradeMatrixResponse, AppError> {
        use sea_orm::{QueryFilter, ColumnTrait, EntityTrait};

        let engine = self.engine_for(org_id, period_id).await?;

        let class_subject = svc_academic::entities::class_subject::Entity::find_by_id(class_subject_id.to_string())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("class_subject", class_subject_id))?;

        let enrollments = svc_academic::entities::enrollment::Entity::find()
            .filter(svc_academic::entities::enrollment::Column::ClassId.eq(&class_subject.class_id))
            .filter(svc_academic::entities::enrollment::Column::PeriodId.eq(period_id))
            .all(&self.query.db)
            .await?;

        let grades = grade::Entity::find()
            .filter(grade::Column::PeriodId.eq(period_id))
            .filter(grade::Column::ClassSubjectId.eq(class_subject_id))
            .all(&self.query.db)
            .await?;

        let mut grades_by_enrollment: std::collections::HashMap<String, Vec<grade::Model>> = std::collections::HashMap::new();
        for g in grades {
            grades_by_enrollment.entry(g.enrollment_id.clone()).or_default().push(g);
        }

        let mut types_set = std::collections::HashSet::new();
        for g_list in grades_by_enrollment.values() {
            for g in g_list {
                types_set.insert(g.exam_type.clone());
            }
        }
        let assessment_types: Vec<String> = types_set.into_iter().collect();

        let mut students = Vec::new();
        for enrollment in enrollments {
            let student = svc_iam::entities::person::Entity::find_by_id(enrollment.student_id.clone())
                .one(&self.query.db)
                .await?
                .ok_or_else(|| AppError::not_found("student", &enrollment.student_id))?;

            let student_grades = grades_by_enrollment.get(&enrollment.id).cloned().unwrap_or_default();
            let mut cells = Vec::new();
            let mut total = 0.0;
            let mut max_total = 0.0;

            for g in &student_grades {
                cells.push(GradeCell {
                    grade_id: Some(g.id.clone()),
                    assessment_id: g.assessment_id.clone(),
                    exam_type: g.exam_type.clone(),
                    score: Some(g.score),
                    max_score: g.max_score,
                });
                total += g.score;
                max_total += g.max_score;
            }

            let percentage = if max_total > 0.0 { (total / max_total) * 100.0 } else { 0.0 };
            let grade_letter = engine.map_score(percentage)
                .map(|m| m.grade_letter)
                .unwrap_or_else(|_| "N/A".to_string());

            students.push(StudentGradeRow {
                enrollment_id: enrollment.id,
                student_id: student.id,
                student_name: student.full_name.unwrap_or_else(|| format!("{} {}", student.first_name, student.last_name)),
                student_name_ar: student.name_ar,
                grades: cells,
                total,
                max_total,
                percentage,
                grade_letter,
            });
        }

        Ok(GradeMatrixResponse {
            period_id: period_id.to_string(),
            class_subject_id: class_subject_id.to_string(),
            students,
            assessment_types,
        })
    }
}
