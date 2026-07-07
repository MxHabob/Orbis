//! Autopilot configuration — central mode resolution + defaults.

use shared_kernel::types::ExecutionMode;
use crate::entities::autopilot_config;

/// Runtime-resolved configuration for an autopilot tick.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub org_id: String,
    pub mode: ExecutionMode,
    pub check_interval_minutes: i32,
    pub timezone: String,
    pub require_backup_before_critical: bool,
    pub max_auto_retries: i32,
    pub pause_on_error: bool,
    pub is_enabled: bool,
}

impl ResolvedConfig {
    /// Build from a DB model.
    pub fn from_model(model: &autopilot_config::Model) -> Self {
        Self {
            org_id: model.org_id.clone(),
            mode: ExecutionMode::from_str(&model.mode),
            check_interval_minutes: model.check_interval_minutes,
            timezone: model.timezone.clone(),
            require_backup_before_critical: model.require_backup_before_critical == 1,
            max_auto_retries: model.max_auto_retries,
            pause_on_error: model.pause_on_error == 1,
            is_enabled: model.is_enabled == 1,
        }
    }

    /// Build a sensible default config when no DB record exists.
    pub fn default_for_org(org_id: &str) -> Self {
        Self {
            org_id: org_id.to_string(),
            mode: ExecutionMode::Suggested,
            check_interval_minutes: 60,
            timezone: "Asia/Riyadh".to_string(),
            require_backup_before_critical: true,
            max_auto_retries: 3,
            pause_on_error: true,
            is_enabled: true,
        }
    }

    /// Resolve effective execution mode: event-level override wins over global config.
    pub fn resolve_mode(&self, event_override: Option<&str>) -> ExecutionMode {
        ExecutionMode::resolve(self.mode.as_str(), event_override)
    }
}
