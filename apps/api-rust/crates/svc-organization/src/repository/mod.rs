//! Lifeperiod domain repository — CQRS split.

pub mod commands;
pub mod queries;
pub mod academic_year;

pub use academic_year::AcademicYearRepo;


use sea_orm::DatabaseConnection;

/// Write repository for lifecycle entities.
#[derive(Clone)]
pub struct LifeperiodCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for lifecycle entities.
#[derive(Clone)]
pub struct LifeperiodQueryRepo {
    pub db: DatabaseConnection,
}

impl LifeperiodCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl LifeperiodQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
