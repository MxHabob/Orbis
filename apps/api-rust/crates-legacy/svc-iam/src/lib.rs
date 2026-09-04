//! Orbis IAM Domain — Identity & Access Management.
//!
//! This crate owns:
//! - **Authentication**: Login, JWT tokens, sessions, MFA
//! - **Authorization**: Permission checks, role-based access control
//! - **User Management**: CRUD, profiles, account settings
//! - **Permission System**: Roles, permissions, grants
//!
//! # Platform Trait Implementations
//!
//! - `PermissionChecker` → `IamPermissionChecker` (wraps AuthzService)
//! - `AuditLogger`       → `DbAuditLogger` (writes to audit_log table)

pub mod entities;
pub mod services;
pub mod repository;
pub mod params;

// ── Phase 2: New Microservice Modules ──────────────────────────
pub mod contracts;
pub mod ops;
pub mod validation;
