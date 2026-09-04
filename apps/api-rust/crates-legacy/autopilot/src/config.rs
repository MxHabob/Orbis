//! Autopilot configuration — reads from unified `academic_policies` table.
//!
//! No longer depends on `autopilot_config` table (which was merged into
//! `academic_policies`). The policy model is the single source of truth.

use shared_kernel::types::ExecutionMode;
use platform_policy::entities::academic_policy;

/// Runtime-resolved configuration for an autopilot tick.
///
/// Built from the `academic_policies` row for the organization.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub org_id: String,
    pub mode: ExecutionMode,
    pub is_enabled: bool,
    pub max_auto_retries: i32,
    pub pause_on_error: bool,

    // Autopilot behavior toggles
    pub auto_open_enrollment: bool,
    pub auto_close_enrollment: bool,
    pub auto_open_grade_entry: bool,
    pub auto_close_grade_entry: bool,
    pub auto_lock_periods: bool,
    pub auto_advance_periods: bool,
    pub auto_archive_years: bool,
    pub grace_days_after_deadline: i32,
    pub notify_before_deadline_days: i32,
}

impl ResolvedConfig {
    /// Build from the unified academic_policies model.
    pub fn from_policy(policy: &academic_policy::Model) -> Self {
        Self {
            org_id: policy.org_id.clone(),
            mode: ExecutionMode::from_str(&policy.autopilot_mode),
            is_enabled: policy.autopilot_enabled == 1,
            max_auto_retries: policy.max_auto_retries,
            pause_on_error: policy.pause_on_error == 1,
            auto_open_enrollment: policy.auto_open_enrollment == 1,
            auto_close_enrollment: policy.auto_close_enrollment == 1,
            auto_open_grade_entry: policy.auto_open_grade_entry == 1,
            auto_close_grade_entry: policy.auto_close_grade_entry == 1,
            auto_lock_periods: policy.auto_lock_periods == 1,
            auto_advance_periods: policy.auto_advance_periods == 1,
            auto_archive_years: policy.auto_archive_years == 1,
            grace_days_after_deadline: policy.grace_days_after_deadline,
            notify_before_deadline_days: policy.notify_before_deadline_days,
        }
    }

    /// Build a sensible default config when no policy exists.
    pub fn default_for_org(org_id: &str) -> Self {
        Self {
            org_id: org_id.to_string(),
            mode: ExecutionMode::Suggested,
            is_enabled: true,
            max_auto_retries: 3,
            pause_on_error: false,
            auto_open_enrollment: true,
            auto_close_enrollment: true,
            auto_open_grade_entry: true,
            auto_close_grade_entry: true,
            auto_lock_periods: true,
            auto_advance_periods: false,
            auto_archive_years: false,
            grace_days_after_deadline: 3,
            notify_before_deadline_days: 7,
        }
    }

    /// Resolve effective mode, preferring override if provided
    pub fn resolve_mode(&self, override_mode: Option<&str>) -> ExecutionMode {
        match override_mode {
            Some(m) => ExecutionMode::from_str(m),
            None => self.mode,
        }
    }
}
