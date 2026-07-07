//! Scheduling repository — CQRS split for slots, scheduled lessons, rooms, availability, and solve jobs.

pub mod commands;
pub mod queries;

use sea_orm::DatabaseConnection;

/// Write repository for scheduling entities.
#[derive(Clone)]
pub struct SchedulingCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for scheduling entities.
#[derive(Clone)]
pub struct SchedulingQueryRepo {
    pub db: DatabaseConnection,
}

impl SchedulingCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl SchedulingQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
