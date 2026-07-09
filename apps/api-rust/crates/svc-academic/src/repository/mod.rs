//! Academic domain repository — CQRS split.

pub mod commands;
pub mod queries;

use sea_orm::DatabaseConnection;

/// Write repository for academic entities.
#[derive(Clone)]
pub struct AcademicCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for academic entities.
#[derive(Clone)]
pub struct AcademicQueryRepo {
    pub db: DatabaseConnection,
}

impl AcademicCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl AcademicQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
