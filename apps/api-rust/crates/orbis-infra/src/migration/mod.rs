pub use sea_orm_migration::prelude::*;

mod m20260101_000001_unified_schema;
mod m20261015_000001_assessment_provenance;
mod m20261017_000001_exam_session_details;
mod m20261028_000001_notifications;
mod m20261029_000001_lesson_attendance;
mod m20261030_000001_currency_symbol_position;
mod m20261031_000001_exam_hall_flag;
mod m20261104_000001_grade_absence_flag;
mod m20261105_000001_unique_student_root;
mod m20261124_000001_grade_unique_and_bulk_rollback;
mod m20261124_000002_grade_component_code;

mod m20261125_000001_exam_proctor_uniqueness;
mod m20261125_000002_autopilot_idempotency;
mod m20261125_000003_override_approval_policy;
mod m20261125_000004_refresh_stock_templates;
mod m20261125_000005_finance_ledger_indexes;
mod m20261127_000001_refresh_redesigned_stock_templates;
mod m20261127_000002_remove_monthly_exam_periods;

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
            Box::new(m20261031_000001_exam_hall_flag::Migration),
            Box::new(m20261104_000001_grade_absence_flag::Migration),
            Box::new(m20261105_000001_unique_student_root::Migration),
            Box::new(m20261124_000001_grade_unique_and_bulk_rollback::Migration),
            Box::new(m20261124_000002_grade_component_code::Migration),
            Box::new(m20261125_000001_exam_proctor_uniqueness::Migration),
            Box::new(m20261125_000002_autopilot_idempotency::Migration),
            Box::new(m20261125_000003_override_approval_policy::Migration),
            Box::new(m20261125_000004_refresh_stock_templates::Migration),
            Box::new(m20261125_000005_finance_ledger_indexes::Migration),
            Box::new(m20261127_000001_refresh_redesigned_stock_templates::Migration),
            Box::new(m20261127_000002_remove_monthly_exam_periods::Migration),
        ]
    }
}
