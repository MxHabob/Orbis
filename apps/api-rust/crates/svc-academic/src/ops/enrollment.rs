//! Enrollment operations — student enrollment lifecycle.
//!
//! Handles: enroll, withdraw, transfer + duplicate/capacity checks.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use std::collections::HashMap;
use svc_iam::entities::{person, user};

use crate::contracts::events::event_types;
use crate::entities::{enrollment, class};

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct EnrichedEnrollmentResponse {
    #[serde(flatten)]
    pub enrollment: enrollment::Model,
    pub full_name: String,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub sex: String,
    pub birth_date: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub national_id: Option<String>,
}

// ── Queries ─────────────────────────────────────────────────────

/// Helper to enrich enrollment models with demographics from persons and users.
pub async fn enrich_enrollment_models(
    db: &sea_orm::DatabaseConnection,
    enrollments: &[enrollment::Model],
) -> Result<Vec<EnrichedEnrollmentResponse>, AppError> {
    if enrollments.is_empty() {
        return Ok(Vec::new());
    }

    let student_ids: Vec<String> = enrollments.iter().map(|e| e.student_id.clone()).collect();

    // 1. Query persons directly by ID
    let persons_by_id: HashMap<String, person::Model> = person::Entity::find()
        .filter(person::Column::Id.is_in(student_ids.clone()))
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.id.clone(), p))
        .collect();

    // 2. Query users directly by ID (for student_ids pointing to user records)
    let users_by_id: HashMap<String, user::Model> = user::Entity::find()
        .filter(user::Column::Id.is_in(student_ids.clone()))
        .all(db)
        .await?
        .into_iter()
        .map(|u| (u.id.clone(), u))
        .collect();

    // 3. For users found, query their person records if not already fetched
    let mut missing_person_ids: Vec<String> = Vec::new();
    for u in users_by_id.values() {
        if !persons_by_id.contains_key(&u.person_id) {
            missing_person_ids.push(u.person_id.clone());
        }
    }

    let persons_from_users: HashMap<String, person::Model> = if !missing_person_ids.is_empty() {
        person::Entity::find()
            .filter(person::Column::Id.is_in(missing_person_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect()
    } else {
        HashMap::new()
    };

    let mut enriched = Vec::with_capacity(enrollments.len());
    for e in enrollments {
        let mut val = serde_json::to_value(e).unwrap_or_default();

        let person_opt = persons_by_id.get(&e.student_id).or_else(|| {
            users_by_id
                .get(&e.student_id)
                .and_then(|u| persons_by_id.get(&u.person_id).or_else(|| persons_from_users.get(&u.person_id)))
        });

        let user_opt = users_by_id.get(&e.student_id);

        if let Some(obj) = val.as_object_mut() {
            let full_name = person_opt
                .and_then(|p| p.full_name.clone().or_else(|| Some(p.first_name.clone())))
                .or_else(|| user_opt.map(|u| u.full_name.clone()))
                .unwrap_or_default();

            let name_ar = person_opt.and_then(|p| p.name_ar.clone());
            let name_en = person_opt.and_then(|p| p.name_en.clone());
            let sex = person_opt.and_then(|p| p.gender.clone()).unwrap_or_else(|| "male".into());
            let birth_date = person_opt.and_then(|p| p.date_of_birth.clone());
            let phone = person_opt.and_then(|p| p.phone.clone()).or_else(|| user_opt.and_then(|u| u.phone.clone()));
            let email = user_opt.and_then(|u| u.email.clone());
            let address = person_opt.and_then(|p| p.address.clone());
            let national_id = person_opt.and_then(|p| p.national_id.clone());

            obj.insert("full_name".into(), serde_json::Value::String(full_name));
            obj.insert("name_ar".into(), name_ar.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            obj.insert("name_en".into(), name_en.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            obj.insert("sex".into(), serde_json::Value::String(sex));
            obj.insert("birth_date".into(), birth_date.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            obj.insert("phone".into(), phone.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            obj.insert("email".into(), email.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            obj.insert("address".into(), address.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            obj.insert("national_id".into(), national_id.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
        }

        enriched.push(serde_json::from_value(val).map_err(|e| {
            AppError::validation(format!("Invalid enriched enrollment response: {e}"))
        })?);
    }

    Ok(enriched)
}

use shared_kernel::query_filter::UniversalFilter;

/// List enrollments, optionally filtered by class and/or period.
pub async fn list(
    ctx: &ServiceContext,
    filter: Option<UniversalFilter>,
) -> Result<Vec<enrollment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let mut query = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "student_number" => Some(enrollment::Column::StudentNumber.into_expr().into()),
            "status" => Some(enrollment::Column::Status.into_expr().into()),
            "enrollment_date" => Some(enrollment::Column::EnrollmentDate.into_expr().into()),
            "class_id" => Some(enrollment::Column::ClassId.into_expr().into()),
            "period_id" => Some(enrollment::Column::PeriodId.into_expr().into()),
            "enrollment_type" => Some(enrollment::Column::EnrollmentType.into_expr().into()),
            "withdrawal_date" => Some(enrollment::Column::WithdrawalDate.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.order_by_asc(enrollment::Column::StudentNumber).all(&ctx.db).await?)
}

/// List enriched enrollments with demographics.
pub async fn list_enriched(
    ctx: &ServiceContext,
    filter: Option<UniversalFilter>,
) -> Result<Vec<EnrichedEnrollmentResponse>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let mut query = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "student_number" => Some(enrollment::Column::StudentNumber.into_expr().into()),
            "status" => Some(enrollment::Column::Status.into_expr().into()),
            "enrollment_date" => Some(enrollment::Column::EnrollmentDate.into_expr().into()),
            "class_id" => Some(enrollment::Column::ClassId.into_expr().into()),
            "period_id" => Some(enrollment::Column::PeriodId.into_expr().into()),
            "enrollment_type" => Some(enrollment::Column::EnrollmentType.into_expr().into()),
            "withdrawal_date" => Some(enrollment::Column::WithdrawalDate.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    let enrollments = query.order_by_asc(enrollment::Column::StudentNumber).all(&ctx.db).await?;

    enrich_enrollment_models(&ctx.db, &enrollments).await
}

/// Paginated enrollment list — cursor-based for high-volume lists.
pub async fn list_paginated(
    ctx: &ServiceContext,
    request: shared_kernel::pagination::CursorRequest,
) -> Result<shared_kernel::pagination::CursorPage<enrollment::Model>, AppError> {
    use shared_kernel::pagination::CursorPage;
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let limit = request.effective_limit();
    let mut query = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(enrollment::Column::Id);

    if let Some(f) = request.filter.clone() {
        let condition = f.build_condition(|field| match field {
            "student_number" => Some(enrollment::Column::StudentNumber.into_expr().into()),
            "status" => Some(enrollment::Column::Status.into_expr().into()),
            "enrollment_date" => Some(enrollment::Column::EnrollmentDate.into_expr().into()),
            "class_id" => Some(enrollment::Column::ClassId.into_expr().into()),
            "period_id" => Some(enrollment::Column::PeriodId.into_expr().into()),
            "enrollment_type" => Some(enrollment::Column::EnrollmentType.into_expr().into()),
            "withdrawal_date" => Some(enrollment::Column::WithdrawalDate.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    if let Some(ref cursor) = request.cursor {
        query = query.filter(enrollment::Column::Id.gt(cursor));
    }

    let items = query.limit(limit + 1).all(&ctx.db).await?;
    Ok(CursorPage::from_items_with(items, limit, |e| e.id.clone()))
}

/// Paginated enriched enrollment list.
pub async fn list_paginated_enriched(
    ctx: &ServiceContext,
    request: shared_kernel::pagination::CursorRequest,
) -> Result<shared_kernel::pagination::CursorPage<EnrichedEnrollmentResponse>, AppError> {
    use shared_kernel::pagination::CursorPage;
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let limit = request.effective_limit();
    let mut query = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(enrollment::Column::Id);

    if let Some(f) = request.filter.clone() {
        let condition = f.build_condition(|field| match field {
            "student_number" => Some(enrollment::Column::StudentNumber.into_expr().into()),
            "status" => Some(enrollment::Column::Status.into_expr().into()),
            "enrollment_date" => Some(enrollment::Column::EnrollmentDate.into_expr().into()),
            "class_id" => Some(enrollment::Column::ClassId.into_expr().into()),
            "period_id" => Some(enrollment::Column::PeriodId.into_expr().into()),
            "enrollment_type" => Some(enrollment::Column::EnrollmentType.into_expr().into()),
            "withdrawal_date" => Some(enrollment::Column::WithdrawalDate.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    if let Some(ref cursor) = request.cursor {
        query = query.filter(enrollment::Column::Id.gt(cursor));
    }

    let items = query.limit(limit + 1).all(&ctx.db).await?;
    let enriched_items = enrich_enrollment_models(&ctx.db, &items).await?;

    Ok(CursorPage::from_items_with(enriched_items, limit, |val| {
        val.enrollment.id.clone()
    }))
}

/// Get a single enrollment by ID.
pub async fn get(ctx: &ServiceContext, id: &str) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    enrollment::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("enrollment", id))
}

/// Get a single enriched enrollment by ID with demographic details.
pub async fn get_enriched(ctx: &ServiceContext, id: &str) -> Result<EnrichedEnrollmentResponse, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let enrollment = enrollment::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("enrollment", id))?;

    let enriched = enrich_enrollment_models(&ctx.db, &[enrollment]).await?;
    enriched.into_iter().next().ok_or_else(|| AppError::not_found("enrollment", id))
}

// ── Commands ────────────────────────────────────────────────────

/// Enroll a student in a class.
///
/// Business rules enforced:
/// - No duplicate enrollment in the same period
/// - Class capacity check
/// - Mandatory fee assignment (via FeeAssigner trait)
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CreateEnrollmentPayload {
    pub org_id: Option<String>,
    pub period_id: String,
    pub student_id: String,
    pub class_id: String,
    pub student_number: String,
}

pub async fn create(
    ctx: &ServiceContext,
    req: CreateEnrollmentPayload,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let org_id = req.org_id.clone().unwrap_or_else(|| ctx.org_id.clone());

    // ── Period Gate: enrollment window + capacity + policy ──
    let verdict = ctx.period_gate
        .check_enrollment(&ctx.db, &ctx.org_id, &req.period_id, &req.class_id, None)
        .await?;

    match verdict.decision {
        shared_kernel::period_gate::GateDecision::Blocked => {
            return Err(AppError::policy_blocked(
                verdict.reason.unwrap_or_else(|| "التسجيل مغلق".into()),
            ));
        }
        shared_kernel::period_gate::GateDecision::RequiresOverride => {
            // Create as draft instead — caller must send override request
            return create_draft(ctx, req, &org_id, &verdict).await;
        }
        _ => { /* Allowed or Late — proceed */ }
    }

    // Business rule: no duplicate enrollment in the same period
    let existing = enrollment::Entity::find()
        .filter(enrollment::Column::StudentId.eq(&req.student_id))
        .filter(enrollment::Column::PeriodId.eq(&req.period_id))
        .filter(enrollment::Column::Status.ne("withdrawn"))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::conflict("Student is already enrolled in this period"));
    }

    // Determine enrollment_type based on gate verdict
    let enrollment_type = if verdict.decision == shared_kernel::period_gate::GateDecision::Late {
        "late"
    } else {
        "regular"
    };

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let period_id = req.period_id.clone();
    let class_id = req.class_id.clone();

    let model = enrollment::Model {
        id: id.clone(),
        org_id: org_id.clone(),
        period_id: req.period_id,
        student_id: req.student_id,
        class_id: class_id.clone(),
        student_number: req.student_number,
        status: "active".into(),
        enrollment_date: now.clone(),
        enrollment_type: enrollment_type.into(),
        withdrawal_date: None,
        withdrawal_reason: None,
        meta: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: enrollment::ActiveModel = model.into();

    let txn = ctx.db.begin().await
        .map_err(|e| AppError::internal(format!("Failed to start enrollment transaction: {e}")))?;

    let created = active.insert(&txn).await?;

    infra_persistence::fee_assignment::assign_mandatory_fees(
        &txn,
        &org_id,
        &created.id,
        &class_id,
        &period_id,
    )
    .await?;

    txn.commit().await
        .map_err(|e| AppError::internal(format!("Failed to commit enrollment transaction: {e}")))?;

    let id = created.id.clone();

    // Audit
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.create".into(),
        entity_type: "enrollment".into(),
        entity_id: id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    // Domain event
    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_CREATED.into(),
        entity_type: "enrollment".into(),
        entity_id: id,
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(created)
}

/// Withdraw a student from enrollment.
pub async fn withdraw(
    ctx: &ServiceContext,
    enrollment_id: &str,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: enrollment::ActiveModel = existing.into();
    active.status = Set("withdrawn".into());
    active.withdrawal_date = Set(Some(Utc::now().to_rfc3339()));
    active.withdrawal_reason = Set(Some("Not specified".to_string()));
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.withdraw".into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_WITHDRAWN.into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(updated)
}

/// Transfer a student to a different class.
pub async fn transfer(
    ctx: &ServiceContext,
    enrollment_id: &str,
    new_class_id: &str,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentTransfer).await?;

    // Verify new class exists
    class::Entity::find_by_id(new_class_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", new_class_id))?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: enrollment::ActiveModel = existing.into();
    active.class_id = Set(new_class_id.to_string());
    active.enrollment_type = Set("transfer".into());
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.transfer".into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_TRANSFERRED.into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(updated)
}

/// Check if a student is already enrolled (for duplicate detection UI).
pub async fn check_duplicate(
    ctx: &ServiceContext,
    national_id: Option<String>,
    email: Option<String>,
    phone: Option<String>,
) -> Result<Option<enrollment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let mut person_ids = Vec::new();

    if let Some(nid) = national_id {
        if !nid.is_empty() {
            let p = svc_iam::entities::person::Entity::find()
                .filter(svc_iam::entities::person::Column::NationalId.eq(nid))
                .one(&ctx.db)
                .await?;
            if let Some(person) = p {
                person_ids.push(person.id);
            }
        }
    }
    if let Some(em) = email {
        if !em.is_empty() {
            let u = svc_iam::entities::user::Entity::find()
                .filter(svc_iam::entities::user::Column::Email.eq(em))
                .one(&ctx.db)
                .await?;
            if let Some(user) = u {
                person_ids.push(user.person_id);
            }
        }
    }
    if let Some(ph) = phone {
        if !ph.is_empty() {
            let p = svc_iam::entities::person::Entity::find()
                .filter(svc_iam::entities::person::Column::Phone.eq(ph))
                .one(&ctx.db)
                .await?;
            if let Some(person) = p {
                person_ids.push(person.id);
            }
        }
    }

    if person_ids.is_empty() {
        return Ok(None);
    }

    Ok(enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .filter(enrollment::Column::StudentId.is_in(person_ids))
        .filter(enrollment::Column::Status.ne("withdrawn"))
        .one(&ctx.db)
        .await?)
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct PromotePayload {
    pub is_promoted: i32,
    pub promotion_decision: Option<String>,
    pub promotion_notes: Option<String>,
}

pub async fn promote(
    ctx: &ServiceContext,
    enrollment_id: &str,
    req: PromotePayload,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut meta_json: serde_json::Value = existing.meta.as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    let mut active: enrollment::ActiveModel = existing.into();
    meta_json["is_promoted"] = serde_json::json!(req.is_promoted);
    if let Some(dec) = req.promotion_decision {
        meta_json["promotion_decision"] = serde_json::json!(dec);
    }
    if let Some(notes) = req.promotion_notes {
        meta_json["promotion_notes"] = serde_json::json!(notes);
    }
    active.meta = Set(Some(meta_json.to_string()));
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.promote".into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_PROMOTED.into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(updated)
}

// ── Period Gate Integration ─────────────────────────────────────

/// Check enrollment eligibility without performing the enrollment.
/// Returns the GateVerdict for the frontend to display appropriate UI.
pub async fn check_eligibility(
    ctx: &ServiceContext,
    period_id: &str,
    class_id: &str,
) -> Result<shared_kernel::period_gate::GateVerdict, AppError> {
    ctx.period_gate
        .check_enrollment(&ctx.db, &ctx.org_id, period_id, class_id, None)
        .await
}

/// Create a draft enrollment (used when PeriodGate returns RequiresOverride).
/// The draft is not active — it must be confirmed after override approval.
async fn create_draft(
    ctx: &ServiceContext,
    req: CreateEnrollmentPayload,
    org_id: &str,
    verdict: &shared_kernel::period_gate::GateVerdict,
) -> Result<enrollment::Model, AppError> {
    // Still check for duplicates
    let existing = enrollment::Entity::find()
        .filter(enrollment::Column::StudentId.eq(&req.student_id))
        .filter(enrollment::Column::PeriodId.eq(&req.period_id))
        .filter(enrollment::Column::Status.is_in(["active", "draft"]))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::conflict("Student already has an active or draft enrollment"));
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = enrollment::Model {
        id: id.clone(),
        org_id: org_id.to_string(),
        period_id: req.period_id,
        student_id: req.student_id,
        class_id: req.class_id,
        student_number: req.student_number,
        status: "draft".into(),
        enrollment_date: now.clone(),
        enrollment_type: verdict.override_type.clone().unwrap_or_else(|| "override".into()),
        withdrawal_date: None,
        withdrawal_reason: None,
        meta: Some(serde_json::json!({
            "gate_reason": verdict.reason,
            "override_type": verdict.override_type,
        }).to_string()),
        created_at: now.clone(),
        updated_at: now,
    };

    let active: enrollment::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.create_draft".into(),
        entity_type: "enrollment".into(),
        entity_id: id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: "enrollment.draft_created".into(),
        entity_type: "enrollment".into(),
        entity_id: id,
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(created)
}

/// Confirm a draft enrollment — converts status from "draft" to "active".
/// Called after an override request is approved.
pub async fn confirm_draft(
    ctx: &ServiceContext,
    enrollment_id: &str,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let existing = get(ctx, enrollment_id).await?;

    if existing.status != "draft" {
        return Err(AppError::validation(format!(
            "Enrollment '{}' is not a draft (status: {})", enrollment_id, existing.status
        )));
    }

    let old_data = serde_json::to_value(&existing).ok();

    let txn = ctx.db.begin().await
        .map_err(|e| AppError::internal(format!("Failed to start confirm transaction: {e}")))?;

    let mut active: enrollment::ActiveModel = existing.into();
    active.status = Set("active".into());
    active.updated_at = Set(Utc::now().to_rfc3339());
    let confirmed = active.update(&txn).await?;

    infra_persistence::fee_assignment::assign_mandatory_fees(
        &txn,
        &ctx.org_id,
        enrollment_id,
        &confirmed.class_id,
        &confirmed.period_id,
    )
    .await?;

    txn.commit().await
        .map_err(|e| AppError::internal(format!("Failed to commit confirm transaction: {e}")))?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.confirm_draft".into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&confirmed).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_CREATED.into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&confirmed).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(confirmed)
}
