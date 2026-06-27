use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::entities::{grade, grade_history};

// ── Request / Response DTOs ──

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

/// Bulk grade entry for a single subject/assessment across an entire class
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkSubjectGradeEntry {
    pub cycle_id: String,
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
    pub cycle_id: String,
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

// ── Report Card ──

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GradeReportCard {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub cycle_name: String,
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

// ── Analytics ──

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
    pub excellent: usize,   // ممتاز  90-100
    pub very_good: usize,   // جيد جداً 80-89
    pub good: usize,        // جيد 70-79
    pub acceptable: usize,  // مقبول 60-69
    pub weak: usize,        // ضعيف 50-59
    pub fail: usize,        // راسب <50
}

// ── Service Implementation ──

impl super::Service {
    /// Flexible grading scale: percentage → grade letter
    /// Works for Yemeni, Saudi, and any institution
    pub fn score_to_grade_letter(percentage: f64) -> String {
        match percentage as i32 {
            90..=100 => "ممتاز".to_string(),
            80..=89  => "جيد جداً".to_string(),
            70..=79  => "جيد".to_string(),
            60..=69  => "مقبول".to_string(),
            50..=59  => "ضعيف".to_string(),
            _        => "راسب".to_string(),
        }
    }

    /// Flexible GPA scale: percentage → grade points (4.0 scale)
    pub fn score_to_grade_points(percentage: f64) -> f64 {
        match percentage as i32 {
            90..=100 => 4.0,
            80..=89  => 3.5,
            70..=79  => 3.0,
            60..=69  => 2.5,
            50..=59  => 2.0,
            _        => 0.0,
        }
    }

    /// Check if the academic cycle allows grade entry
    async fn check_cycle_open_for_grading(&self, cycle_id: &str) -> Result<(), String> {
        let cycle = self.repo.get_cycle_by_id(cycle_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "الدورة الأكاديمية غير موجودة".to_string())?;

        if cycle.is_locked == 1 {
            return Err("الدورة الأكاديمية مقفلة. لا يمكن إدخال أو تعديل الدرجات. تواصل مع الإدارة لفتح القفل.".to_string());
        }

        if cycle.is_grade_entry_open == 0 {
            return Err("فترة إدخال الدرجات مغلقة لهذه الدورة. يرجى فتح فترة الإدخال أولاً.".to_string());
        }

        // Check date range if specified
        if let (Some(ref start), Some(ref end)) = (&cycle.grade_entry_start, &cycle.grade_entry_end) {
            let now = Utc::now().to_rfc3339();
            if now < *start {
                return Err(format!("فترة إدخال الدرجات لم تبدأ بعد. تبدأ في: {}", start));
            }
            if now > *end {
                return Err(format!("فترة إدخال الدرجات انتهت في: {}", end));
            }
        }

        Ok(())
    }

    /// Save grade history before any update
    async fn save_grade_history(&self, existing: &grade::Model, changed_by: &str, change_reason: Option<String>, change_type: &str) -> Result<(), String> {
        let history = grade_history::Model {
            id: Uuid::new_v4().to_string(),
            grade_id: existing.id.clone(),
            score: Some(existing.score),
            grade_letter: existing.grade_letter.clone(),
            is_absent: None,
            is_excused: None,
            comment: existing.notes.clone(),
            changed_by: Some(changed_by.to_string()),
            changed_at: Utc::now().to_rfc3339(),
            change_reason,
            change_type: change_type.to_string(),
            snapshot: Some(serde_json::to_string(existing).unwrap_or_default()),
            created_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_grade_history(history).await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn submit_grade(&self, org_id: &str, graded_by: &str, req: CreateGradeRequest) -> Result<grade::Model, String> {
        // Validate enrollment exists
        let enrollment = self.repo.get_enrollment_by_id(&req.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "enrollment not found".to_string())?;

        // Check cycle is open for grading
        self.check_cycle_open_for_grading(&enrollment.cycle_id).await?;

        if req.score < 0.0 || req.score > req.max_score {
            return Err(format!("الدرجة يجب أن تكون بين 0 و {}", req.max_score));
        }

        let percentage = (req.score / req.max_score) * 100.0;
        let grade_letter = Self::score_to_grade_letter(percentage);
        let grade_points = Self::score_to_grade_points(percentage);

        let g = grade::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: enrollment.cycle_id.clone(),
            enrollment_id: req.enrollment_id,
            assessment_id: req.assessment_id,
            class_subject_id: req.class_subject_id,
            subject_name: req.subject_name,
            subject_name_ar: req.subject_name_ar,
            score: req.score,
            max_score: req.max_score,
            weight: req.weight.unwrap_or(1.0),
            grade_letter: Some(grade_letter),
            grade_points: Some(grade_points),
            exam_type: req.exam_type,
            semester: req.semester,
            notes: req.notes,
            graded_by: graded_by.to_string(),
            graded_at: Utc::now().to_rfc3339(),
            is_final: req.is_final.unwrap_or(0),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        self.repo.create_grade(g).await
            .map_err(|e| e.to_string())
    }

    pub async fn submit_grades_bulk(&self, org_id: &str, graded_by: &str, req: BulkGradeRequest) -> Result<Vec<grade::Model>, String> {
        let mut results = Vec::new();
        for grade_req in req.grades {
            let result = self.submit_grade(org_id, graded_by, grade_req).await?;
            results.push(result);
        }
        Ok(results)
    }

    /// Bulk grade entry for a single subject across all students in a class.
    /// This is the main "Control" function - upserts grades (creates if new, updates if existing).
    pub async fn bulk_subject_grade_entry(&self, org_id: &str, graded_by: &str, req: BulkSubjectGradeEntry) -> Result<BulkSubjectGradeResult, String> {
        // Check cycle is open for grading
        self.check_cycle_open_for_grading(&req.cycle_id).await?;

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
                result.errors.push(format!(
                    "الطالب {}: الدرجة {} خارج النطاق (0 - {})",
                    student.enrollment_id, student.score, req.max_score
                ));
                continue;
            }

            // Check if grade already exists for this enrollment + assessment
            let existing = if let Some(ref aid) = req.assessment_id {
                self.repo.find_grade_by_enrollment_assessment(&student.enrollment_id, aid).await
                    .map_err(|e| e.to_string())?
            } else {
                None
            };

            let percentage = (student.score / req.max_score) * 100.0;
            let grade_letter = Self::score_to_grade_letter(percentage);
            let grade_points = Self::score_to_grade_points(percentage);

            match existing {
                Some(existing_grade) => {
                    // Update existing - save history first
                    self.save_grade_history(&existing_grade, graded_by, Some("تعديل جماعي".to_string()), "bulk_update").await?;

                    let updated = self.repo.update_grade(
                        &existing_grade.id,
                        student.score,
                        req.max_score,
                        Some(grade_letter),
                        Some(grade_points),
                        student.notes.clone(),
                    ).await.map_err(|e| e.to_string())?;

                    result.updated += 1;
                    result.grades.push(updated);
                }
                None => {
                    // Create new
                    let enrollment = self.repo.get_enrollment_by_id(&student.enrollment_id).await
                        .map_err(|e| e.to_string())?;

                    if enrollment.is_none() {
                        result.failed += 1;
                        result.errors.push(format!("التسجيل {} غير موجود", student.enrollment_id));
                        continue;
                    }

                    let g = grade::Model {
                        id: Uuid::new_v4().to_string(),
                        org_id: org_id.to_string(),
                        cycle_id: req.cycle_id.clone(),
                        enrollment_id: student.enrollment_id.clone(),
                        assessment_id: req.assessment_id.clone(),
                        class_subject_id: req.class_subject_id.clone(),
                        subject_name: req.subject_name.clone(),
                        subject_name_ar: req.subject_name_ar.clone(),
                        score: student.score,
                        max_score: req.max_score,
                        weight: req.weight.unwrap_or(1.0),
                        grade_letter: Some(grade_letter),
                        grade_points: Some(grade_points),
                        exam_type: req.exam_type.clone(),
                        semester: req.semester.clone(),
                        notes: student.notes.clone(),
                        graded_by: graded_by.to_string(),
                        graded_at: Utc::now().to_rfc3339(),
                        is_final: req.is_final.unwrap_or(0),
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                    };

                    match self.repo.create_grade(g).await {
                        Ok(created) => {
                            result.created += 1;
                            result.grades.push(created);
                        }
                        Err(e) => {
                            result.failed += 1;
                            result.errors.push(format!("خطأ في إنشاء درجة {}: {}", student.enrollment_id, e));
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    pub async fn get_grades(&self, org_id: &str, params: crate::params::GradeParams) -> Result<Vec<grade::Model>, String> {
        self.repo.get_grades(org_id, params).await
            .map_err(|e| e.to_string())
    }

    pub async fn update_grade(&self, id: &str, graded_by: &str, req: UpdateGradeRequest) -> Result<grade::Model, String> {
        if req.score < 0.0 || req.score > req.max_score {
            return Err(format!("الدرجة يجب أن تكون بين 0 و {}", req.max_score));
        }

        // Get existing grade to check cycle and save history
        let existing = self.repo.get_grade_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "الدرجة غير موجودة".to_string())?;

        // Check cycle is open for grading
        self.check_cycle_open_for_grading(&existing.cycle_id).await?;

        // Save history before update
        self.save_grade_history(&existing, graded_by, req.change_reason.clone(), "update").await?;

        let percentage = (req.score / req.max_score) * 100.0;
        let grade_letter = Self::score_to_grade_letter(percentage);
        let grade_points = Self::score_to_grade_points(percentage);

        self.repo.update_grade(id, req.score, req.max_score, Some(grade_letter), Some(grade_points), req.notes).await
            .map_err(|e| e.to_string())
    }

    /// Get the grade matrix for a class+subject: all students × all assessment types
    pub async fn get_grade_matrix(&self, _org_id: &str, cycle_id: &str, class_id: &str, subject_id: &str) -> Result<GradeMatrixResponse, String> {
        // Find class_subject
        let cs = self.repo.find_class_subject_for_assessment(class_id, subject_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "لم يتم العثور على ربط الفصل بالمادة".to_string())?;

        // Get all enrollments in this class
        let enrollments = self.repo.get_enrollments_by_class(class_id).await
            .map_err(|e| e.to_string())?;

        // Get all grades for this class_subject in cycle
        let all_grades = self.repo.get_grade_matrix(&cs.id, cycle_id).await
            .map_err(|e| e.to_string())?;

        // Get all assessments for this class_subject
        let assessments = self.repo.get_assessments(&cs.id, cycle_id).await
            .map_err(|e| e.to_string())?;

        let assessment_types: Vec<String> = assessments.iter().map(|a| a.item_type.clone()).collect();

        // Build matrix
        let mut students = Vec::new();
        for enrollment in &enrollments {
            let student = self.repo.get_user_by_id(&enrollment.student_id).await
                .map_err(|e| e.to_string())?;

            let student_grades: Vec<&grade::Model> = all_grades.iter()
                .filter(|g| g.enrollment_id == enrollment.id)
                .collect();

            let mut cells = Vec::new();
            let mut total = 0.0f64;
            let mut max_total = 0.0f64;

            for assessment in &assessments {
                let grade_for_assessment = student_grades.iter()
                    .find(|g| g.assessment_id.as_deref() == Some(&assessment.id));

                cells.push(GradeCell {
                    grade_id: grade_for_assessment.map(|g| g.id.clone()),
                    assessment_id: Some(assessment.id.clone()),
                    exam_type: assessment.item_type.clone(),
                    score: grade_for_assessment.map(|g| g.score),
                    max_score: assessment.max_score,
                });

                if let Some(g) = grade_for_assessment {
                    total += g.score;
                }
                max_total += assessment.max_score;
            }

            let percentage = if max_total > 0.0 { (total / max_total) * 100.0 } else { 0.0 };
            let grade_letter = Self::score_to_grade_letter(percentage);

            students.push(StudentGradeRow {
                enrollment_id: enrollment.id.clone(),
                student_id: enrollment.student_id.clone(),
                student_name: student.as_ref().map(|u| u.full_name.clone()).unwrap_or_default(),
                student_name_ar: student.as_ref().and_then(|u| u.name_ar.clone()),
                grades: cells,
                total,
                max_total,
                percentage: (percentage * 100.0).round() / 100.0,
                grade_letter,
            });
        }

        Ok(GradeMatrixResponse {
            cycle_id: cycle_id.to_string(),
            class_subject_id: cs.id,
            students,
            assessment_types,
        })
    }

    /// Generate a full report card for a student enrollment
    pub async fn generate_report_card(&self, enrollment_id: &str) -> Result<GradeReportCard, String> {
        let enrollment = self.repo.get_enrollment_by_id(enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "enrollment not found".to_string())?;

        let student = self.repo.get_user_by_id(&enrollment.student_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "student not found".to_string())?;

        let cycle = self.repo.get_cycle_by_id(&enrollment.cycle_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "academic cycle not found".to_string())?;

        let passing_grade = cycle.passing_grade.unwrap_or(50) as f64;

        // Get all final grades for this enrollment
        let grades = self.repo.get_grades_by_enrollment_final(enrollment_id).await
            .map_err(|e| e.to_string())?;

        // Group grades by subject and compute weighted score per subject
        let mut subject_map: std::collections::HashMap<String, Vec<&grade::Model>> = std::collections::HashMap::new();
        for g in &grades {
            subject_map.entry(g.subject_name.clone()).or_default().push(g);
        }

        let mut subjects = Vec::new();
        let mut total_weight = 0.0f64;
        let mut weighted_points = 0.0f64;
        let mut passed_subjects = 0usize;
        let mut failed_subjects = 0usize;

        for (subject_name, subject_grades) in &subject_map {
            // Weighted average for this subject: Σ(score/max_score × weight) / Σ(weight) × 100
            let sw: f64 = subject_grades.iter().map(|g| g.weight).sum();
            let weighted_sum: f64 = subject_grades.iter()
                .map(|g| (g.score / g.max_score) * g.weight)
                .sum();
            let percentage = if sw > 0.0 { (weighted_sum / sw) * 100.0 } else { 0.0 };

            let grade_letter = Self::score_to_grade_letter(percentage);
            let grade_points = Self::score_to_grade_points(percentage);
            let passed = percentage >= passing_grade;

            if passed { passed_subjects += 1; } else { failed_subjects += 1; }

            total_weight += sw;
            weighted_points += grade_points * sw;

            subjects.push(SubjectGradeSummary {
                subject_name: subject_name.clone(),
                subject_name_ar: subject_grades.first().and_then(|g| g.subject_name_ar.clone()),
                final_score: weighted_sum / sw * 100.0,
                max_score: 100.0,
                percentage,
                grade_letter,
                grade_points,
                passed,
            });
        }

        // GPA = Σ(grade_points × weight) / Σ(weight)
        let overall_gpa = if total_weight > 0.0 { weighted_points / total_weight } else { 0.0 };

        // Overall percentage
        let overall_percentage = if !subjects.is_empty() {
            subjects.iter().map(|s| s.percentage).sum::<f64>() / subjects.len() as f64
        } else {
            0.0
        };

        let overall_letter = Self::score_to_grade_letter(overall_percentage);

        // Sort subjects alphabetically
        subjects.sort_by(|a, b| a.subject_name.cmp(&b.subject_name));

        Ok(GradeReportCard {
            enrollment_id: enrollment_id.to_string(),
            student_id: student.id,
            student_name: student.full_name,
            cycle_name: cycle.name,
            subjects,
            overall_percentage: (overall_percentage * 100.0).round() / 100.0,
            overall_gpa: (overall_gpa * 100.0).round() / 100.0,
            overall_letter,
            total_subjects: subject_map.len(),
            passed_subjects,
            failed_subjects,
        })
    }

    /// Class-wide grade analytics for a specific subject
    pub async fn get_grade_analytics(&self, org_id: &str, params: crate::params::GradeParams) -> Result<GradeAnalytics, String> {
        let grades = self.repo.get_grades(org_id, params).await
            .map_err(|e| e.to_string())?;

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
                    excellent: 0, very_good: 0, good: 0, acceptable: 0, weak: 0, fail: 0,
                },
            });
        }

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

        let pass_count = percentages.iter().filter(|&&p| p >= 50.0).count();
        let fail_count = count - pass_count;
        let pass_rate = (pass_count as f64 / count as f64) * 100.0;

        // Grade distribution (Yemeni scale)
        let mut dist = GradeDistribution {
            excellent: 0, very_good: 0, good: 0, acceptable: 0, weak: 0, fail: 0,
        };
        for &pct in &percentages {
            match pct as i32 {
                90..=100 => dist.excellent += 1,
                80..=89  => dist.very_good += 1,
                70..=79  => dist.good += 1,
                60..=69  => dist.acceptable += 1,
                50..=59  => dist.weak += 1,
                _        => dist.fail += 1,
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
}
