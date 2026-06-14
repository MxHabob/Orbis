use sea_orm::DatabaseConnection;

pub mod user;
pub mod class;
pub mod behavior;
pub mod notification;

#[derive(Clone)]
pub struct Repository {
    pub db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
