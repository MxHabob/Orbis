//! Orbis Academic Domain — Core academic structures.
//!
//! This crate owns:
//! - **Classes**: Classrooms with capacity, level, teacher assignment
//! - **Subjects**: Curriculum subjects with hours/weights
//! - **Class Subjects**: Subject-to-class mapping with teacher
//! - **Org Units**: Departments, branches, divisions
//! - **Enrollments**: Student enrollment lifecycle
//! - **Student Guardians**: Parent/guardian relationships
//! - **Documents**: Student and staff documents

pub mod entities;
pub mod repository;

pub mod params;

pub mod contracts;
pub mod ops;
pub mod validation;


