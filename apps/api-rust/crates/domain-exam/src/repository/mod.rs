//! Exam repository — CQRS split for sessions, schedules, proctoring, seating/secret numbers.

pub mod commands;
pub mod queries;

use sea_orm::DatabaseConnection;

/// Write repository for exam entities.
#[derive(Clone)]
pub struct ExamCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for exam entities.
#[derive(Clone)]
pub struct ExamQueryRepo {
    pub db: DatabaseConnection,
}

impl ExamCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl ExamQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
