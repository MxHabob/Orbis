//! Finance repository — CQRS split for ledger, templates, student fees, payments, and treasuries.

pub mod commands;
pub mod queries;

use sea_orm::DatabaseConnection;

/// Write repository for finance entities.
#[derive(Clone)]
pub struct FinanceCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for finance entities.
#[derive(Clone)]
pub struct FinanceQueryRepo {
    pub db: DatabaseConnection,
}

impl FinanceCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl FinanceQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
