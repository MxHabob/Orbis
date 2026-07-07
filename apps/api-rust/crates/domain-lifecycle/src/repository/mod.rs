//! Lifecycle domain repository — CQRS split.

pub mod commands;
pub mod queries;
pub mod academic_year;

pub use academic_year::AcademicYearRepo;


use sea_orm::DatabaseConnection;

/// Write repository for lifecycle entities.
#[derive(Clone)]
pub struct LifecycleCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for lifecycle entities.
#[derive(Clone)]
pub struct LifecycleQueryRepo {
    pub db: DatabaseConnection,
}

impl LifecycleCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl LifecycleQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
