//! Report Use Cases — application-layer orchestration.
//!
//! These use cases coordinate domain logic, ports, and cross-cutting
//! concerns (authorization, audit) without coupling to infrastructure.

pub mod generate_report;
pub mod manage_template;
pub mod list_reports;

