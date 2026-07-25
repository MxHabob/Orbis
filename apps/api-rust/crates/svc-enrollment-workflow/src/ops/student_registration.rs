//! Student Registration — multi-step operation using ProcessManager.
//!
//! Steps:
//!   1. Validate (settings-driven required fields)
//!   2. Check duplicates (email, national_id)
//!   3. Create student user (IAM)
//!   4. Generate student number (settings-driven format)
//!   5. Create enrollment
//!   6. Link guardian (if provided)
//!   7. Auto-assign mandatory fees (via FeeAssigner)
//!
//! If any step fails, ProcessManager compensates all completed steps.

use sea_orm::{ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter, PaginatorTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use tracing::info;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::process::ProcessState;
use shared_kernel::traits::{AuditEntry, Permission};

use svc_iam::entities::user;
use svc_academic::entities::{enrollment, student_guardian, class};

/// Student registration request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterStudentRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub nationality: Option<String>,
    pub national_id: Option<String>,
    pub address: Option<String>,
    pub class_id: Option<String>,
    pub guardian_name: Option<String>,
    pub guardian_phone: Option<String>,
    pub guardian_relationship: Option<String>,
}

/// Result of a successful registration.
#[derive(Debug, Clone, Serialize)]
pub struct RegistrationResult {
    pub user_id: String,
    pub enrollment_id: String,
    pub student_number: String,
    pub fees_assigned: usize,
    pub guardian_linked: bool,
}

/// Register a student using ProcessManager for compensating transactions.
pub async fn register_student(
    ctx: &ServiceContext,
    req: RegisterStudentRequest,
) -> Result<RegistrationResult, AppError> {
    // ── Permission Check ─────────────────────────────────────────
    ctx.permissions
        .check(&ctx.actor, Permission::EnrollmentWrite)
        .await?;

    let tx = ctx.db.begin().await.map_err(|e| AppError::internal(format!("Failed to begin transaction: {}", e)))?;

    // ── Initialize Process ───────────────────────────────────────
    let mut process = ProcessState::start(
        &ctx.db,
        "student_registration",
        &ctx.org_id,
        &[
            "validate",
            "check_duplicates",
            "create_user",
            "generate_number",
            "create_enrollment",
            "link_guardian",
            "assign_fees",
        ],
    ).await?;

    // ── Step 1: Validate ─────────────────────────────────────────
    process.begin_step(&ctx.db, "validate").await?;
    let settings = ctx.settings.get_registration_settings(&ctx.org_id).await?;
    for field in &settings.required_fields {
        match field.as_str() {
            "email" => {
                if req.email.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                    process.fail_step(&ctx.db, "validate", "Email is required by organization policy").await?;
                    return Err(AppError::validation("Email is required by organization policy"));
                }
            }
            "phone" => {
                if req.phone.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                    process.fail_step(&ctx.db, "validate", "Phone is required by organization policy").await?;
                    return Err(AppError::validation("Phone is required by organization policy"));
                }
            }
            "national_id" => {
                if req.national_id.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                    process.fail_step(&ctx.db, "validate", "National ID is required by organization policy").await?;
                    return Err(AppError::validation("National ID is required by organization policy"));
                }
            }
            _ => {}
        }
    }
    process.complete_step(&ctx.db, "validate", serde_json::json!({"fields_checked": settings.required_fields.len()})).await?;

    // ── Step 2: Check Duplicates ─────────────────────────────────
    process.begin_step(&ctx.db, "check_duplicates").await?;
    if let Some(ref email) = req.email {
        if !email.trim().is_empty() {
            let exists = user::Entity::find()
                .filter(user::Column::OrgId.eq(&ctx.org_id))
                .filter(user::Column::Email.eq(email))
                .one(&tx)
                .await?
                .is_some();
            if exists {
                process.fail_step(&ctx.db, "check_duplicates", "Email already registered").await?;
                return Err(AppError::conflict(format!("Email '{}' is already registered", email)));
            }
        }
    }
    if let Some(ref nid) = req.national_id {
        if !nid.trim().is_empty() {
            let exists = user::Entity::find()
                .filter(user::Column::OrgId.eq(&ctx.org_id))
                .filter(user::Column::NationalId.eq(nid))
                .one(&tx)
                .await?
                .is_some();
            if exists {
                process.fail_step(&ctx.db, "check_duplicates", "National ID already registered").await?;
                return Err(AppError::conflict(format!("National ID '{}' is already registered", nid)));
            }
        }
    }
    process.complete_step(&ctx.db, "check_duplicates", serde_json::json!({"passed": true})).await?;

    // ── Step 3: Create Student User ──────────────────────────────
    process.begin_step(&ctx.db, "create_user").await?;
    let student_user_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let cost = if cfg!(debug_assertions) { 4 } else { 10 };
    let password_hash = bcrypt::hash(req.password.as_deref().unwrap_or("123456"), cost)
        .map_err(|e| AppError::internal(format!("Bcrypt hashing failed: {}", e)))?;

    let new_user = user::ActiveModel {
        id: sea_orm::ActiveValue::Set(student_user_id.clone()),
        org_id: sea_orm::ActiveValue::Set(ctx.org_id.clone()),
        email: sea_orm::ActiveValue::Set(req.email.clone()),
        phone: sea_orm::ActiveValue::Set(req.phone.clone()),
        password_hash: sea_orm::ActiveValue::Set(Some(password_hash)),
        full_name: sea_orm::ActiveValue::Set(req.name.clone()),
        name_ar: sea_orm::ActiveValue::Set(Some(req.name.clone())),
        sex: sea_orm::ActiveValue::Set(req.gender.clone()),
        date_of_birth: sea_orm::ActiveValue::Set(req.birth_date.clone()),
        nationality: sea_orm::ActiveValue::Set(req.nationality.clone()),
        national_id: sea_orm::ActiveValue::Set(req.national_id.clone()),
        address: sea_orm::ActiveValue::Set(req.address.clone()),
        primary_role: sea_orm::ActiveValue::Set("student".to_string()),
        status: sea_orm::ActiveValue::Set("active".to_string()),
        created_at: sea_orm::ActiveValue::Set(now.clone()),
        updated_at: sea_orm::ActiveValue::Set(now.clone()),
        created_by: sea_orm::ActiveValue::Set(Some(ctx.actor.as_str().to_string())),
        ..Default::default()
    };

    match new_user.insert(&tx).await {
        Ok(_) => {
            process.complete_step(&ctx.db, "create_user", serde_json::json!({"user_id": &student_user_id})).await?;
        }
        Err(e) => {
            process.fail_step(&ctx.db, "create_user", &e.to_string()).await?;
            tx.rollback().await.map_err(|e| AppError::internal(format!("Rollback failed: {}", e)))?;
            return Err(AppError::internal(format!("Failed to create student user: {}", e)));
        }
    }

    // ── Step 4: Generate Student Number ──────────────────────────
    process.begin_step(&ctx.db, "generate_number").await?;
    let sequence = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .count(&tx)
        .await? + 1;
    let prefix = &settings.student_number_prefix;
    let digits = settings.student_number_digits;
    let student_number = format!("{}{:0width$}", prefix, sequence, width = digits);
    process.complete_step(&ctx.db, "generate_number", serde_json::json!({"number": &student_number})).await?;

    // ── Step 5: Create Enrollment ────────────────────────────────
    process.begin_step(&ctx.db, "create_enrollment").await?;
    let class_id = req.class_id.clone().unwrap_or_default();
    let enrollment_id = uuid::Uuid::new_v4().to_string();

    let class_period_id = if !class_id.is_empty() {
        class::Entity::find_by_id(class_id.clone())
            .one(&tx)
            .await?
            .map(|c| c.period_id)
            .unwrap_or_default()
    } else {
        String::new()
    };

    let new_enrollment = enrollment::ActiveModel {
        id: sea_orm::ActiveValue::Set(enrollment_id.clone()),
        org_id: sea_orm::ActiveValue::Set(ctx.org_id.clone()),
        period_id: sea_orm::ActiveValue::Set(class_period_id.clone()),
        student_id: sea_orm::ActiveValue::Set(student_user_id.clone()),
        class_id: sea_orm::ActiveValue::Set(class_id.clone()),
        student_number: sea_orm::ActiveValue::Set(student_number.clone()),
        status: sea_orm::ActiveValue::Set("active".to_string()),
        enrollment_date: sea_orm::ActiveValue::Set(now.clone()),
        enrollment_type: sea_orm::ActiveValue::Set("standard".to_string()),
        created_at: sea_orm::ActiveValue::Set(now.clone()),
        updated_at: sea_orm::ActiveValue::Set(now.clone()),
        ..Default::default()
    };

    match new_enrollment.insert(&tx).await {
        Ok(_) => {
            process.complete_step(&ctx.db, "create_enrollment", serde_json::json!({"enrollment_id": &enrollment_id})).await?;
        }
        Err(e) => {
            process.fail_step(&ctx.db, "create_enrollment", &e.to_string()).await?;
            tx.rollback().await.map_err(|e| AppError::internal(format!("Rollback failed: {}", e)))?;
            return Err(AppError::internal(format!("Failed to create enrollment: {}", e)));
        }
    }

    // ── Step 6: Link Guardian ────────────────────────────────────
    process.begin_step(&ctx.db, "link_guardian").await?;
    let guardian_linked = if let (Some(ref g_name), Some(ref g_phone)) = (&req.guardian_name, &req.guardian_phone) {
        if !g_name.trim().is_empty() && !g_phone.trim().is_empty() {
            let guardian_id = uuid::Uuid::new_v4().to_string();

            // Create guardian user
            let g_user = user::ActiveModel {
                id: sea_orm::ActiveValue::Set(guardian_id.clone()),
                org_id: sea_orm::ActiveValue::Set(ctx.org_id.clone()),
                phone: sea_orm::ActiveValue::Set(Some(g_phone.clone())),
                full_name: sea_orm::ActiveValue::Set(g_name.clone()),
                name_ar: sea_orm::ActiveValue::Set(Some(g_name.clone())),
                primary_role: sea_orm::ActiveValue::Set("guardian".to_string()),
                status: sea_orm::ActiveValue::Set("active".to_string()),
                created_at: sea_orm::ActiveValue::Set(now.clone()),
                updated_at: sea_orm::ActiveValue::Set(now.clone()),
                ..Default::default()
            };
            g_user.insert(&tx).await?;

            // Create link
            let link = student_guardian::ActiveModel {
                id: sea_orm::ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
                student_id: sea_orm::ActiveValue::Set(student_user_id.clone()),
                guardian_id: sea_orm::ActiveValue::Set(guardian_id.clone()),
                relationship: sea_orm::ActiveValue::Set(
                    req.guardian_relationship.clone().unwrap_or_else(|| "father".to_string())
                ),
                is_primary: sea_orm::ActiveValue::Set(1),
                is_emergency_contact: sea_orm::ActiveValue::Set(1),
                is_authorized_pickup: sea_orm::ActiveValue::Set(1),
                can_view_grades: sea_orm::ActiveValue::Set(1),
                can_view_attendance: sea_orm::ActiveValue::Set(1),
                can_view_behavior: sea_orm::ActiveValue::Set(1),
                can_view_finance: sea_orm::ActiveValue::Set(1),
                can_view_schedule: sea_orm::ActiveValue::Set(1),
                can_receive_notifications: sea_orm::ActiveValue::Set(1),
                is_payer: sea_orm::ActiveValue::Set(1),
                created_at: sea_orm::ActiveValue::Set(now.clone()),
                updated_at: sea_orm::ActiveValue::Set(now.clone()),
                ..Default::default()
            };
            link.insert(&tx).await?;

            true
        } else {
            false
        }
    } else {
        false
    };
    process.complete_step(&ctx.db, "link_guardian", serde_json::json!({"linked": guardian_linked})).await?;

    // ── Step 7: Assign Mandatory Fees ────────────────────────────
    process.begin_step(&ctx.db, "assign_fees").await?;
    let fees_assigned = if settings.auto_assign_fees {
        match ctx.fee_assigner.assign_mandatory_fees(
            &ctx.org_id,
            &enrollment_id,
            &class_id,
            &class_period_id,
        ).await {
            Ok(ids) => ids.len(),
            Err(e) => {
                tracing::warn!(error = %e, "Fee assignment failed — continuing without fees");
                0
            }
        }
    } else {
        0
    };
    process.complete_step(&ctx.db, "assign_fees", serde_json::json!({"count": fees_assigned})).await?;

    // ── Complete Process ─────────────────────────────────────────
    process.complete(&ctx.db).await?;

    // ── Event Sourcing ───────────────────────────────────────────
    let payload = serde_json::json!({
        "user_id": &student_user_id,
        "enrollment_id": &enrollment_id,
        "student_number": &student_number,
        "class_id": &class_id,
        "name": &req.name,
        "guardian_linked": guardian_linked,
        "fees_assigned": fees_assigned,
    });

    ctx.event_store.append(
        "enrollment",
        &enrollment_id,
        "enrollment.created",
        &payload,
        &ctx.actor,
        ctx.correlation_id.as_deref(),
    ).await?;

    // ── Event Bus (for reactive pipeline + plugins) ──────────────
    ctx.event_bus.publish(shared_kernel::event_bus::EventEnvelope {
        event_type: "enrollment.created".to_string(),
        entity_type: "enrollment".to_string(),
        entity_id: enrollment_id.clone(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().to_string(),
        actor_type: ctx.actor.actor_type().to_string(),
        correlation_id: ctx.correlation_id.clone(),
        payload: payload.clone(),
        chain_depth: 0,
    });

    // ── Audit Log ────────────────────────────────────────────────
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "student.registered".to_string(),
        entity_type: "enrollment".to_string(),
        entity_id: enrollment_id.clone(),
        old_data: None,
        new_data: Some(payload),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    info!(
        org_id = %ctx.org_id,
        enrollment_id = %enrollment_id,
        student_number = %student_number,
        "Student registered successfully"
    );

    tx.commit().await.map_err(|e| AppError::internal(format!("Commit failed: {}", e)))?;

    Ok(RegistrationResult {
        user_id: student_user_id,
        enrollment_id,
        student_number,
        fees_assigned,
        guardian_linked,
    })
}
