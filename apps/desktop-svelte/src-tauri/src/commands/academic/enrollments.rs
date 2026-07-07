use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Types ──────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct EnrollmentParams {
    pub class_id: Option<String>,
    pub status: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateEnrollmentRequest {
    pub student_id: String,
    pub class_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WithdrawRequest {
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TransferRequest {
    pub new_class_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PromoteRequest {
    pub student_ids: Vec<String>,
    pub target_class_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PromotionResult {
    pub promoted_ids: Vec<String>,
    pub failed_ids: Vec<String>,
}

// ── Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_enrollments(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: EnrollmentParams,
) -> CmdResult<Vec<serde_json::Value>> {
    let db = state.active_db().await;
    let mut query = domain_academic::entities::enrollment::Entity::find()
        .filter(domain_academic::entities::enrollment::Column::OrgId.eq(org_id));
        
    if let Some(class_id) = params.class_id {
        query = query.filter(domain_academic::entities::enrollment::Column::ClassId.eq(class_id));
    }
    if let Some(status) = params.status {
        query = query.filter(domain_academic::entities::enrollment::Column::Status.eq(status));
    }
    
    let enrollments = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    
    let mut result = Vec::new();
    for e in enrollments {
        let user = domain_iam::entities::user::Entity::find_by_id(&e.student_id)
            .one(&db)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;
            
        let class = domain_academic::entities::class::Entity::find_by_id(&e.class_id)
            .one(&db)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;
            
        let val = serde_json::json!({
            "id": e.id,
            "org_id": e.org_id,
            "student_id": e.student_id,
            "class_id": e.class_id,
            "status": e.status,
            "enrolled_at": e.enrollment_date,
            "withdrawn_at": e.withdrawal_date,
            "withdrawal_reason": e.withdrawal_reason,
            "student_name": user.as_ref().map(|u| u.full_name.clone()),
            "student_email": user.as_ref().map(|u| u.email.clone()),
            "class_name": class.as_ref().map(|c| c.name.clone()),
            "class_name_ar": class.as_ref().map(|c| c.name_ar.clone()),
        });
        result.push(val);
    }
    Ok(result)
}

#[tauri::command]
pub async fn get_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let e = domain_academic::entities::enrollment::Entity::find_by_id(enrollment_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Enrollment not found".to_string()))?;
        
    let user = domain_iam::entities::user::Entity::find_by_id(&e.student_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    let class = domain_academic::entities::class::Entity::find_by_id(&e.class_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    let val = serde_json::json!({
        "id": e.id,
        "org_id": e.org_id,
        "student_id": e.student_id,
        "class_id": e.class_id,
        "status": e.status,
        "enrolled_at": e.enrollment_date,
        "withdrawn_at": e.withdrawal_date,
        "withdrawal_reason": e.withdrawal_reason,
        "student": user,
        "class": class,
    });
    
    Ok(val)
}

#[tauri::command]
pub async fn create_enrollment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateEnrollmentRequest,
) -> CmdResult<domain_academic::entities::enrollment::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    
    let existing = domain_academic::entities::enrollment::Entity::find()
        .filter(domain_academic::entities::enrollment::Column::OrgId.eq(&org_id))
        .filter(domain_academic::entities::enrollment::Column::StudentId.eq(&request.student_id))
        .filter(domain_academic::entities::enrollment::Column::Status.eq("active"))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    if existing.is_some() {
        return Err(CommandError::from("Student is already active in another class".to_string()));
    }
    
    let model = domain_academic::entities::enrollment::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id,
        cycle_id: "".to_string(),
        student_id: request.student_id,
        class_id: request.class_id,
        student_number: format!("ST{}", uuid::Uuid::new_v4().to_string()[..8].to_uppercase()),
        status: "active".to_string(),
        enrollment_date: now.clone(),
        enrollment_type: "standard".to_string(),
        withdrawal_date: None,
        withdrawal_reason: None,
        final_grade: None,
        final_score: None,
        final_rank: None,
        is_promoted: None,
        promotion_decision: None,
        promotion_notes: None,
        meta: None,
        created_at: now.clone(),
        updated_at: now,
    };
    
    let active: domain_academic::entities::enrollment::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn withdraw_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
    request: WithdrawRequest,
) -> CmdResult<domain_academic::entities::enrollment::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let e = domain_academic::entities::enrollment::Entity::find_by_id(enrollment_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Enrollment not found".to_string()))?;
        
    let now = chrono::Utc::now().to_rfc3339();
    let mut active: domain_academic::entities::enrollment::ActiveModel = e.into();
    active.status = Set("withdrawn".to_string());
    active.withdrawal_date = Set(Some(now.clone()));
    active.withdrawal_reason = Set(Some(request.reason));
    active.updated_at = Set(now);
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn transfer_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
    request: TransferRequest,
) -> CmdResult<domain_academic::entities::enrollment::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let e = domain_academic::entities::enrollment::Entity::find_by_id(enrollment_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Enrollment not found".to_string()))?;
        
    let mut active: domain_academic::entities::enrollment::ActiveModel = e.into();
    active.class_id = Set(request.new_class_id);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn promote_students(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: PromoteRequest,
) -> CmdResult<PromotionResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let mut promoted_ids = Vec::new();
    let mut failed_ids = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();
    
    for s_id in request.student_ids {
        let enrollment = domain_academic::entities::enrollment::Entity::find()
            .filter(domain_academic::entities::enrollment::Column::OrgId.eq(&org_id))
            .filter(domain_academic::entities::enrollment::Column::StudentId.eq(&s_id))
            .filter(domain_academic::entities::enrollment::Column::Status.eq("active"))
            .one(&db)
            .await;
            
        match enrollment {
            Ok(Some(e)) => {
                let mut active: domain_academic::entities::enrollment::ActiveModel = e.into();
                active.class_id = Set(request.target_class_id.clone());
                active.updated_at = Set(now.clone());
                if active.update(&db).await.is_ok() {
                    promoted_ids.push(s_id);
                } else {
                    failed_ids.push(s_id);
                }
            }
            _ => {
                failed_ids.push(s_id);
            }
        }
    }
    
    Ok(PromotionResult { promoted_ids, failed_ids })
}

#[tauri::command]
pub async fn transition_enrollment_status(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    enrollment_id: String,
    target_status: String,
    withdrawal_reason: Option<String>,
) -> CmdResult<domain_academic::entities::enrollment::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let e = domain_academic::entities::enrollment::Entity::find_by_id(enrollment_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Enrollment not found".to_string()))?;
        
    let now = chrono::Utc::now().to_rfc3339();
    let mut active: domain_academic::entities::enrollment::ActiveModel = e.into();
    active.status = Set(target_status.clone());
    if target_status == "withdrawn" {
        active.withdrawal_date = Set(Some(now.clone()));
        active.withdrawal_reason = Set(withdrawal_reason);
    }
    active.updated_at = Set(now);
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn check_student_duplicate(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    email: Option<String>,
    phone: Option<String>,
    national_id: Option<String>,
) -> CmdResult<Vec<domain_iam::entities::user::Model>> {
    let db = state.active_db().await;
    let mut query = domain_iam::entities::user::Entity::find()
        .filter(domain_iam::entities::user::Column::OrgId.eq(org_id))
        .filter(domain_iam::entities::user::Column::PrimaryRole.eq("student"));
        
    let mut has_filter = false;
    if let Some(e) = email {
        query = query.filter(domain_iam::entities::user::Column::Email.eq(e));
        has_filter = true;
    }
    if let Some(p) = phone {
        query = query.filter(domain_iam::entities::user::Column::Phone.eq(p));
        has_filter = true;
    }
    if let Some(n) = national_id {
        query = query.filter(domain_iam::entities::user::Column::NationalId.eq(n));
        has_filter = true;
    }
    
    if !has_filter {
        return Ok(Vec::new());
    }
    
    let list = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}
