//! Orbis Lifeperiod Domain — Organization & Academic Year Management.
//!
//! This crate owns:
//! - **Organization**: Tenant configuration, branding, limits
//! - **Academic Periods**: Years, semesters, quarters (hierarchical)
//! - **Calendar Events**: Academic milestones with execution modes
//! - **Policies**: Academic grading defaults, promotion rules, overrides
//!
//! # Platform Trait Implementations
//!
//! This crate provides concrete implementations for:
//! - `SettingsProvider` → `OrgSettingsProvider` (reads org.settings JSON)
//! - `PolicyEngine`     → `OrgPolicyEngine` (reads org_academic_policies)

pub mod entities;
pub mod repository;
pub mod services;
pub mod params;

// ── Phase 1: New Microservice Modules ──────────────────────────
pub mod contracts;
pub mod ops;
