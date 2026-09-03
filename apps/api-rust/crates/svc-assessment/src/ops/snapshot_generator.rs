use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use tracing::info;

use crate::entities::{calculation_policy, grade_scale_item, period_overall_result, period_score};
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
) -> Result<period_score::Model, AppError> {
    
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

    let mut engine = CalcEngine::new();
    for (key, val) in &raw_scores {
        engine.set_variable(key, *val);
    }
    if let Ok(Some(sid)) = crate::ops::resolution::resolve_scheme_id(&ctx.db, &ctx.org_id, class_subject_id, period_id).await {
        if let Ok(comps) = crate::ops::resolution::scheme_components(&ctx.db, &sid).await {
            for c in comps {
                if !engine.context.contains_key(&c.component_type) {
                    engine.set_variable(&c.component_type, 0.0);
                }
            }
        }
    }

    let raw_total = engine.evaluate(&ast)?;
    let final_score = apply_rounding(raw_total, &policy.rounding_policy);
    let mapped_grade = mapper.map_score(final_score)?;

    let enrollment_id = {
        let mut found = None;
        if let Ok(Some(cs)) = svc_academic::entities::class_subject::Entity::find_by_id(class_subject_id.to_string()).one(&ctx.db).await {
            if let Ok(Some(enr)) = svc_academic::entities::enrollment::Entity::find()
                .filter(svc_academic::entities::enrollment::Column::StudentId.eq(student_id))
                .filter(svc_academic::entities::enrollment::Column::ClassId.eq(&cs.class_id))
                .filter(svc_academic::entities::enrollment::Column::Status.eq("active"))
                .one(&ctx.db)
                .await
            {
                found = Some(enr.id);
            }
        }
        found
    };

    let now = chrono::Utc::now().to_rfc3339();
    let existing = period_score::Entity::find()
        .filter(period_score::Column::OrgId.eq(&ctx.org_id))
        .filter(period_score::Column::StudentId.eq(student_id))
        .filter(period_score::Column::PeriodId.eq(period_id))
        .filter(period_score::Column::ClassSubjectId.eq(class_subject_id))
        .one(&ctx.db)
        .await?;

    let inserted = if let Some(ex) = existing {
        if ex.status == "finalized" {
            return Err(AppError::validation("Period score is finalized and cannot be recalculated"));
        }
        let mut active: period_score::ActiveModel = ex.into();
        active.enrollment_id = Set(enrollment_id.clone());
        active.total_score = Set(Some(final_score));
        active.grade_label = Set(Some(mapped_grade.label.clone()));
        active.gpa_points = Set(mapped_grade.gpa_points);
        active.is_passing = Set(Some(if mapped_grade.is_passing { 1 } else { 0 }));
        active.calculation_version = Set(policy.version);
        active.status = Set("official".to_string());
        active.updated_at = Set(now);
        active.update(&ctx.db).await?
    } else {
        period_score::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            org_id: Set(ctx.org_id.clone()),
            student_id: Set(student_id.to_string()),
            enrollment_id: Set(enrollment_id),
            period_id: Set(period_id.to_string()),
            class_subject_id: Set(class_subject_id.to_string()),
            total_score: Set(Some(final_score)),
            grade_label: Set(Some(mapped_grade.label.clone())),
            gpa_points: Set(mapped_grade.gpa_points),
            is_passing: Set(Some(if mapped_grade.is_passing { 1 } else { 0 })),
            rank_in_class: Set(None),
            rank_in_level: Set(None),
            calculation_version: Set(policy.version),
            status: Set("official".to_string()),
            approved_at: Set(None),
            approved_by: Set(None),
            metadata: Set(None),
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
        }
        .insert(&ctx.db)
        .await?
    };
    
    // Fire Domain Event
    let event = shared_kernel::event_bus::EventEnvelope {
        event_type: "PeriodScoreCalculated".to_string(),
        entity_type: "period_score".to_string(),
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
        "Generated period score"
    );

    Ok(inserted)
}

/// Compute rankings for all student snapshots in a given period + class_subject.
///
/// This function:
/// 1. Fetches all snapshots for the (period, class_subject) pair.
/// 2. Sorts by `total_score` descending.
/// 3. Assigns `rank_in_class` (1..N) with tie handling (same score = same rank).
/// 4. For `rank_in_level`, it fetches all snapshots across all class_subjects
///    for the same period and computes a cumulative GPA rank.
pub async fn compute_rankings(
    ctx: &ServiceContext,
    period_id: &str,
    class_subject_id: &str,
) -> Result<usize, AppError> {
    let policy = crate::ops::ranking_policy::resolve_ranking_policy(ctx, period_id).await?;
    let snapshots = period_score::Entity::find()
        .filter(period_score::Column::OrgId.eq(&ctx.org_id))
        .filter(period_score::Column::PeriodId.eq(period_id))
        .filter(period_score::Column::ClassSubjectId.eq(class_subject_id))
        .all(&ctx.db)
        .await?;

    if snapshots.is_empty() {
        return Ok(0);
    }

    let metric = policy.ranking_metric.as_str();
    let entries: Vec<crate::ops::ranking_policy::RankEntry> = snapshots
        .iter()
        .map(|s| {
            let score = match metric {
                "gpa" => s.gpa_points.unwrap_or(0.0),
                "percentage" => s.total_score.unwrap_or(0.0),
                _ => s.total_score.unwrap_or(0.0),
            };
            crate::ops::ranking_policy::RankEntry {
                student_id: s.student_id.clone(),
                score,
                is_passing: s.is_passing.unwrap_or(0) == 1,
            }
        })
        .collect();

    let rankings = crate::ops::ranking_policy::compute_rankings_with_policy(&entries, &policy);
    let now = chrono::Utc::now().to_rfc3339();
    for rr in &rankings {
        if let Some(snap) = snapshots.iter().find(|s| s.student_id == rr.student_id) {
            let mut active: period_score::ActiveModel = snap.clone().into();
            active.rank_in_class = Set(rr.rank);
            active.updated_at = Set(now.clone());
            active.update(&ctx.db).await?;
        }
    }

    let all_period_snapshots = period_score::Entity::find()
        .filter(period_score::Column::OrgId.eq(&ctx.org_id))
        .filter(period_score::Column::PeriodId.eq(period_id))
        .all(&ctx.db)
        .await?;

    // Build per-student average GPA
    let mut student_gpas: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    for s in &all_period_snapshots {
        if let Some(gpa) = s.gpa_points {
            student_gpas.entry(s.student_id.clone()).or_default().push(gpa);
        }
    }

    let mut student_avg: Vec<(String, f64)> = student_gpas
        .into_iter()
        .map(|(sid, gpas)| {
            let avg = gpas.iter().sum::<f64>() / gpas.len() as f64;
            (sid, avg)
        })
        .collect();

    // Sort descending by average GPA
    student_avg.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Assign rank_in_level
    let mut level_ranks: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let mut level_rank = 1i32;
    let mut prev_avg: Option<f64> = None;
    let mut level_tie_count = 0i32;

    for (student_id, avg) in &student_avg {
        if let Some(prev) = prev_avg {
            if (avg - prev).abs() < 0.001 {
                level_tie_count += 1;
            } else {
                level_rank += level_tie_count;
                level_tie_count = 1;
            }
        } else {
            level_tie_count = 1;
        }
        prev_avg = Some(*avg);
        level_ranks.insert(student_id.clone(), level_rank);
    }

    // Update rank_in_level for snapshots that belong to the requested class_subject
    for snapshot in &snapshots {
        if let Some(&r) = level_ranks.get(&snapshot.student_id) {
            let mut active: period_score::ActiveModel = snapshot.clone().into();
            active.rank_in_level = Set(Some(r));
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            active.update(&ctx.db).await?;
        }
    }

    info!(
        period_id=%period_id,
        students=%student_avg.len(),
        "Computed rank_in_level across all class subjects"
    );

    Ok(snapshots.len())
}

/// Apply a Pass/Fail policy to a student's snapshots for a given period.
///
/// Uses the `pass_fail_policies` table for the period, falling back to
/// `academic_policies.default_passing_grade`.
pub async fn apply_pass_fail_for_student(
    ctx: &ServiceContext,
    student_id: &str,
    period_id: &str,
) -> Result<bool, AppError> {
    use crate::entities::pass_fail_policy;

    // Try to load period-specific pass/fail policy, falling back to org-wide policy (period_id IS NULL)
    let policy = match pass_fail_policy::Entity::find()
        .filter(pass_fail_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(pass_fail_policy::Column::PeriodId.eq(period_id))
        .one(&ctx.db)
        .await?
    {
        Some(p) => Some(p),
        None => {
            pass_fail_policy::Entity::find()
                .filter(pass_fail_policy::Column::OrgId.eq(&ctx.org_id))
                .filter(pass_fail_policy::Column::PeriodId.is_null())
                .one(&ctx.db)
                .await?
        }
    };

    let min_pass = policy.as_ref().map(|p| p.min_passing_score)
        .unwrap_or(crate::ops::assessment_engine::AssessmentEngine::get_org_passing_grade(&ctx.db, &ctx.org_id).await);

    let requires_all = policy.as_ref().map(|p| p.requires_all_subjects == 1).unwrap_or(false);
    let max_fail = policy.as_ref().map(|p| p.max_failed_subjects).unwrap_or(0);

    // Load all snapshots for this student in this period
    let snapshots = period_score::Entity::find()
        .filter(period_score::Column::OrgId.eq(&ctx.org_id))
        .filter(period_score::Column::StudentId.eq(student_id))
        .filter(period_score::Column::PeriodId.eq(period_id))
        .all(&ctx.db)
        .await?;

    let mut failed_count = 0;
    for s in &snapshots {
        let score = s.total_score.unwrap_or(0.0);
        if score < min_pass {
            failed_count += 1;
        }
    }

    let overall_pass = if requires_all {
        failed_count == 0
    } else {
        failed_count <= max_fail
    };

    info!(
        student_id=%student_id,
        period_id=%period_id,
        failed_count=%failed_count,
        overall_pass=%overall_pass,
        "Applied pass/fail policy for student"
    );

    let enrollment_id = snapshots
        .iter()
        .find_map(|s| s.enrollment_id.clone())
        .unwrap_or_default();
    let avg = if snapshots.is_empty() {
        0.0
    } else {
        snapshots.iter().map(|s| s.total_score.unwrap_or(0.0)).sum::<f64>() / snapshots.len() as f64
    };
    let decision = if overall_pass { "passed" } else { "failed" };
    let now = chrono::Utc::now().to_rfc3339();
    let existing_overall = period_overall_result::Entity::find()
        .filter(period_overall_result::Column::OrgId.eq(&ctx.org_id))
        .filter(period_overall_result::Column::StudentId.eq(student_id))
        .filter(period_overall_result::Column::PeriodId.eq(period_id))
        .one(&ctx.db)
        .await?;
    if let Some(ex) = existing_overall {
        let mut active: period_overall_result::ActiveModel = ex.into();
        active.enrollment_id = Set(enrollment_id);
        active.passed = Set(if overall_pass { 1 } else { 0 });
        active.failed_subject_count = Set(failed_count as i32);
        active.policy_id = Set(policy.as_ref().map(|p| p.id.clone()));
        active.decision = Set(decision.to_string());
        active.percentage = Set(Some(avg));
        active.updated_at = Set(now);
        active.update(&ctx.db).await?;
    } else {
        period_overall_result::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            org_id: Set(ctx.org_id.clone()),
            student_id: Set(student_id.to_string()),
            enrollment_id: Set(enrollment_id),
            period_id: Set(period_id.to_string()),
            passed: Set(if overall_pass { 1 } else { 0 }),
            failed_subject_count: Set(failed_count as i32),
            policy_id: Set(policy.as_ref().map(|p| p.id.clone())),
            decision: Set(decision.to_string()),
            percentage: Set(Some(avg)),
            created_at: Set(now.clone()),
            updated_at: Set(now),
        }
        .insert(&ctx.db)
        .await?;
    }

    Ok(overall_pass)
}

pub async fn list_period_scores(
    ctx: &ServiceContext,
    period_id: &str,
    class_subject_id: Option<&str>,
) -> Result<Vec<period_score::Model>, AppError> {
    use shared_kernel::traits::Permission;
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;
    let mut q = period_score::Entity::find()
        .filter(period_score::Column::OrgId.eq(&ctx.org_id))
        .filter(period_score::Column::PeriodId.eq(period_id));
    if let Some(cs) = class_subject_id {
        q = q.filter(period_score::Column::ClassSubjectId.eq(cs));
    }
    Ok(q.all(&ctx.db).await?)
}

pub async fn list_overall_results(
    ctx: &ServiceContext,
    period_id: &str,
) -> Result<Vec<period_overall_result::Model>, AppError> {
    use shared_kernel::traits::Permission;
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;
    Ok(period_overall_result::Entity::find()
        .filter(period_overall_result::Column::OrgId.eq(&ctx.org_id))
        .filter(period_overall_result::Column::PeriodId.eq(period_id))
        .all(&ctx.db)
        .await?)
}
