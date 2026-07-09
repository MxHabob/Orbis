use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Types ──────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BulkStudentRow {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub national_id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub nationality: Option<String>,
    pub address: Option<String>,
    pub class_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BulkGradeRow {
    pub student_id: Option<String>,
    pub student_national_id: Option<String>,
    pub subject_id: Option<String>,
    pub subject_code: Option<String>,
    pub subject_name: Option<String>,
    pub assessment_id: Option<String>,
    pub assessment_name: Option<String>,
    pub grade: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BulkImportResult {
    pub total_rows: usize,
    pub success_count: usize,
    pub failure_count: usize,
    pub errors: Vec<String>,
}

// ── Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn bulk_create_students(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    created_by: String,
    rows: Vec<BulkStudentRow>,
) -> CmdResult<BulkImportResult> {
    let db = state.active_db().await;
    let total_rows = rows.len();

    let settings_service = Arc::new(domain_lifecycle::services::settings::OrganizationSettingsService::new(Arc::new(
        domain_lifecycle::repository::LifecycleQueryRepo::new(db.clone()),
    )));
    let registration_service = Arc::new(crate::features::student_registration::StudentRegistrationService::new(settings_service));

    use sea_orm::TransactionTrait;
    db.transaction::<_, BulkImportResult, sea_orm::DbErr>(move |txn| {
        let org_id = org_id.clone();
        let created_by = created_by.clone();
        let rows = rows.clone();
        let registration_service = registration_service.clone();

        Box::pin(async move {
            let mut success_count = 0;

            for (idx, row) in rows.into_iter().enumerate() {
                let req = crate::features::student_registration::RegisterStudentRequest {
                    name: row.name,
                    email: row.email,
                    phone: row.phone,
                    password: row.password,
                    gender: row.gender,
                    birth_date: row.birth_date,
                    nationality: row.nationality,
                    national_id: row.national_id,
                    address: row.address,
                    class_id: row.class_id,
                    guardian_name: None,
                    guardian_phone: None,
                    guardian_relationship: None,
                };

                registration_service
                    .register_student(txn, &org_id, req, Some(&created_by))
                    .await
                    .map_err(|e| sea_orm::DbErr::Custom(format!("الصف {}: {}", idx + 1, e)))?;

                success_count += 1;
            }

            Ok(BulkImportResult {
                total_rows,
                success_count,
                failure_count: 0,
                errors: Vec::new(),
            })
        })
    }).await.map_err(|e| CommandError::new(e.to_string()))
}


#[tauri::command]
pub async fn bulk_create_teachers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    created_by: String,
    rows: Vec<BulkStudentRow>,
) -> CmdResult<BulkImportResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut errors = Vec::new();
    let total_rows = rows.len();
    let now = chrono::Utc::now().to_rfc3339();

    for (idx, row) in rows.into_iter().enumerate() {
        let id = uuid::Uuid::new_v4().to_string();
        let cost = if cfg!(debug_assertions) { 4 } else { 10 };
        let password_hash = bcrypt::hash(row.password.as_deref().unwrap_or("123456"), cost)
            .unwrap_or_default();

        let user = domain_iam::entities::user::Model {
            id,
            org_id: org_id.clone(),
            email: row.email,
            email_verified: 1,
            email_verified_at: Some(now.clone()),
            phone: row.phone,
            phone_verified: 0,
            phone_verified_at: None,
            password_hash: Some(password_hash),
            password_changed_at: None,
            password_expires_at: None,
            mfa_enabled: 0,
            mfa_secret: None,
            mfa_backup_codes: None,
            full_name: row.name.clone(),
            name_ar: Some(row.name.clone()),
            name_en: None,
            avatar: None,
            sex: row.gender,
            date_of_birth: row.birth_date,
            place_of_birth: None,
            nationality: row.nationality,
            national_id: row.national_id,
            passport_number: None,
            address: row.address,
            city: None,
            country: None,
            postal_code: None,
            emergency_contact: None,
            blood_type: None,
            allergies: None,
            medical_conditions: None,
            medical_notes: None,
            social_links: None,
            bio: None,
            primary_role: "teacher".to_string(),
            status: "active".to_string(),
            last_login_at: None,
            last_login_ip: None,
            login_count: Some(0),
            failed_logins: Some(0),
            locked_until: None,
            prefs: "{}".to_string(),
            meta: "{}".to_string(),
            is_deleted: 0,
            deleted_at: None,
            deleted_by: None,
            created_at: now.clone(),
            updated_at: now.clone(),
            created_by: Some(created_by.clone()),
        };

        let active: domain_iam::entities::user::ActiveModel = user.into();
        match active.insert(&db).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                failure_count += 1;
                errors.push(format!("Row {}: Failed to insert teacher user: {}", idx + 1, e));
            }
        }
    }

    Ok(BulkImportResult {
        total_rows,
        success_count,
        failure_count,
        errors,
    })
}

#[tauri::command]
pub async fn bulk_create_grades(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    cycle_id: Option<String>,
    rows: Vec<BulkGradeRow>,
) -> CmdResult<BulkImportResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut errors = Vec::new();
    let total_rows = rows.len();
    let now = chrono::Utc::now().to_rfc3339();
    
    let resolved_cycle_id = cycle_id.unwrap_or_else(|| "default_cycle".to_string());

    for (idx, row) in rows.into_iter().enumerate() {
        let enrollment_opt = if let Some(student_id) = &row.student_id {
            domain_academic::entities::enrollment::Entity::find()
                .filter(domain_academic::entities::enrollment::Column::OrgId.eq(&org_id))
                .filter(domain_academic::entities::enrollment::Column::StudentId.eq(student_id))
                .filter(domain_academic::entities::enrollment::Column::Status.eq("active"))
                .one(&db)
                .await
                .ok()
                .flatten()
        } else if let Some(nat_id) = &row.student_national_id {
            let user = domain_iam::entities::user::Entity::find()
                .filter(domain_iam::entities::user::Column::OrgId.eq(&org_id))
                .filter(domain_iam::entities::user::Column::NationalId.eq(nat_id))
                .one(&db)
                .await
                .ok()
                .flatten();
                
            if let Some(u) = user {
                domain_academic::entities::enrollment::Entity::find()
                    .filter(domain_academic::entities::enrollment::Column::OrgId.eq(&org_id))
                    .filter(domain_academic::entities::enrollment::Column::StudentId.eq(u.id))
                    .filter(domain_academic::entities::enrollment::Column::Status.eq("active"))
                    .one(&db)
                    .await
                    .ok()
                    .flatten()
            } else {
                None
            }
        } else {
            None
        };
        
        let enrollment = match enrollment_opt {
            Some(e) => e,
            None => {
                failure_count += 1;
                errors.push(format!("Row {}: Active enrollment not found for student", idx + 1));
                continue;
            }
        };

        let id = uuid::Uuid::new_v4().to_string();
        let grade_model = domain_assessment::entities::grade::Model {
            id,
            org_id: org_id.clone(),
            cycle_id: resolved_cycle_id.clone(),
            enrollment_id: enrollment.id,
            assessment_id: row.assessment_id,
            class_subject_id: None,
            subject_name: row.subject_name.unwrap_or_else(|| "General".to_string()),
            subject_name_ar: None,
            score: row.grade,
            max_score: 100.0,
            weight: 100.0,
            grade_letter: None,
            grade_points: None,
            exam_type: "quiz".to_string(),
            semester: "first".to_string(),
            notes: None,
            graded_by: graded_by.clone(),
            graded_at: now.clone(),
            is_final: 0,
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        let active: domain_assessment::entities::grade::ActiveModel = grade_model.into();
        match active.insert(&db).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                failure_count += 1;
                errors.push(format!("Row {}: Failed to insert grade: {}", idx + 1, e));
            }
        }
    }

    Ok(BulkImportResult {
        total_rows,
        success_count,
        failure_count,
        errors,
    })
}
