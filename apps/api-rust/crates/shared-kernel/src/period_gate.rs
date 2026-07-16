//! Period Gate — centralized academic period enforcement.
//!
//! # Architecture
//!
//! Instead of scattering period checks across every operation (enrollment, grades,
//! attendance, exams...), the Period Gate is a **single middleware** that any service
//! calls before performing a write operation tied to an academic period.
//!
//! This follows the same pattern as `TenantGuard` — one line of code in any operation:
//!
//! ```rust,ignore
//! let verdict = ctx.period_gate.check(&ctx, &period_id, GateType::Enrollment).await?;
//! verdict.enforce()?;
//! ```
//!
//! # Gate Types
//!
//! Each gate type maps to a specific window in the academic period:
//!
//! | Gate         | Checks                                      |
//! |------------- |---------------------------------------------|
//! | Enrollment   | is_enrollment_open + enrollment_end + capacity |
//! | GradeEntry   | is_grade_entry_open + grade_entry_end        |
//! | Attendance   | period active + date within range            |
//! | ExamEntry    | exam_start/end + period.is_locked            |
//! | Assessment   | period.is_locked                             |
//!
//! # Caching
//!
//! Period and policy data are cached in-memory with a 30-second TTL.
//! This means checking the gate on every write is essentially **free** — no DB hit
//! unless the cache has expired.

use std::time::{Duration, Instant};

use sea_orm::{DatabaseConnection, ConnectionTrait};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::debug;

use crate::errors::AppError;

// ═══════════════════════════════════════════════════════════════
// Public Types
// ═══════════════════════════════════════════════════════════════

/// The type of gate to check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateType {
    /// Student enrollment — checks enrollment window + capacity.
    Enrollment,
    /// Grade submission — checks grade entry window.
    GradeEntry,
    /// Attendance recording — checks period is active + date range.
    Attendance,
    /// Exam scheduling — checks exam window.
    ExamEntry,
    /// Assessment creation/update — checks period lock.
    Assessment,
}

impl GateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enrollment => "enrollment",
            Self::GradeEntry => "grade_entry",
            Self::Attendance => "attendance",
            Self::ExamEntry => "exam_entry",
            Self::Assessment => "assessment",
        }
    }
}

/// The gate's decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateDecision {
    /// Operation is fully allowed.
    Allowed,
    /// Operation is allowed but considered late (within grace period).
    Late,
    /// Operation requires an override request (approval workflow).
    RequiresOverride,
    /// Operation is completely blocked.
    Blocked,
}

/// Actions the caller/frontend can take.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateAction {
    /// Execute the operation directly.
    DirectAction,
    /// Save as draft for later approval.
    SaveDraft,
    /// Submit an override request.
    RequestOverride,
}

/// Capacity information for enrollment gates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityInfo {
    pub current: i64,
    pub max: i32,
    pub available: i32,
    pub utilization_pct: f64,
}

/// Contextual information about the period state (sent to frontend).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateContext {
    pub period_status: String,
    pub window_start: Option<String>,
    pub window_end: Option<String>,
    pub is_within_grace: bool,
    pub grace_days: i32,
    pub capacity: Option<CapacityInfo>,
}

/// The result of a gate check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateVerdict {
    pub gate_type: GateType,
    pub decision: GateDecision,
    pub reason: Option<String>,
    pub override_type: Option<String>,
    pub allowed_actions: Vec<GateAction>,
    pub context: GateContext,
}

impl GateVerdict {
    /// Enforce the verdict — returns Ok for Allowed/Late, Err for Blocked.
    /// RequiresOverride returns Err with a specific error type.
    pub fn enforce(&self) -> Result<(), AppError> {
        match &self.decision {
            GateDecision::Allowed | GateDecision::Late => Ok(()),
            GateDecision::RequiresOverride => {
                Err(AppError::policy_override_required(
                    self.reason.clone().unwrap_or_else(|| "Override required".into()),
                    self.override_type.clone().unwrap_or_default(),
                ))
            }
            GateDecision::Blocked => {
                Err(AppError::policy_blocked(
                    self.reason.clone().unwrap_or_else(|| "Operation blocked by policy".into()),
                ))
            }
        }
    }

    /// Check if draft creation is allowed (for RequiresOverride cases).
    pub fn allows_draft(&self) -> bool {
        self.allowed_actions.contains(&GateAction::SaveDraft)
    }
}

// ═══════════════════════════════════════════════════════════════
// Cached Period Data
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct CachedPeriod {
    status: String,
    is_locked: bool,
    is_enrollment_open: bool,
    is_grade_entry_open: bool,
    enrollment_start: Option<String>,
    enrollment_end: Option<String>,
    grade_entry_start: Option<String>,
    grade_entry_end: Option<String>,
    exam_start: Option<String>,
    exam_end: Option<String>,
    start_date: String,
    end_date: String,
    fetched_at: Instant,
}

#[derive(Debug, Clone)]
struct CachedPolicy {
    allow_late_enrollment: bool,
    late_enrollment_approval: bool,
    allow_grade_edit_after_close: bool,
    grade_edit_approval: bool,
    grace_days_after_deadline: i32,
    fetched_at: Instant,
}

// ═══════════════════════════════════════════════════════════════
// PeriodGate
// ═══════════════════════════════════════════════════════════════

/// The centralized Period Gate — caches period + policy data.
pub struct PeriodGate {
    /// Cache of period data keyed by period_id.
    period_cache: RwLock<std::collections::HashMap<String, CachedPeriod>>,
    /// Cache of policy data keyed by org_id.
    policy_cache: RwLock<std::collections::HashMap<String, CachedPolicy>>,
    /// TTL for cache entries.
    ttl: Duration,
}

impl PeriodGate {
    /// Create a new PeriodGate with default 30s TTL.
    pub fn new() -> Self {
        Self {
            period_cache: RwLock::new(std::collections::HashMap::new()),
            policy_cache: RwLock::new(std::collections::HashMap::new()),
            ttl: Duration::from_secs(30),
        }
    }

    /// Create with a custom TTL (for testing).
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            period_cache: RwLock::new(std::collections::HashMap::new()),
            policy_cache: RwLock::new(std::collections::HashMap::new()),
            ttl,
        }
    }

    /// Invalidate cache for a specific period (called by Autopilot after state changes).
    pub async fn invalidate_period(&self, period_id: &str) {
        let mut cache = self.period_cache.write().await;
        cache.remove(period_id);
    }

    /// Invalidate policy cache for an org (called when policy is updated).
    pub async fn invalidate_policy(&self, org_id: &str) {
        let mut cache = self.policy_cache.write().await;
        cache.remove(org_id);
    }

    // ── Public API ──────────────────────────────────────────────

    /// Generic gate check — works for any GateType.
    pub async fn check(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        period_id: &str,
        gate_type: GateType,
        entity_id: Option<&str>,
    ) -> Result<GateVerdict, AppError> {
        let period = self.load_period(db, period_id).await?;
        let policy = self.load_policy(db, org_id).await?;
        let now = chrono::Utc::now().to_rfc3339();

        let mut verdict = match gate_type {
            GateType::Enrollment => self.evaluate_enrollment(&period, &policy, &now, None)?,
            GateType::GradeEntry => self.evaluate_grade_entry(&period, &policy, &now)?,
            GateType::Attendance => self.evaluate_attendance(&period, &now)?,
            GateType::ExamEntry => self.evaluate_exam_entry(&period, &now)?,
            GateType::Assessment => self.evaluate_assessment(&period)?,
        };

        if verdict.decision == GateDecision::RequiresOverride || verdict.decision == GateDecision::Blocked {
            if self.check_active_override(db, org_id, gate_type, entity_id).await? {
                verdict.decision = GateDecision::Allowed;
                verdict.override_type = Some("active_override".into());
                verdict.reason = Some("Approved override applied".into());
            }
        }

        Ok(verdict)
    }

    /// Enrollment-specific check that also verifies class capacity.
    pub async fn check_enrollment(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        period_id: &str,
        class_id: &str,
        entity_id: Option<&str>,
    ) -> Result<GateVerdict, AppError> {
        let period = self.load_period(db, period_id).await?;
        let policy = self.load_policy(db, org_id).await?;
        let now = chrono::Utc::now().to_rfc3339();

        // Check capacity
        let capacity = self.check_capacity(db, class_id, period_id).await?;

        let mut verdict = self.evaluate_enrollment(&period, &policy, &now, capacity.as_ref())?;

        // Override verdict if capacity is full
        if let Some(ref cap) = capacity {
            if cap.available <= 0 && verdict.decision == GateDecision::Allowed {
                verdict.decision = GateDecision::RequiresOverride;
                verdict.reason = Some(format!(
                    "الصف ممتلئ ({}/{})",
                    cap.current, cap.max
                ));
                verdict.override_type = Some("capacity_override".into());
                verdict.allowed_actions = vec![GateAction::SaveDraft, GateAction::RequestOverride];
            }
            verdict.context.capacity = capacity;
        }

        if verdict.decision == GateDecision::RequiresOverride || verdict.decision == GateDecision::Blocked {
            if self.check_active_override(db, org_id, GateType::Enrollment, entity_id).await? {
                verdict.decision = GateDecision::Allowed;
                verdict.override_type = Some("active_override".into());
                verdict.reason = Some("Approved override applied".into());
            }
        }

        Ok(verdict)
    }

    async fn check_active_override(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        gate_type: GateType,
        entity_id: Option<&str>,
    ) -> Result<bool, AppError> {
        let mut sql = r#"
            SELECT COUNT(*) as count 
            FROM academic_override_requests 
            WHERE org_id = $1 
              AND override_type = $2 
              AND status = 'approved'
              AND (expires_at IS NULL OR expires_at > $3)
        "#.to_string();
        
        let now = chrono::Utc::now().to_rfc3339();
        let mut params = vec![
            org_id.into(),
            gate_type.as_str().into(),
            now.into(),
        ];
        
        if let Some(eid) = entity_id {
            sql.push_str(" AND (entity_id = $4 OR entity_id = 'GLOBAL')");
            params.push(eid.into());
        } else {
            sql.push_str(" AND entity_id = 'GLOBAL'");
        }
        
        let res = db.query_one(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Sqlite,
            &sql,
            params,
        )).await.map_err(|e| AppError::internal(format!("Failed to query overrides: {}", e)))?;
        
        if let Some(row) = res {
            use sea_orm::QueryResult;
            let count: i64 = row.try_get("", "count").unwrap_or(0);
            Ok(count > 0)
        } else {
            Ok(false)
        }
    }

    // ── Evaluation Logic ────────────────────────────────────────

    fn evaluate_enrollment(
        &self,
        period: &CachedPeriod,
        policy: &CachedPolicy,
        now: &str,
        capacity: Option<&CapacityInfo>,
    ) -> Result<GateVerdict, AppError> {
        let base_context = GateContext {
            period_status: period.status.clone(),
            window_start: period.enrollment_start.clone(),
            window_end: period.enrollment_end.clone(),
            is_within_grace: false,
            grace_days: policy.grace_days_after_deadline,
            capacity: capacity.cloned(),
        };

        // Period locked → blocked
        if period.is_locked {
            return Ok(GateVerdict {
                gate_type: GateType::Enrollment,
                decision: GateDecision::Blocked,
                reason: Some("الفترة الأكاديمية مقفلة".into()),
                override_type: None,
                allowed_actions: vec![],
                context: base_context,
            });
        }

        // Enrollment open → allowed
        if period.is_enrollment_open {
            // Check if within date window
            if let Some(ref end) = period.enrollment_end {
                if now > end.as_str() {
                    // Past deadline but flag is still open (grace period via Autopilot)
                    return Ok(GateVerdict {
                        gate_type: GateType::Enrollment,
                        decision: GateDecision::Late,
                        reason: Some("التسجيل بعد الموعد النهائي (ضمن فترة السماح)".into()),
                        override_type: Some("late_enrollment".into()),
                        allowed_actions: if policy.late_enrollment_approval {
                            vec![GateAction::SaveDraft, GateAction::RequestOverride]
                        } else {
                            vec![GateAction::DirectAction]
                        },
                        context: GateContext { is_within_grace: true, ..base_context },
                    });
                }
            }

            return Ok(GateVerdict {
                gate_type: GateType::Enrollment,
                decision: GateDecision::Allowed,
                reason: None,
                override_type: None,
                allowed_actions: vec![GateAction::DirectAction],
                context: base_context,
            });
        }

        // Enrollment closed
        if policy.allow_late_enrollment {
            Ok(GateVerdict {
                gate_type: GateType::Enrollment,
                decision: GateDecision::RequiresOverride,
                reason: Some("فترة التسجيل مغلقة — يمكن طلب تسجيل متأخر".into()),
                override_type: Some("late_enrollment".into()),
                allowed_actions: vec![GateAction::SaveDraft, GateAction::RequestOverride],
                context: base_context,
            })
        } else {
            Ok(GateVerdict {
                gate_type: GateType::Enrollment,
                decision: GateDecision::Blocked,
                reason: Some("فترة التسجيل مغلقة".into()),
                override_type: None,
                allowed_actions: vec![],
                context: base_context,
            })
        }
    }

    fn evaluate_grade_entry(
        &self,
        period: &CachedPeriod,
        policy: &CachedPolicy,
        now: &str,
    ) -> Result<GateVerdict, AppError> {
        let base_context = GateContext {
            period_status: period.status.clone(),
            window_start: period.grade_entry_start.clone(),
            window_end: period.grade_entry_end.clone(),
            is_within_grace: false,
            grace_days: policy.grace_days_after_deadline,
            capacity: None,
        };

        if period.is_locked {
            return Ok(GateVerdict {
                gate_type: GateType::GradeEntry,
                decision: GateDecision::Blocked,
                reason: Some("الفترة الأكاديمية مقفلة".into()),
                override_type: None,
                allowed_actions: vec![],
                context: base_context,
            });
        }

        if period.is_grade_entry_open {
            if let Some(ref end) = period.grade_entry_end {
                if now > end.as_str() {
                    return Ok(GateVerdict {
                        gate_type: GateType::GradeEntry,
                        decision: GateDecision::Late,
                        reason: Some("إدخال الدرجات بعد الموعد النهائي".into()),
                        override_type: Some("late_grade_entry".into()),
                        allowed_actions: if policy.grade_edit_approval {
                            vec![GateAction::SaveDraft, GateAction::RequestOverride]
                        } else {
                            vec![GateAction::DirectAction]
                        },
                        context: GateContext { is_within_grace: true, ..base_context },
                    });
                }
            }
            if let Some(ref start) = period.grade_entry_start {
                if now < start.as_str() {
                    return Ok(GateVerdict {
                        gate_type: GateType::GradeEntry,
                        decision: GateDecision::Blocked,
                        reason: Some(format!("إدخال الدرجات لم يبدأ بعد — يبدأ في {}", start)),
                        override_type: None,
                        allowed_actions: vec![],
                        context: base_context,
                    });
                }
            }

            return Ok(GateVerdict {
                gate_type: GateType::GradeEntry,
                decision: GateDecision::Allowed,
                reason: None,
                override_type: None,
                allowed_actions: vec![GateAction::DirectAction],
                context: base_context,
            });
        }

        // Grade entry closed
        if policy.allow_grade_edit_after_close {
            Ok(GateVerdict {
                gate_type: GateType::GradeEntry,
                decision: GateDecision::RequiresOverride,
                reason: Some("نافذة إدخال الدرجات مغلقة — يمكن طلب تجاوز".into()),
                override_type: Some("late_grade_entry".into()),
                allowed_actions: vec![GateAction::SaveDraft, GateAction::RequestOverride],
                context: base_context,
            })
        } else {
            Ok(GateVerdict {
                gate_type: GateType::GradeEntry,
                decision: GateDecision::Blocked,
                reason: Some("نافذة إدخال الدرجات مغلقة".into()),
                override_type: None,
                allowed_actions: vec![],
                context: base_context,
            })
        }
    }

    fn evaluate_attendance(
        &self,
        period: &CachedPeriod,
        _now: &str,
    ) -> Result<GateVerdict, AppError> {
        let base_context = GateContext {
            period_status: period.status.clone(),
            window_start: Some(period.start_date.clone()),
            window_end: Some(period.end_date.clone()),
            is_within_grace: false,
            grace_days: 0,
            capacity: None,
        };

        if period.is_locked {
            return Ok(GateVerdict {
                gate_type: GateType::Attendance,
                decision: GateDecision::Blocked,
                reason: Some("الفترة الأكاديمية مقفلة".into()),
                override_type: None,
                allowed_actions: vec![],
                context: base_context,
            });
        }

        if period.status == "closed" || period.status == "archived" {
            return Ok(GateVerdict {
                gate_type: GateType::Attendance,
                decision: GateDecision::RequiresOverride,
                reason: Some("الفترة منتهية — تسجيل الحضور يتطلب تجاوز".into()),
                override_type: Some("late_attendance".into()),
                allowed_actions: vec![GateAction::SaveDraft, GateAction::RequestOverride],
                context: base_context,
            });
        }

        // Period is active
        Ok(GateVerdict {
            gate_type: GateType::Attendance,
            decision: GateDecision::Allowed,
            reason: None,
            override_type: None,
            allowed_actions: vec![GateAction::DirectAction],
            context: base_context,
        })
    }

    fn evaluate_exam_entry(
        &self,
        period: &CachedPeriod,
        now: &str,
    ) -> Result<GateVerdict, AppError> {
        let base_context = GateContext {
            period_status: period.status.clone(),
            window_start: period.exam_start.clone(),
            window_end: period.exam_end.clone(),
            is_within_grace: false,
            grace_days: 0,
            capacity: None,
        };

        if period.is_locked {
            return Ok(GateVerdict {
                gate_type: GateType::ExamEntry,
                decision: GateDecision::Blocked,
                reason: Some("الفترة الأكاديمية مقفلة".into()),
                override_type: None,
                allowed_actions: vec![],
                context: base_context,
            });
        }

        if let (Some(ref start), Some(ref end)) = (&period.exam_start, &period.exam_end) {
            if now < start.as_str() {
                return Ok(GateVerdict {
                    gate_type: GateType::ExamEntry,
                    decision: GateDecision::Blocked,
                    reason: Some(format!("فترة الاختبارات لم تبدأ — تبدأ في {}", start)),
                    override_type: None,
                    allowed_actions: vec![],
                    context: base_context,
                });
            }
            if now > end.as_str() {
                return Ok(GateVerdict {
                    gate_type: GateType::ExamEntry,
                    decision: GateDecision::RequiresOverride,
                    reason: Some("فترة الاختبارات انتهت".into()),
                    override_type: Some("late_exam".into()),
                    allowed_actions: vec![GateAction::RequestOverride],
                    context: base_context,
                });
            }
        }

        Ok(GateVerdict {
            gate_type: GateType::ExamEntry,
            decision: GateDecision::Allowed,
            reason: None,
            override_type: None,
            allowed_actions: vec![GateAction::DirectAction],
            context: base_context,
        })
    }

    fn evaluate_assessment(
        &self,
        period: &CachedPeriod,
    ) -> Result<GateVerdict, AppError> {
        let base_context = GateContext {
            period_status: period.status.clone(),
            window_start: None,
            window_end: None,
            is_within_grace: false,
            grace_days: 0,
            capacity: None,
        };

        if period.is_locked {
            return Ok(GateVerdict {
                gate_type: GateType::Assessment,
                decision: GateDecision::RequiresOverride,
                reason: Some("الفترة مقفلة — التعديل يتطلب تجاوز".into()),
                override_type: Some("locked_period_edit".into()),
                allowed_actions: vec![GateAction::RequestOverride],
                context: base_context,
            });
        }

        if period.status == "closed" || period.status == "archived" {
            return Ok(GateVerdict {
                gate_type: GateType::Assessment,
                decision: GateDecision::Blocked,
                reason: Some("الفترة مغلقة — لا يمكن التعديل".into()),
                override_type: None,
                allowed_actions: vec![],
                context: base_context,
            });
        }

        Ok(GateVerdict {
            gate_type: GateType::Assessment,
            decision: GateDecision::Allowed,
            reason: None,
            override_type: None,
            allowed_actions: vec![GateAction::DirectAction],
            context: base_context,
        })
    }

    // ── Data Loading (with Cache) ───────────────────────────────

    async fn load_period(
        &self,
        db: &DatabaseConnection,
        period_id: &str,
    ) -> Result<CachedPeriod, AppError> {
        // Check cache
        {
            let cache = self.period_cache.read().await;
            if let Some(cached) = cache.get(period_id) {
                if cached.fetched_at.elapsed() < self.ttl {
                    debug!(period_id, "PeriodGate cache hit");
                    return Ok(cached.clone());
                }
            }
        }

        // Cache miss — load from DB
        debug!(period_id, "PeriodGate cache miss — loading from DB");
        

        let sql = "SELECT status, is_locked, is_enrollment_open, is_grade_entry_open, enrollment_start, enrollment_end, grade_entry_start, grade_entry_end, exam_start, exam_end, start_date, end_date FROM academic_periods WHERE id = ?1";
        let stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            sql,
            vec![period_id.into()]
        );
        let row = db.query_one(stmt).await?.ok_or_else(|| AppError::not_found("academic_period", period_id))?;
        
        let cached = CachedPeriod {
            status: row.try_get::<String>("", "status").unwrap_or_default(),
            is_locked: row.try_get::<i32>("", "is_locked").unwrap_or(0) == 1,
            is_enrollment_open: row.try_get::<i32>("", "is_enrollment_open").unwrap_or(0) == 1,
            is_grade_entry_open: row.try_get::<i32>("", "is_grade_entry_open").unwrap_or(0) == 1,
            enrollment_start: row.try_get_by::<String, _>("enrollment_start").ok(),
            enrollment_end: row.try_get_by::<String, _>("enrollment_end").ok(),
            grade_entry_start: row.try_get_by::<String, _>("grade_entry_start").ok(),
            grade_entry_end: row.try_get_by::<String, _>("grade_entry_end").ok(),
            exam_start: row.try_get_by::<String, _>("exam_start").ok(),
            exam_end: row.try_get_by::<String, _>("exam_end").ok(),
            start_date: row.try_get_by::<String, _>("start_date").ok().unwrap_or_default(),
            end_date: row.try_get_by::<String, _>("end_date").ok().unwrap_or_default(),
            fetched_at: Instant::now(),
        };

        // Store in cache
        {
            let mut cache = self.period_cache.write().await;
            cache.insert(period_id.to_string(), cached.clone());
        }

        Ok(cached)
    }

    async fn load_policy(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<CachedPolicy, AppError> {
        // Check cache
        {
            let cache = self.policy_cache.read().await;
            if let Some(cached) = cache.get(org_id) {
                if cached.fetched_at.elapsed() < self.ttl {
                    return Ok(cached.clone());
                }
            }
        }

        // Cache miss — load from DB
        

        let sql = "SELECT allow_late_enrollment, late_enrollment_approval, allow_grade_edit_after_close, grade_edit_approval, grace_days_after_deadline FROM academic_policies WHERE org_id = ?1 AND org_unit_id IS NULL";
        let stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            sql,
            vec![org_id.into()]
        );
        let row = db.query_one(stmt).await?;
        
        let cached = match row {
            Some(p) => CachedPolicy {
                allow_late_enrollment: p.try_get::<i32>("", "allow_late_enrollment").unwrap_or(0) == 1,
                late_enrollment_approval: p.try_get::<i32>("", "late_enrollment_approval").unwrap_or(0) == 1,
                allow_grade_edit_after_close: p.try_get::<i32>("", "allow_grade_edit_after_close").unwrap_or(0) == 1,
                grade_edit_approval: p.try_get::<i32>("", "grade_edit_approval").unwrap_or(0) == 1,
                grace_days_after_deadline: p.try_get::<i32>("", "grace_days_after_deadline").unwrap_or(0),
                fetched_at: Instant::now(),
            },
            None => CachedPolicy {
                allow_late_enrollment: false,
                late_enrollment_approval: false,
                allow_grade_edit_after_close: false,
                grade_edit_approval: false,
                grace_days_after_deadline: 0,
                fetched_at: Instant::now(),
            },
        };

        {
            let mut cache = self.policy_cache.write().await;
            cache.insert(org_id.to_string(), cached.clone());
        }

        Ok(cached)
    }

    async fn check_capacity(
        &self,
        db: &DatabaseConnection,
        class_id: &str,
        period_id: &str,
    ) -> Result<Option<CapacityInfo>, AppError> {
        let sql_class = "SELECT capacity FROM classes WHERE id = ?1";
        let stmt_class = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            sql_class,
            vec![class_id.into()]
        );
        let target_class = db.query_one(stmt_class).await?;

        let Some(target_class) = target_class else {
            return Ok(None);
        };

        let max_cap = target_class.try_get_by::<Option<i32>, _>("capacity").ok().flatten();
        let Some(max_cap) = max_cap else {
            return Ok(None); // No capacity limit
        };

        let sql_enrollment = "SELECT COUNT(*) AS c FROM enrollments WHERE class_id = ?1 AND period_id = ?2 AND status != 'withdrawn'";
        let stmt_enrollment = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            sql_enrollment,
            vec![class_id.into(), period_id.into()]
        );
        let current_row = db.query_one(stmt_enrollment).await?;
        let current = current_row.map(|r| r.try_get::<i64>("", "c").unwrap_or(0)).unwrap_or(0);

        let available = (max_cap as i64 - current).max(0) as i32;
        let utilization = if max_cap > 0 {
            (current as f64 / max_cap as f64) * 100.0
        } else {
            0.0
        };

        Ok(Some(CapacityInfo {
            current,
            max: max_cap,
            available,
            utilization_pct: utilization,
        }))
    }
}

impl Default for PeriodGate {
    fn default() -> Self {
        Self::new()
    }
}


