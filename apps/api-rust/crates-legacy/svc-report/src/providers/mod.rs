//! Report Data Providers — domain-specific query functions that produce `ReportDataset`.
//!
//! Each provider is a standalone async function that:
//! 1. Receives a `&DatabaseConnection`, `org_id`, and validated `ReportRequest`
//! 2. Builds efficient SQL queries with proper joins and aggregations
//! 3. Returns a fully populated `ReportDataset`
//!
//! Providers are registered with the `ReportRegistry` at application startup.

pub mod finance;
pub mod academic;
pub mod exam;
pub mod attendance;
