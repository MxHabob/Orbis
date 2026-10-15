pub use sea_orm_migration::prelude::*;

mod m20260101_000001_unified_schema;
mod m20261015_000001_assessment_provenance;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_unified_schema::Migration),
            Box::new(m20261015_000001_assessment_provenance::Migration),
        ]
    }
}
