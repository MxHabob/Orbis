//! Orbis Scheduling Domain — Timetable solver, time slots, room management, and teacher availability.
//!
//! This crate owns:
//! - **Time Slots**: Period settings for lessons
//! - **Scheduled Lessons**: The actual generated schedule grid
//! - **Rooms**: Classrooms and labs where lessons occur
//! - **Teacher Availability**: Weekly slot preferences
//! - **Solve Jobs**: Timetable auto-scheduling solver tracking

pub mod entities;
pub mod params;
pub mod repository;
pub mod services;

pub mod contracts;
pub mod ops;
pub mod validation;


