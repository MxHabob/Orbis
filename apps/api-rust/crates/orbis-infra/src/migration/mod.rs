pub use sea_orm_migration::prelude::*;

mod m20260101_000001_unified_schema;
mod m20261015_000001_assessment_provenance;
mod m20261017_000001_exam_session_details;
mod m20261028_000001_notifications;
mod m20261029_000001_lesson_attendance;
mod m20261030_000001_currency_symbol_position;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_unified_schema::Migration),
            Box::new(m20261015_000001_assessment_provenance::Migration),
            Box::new(m20261017_000001_exam_session_details::Migration),
            Box::new(m20261028_000001_notifications::Migration),
            Box::new(m20261029_000001_lesson_attendance::Migration),
            Box::new(m20261030_000001_currency_symbol_position::Migration),
        ]
    }
}
