//! IAM domain repository — CQRS split into command and query repos.
//!
//! - `IamCommandRepo`: Write operations (create, update, delete)
//! - `IamQueryRepo`: Read operations (get, list, search)
//!
//! Both hold a `DatabaseConnection` reference. The split enables:
//! - Read queries to use optimized SQL (joins, projections)
//! - Write operations to enforce domain invariants

pub mod commands;
pub mod queries;

use sea_orm::DatabaseConnection;

/// Write repository for IAM entities.
#[derive(Clone)]
pub struct IamCommandRepo {
    pub db: DatabaseConnection,
}

/// Read repository for IAM entities.
#[derive(Clone)]
pub struct IamQueryRepo {
    pub db: DatabaseConnection,
}

impl IamCommandRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl IamQueryRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
