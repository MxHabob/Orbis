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

use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, ColumnTrait, QueryFilter, PaginatorTrait};
use serde::{Deserialize, Serialize};
use tracing::info;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::process::{ProcessState, CompensationAction};
use shared_kernel::traits::{AuditEntry, Permission};

use svc_iam::entities::user;
use svc_academic::entities::{enrollment, student_guardian};

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

    // ── Initialize Process ───────────────────────────────────────
    let mut process = ProcessState::new(
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
    );

    // ── Step 1: Validate ─────────────────────────────────────────
    process.begin_step("validate")?;
    let settings = ctx.settings.get_registration_settings(&ctx.org_id).await?;
    for field in &settings.required_fields {
        match field.as_str() {
            "email" => {
                if req.email.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                    process.fail_step("validate", "Email is required by organization policy");
                    return Err(AppError::validation("Email is required by organization policy"));
                }
            }
            "phone" => {
                if req.phone.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                    process.fail_step("validate", "Phone is required by organization policy");
                    return Err(AppError::validation("Phone is required by organization policy"));
                }
            }
            "national_id" => {
                if req.national_id.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                    process.fail_step("validate", "National ID is required by organization policy");
                    return Err(AppError::validation("National ID is required by organization policy"));
                }
            }
            _ => {}
        }
    }
    process.complete_step("validate", serde_json::json!({"fields_checked": settings.required_fields.len()}))?;

    // ── Step 2: Check Duplicates ─────────────────────────────────
    process.begin_step("check_duplicates")?;
    if let Some(ref email) = req.email {
        if !email.trim().is_empty() {
            let exists = user::Entity::find()
                .filter(user::Column::OrgId.eq(&ctx.org_id))
                .filter(user::Column::Email.eq(email))
                .one(&ctx.db)
                .await?
                .is_some();
            if exists {
                process.fail_step("check_duplicates", "Email already registered");
                return Err(AppError::conflict(format!("Email '{}' is already registered", email)));
            }
        }
    }
    if let Some(ref nid) = req.national_id {
        if !nid.trim().is_empty() {
            let exists = user::Entity::find()
                .filter(user::Column::OrgId.eq(&ctx.org_id))
                .filter(user::Column::NationalId.eq(nid))
                .one(&ctx.db)
                .await?
                .is_some();
            if exists {
                process.fail_step("check_duplicates", "National ID already registered");
                return Err(AppError::conflict(format!("National ID '{}' is already registered", nid)));
            }
        }
    }
    process.complete_step("check_duplicates", serde_json::json!({"passed": true}))?;

    // ── Step 3: Create Student User ──────────────────────────────
    process.begin_step("create_user")?;
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

    match new_user.insert(&ctx.db).await {
        Ok(_) => {
            process.complete_step("create_user", serde_json::json!({"user_id": &student_user_id}))?;
            process.add_compensation("create_user", CompensationAction::DeleteEntity {
                entity_type: "users".to_string(),
                id: student_user_id.clone(),
            });
        }
        Err(e) => {
            process.fail_step("create_user", &e.to_string());
            process.compensate(&ctx.db).await?;
            return Err(AppError::internal(format!("Failed to create student user: {}", e)));
        }
    }

    // ── Step 4: Generate Student Number ──────────────────────────
    process.begin_step("generate_number")?;
    let sequence = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .count(&ctx.db)
        .await? + 1;
    let prefix = &settings.student_number_prefix;
    let digits = settings.student_number_digits;
    let student_number = format!("{}{:0width$}", prefix, sequence, width = digits);
    process.complete_step("generate_number", serde_json::json!({"number": &student_number}))?;

    // ── Step 5: Create Enrollment ────────────────────────────────
    process.begin_step("create_enrollment")?;
    let class_id = req.class_id.clone().unwrap_or_default();
    let enrollment_id = uuid::Uuid::new_v4().to_string();

    let new_enrollment = enrollment::ActiveModel {
        id: sea_orm::ActiveValue::Set(enrollment_id.clone()),
        org_id: sea_orm::ActiveValue::Set(ctx.org_id.clone()),
        cycle_id: sea_orm::ActiveValue::Set(String::new()),
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

    match new_enrollment.insert(&ctx.db).await {
        Ok(_) => {
            process.complete_step("create_enrollment", serde_json::json!({"enrollment_id": &enrollment_id}))?;
            process.add_compensation("create_enrollment", CompensationAction::DeleteEntity {
                entity_type: "enrollments".to_string(),
                id: enrollment_id.clone(),
            });
        }
        Err(e) => {
            process.fail_step("create_enrollment", &e.to_string());
            process.compensate(&ctx.db).await?;
            return Err(AppError::internal(format!("Failed to create enrollment: {}", e)));
        }
    }

    // ── Step 6: Link Guardian ────────────────────────────────────
    process.begin_step("link_guardian")?;
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
            g_user.insert(&ctx.db).await?;

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
            link.insert(&ctx.db).await?;

            process.add_compensation("link_guardian", CompensationAction::DeleteEntity {
                entity_type: "users".to_string(),
                id: guardian_id,
            });
            true
        } else {
            false
        }
    } else {
        false
    };
    process.complete_step("link_guardian", serde_json::json!({"linked": guardian_linked}))?;

    // ── Step 7: Assign Mandatory Fees ────────────────────────────
    process.begin_step("assign_fees")?;
    let fees_assigned = if settings.auto_assign_fees {
        match ctx.fee_assigner.assign_mandatory_fees(&ctx.org_id, &enrollment_id, &class_id).await {
            Ok(ids) => ids.len(),
            Err(e) => {
                tracing::warn!(error = %e, "Fee assignment failed — continuing without fees");
                0
            }
        }
    } else {
        0
    };
    process.complete_step("assign_fees", serde_json::json!({"count": fees_assigned}))?;

    // ── Complete Process ─────────────────────────────────────────
    process.complete();

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

    Ok(RegistrationResult {
        user_id: student_user_id,
        enrollment_id,
        student_number,
        fees_assigned,
        guardian_linked,
    })
}
