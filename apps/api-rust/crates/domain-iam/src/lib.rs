//! Orbis IAM Domain — Identity & Access Management.
//!
//! This crate owns:
//! - **Authentication**: Login, JWT tokens, sessions, MFA
//! - **Authorization**: Permission checks, role-based access control
//! - **User Management**: CRUD, profiles, account settings
//! - **Permission System**: Roles, permissions, grants
//!
//! # Independence
//!
//! This crate has **no dependencies on other domain crates**.
//! Other domains depend on `domain-iam` for user/permission types.

pub mod entities;
pub mod services;
pub mod repository;
