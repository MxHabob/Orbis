//! IAM domain services.
//!
//! Business logic for authentication, authorization, and user management.
//! Services use `AppError` from shared-kernel (not `String`).

pub mod auth;
pub mod authz;
pub mod user_management;
