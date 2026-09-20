pub use sea_orm_migration::prelude::*;

pub mod m20240101_000001_initial_schema;
pub mod m20240102_000001_backfill_admin_permissions;
pub mod m20240103_000001_normalized_schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_initial_schema::Migration),
            Box::new(m20240102_000001_backfill_admin_permissions::Migration),
            Box::new(m20240103_000001_normalized_schema::Migration),
        ]
    }
}
