pub use sea_orm_migration::prelude::*;

mod m20260101_000001_unified_schema;
mod m20261015_000001_assessment_provenance;
mod m20261017_000001_exam_session_details;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_unified_schema::Migration),
            Box::new(m20261015_000001_assessment_provenance::Migration),
            Box::new(m20261017_000001_exam_session_details::Migration),
        ]
    }
}
