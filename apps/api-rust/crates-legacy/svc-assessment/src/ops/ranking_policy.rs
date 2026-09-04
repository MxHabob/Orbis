//! Ranking Policy Operations — CRUD + configurable ranking computation.
//!
//! The ranking engine reads a `RankingPolicy` to determine:
//! - Whether ranking is enabled
//! - The ranking scope (class, grade, section, branch, program)
//! - The ranking method (dense, standard, modified_competition)
//! - Tie handling behaviour
//! - Whether failed students participate
//! - Which metric to rank by (total_score, gpa, percentage)
//!
//! Ranking values are always stored as numeric integers.
//! Localized ordinal text is a presentation concern (see `ordinal.rs`).

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
    ConnectionTrait, Statement,
};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::ranking_policy;

// ── CRUD ────────────────────────────────────────────────────────

use shared_kernel::query_filter::UniversalFilter;

/// List all ranking policies for the current organization.
pub async fn list(ctx: &ServiceContext, filter: Option<UniversalFilter>) -> Result<Vec<ranking_policy::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    let mut query = ranking_policy::Entity::find()
        .filter(ranking_policy::Column::OrgId.eq(&ctx.org_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "name" => Some(ranking_policy::Column::Name.into_expr().into()),
            "is_enabled" => Some(ranking_policy::Column::IsEnabled.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.all(&ctx.db).await?)
}

/// Get a single ranking policy by ID.
pub async fn get(
    ctx: &ServiceContext,
    policy_id: &str,
) -> Result<ranking_policy::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    ranking_policy::Entity::find_by_id(policy_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("ranking_policy", policy_id))
}

/// Create a new ranking policy.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<ranking_policy::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    #[derive(serde::Deserialize)]
    struct Req {
        name: String,
        period_id: Option<String>,
        is_enabled: Option<bool>,
        ranking_scope: Option<String>,
        ranking_method: Option<String>,
        tie_handling: Option<String>,
        include_failed_students: Option<bool>,
        ranking_metric: Option<String>,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid ranking policy payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let model = ranking_policy::Model {
        id: Uuid::new_v4().to_string(),
        org_id: ctx.org_id.clone(),
        period_id: req.period_id,
        name: req.name,
        is_enabled: if req.is_enabled.unwrap_or(true) { 1 } else { 0 },
        ranking_scope: req.ranking_scope.unwrap_or_else(|| "class".to_string()),
        ranking_method: req.ranking_method.unwrap_or_else(|| "standard".to_string()),
        tie_handling: req.tie_handling.unwrap_or_else(|| "same_rank".to_string()),
        include_failed_students: if req.include_failed_students.unwrap_or(false) { 1 } else { 0 },
        ranking_metric: req.ranking_metric.unwrap_or_else(|| "total_score".to_string()),
        created_at: now.clone(),
        updated_at: now,
    };

    let active: ranking_policy::ActiveModel = model.into();
    let inserted = active.insert(&ctx.db).await?;
    Ok(inserted)
}

/// Update a ranking policy.
pub async fn update(
    ctx: &ServiceContext,
    policy_id: &str,
    data: serde_json::Value,
) -> Result<ranking_policy::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = get(ctx, policy_id).await?;
    let mut active: ranking_policy::ActiveModel = existing.into();

    if let Some(name) = data.get("name").and_then(|v| v.as_str()) {
        active.name = Set(name.to_string());
    }
    if let Some(enabled) = data.get("is_enabled").and_then(|v| v.as_bool()) {
        active.is_enabled = Set(if enabled { 1 } else { 0 });
    }
    if let Some(scope) = data.get("ranking_scope").and_then(|v| v.as_str()) {
        active.ranking_scope = Set(scope.to_string());
    }
    if let Some(method) = data.get("ranking_method").and_then(|v| v.as_str()) {
        active.ranking_method = Set(method.to_string());
    }
    if let Some(tie) = data.get("tie_handling").and_then(|v| v.as_str()) {
        active.tie_handling = Set(tie.to_string());
    }
    if let Some(inc_failed) = data.get("include_failed_students").and_then(|v| v.as_bool()) {
        active.include_failed_students = Set(if inc_failed { 1 } else { 0 });
    }
    if let Some(metric) = data.get("ranking_metric").and_then(|v| v.as_str()) {
        active.ranking_metric = Set(metric.to_string());
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;
    Ok(updated)
}

/// Delete a ranking policy.
pub async fn delete(ctx: &ServiceContext, policy_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    ranking_policy::Entity::delete_by_id(policy_id.to_string())
        .exec(&ctx.db)
        .await?;
    Ok(())
}

// ── Ranking Computation ─────────────────────────────────────────

/// A student's score entry for ranking.
#[derive(Debug, Clone)]
pub struct RankEntry {
    pub student_id: String,
    pub score: f64,
    pub is_passing: bool,
}

/// A computed ranking result.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RankResult {
    pub student_id: String,
    pub rank: Option<i32>,
}

/// Resolve the effective ranking policy for a period (or org default).
pub async fn resolve_ranking_policy(
    ctx: &ServiceContext,
    period_id: &str,
) -> Result<ranking_policy::Model, AppError> {
    // Try period-specific first
    let period_policy = ranking_policy::Entity::find()
        .filter(ranking_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(ranking_policy::Column::PeriodId.eq(period_id))
        .one(&ctx.db)
        .await?;

    if let Some(p) = period_policy {
        return Ok(p);
    }

    // Fallback to org-wide default (period_id IS NULL)
    ranking_policy::Entity::find()
        .filter(ranking_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(ranking_policy::Column::PeriodId.is_null())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::validation(
            "No ranking policy found. Create a RankingPolicy for this organization."
        ))
}

/// Compute rankings for a set of students according to the given policy.
///
/// This is the **policy-driven** replacement for the old hard-coded
/// `compute_rankings()` in `snapshot_generator.rs`.
///
/// Returns `None` rank for students excluded from ranking (e.g. failed
/// students when `include_failed_students = false`, or when ranking
/// is disabled entirely).
pub fn compute_rankings_with_policy(
    entries: &[RankEntry],
    policy: &ranking_policy::Model,
) -> Vec<RankResult> {
    // If ranking is disabled, return all with rank = None
    if policy.is_enabled == 0 {
        return entries
            .iter()
            .map(|e| RankResult {
                student_id: e.student_id.clone(),
                rank: None,
            })
            .collect();
    }

    // Partition into rankable and non-rankable
    let (mut rankable, non_rankable): (Vec<_>, Vec<_>) = entries
        .iter()
        .cloned()
        .partition(|e| {
            if policy.include_failed_students == 0 {
                e.is_passing
            } else {
                true
            }
        });

    // Sort by score descending
    rankable.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    let mut results: Vec<RankResult> = Vec::with_capacity(entries.len());
    let method = policy.ranking_method.as_str();

    let mut current_rank = 1i32;
    let mut i = 0;

    while i < rankable.len() {
        // Count ties
        let mut tie_count = 1;
        while i + tie_count < rankable.len()
            && (rankable[i].score - rankable[i + tie_count].score).abs() < 0.001
        {
            tie_count += 1;
        }

        // Assign ranks based on method
        for j in 0..tie_count {
            results.push(RankResult {
                student_id: rankable[i + j].student_id.clone(),
                rank: Some(current_rank),
            });
        }

        // Advance rank counter based on method
        match method {
            "dense" => {
                // Dense: 1, 2, 2, 3 (next rank is always +1)
                current_rank += 1;
            }
            "modified_competition" | "standard" | _ => {
                // Standard: 1, 2, 2, 4 (next rank skips tied positions)
                current_rank += tie_count as i32;
            }
        }

        i += tie_count;
    }

    // Non-rankable students get rank = None
    for entry in &non_rankable {
        results.push(RankResult {
            student_id: entry.student_id.clone(),
            rank: None,
        });
    }

    results
}

/// Convenience: compute rankings for term_results in a period using the DB policy.
pub async fn compute_term_rankings(
    ctx: &ServiceContext,
    term_period_id: &str,
    class_subject_id: &str,
) -> Result<Vec<RankResult>, AppError> {
    use crate::entities::term_result;

    let policy = resolve_ranking_policy(ctx, term_period_id).await?;

    // Load all term results for this period + class_subject
    let results = term_result::Entity::find()
        .filter(term_result::Column::OrgId.eq(&ctx.org_id))
        .filter(term_result::Column::TermPeriodId.eq(term_period_id))
        .filter(term_result::Column::ClassSubjectId.eq(class_subject_id))
        .all(&ctx.db)
        .await?;

    let metric = policy.ranking_metric.as_str();
    let entries: Vec<RankEntry> = results
        .iter()
        .map(|r| {
            let score = match metric {
                "gpa" => r.gpa_points.unwrap_or(0.0),
                "percentage" => r.percentage,
                _ => r.total_score, // "total_score" default
            };
            RankEntry {
                student_id: r.student_id.clone(),
                score,
                is_passing: r.is_passing == 1,
            }
        })
        .collect();

    let rankings = compute_rankings_with_policy(&entries, &policy);

    // Persist ranks back to term_results
    for rr in &rankings {
        if let Some(rank) = rr.rank {
            let tr = results.iter().find(|r| r.student_id == rr.student_id);
            if let Some(tr) = tr {
                let stmt = Statement::from_sql_and_values(
                    ctx.db.get_database_backend(),
                    "UPDATE term_results SET rank = $1, rank_scope = $2, updated_at = $3 WHERE id = $4",
                    vec![
                        rank.into(),
                        policy.ranking_scope.clone().into(),
                        Utc::now().to_rfc3339().into(),
                        tr.id.clone().into(),
                    ],
                );
                ctx.db.execute(stmt).await?;
            }
        }
    }

    Ok(rankings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_policy(method: &str, include_failed: bool) -> ranking_policy::Model {
        ranking_policy::Model {
            id: "test".into(),
            org_id: "org".into(),
            period_id: None,
            name: "Test".into(),
            is_enabled: 1,
            ranking_scope: "class".into(),
            ranking_method: method.into(),
            tie_handling: "same_rank".into(),
            include_failed_students: if include_failed { 1 } else { 0 },
            ranking_metric: "total_score".into(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    fn make_entry(id: &str, score: f64, passing: bool) -> RankEntry {
        RankEntry {
            student_id: id.into(),
            score,
            is_passing: passing,
        }
    }

    #[test]
    fn test_standard_ranking() {
        let policy = make_policy("standard", true);
        let entries = vec![
            make_entry("a", 95.0, true),
            make_entry("b", 85.0, true),
            make_entry("c", 85.0, true),
            make_entry("d", 70.0, true),
        ];

        let results = compute_rankings_with_policy(&entries, &policy);
        let rank_of = |id: &str| results.iter().find(|r| r.student_id == id).unwrap().rank;

        assert_eq!(rank_of("a"), Some(1));
        assert_eq!(rank_of("b"), Some(2));
        assert_eq!(rank_of("c"), Some(2));
        assert_eq!(rank_of("d"), Some(4)); // skips 3
    }

    #[test]
    fn test_dense_ranking() {
        let policy = make_policy("dense", true);
        let entries = vec![
            make_entry("a", 95.0, true),
            make_entry("b", 85.0, true),
            make_entry("c", 85.0, true),
            make_entry("d", 70.0, true),
        ];

        let results = compute_rankings_with_policy(&entries, &policy);
        let rank_of = |id: &str| results.iter().find(|r| r.student_id == id).unwrap().rank;

        assert_eq!(rank_of("a"), Some(1));
        assert_eq!(rank_of("b"), Some(2));
        assert_eq!(rank_of("c"), Some(2));
        assert_eq!(rank_of("d"), Some(3)); // no skip
    }

    #[test]
    fn test_exclude_failed_students() {
        let policy = make_policy("standard", false);
        let entries = vec![
            make_entry("a", 90.0, true),
            make_entry("b", 80.0, true),
            make_entry("c", 40.0, false),
        ];

        let results = compute_rankings_with_policy(&entries, &policy);
        let rank_of = |id: &str| results.iter().find(|r| r.student_id == id).unwrap().rank;

        assert_eq!(rank_of("a"), Some(1));
        assert_eq!(rank_of("b"), Some(2));
        assert_eq!(rank_of("c"), None); // excluded
    }

    #[test]
    fn test_disabled_ranking() {
        let mut policy = make_policy("standard", true);
        policy.is_enabled = 0;

        let entries = vec![
            make_entry("a", 95.0, true),
            make_entry("b", 85.0, true),
        ];

        let results = compute_rankings_with_policy(&entries, &policy);
        assert!(results.iter().all(|r| r.rank.is_none()));
    }
}
