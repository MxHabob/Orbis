//! Exam subject operations — adding subjects to exam sessions.

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::exam_subject;

#[derive(serde::Deserialize)]
pub struct CreateExamSubjectRequest {
    pub session_id: String,
    pub subject_id: String,
    pub max_score: f64,
    pub class_subject_id: Option<String>,
    pub duration_minutes: Option<i32>,
    pub exam_type: Option<String>,
}

use shared_kernel::query_filter::UniversalFilter;

pub async fn list(
    ctx: &ServiceContext,
    session_id: &str,
    filter: Option<UniversalFilter>,
) -> Result<Vec<exam_subject::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    let mut query = exam_subject::Entity::find()
        .filter(exam_subject::Column::ExamSessionId.eq(session_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "exam_type" => Some(exam_subject::Column::ExamType.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.all(&ctx.db).await?)
}

pub async fn add(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<exam_subject::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let req: CreateExamSubjectRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid payload: {e}")))?;

    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let model = exam_subject::Model {
        id: id.clone(),
        exam_session_id: req.session_id,
        subject_id: req.subject_id,
        class_subject_id: req.class_subject_id,
        max_score: req.max_score,
        passing_score: req.max_score * 0.5,
        duration_minutes: req.duration_minutes.unwrap_or(120),
        exam_type: req.exam_type.unwrap_or_else(|| "written".to_string()),
        is_included: 1,
        weight: 1.0,
        instructions: None,
        instructions_ar: None,
        sort_order: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: exam_subject::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    Ok(created)
}
