use crate::ports::repositories::{AssessmentSchemeRepository, GradeRepository};
use crate::context::RequestContext;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use std::sync::Arc;
use crate::usecases::assessment::calculation_engine::{CalcEngine, AstNode};

pub struct CalculateGradesUseCase {
    pub scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
}

impl CalculateGradesUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        scheme_id: Uuid,
        student_id: Uuid,
    ) -> Result<f64, DomainError> {
        let scheme = self.scheme_repo.find_by_id(scheme_id).await?
            .ok_or_else(|| DomainError::NotFound("Assessment Scheme not found".into()))?;

        let formula_val = scheme.formula
            .ok_or_else(|| DomainError::Validation("Scheme has no calculation formula".into()))?;

        let formula: AstNode = serde_json::from_value(formula_val)
            .map_err(|e| DomainError::Validation(format!("Invalid formula format: {}", e)))?;

        let grades = self.grade_repo.find_by_student(student_id).await?;
        
use rust_decimal::prelude::ToPrimitive;

        let mut engine = CalcEngine::new();
        for grade in grades {
            let var_name = grade.component_id.map(|cid| cid.to_string()).unwrap_or_else(|| grade.class_subject_id.to_string());
            engine.set_variable(&var_name, grade.score.to_f64().unwrap_or(0.0));
        }

        let final_score = engine.evaluate(&formula)?;

        let final_score = if final_score > scheme.max_score {
            scheme.max_score
        } else {
            final_score
        };

        Ok(final_score)
    }
}
