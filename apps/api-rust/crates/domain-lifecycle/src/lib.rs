//! Orbis Lifecycle Domain — Organization & Academic Year Management.
//!
//! This crate owns:
//! - **Organization**: Tenant configuration, branding, limits
//! - **Academic Cycles**: Years, semesters, quarters (hierarchical)
//! - **Calendar Events**: Academic milestones with execution modes
//! - **Policies**: Academic grading defaults, promotion rules, overrides
//!
//! # Independence
//!
//! No dependency on other domain crates. Users reference organizations
//! via `org_id` (String). This crate is the source of truth for
//! organization and cycle lifecycle.

pub mod entities;
pub mod repository;
pub mod services;
pub mod params;

