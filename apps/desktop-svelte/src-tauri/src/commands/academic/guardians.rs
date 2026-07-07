use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Types ──────────────────────────────────────────

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CreateGuardianLinkRequest {
    pub student_id: String,
    pub guardian_id: String,
    pub relationship: String,
    pub relationship_ar: Option<String>,
    pub is_primary: bool,
    pub is_emergency_contact: bool,
    pub is_authorized_pickup: bool,
    pub can_view_grades: bool,
    pub can_view_attendance: bool,
    pub can_view_behavior: bool,
    pub can_view_finance: bool,
    pub can_view_schedule: bool,
    pub can_receive_notifications: bool,
    pub is_payer: bool,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UpdateGuardianLinkRequest {
    pub relationship: String,
    pub relationship_ar: Option<String>,
    pub is_primary: bool,
    pub is_emergency_contact: bool,
    pub is_authorized_pickup: bool,
    pub can_view_grades: bool,
    pub can_view_attendance: bool,
    pub can_view_behavior: bool,
    pub can_view_finance: bool,
    pub can_view_schedule: bool,
    pub can_receive_notifications: bool,
    pub is_payer: bool,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
}

// ── Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_guardians_by_student(
    state: State<'_, Arc<AppState>>,
    student_id: String,
) -> CmdResult<Vec<domain_academic::entities::student_guardian::Model>> {
    let db = state.active_db().await;
    let list = domain_academic::entities::student_guardian::Entity::find()
        .filter(domain_academic::entities::student_guardian::Column::StudentId.eq(student_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn get_students_by_guardian(
    state: State<'_, Arc<AppState>>,
    guardian_id: String,
) -> CmdResult<Vec<domain_academic::entities::student_guardian::Model>> {
    let db = state.active_db().await;
    let list = domain_academic::entities::student_guardian::Entity::find()
        .filter(domain_academic::entities::student_guardian::Column::GuardianId.eq(guardian_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn create_guardian_link(
    state: State<'_, Arc<AppState>>,
    request: CreateGuardianLinkRequest,
) -> CmdResult<domain_academic::entities::student_guardian::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    
    let model = domain_academic::entities::student_guardian::Model {
        id,
        student_id: request.student_id,
        guardian_id: request.guardian_id,
        relationship: request.relationship,
        relationship_ar: request.relationship_ar,
        is_primary: if request.is_primary { 1 } else { 0 },
        is_emergency_contact: if request.is_emergency_contact { 1 } else { 0 },
        is_authorized_pickup: if request.is_authorized_pickup { 1 } else { 0 },
        can_view_grades: if request.can_view_grades { 1 } else { 0 },
        can_view_attendance: if request.can_view_attendance { 1 } else { 0 },
        can_view_behavior: if request.can_view_behavior { 1 } else { 0 },
        can_view_finance: if request.can_view_finance { 1 } else { 0 },
        can_view_schedule: if request.can_view_schedule { 1 } else { 0 },
        can_receive_notifications: if request.can_receive_notifications { 1 } else { 0 },
        is_payer: if request.is_payer { 1 } else { 0 },
        payment_share_percent: request.payment_share_percent,
        occupation: request.occupation,
        employer: request.employer,
        work_phone: request.work_phone,
        notes: request.notes,
        created_at: now.clone(),
        updated_at: now,
    };
    
    let active: domain_academic::entities::student_guardian::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_guardian_link(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateGuardianLinkRequest,
) -> CmdResult<domain_academic::entities::student_guardian::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let link = domain_academic::entities::student_guardian::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Guardian link not found".to_string()))?;
        
    let mut active: domain_academic::entities::student_guardian::ActiveModel = link.into();
    active.relationship = Set(request.relationship);
    active.relationship_ar = Set(request.relationship_ar);
    active.is_primary = Set(if request.is_primary { 1 } else { 0 });
    active.is_emergency_contact = Set(if request.is_emergency_contact { 1 } else { 0 });
    active.is_authorized_pickup = Set(if request.is_authorized_pickup { 1 } else { 0 });
    active.can_view_grades = Set(if request.can_view_grades { 1 } else { 0 });
    active.can_view_attendance = Set(if request.can_view_attendance { 1 } else { 0 });
    active.can_view_behavior = Set(if request.can_view_behavior { 1 } else { 0 });
    active.can_view_finance = Set(if request.can_view_finance { 1 } else { 0 });
    active.can_view_schedule = Set(if request.can_view_schedule { 1 } else { 0 });
    active.can_receive_notifications = Set(if request.can_receive_notifications { 1 } else { 0 });
    active.is_payer = Set(if request.is_payer { 1 } else { 0 });
    active.payment_share_percent = Set(request.payment_share_percent);
    active.occupation = Set(request.occupation);
    active.employer = Set(request.employer);
    active.work_phone = Set(request.work_phone);
    active.notes = Set(request.notes);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_guardian_link(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let link = domain_academic::entities::student_guardian::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Guardian link not found".to_string()))?;
        
    let _ = domain_academic::entities::student_guardian::Entity::delete_by_id(link.id)
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn search_guardians(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    query: String,
) -> CmdResult<Vec<domain_iam::entities::user::Model>> {
    let db = state.active_db().await;
    let guardians = domain_iam::entities::user::Entity::find()
        .filter(domain_iam::entities::user::Column::OrgId.eq(org_id))
        .filter(domain_iam::entities::user::Column::PrimaryRole.eq("guardian"))
        .filter(
            domain_iam::entities::user::Column::FullName.like(format!("%{}%", query))
                .or(domain_iam::entities::user::Column::Email.like(format!("%{}%", query)))
                .or(domain_iam::entities::user::Column::Phone.like(format!("%{}%", query)))
        )
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(guardians)
}
