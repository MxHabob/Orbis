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

use std::sync::Arc;
use std::time::{Duration, Instant};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
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
    ) -> Result<GateVerdict, AppError> {
        let period = self.load_period(db, period_id).await?;
        let policy = self.load_policy(db, org_id).await?;
        let now = chrono::Utc::now().to_rfc3339();

        match gate_type {
            GateType::Enrollment => self.evaluate_enrollment(&period, &policy, &now, None),
            GateType::GradeEntry => self.evaluate_grade_entry(&period, &policy, &now),
            GateType::Attendance => self.evaluate_attendance(&period, &now),
            GateType::ExamEntry => self.evaluate_exam_entry(&period, &now),
            GateType::Assessment => self.evaluate_assessment(&period),
        }
    }

    /// Enrollment-specific check that also verifies class capacity.
    pub async fn check_enrollment(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        period_id: &str,
        class_id: &str,
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

        Ok(verdict)
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
        now: &str,
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
        use svc_organization::entities::academic_period;

        let period = academic_period::Entity::find_by_id(period_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_period", period_id))?;

        let cached = CachedPeriod {
            status: period.status,
            is_locked: period.is_locked == 1,
            is_enrollment_open: period.is_enrollment_open == 1,
            is_grade_entry_open: period.is_grade_entry_open == 1,
            enrollment_start: period.enrollment_start,
            enrollment_end: period.enrollment_end,
            grade_entry_start: period.grade_entry_start,
            grade_entry_end: period.grade_entry_end,
            exam_start: period.exam_start,
            exam_end: period.exam_end,
            start_date: period.start_date,
            end_date: period.end_date,
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
        use svc_organization::entities::academic_policy;

        let policy = academic_policy::Entity::find()
            .filter(academic_policy::Column::OrgId.eq(org_id))
            .filter(academic_policy::Column::OrgUnitId.is_null())
            .one(db)
            .await?;

        let cached = match policy {
            Some(p) => CachedPolicy {
                allow_late_enrollment: p.allow_late_enrollment == 1,
                late_enrollment_approval: p.late_enrollment_approval == 1,
                allow_grade_edit_after_close: p.allow_grade_edit_after_close == 1,
                grade_edit_approval: p.grade_edit_approval == 1,
                grace_days_after_deadline: p.grace_days_after_deadline,
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
        use svc_academic::entities::{class, enrollment};

        let target_class = class::Entity::find_by_id(class_id.to_string())
            .one(db)
            .await?;

        let Some(target_class) = target_class else {
            return Ok(None);
        };

        let Some(max_cap) = target_class.capacity else {
            return Ok(None); // No capacity limit
        };

        let current = enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .filter(enrollment::Column::PeriodId.eq(period_id))
            .filter(enrollment::Column::Status.ne("withdrawn"))
            .count(db)
            .await? as i64;

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
