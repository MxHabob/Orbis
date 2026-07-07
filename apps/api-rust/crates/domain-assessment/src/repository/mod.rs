//! Assessment repository — CQRS split for grades, attendance, and behavior.

pub mod commands;
pub mod queries;

use sea_orm::DatabaseConnection;

/// Write repository for assessment entities.
#[derive(Clone)]
pub struct AssessmentCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for assessment entities.
#[derive(Clone)]
pub struct AssessmentQueryRepo {
    pub db: DatabaseConnection,
}

impl AssessmentCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl AssessmentQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
