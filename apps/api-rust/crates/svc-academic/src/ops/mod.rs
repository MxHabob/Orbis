//! Academic operations — the single source of truth for all academic business logic.
//!
//! Every function takes a `ServiceContext` and enforces:
//!   ✓ Permission checks   ✓ Audit logging   ✓ Domain events   ✓ Validation
//!
//! Consumers:
//!   - Tauri Desktop Commands
//!   - API Gateway HTTP Handlers
//!   - AI Agent Action Executor
//!   - Autopilot Engine

pub mod class;
pub mod subject;
pub mod enrollment;
pub mod class_subject;
pub mod guardian;
pub mod document;
pub mod notification;
pub mod scope;
