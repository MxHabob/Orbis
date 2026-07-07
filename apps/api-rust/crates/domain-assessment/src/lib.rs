//! Orbis Assessment Domain — Grades, Attendance, Behavior.
//!
//! This crate owns all student performance tracking:
//! - **Grades**: Entry, history, analytics
//! - **Assessments**: Test/quiz/exam definitions
//! - **Attendance**: Daily, summary, analytics
//! - **Behavior**: Incident records, patterns

pub mod entities;
pub mod params;
pub mod repository;
pub mod services;
