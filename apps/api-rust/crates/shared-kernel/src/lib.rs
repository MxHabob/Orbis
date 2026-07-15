pub mod types;
pub mod errors;
pub mod config;
pub mod traits;
pub mod event_bus;
pub mod currency;

// ── Phase 0: New Platform Modules ───────────────────────────────
pub mod settings;
pub mod context;
pub mod event_store;
pub mod job_queue;
pub mod process;
pub mod reactive;
pub mod plugin;
pub mod tenant_guard;
pub mod pagination;
pub mod period_gate;
