//! Assessment domain operations — the single source of truth.
//!
//! Covers: Grades, Attendance, Behavior, Assessments.

pub mod grade;
pub mod attendance;
pub mod behavior;

pub mod assessment_engine;
pub mod at_risk_predictor;
pub mod calc_engine;
pub mod grade_mapper;
pub mod snapshot_generator;
pub mod snapshot_job;
