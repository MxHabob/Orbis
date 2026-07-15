use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use tracing::info;

use crate::entities::{calculation_policy, grade_scale_item, student_assessment_snapshot};
use crate::ops::calc_engine::{CalcEngine, AstNode, apply_rounding};
use crate::ops::grade_mapper::{GradeMapper, GradeScaleItemData};

pub async fn generate_snapshot(
    ctx: &ServiceContext,
    student_id: &str,
    period_id: &str,
    class_subject_id: &str,
    policy_id: &str,
    scale_id: &str,
    raw_scores: std::collections::HashMap<String, f64>,
) -> Result<serde_json::Value, AppError> {
    let job = crate::ops::snapshot_job::SnapshotJob {
        ctx: ctx.clone(),
        student_id: student_id.to_string(),
        period_id: period_id.to_string(),
        class_subject_id: class_subject_id.to_string(),
        policy_id: policy_id.to_string(),
        scale_id: scale_id.to_string(),
        raw_scores,
    };
    
    ctx.job_queue.try_enqueue(job)
        .map_err(|e| AppError::internal(format!("Failed to enqueue snapshot job: {}", e)))?;
        
    Ok(serde_json::json!({
        "status": "processing",
        "message": "Snapshot generation queued successfully."
    }))
}

pub async fn execute_snapshot_generation(
    ctx: &ServiceContext,
    student_id: &str,
    period_id: &str,
    class_subject_id: &str,
    policy_id: &str,
    scale_id: &str,
    raw_scores: std::collections::HashMap<String, f64>,
) -> Result<student_assessment_snapshot::Model, AppError> {
    
    // 1. Fetch Policy
    let policy = calculation_policy::Entity::find_by_id(policy_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("calculation_policy", policy_id))?;

    // Parse the formula expression into an AST
    let ast: AstNode = serde_json::from_str(&policy.formula_expression).map_err(|e| {
        AppError::validation(format!("Invalid calculation policy formula: {}", e))
    })?;

    // 2. Fetch Grade Scale Items
    let scale_items = grade_scale_item::Entity::find()
        .filter(grade_scale_item::Column::ScaleId.eq(scale_id.to_string()))
        .all(&ctx.db)
        .await?;

    let mapper_items: Vec<GradeScaleItemData> = scale_items.into_iter().map(|i| GradeScaleItemData {
        label: i.grade_label,
        min_score: i.min_score,
        max_score: i.max_score,
        gpa_points: i.gpa_points,
        is_passing: i.is_passing == 1,
    }).collect();

    let mapper = GradeMapper::new(mapper_items);

    // 3. Evaluate via CalcEngine
    let mut engine = CalcEngine::new();
    for (key, val) in raw_scores {
        engine.set_variable(&key, val);
    }

    let raw_total = engine.evaluate(&ast)?;
    let final_score = apply_rounding(raw_total, &policy.rounding_policy);

    // 4. Map Grade
    let mapped_grade = mapper.map_score(final_score)?;

    // 5. Create Snapshot
    let now = chrono::Utc::now().to_rfc3339();
    let snapshot_id = uuid::Uuid::new_v4().to_string();

    let snapshot_am = student_assessment_snapshot::ActiveModel {
        id: Set(snapshot_id.clone()),
        org_id: Set(ctx.org_id.clone()),
        student_id: Set(student_id.to_string()),
        period_id: Set(period_id.to_string()),
        class_subject_id: Set(class_subject_id.to_string()),
        total_score: Set(Some(final_score)),
        grade_label: Set(Some(mapped_grade.label)),
        gpa_points: Set(mapped_grade.gpa_points),
        is_passing: Set(Some(if mapped_grade.is_passing { 1 } else { 0 })),
        rank_in_class: Set(None), // Calculated later in batch processing
        rank_in_level: Set(None),
        calculation_version: Set(policy.version),
        status: Set("official".to_string()),
        approved_at: Set(None),
        approved_by: Set(None),
        metadata: Set(None),
        created_at: Set(now.clone()),
        updated_at: Set(now.clone()),
    };

    let inserted = snapshot_am.insert(&ctx.db).await?;
    
    // Fire Domain Event
    let event = shared_kernel::event_bus::EventEnvelope {
        event_type: "AssessmentSnapshotCreated".to_string(),
        entity_type: "student_assessment_snapshot".to_string(),
        entity_id: inserted.id.clone(),
        org_id: ctx.org_id.clone(),
        actor: match &ctx.actor {
            shared_kernel::traits::Actor::User(id) => id.clone(),
            shared_kernel::traits::Actor::System => "SYSTEM".to_string(),
            shared_kernel::traits::Actor::Agent(id) => id.clone(),
            shared_kernel::traits::Actor::Autopilot => "AUTOPILOT".to_string(),
        },
        actor_type: match &ctx.actor {
            shared_kernel::traits::Actor::User(_) => "User".to_string(),
            shared_kernel::traits::Actor::System => "System".to_string(),
            shared_kernel::traits::Actor::Agent(_) => "Agent".to_string(),
            shared_kernel::traits::Actor::Autopilot => "Autopilot".to_string(),
        },
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({
            "student_id": inserted.student_id,
            "period_id": inserted.period_id,
            "class_subject_id": inserted.class_subject_id,
            "total_score": inserted.total_score,
            "grade_label": inserted.grade_label,
            "gpa_points": inserted.gpa_points,
            "is_passing": inserted.is_passing,
        }),
        chain_depth: 0,
    };
    ctx.event_bus.publish(event);

    info!(
        student_id=%student_id,
        class_subject_id=%class_subject_id,
        score=%final_score,
        "Generated Student Assessment Snapshot"
    );

    Ok(inserted)
}
