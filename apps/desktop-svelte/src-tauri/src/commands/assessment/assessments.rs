use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set, QueryOrder, QuerySelect};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Structs ────────────────────────────────────────────────

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

#[derive(serde::Deserialize)]
pub struct UpdateAssessmentRequest {
    pub title: Option<String>,
    pub title_ar: Option<String>,
    pub max_score: Option<serde_json::Value>,
    pub weight: Option<serde_json::Value>,
    pub item_type: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BehaviorParams {
    pub limit: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AIBehaviorSubmission {
    pub org_id: String,
    pub cycle_id: String,
    pub enrollment_id: String,
    pub behavior_type: String,
    pub severity: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub description: String,
    pub incident_date: String,
    pub reported_by: String,
    pub ai_confidence: Option<f64>,
    pub camera_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UpdateBehaviorRequest {
    pub severity: Option<String>,
    pub action_taken: Option<String>,
    pub action_details: Option<String>,
    pub status: Option<String>,
    pub points: Option<i32>,
}

// ── Assessment Commands ──────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_assessments(
    state: State<'_, Arc<AppState>>,
    class_id: String,
    subject_id: String,
    term_id: String,
    month: Option<serde_json::Value>,
) -> CmdResult<Vec<domain_assessment::entities::assessment::Model>> {
    let db = state.active_db().await;

    // Find class_subject
    let class_sub = domain_academic::entities::class_subject::Entity::find()
        .filter(domain_academic::entities::class_subject::Column::ClassId.eq(class_id.clone()))
        .filter(domain_academic::entities::class_subject::Column::SubjectId.eq(subject_id.clone()))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let class_subject_id = match class_sub {
        Some(cs) => cs.id,
        None => return Ok(vec![]),
    };

    let mut query = domain_assessment::entities::assessment::Entity::find()
        .filter(domain_assessment::entities::assessment::Column::ClassSubjectId.eq(class_subject_id))
        .filter(domain_assessment::entities::assessment::Column::CycleId.eq(term_id));

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
                domain_assessment::entities::assessment::Column::AssessmentDate.like(&format!("%{}%", month_str))
                    .or(domain_assessment::entities::assessment::Column::DueDate.like(&format!("%{}%", month_str)))
                    .or(domain_assessment::entities::assessment::Column::GradeGroupName.eq(m.to_string()))
            );
        }
    }

    query
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_assessment(
    state: State<'_, Arc<AppState>>,
    request: CreateAssessmentRequest,
) -> CmdResult<domain_assessment::entities::assessment::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;

    // Find class_subject
    let class_sub = domain_academic::entities::class_subject::Entity::find()
        .filter(domain_academic::entities::class_subject::Column::ClassId.eq(request.class_id.clone()))
        .filter(domain_academic::entities::class_subject::Column::SubjectId.eq(request.subject_id.clone()))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let class_subject_id = match class_sub {
        Some(cs) => cs.id,
        None => {
            let new_id = uuid::Uuid::new_v4().to_string();
            let new_cs = domain_academic::entities::class_subject::ActiveModel {
                id: Set(new_id.clone()),
                class_id: Set(request.class_id.clone()),
                subject_id: Set(request.subject_id.clone()),
                cycle_id: Set(request.term_id.clone()),
                primary_teacher_id: Set("".to_string()),
                is_active: Set(1),
                created_at: Set(chrono::Utc::now().to_rfc3339()),
                updated_at: Set(chrono::Utc::now().to_rfc3339()),
                ..Default::default()
            };
            new_cs.insert(&db)
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

    let now_str = chrono::Utc::now().to_rfc3339();

    let new_assessment = domain_assessment::entities::assessment::ActiveModel {
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
        .insert(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_assessment(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateAssessmentRequest,
) -> CmdResult<domain_assessment::entities::assessment::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let assess = domain_assessment::entities::assessment::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Assessment not found".to_string()))?;

    let mut active: domain_assessment::entities::assessment::ActiveModel = assess.into();

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
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());

    active
        .update(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_assessment(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let assess = domain_assessment::entities::assessment::Entity::find_by_id(id.clone())
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Assessment not found".to_string()))?;

    domain_assessment::entities::assessment::Entity::delete_by_id(assess.id)
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

// ── Behavior Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_behavior_records(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: BehaviorParams,
) -> CmdResult<Vec<domain_assessment::entities::behavior_record::Model>> {
    let db = state.active_db().await;
    let mut query = domain_assessment::entities::behavior_record::Entity::find()
        .filter(domain_assessment::entities::behavior_record::Column::OrgId.eq(org_id))
        .order_by_desc(domain_assessment::entities::behavior_record::Column::IncidentDate);
        
    if let Some(limit) = params.limit {
        query = query.limit(limit);
    }
    
    let list = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn submit_ai_behavior(
    state: State<'_, Arc<AppState>>,
    request: AIBehaviorSubmission,
) -> CmdResult<domain_assessment::entities::behavior_record::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    
    let model = domain_assessment::entities::behavior_record::Model {
        id,
        org_id: request.org_id,
        cycle_id: request.cycle_id,
        enrollment_id: request.enrollment_id,
        behavior_type: request.behavior_type,
        severity: request.severity,
        category: None,
        title: request.title,
        title_ar: request.title_ar,
        description: request.description,
        incident_date: request.incident_date,
        incident_time: None,
        location: None,
        points: Some(0),
        action_taken: None,
        action_details: None,
        action_completed: Some(0),
        reported_by: request.reported_by,
        reviewed_by: None,
        reviewed_at: None,
        witnesses: None,
        parent_notified: 0,
        parent_notified_at: None,
        parent_response: None,
        evidence_urls: None,
        status: "pending".to_string(),
        resolution_notes: None,
        resolved_at: None,
        resolved_by: None,
        is_private: 0,
        ai_source: Some("autopilot".to_string()),
        ai_confidence: request.ai_confidence,
        ai_model_version: Some("v1".to_string()),
        camera_id: request.camera_id,
        created_at: now.clone(),
        updated_at: now,
    };
    
    let active: domain_assessment::entities::behavior_record::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_behavior_record(
    state: State<'_, Arc<AppState>>,
    record_id: String,
    reviewed_by: String,
    request: UpdateBehaviorRequest,
) -> CmdResult<domain_assessment::entities::behavior_record::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let record = domain_assessment::entities::behavior_record::Entity::find_by_id(record_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Behavior record not found".to_string()))?;
        
    let mut active: domain_assessment::entities::behavior_record::ActiveModel = record.into();
    if let Some(severity) = request.severity {
        active.severity = Set(severity);
    }
    if let Some(action_taken) = request.action_taken {
        active.action_taken = Set(Some(action_taken));
    }
    if let Some(action_details) = request.action_details {
        active.action_details = Set(Some(action_details));
    }
    if let Some(status) = request.status {
        active.status = Set(status);
    }
    if let Some(points) = request.points {
        active.points = Set(Some(points));
    }
    active.reviewed_by = Set(Some(reviewed_by));
    active.reviewed_at = Set(Some(chrono::Utc::now().to_rfc3339()));
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_behavior_record(
    state: State<'_, Arc<AppState>>,
    record_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let _record = domain_assessment::entities::behavior_record::Entity::find_by_id(&record_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Behavior record not found".to_string()))?;
        
    let _ = domain_assessment::entities::behavior_record::Entity::delete_by_id(record_id)
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}
