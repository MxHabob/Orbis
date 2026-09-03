//! Autopilot entities — SeaORM models for autopilot_tasks
//! and autopilot_execution_log tables.
//!
//! Note: autopilot_config was merged into academic_policies.
//! Config is now read via `svc_organization::entities::academic_policy`.

pub mod autopilot_task;
