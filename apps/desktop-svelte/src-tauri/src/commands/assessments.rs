use tauri::State;
use std::sync::Arc;
use sea_orm::{entity::prelude::*, QueryFilter, ActiveModelTrait, ActiveValue::Set, EntityTrait};
use core_domain::entities::{assessment, class_subject};
use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_assessments(
    state: State<'_, Arc<AppState>>,
    class_id: String,
    subject_id: String,
    term_id: String,
    month: Option<serde_json::Value>,
) -> CmdResult<Vec<assessment::Model>> {
    // Find class_subject
    let class_sub = class_subject::Entity::find()
        .filter(class_subject::Column::ClassId.eq(class_id.clone()))
        .filter(class_subject::Column::SubjectId.eq(subject_id.clone()))
        .one(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let class_subject_id = match class_sub {
        Some(cs) => cs.id,
        None => return Ok(vec![]),
    };

    let mut query = assessment::Entity::find()
        .filter(assessment::Column::ClassSubjectId.eq(class_subject_id))
        .filter(assessment::Column::CycleId.eq(term_id));

    // Handle month filter (can be i32, string, etc.)
    if let Some(m_val) = month {
        let m_opt = match m_val {
            serde_json::Value::Number(num) => num.as_i64().map(|i| i as i32),
            serde_json::Value::String(s) => s.parse::<i32>().ok(),
            _ => None,
        };
        if let Some(m) = m_opt {
            let month_str = format!("-{:02}-", m);
            query = query.filter(
                assessment::Column::AssessmentDate.like(&format!("%{}%", month_str))
                    .or(assessment::Column::DueDate.like(&format!("%{}%", month_str)))
                    .or(assessment::Column::GradeGroupName.eq(m.to_string()))
            );
        }
    }

    query
        .all(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[derive(serde::Deserialize)]
pub struct CreateAssessmentRequest {
    pub class_id: String,
    pub subject_id: String,
    pub term_id: String,
    pub month: Option<serde_json::Value>,
    pub title: String,
    pub title_ar: Option<String>,
    pub max_score: serde_json::Value,
    pub weight: Option<serde_json::Value>,
    pub item_type: String,
}

#[tauri::command]
pub async fn create_assessment(
    state: State<'_, Arc<AppState>>,
    request: CreateAssessmentRequest,
) -> CmdResult<assessment::Model> {
    // Find class_subject
    let class_sub = class_subject::Entity::find()
        .filter(class_subject::Column::ClassId.eq(request.class_id.clone()))
        .filter(class_subject::Column::SubjectId.eq(request.subject_id.clone()))
        .one(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let class_subject_id = match class_sub {
        Some(cs) => cs.id,
        None => {
            let new_id = uuid::Uuid::new_v4().to_string();
            let new_cs = class_subject::ActiveModel {
                id: Set(new_id.clone()),
                class_id: Set(request.class_id.clone()),
                subject_id: Set(request.subject_id.clone()),
                cycle_id: Set(request.term_id.clone()),
                primary_teacher_id: Set("".to_string()),
                is_active: Set(1),
                created_at: Set(chrono::Utc::now().naive_utc().to_string()),
                updated_at: Set(chrono::Utc::now().naive_utc().to_string()),
                ..Default::default()
            };
            new_cs.insert(&state.service().await.repo.db)
                .await
                .map_err(|e| CommandError::new(e.to_string()))?;
            new_id
        }
    };

    let max_score: f64 = match request.max_score {
        serde_json::Value::Number(num) => num.as_f64().unwrap_or(10.0),
        serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(10.0),
        _ => 10.0,
    };
    let weight: f64 = match request.weight {
        Some(serde_json::Value::Number(num)) => num.as_f64().unwrap_or(1.0),
        Some(serde_json::Value::String(s)) => s.parse::<f64>().unwrap_or(1.0),
        _ => 1.0,
    };

    let month_str = match request.month {
        Some(serde_json::Value::Number(num)) => num.to_string(),
        Some(serde_json::Value::String(s)) => s,
        _ => "1".to_string(),
    };

    let now_str = chrono::Utc::now().naive_utc().to_string();

    let new_assessment = assessment::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        org_id: Set("".to_string()),
        cycle_id: Set(request.term_id),
        class_subject_id: Set(class_subject_id),
        title: Set(request.title),
        title_ar: Set(request.title_ar),
        item_type: Set(request.item_type),
        max_score: Set(max_score),
        weight: Set(weight),
        grade_group_name: Set(month_str),
        assessment_date: Set(Some(now_str.clone())),
        is_published: Set(1),
        is_locked: Set(0),
        created_by: Set("".to_string()),
        created_at: Set(now_str.clone()),
        updated_at: Set(now_str),
        ..Default::default()
    };

    new_assessment
        .insert(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[derive(serde::Deserialize)]
pub struct UpdateAssessmentRequest {
    pub title: Option<String>,
    pub title_ar: Option<String>,
    pub max_score: Option<serde_json::Value>,
    pub weight: Option<serde_json::Value>,
    pub item_type: Option<String>,
}

#[tauri::command]
pub async fn update_assessment(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateAssessmentRequest,
) -> CmdResult<assessment::Model> {
    let assess = assessment::Entity::find_by_id(id)
        .one(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Assessment not found"))?;

    let mut active: assessment::ActiveModel = assess.into();

    if let Some(title) = request.title {
        active.title = Set(title);
    }
    if let Some(title_ar) = request.title_ar {
        active.title_ar = Set(Some(title_ar));
    }
    if let Some(max_score_val) = request.max_score {
        let max_score: f64 = match max_score_val {
            serde_json::Value::Number(num) => num.as_f64().unwrap_or(10.0),
            serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(10.0),
            _ => 10.0,
        };
        active.max_score = Set(max_score);
    }
    if let Some(weight_val) = request.weight {
        let weight: f64 = match weight_val {
            serde_json::Value::Number(num) => num.as_f64().unwrap_or(1.0),
            serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(1.0),
            _ => 1.0,
        };
        active.weight = Set(weight);
    }
    if let Some(item_type) = request.item_type {
        active.item_type = Set(item_type);
    }
    active.updated_at = Set(chrono::Utc::now().naive_utc().to_string());

    active
        .update(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_assessment(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    // Delete in SeaORM
    let assess = assessment::Entity::find_by_id(id.clone())
        .one(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Assessment not found"))?;

    assessment::Entity::delete_by_id(assess.id)
        .exec(&state.service().await.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}
