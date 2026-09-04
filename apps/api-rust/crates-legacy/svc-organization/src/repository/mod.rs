//! Organization domain repository — CQRS split.

pub mod commands;
pub mod queries;
pub mod academic_year;

pub use academic_year::AcademicYearRepo;


use sea_orm::DatabaseConnection;

/// Write repository for lifecycle entities.
#[derive(Clone)]
pub struct OrganizationCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for lifecycle entities.
#[derive(Clone)]
pub struct OrganizationQueryRepo {
    pub db: DatabaseConnection,
}

impl OrganizationCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl OrganizationQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

