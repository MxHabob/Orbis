use orbis_domain::entities::course_prerequisite::PrerequisiteType;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::{CoursePrerequisiteRepository, GradeRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrerequisiteCheckResult {
    pub subject_id: Uuid,
    pub prerequisite_subject_id: Uuid,
    pub requirement_type: PrerequisiteType,
    pub is_satisfied: bool,
    pub min_score_required: Option<rust_decimal::Decimal>,
    pub current_score: Option<rust_decimal::Decimal>,
    pub failure_reason: Option<String>,
}

pub struct CheckPrerequisitesUseCase {
    pub prereq_repo: Arc<dyn CoursePrerequisiteRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
}

impl CheckPrerequisitesUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        student_id: Uuid,
        subject_id: Uuid,
    ) -> Result<Vec<PrerequisiteCheckResult>, DomainError> {
        let prereqs = self.prereq_repo.find_by_subject(subject_id).await?;
        let student_grades = self.grade_repo.find_by_student(student_id).await?;

        let mut results = Vec::new();

        for prereq in prereqs {
            // Find grade for the prerequisite subject
            // For now matching on grade scheme / cs
            let matching_grade = student_grades.iter().find(|g| g.class_subject_id == prereq.prerequisite_subject_id);

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
                        PrerequisiteType::Corequisite => (true, None, None), // Can be taken in parallel
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
