use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::entities::grade;

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
}

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
    pub a_plus: usize,
    pub a: usize,
    pub b_plus: usize,
    pub b: usize,
    pub c_plus: usize,
    pub c: usize,
    pub d_plus: usize,
    pub d: usize,
    pub f: usize,
}

impl super::Service {
    /// Saudi grading scale: percentage → grade letter
    pub fn score_to_grade_letter(percentage: f64) -> String {
        match percentage as i32 {
            90..=100 => "A+".to_string(),
            85..=89 => "A".to_string(),
            80..=84 => "B+".to_string(),
            75..=79 => "B".to_string(),
            70..=74 => "C+".to_string(),
            65..=69 => "C".to_string(),
            60..=64 => "D+".to_string(),
            50..=59 => "D".to_string(),
            _ => "F".to_string(),
        }
    }

    /// Saudi grading scale: percentage → grade points (5.0 scale)
    pub fn score_to_grade_points(percentage: f64) -> f64 {
        match percentage as i32 {
            90..=100 => 5.0,
            85..=89 => 4.75,
            80..=84 => 4.5,
            75..=79 => 4.0,
            70..=74 => 3.5,
            65..=69 => 3.0,
            60..=64 => 2.5,
            50..=59 => 2.0,
            _ => 1.0,
        }
    }

    pub async fn submit_grade(&self, org_id: &str, graded_by: &str, req: CreateGradeRequest) -> Result<grade::Model, String> {
        // Validate enrollment exists
        let enrollment = self.repo.get_enrollment_by_id(&req.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "enrollment not found".to_string())?;

        if req.score < 0.0 || req.score > req.max_score {
            return Err(format!("score must be between 0 and {}", req.max_score));
        }

        let percentage = (req.score / req.max_score) * 100.0;
        let grade_letter = Self::score_to_grade_letter(percentage);
        let grade_points = Self::score_to_grade_points(percentage);

        let g = grade::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: enrollment.cycle_id.clone(),
            enrollment_id: req.enrollment_id,
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

    pub async fn get_grades(&self, org_id: &str, params: crate::handler::params::GradeParams) -> Result<Vec<grade::Model>, String> {
        self.repo.get_grades(org_id, params).await
            .map_err(|e| e.to_string())
    }

    pub async fn update_grade(&self, id: &str, req: UpdateGradeRequest) -> Result<grade::Model, String> {
        if req.score < 0.0 || req.score > req.max_score {
            return Err(format!("score must be between 0 and {}", req.max_score));
        }

        let percentage = (req.score / req.max_score) * 100.0;
        let grade_letter = Self::score_to_grade_letter(percentage);
        let grade_points = Self::score_to_grade_points(percentage);

        self.repo.update_grade(id, req.score, req.max_score, Some(grade_letter), Some(grade_points), req.notes).await
            .map_err(|e| e.to_string())
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
    pub async fn get_grade_analytics(&self, org_id: &str, params: crate::handler::params::GradeParams) -> Result<GradeAnalytics, String> {
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
                    a_plus: 0, a: 0, b_plus: 0, b: 0, c_plus: 0, c: 0, d_plus: 0, d: 0, f: 0,
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

        // Grade distribution
        let mut dist = GradeDistribution {
            a_plus: 0, a: 0, b_plus: 0, b: 0, c_plus: 0, c: 0, d_plus: 0, d: 0, f: 0,
        };
        for &pct in &percentages {
            match pct as i32 {
                90..=100 => dist.a_plus += 1,
                85..=89 => dist.a += 1,
                80..=84 => dist.b_plus += 1,
                75..=79 => dist.b += 1,
                70..=74 => dist.c_plus += 1,
                65..=69 => dist.c += 1,
                60..=64 => dist.d_plus += 1,
                50..=59 => dist.d += 1,
                _ => dist.f += 1,
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
