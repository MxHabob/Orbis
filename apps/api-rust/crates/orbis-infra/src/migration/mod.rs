pub use sea_orm_migration::prelude::*;

pub mod m20240901_000001_create_org_table;
pub mod m20240901_000002_create_user_table;
pub mod m20240901_000003_create_person_table;
pub mod m20240901_000004_create_session_table;
pub mod m20240901_000005_create_org_units_table;
pub mod m20240901_000006_create_period_templates_table;
pub mod m20240901_000007_create_academic_years_table;
pub mod m20240901_000008_create_academic_periods_table;
pub mod m20240901_000009_create_subjects_table;
pub mod m20240901_000010_create_classes_table;
pub mod m20240901_000011_create_class_subjects_table;
pub mod m20240901_000012_create_enrollments_table;
pub mod m20240901_000013_create_student_guardians_table;
pub mod m20240901_000014_create_rooms_table;
pub mod m20240901_000015_create_time_slots_table;
pub mod m20240901_000016_create_scheduled_lessons_table;
pub mod m20240901_000017_create_assessment_schemes_table;
pub mod m20240901_000018_create_grades_table;
pub mod m20240901_000019_create_attendances_table;
pub mod m20240901_000020_create_behavior_records_table;
pub mod m20240901_000021_create_exam_sessions_table;
pub mod m20240901_000022_create_exam_schedules_table;
pub mod m20240901_000023_create_seating_numbers_table;
pub mod m20240901_000024_create_secret_numbers_table;
pub mod m20240901_000025_create_exam_proctors_table;
pub mod m20240901_000026_create_accounts_table;
pub mod m20240901_000027_create_journal_entries_table;
pub mod m20240901_000028_create_fee_templates_table;
pub mod m20240901_000029_create_student_fees_table;
pub mod m20240901_000030_create_payments_table;
pub mod m20240901_000031_create_documents_table;
pub mod m20240901_000032_create_integrations_tables;
pub mod m20240901_000033_create_ai_tables;
pub mod m20240901_000034_create_workflow_tables;
pub mod m20240901_000035_create_report_tables;
pub mod m20240901_000036_create_iam_tables;
pub mod m20240901_000037_create_currencies_tables;
pub mod m20240901_000040_rebuild_report_tables;
pub mod m20240901_000041_create_identity_and_mfa_tables;
pub mod m20240901_000042_create_students_table;
pub mod m20240901_000043_create_academic_override_requests_table;
pub mod m20240901_000044_create_course_prerequisites_table;
pub mod m20240901_000045_create_gpa_records_table;
pub mod m20240901_000046_create_promotion_tables;
pub mod m20240901_000047_create_journal_entry_lines_table;
pub mod m20240901_000048_create_discount_templates_table;
pub mod m20240901_000049_update_enrollments_table;
pub mod m20240901_000050_extend_person_profile;
pub mod m20240901_000051_align_all_schema_columns;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240901_000001_create_org_table::Migration),
            Box::new(m20240901_000002_create_user_table::Migration),
            Box::new(m20240901_000003_create_person_table::Migration),
            Box::new(m20240901_000004_create_session_table::Migration),
            Box::new(m20240901_000005_create_org_units_table::Migration),
            Box::new(m20240901_000006_create_period_templates_table::Migration),
            Box::new(m20240901_000007_create_academic_years_table::Migration),
            Box::new(m20240901_000008_create_academic_periods_table::Migration),
            Box::new(m20240901_000009_create_subjects_table::Migration),
            Box::new(m20240901_000010_create_classes_table::Migration),
            Box::new(m20240901_000011_create_class_subjects_table::Migration),
            Box::new(m20240901_000012_create_enrollments_table::Migration),
            Box::new(m20240901_000013_create_student_guardians_table::Migration),
            Box::new(m20240901_000014_create_rooms_table::Migration),
            Box::new(m20240901_000015_create_time_slots_table::Migration),
            Box::new(m20240901_000016_create_scheduled_lessons_table::Migration),
            Box::new(m20240901_000017_create_assessment_schemes_table::Migration),
            Box::new(m20240901_000018_create_grades_table::Migration),
            Box::new(m20240901_000019_create_attendances_table::Migration),
            Box::new(m20240901_000020_create_behavior_records_table::Migration),
            Box::new(m20240901_000021_create_exam_sessions_table::Migration),
            Box::new(m20240901_000022_create_exam_schedules_table::Migration),
            Box::new(m20240901_000023_create_seating_numbers_table::Migration),
            Box::new(m20240901_000024_create_secret_numbers_table::Migration),
            Box::new(m20240901_000025_create_exam_proctors_table::Migration),
            Box::new(m20240901_000026_create_accounts_table::Migration),
            Box::new(m20240901_000027_create_journal_entries_table::Migration),
            Box::new(m20240901_000028_create_fee_templates_table::Migration),
            Box::new(m20240901_000029_create_student_fees_table::Migration),
            Box::new(m20240901_000030_create_payments_table::Migration),
            Box::new(m20240901_000031_create_documents_table::Migration),
            Box::new(m20240901_000032_create_integrations_tables::Migration),
            Box::new(m20240901_000033_create_ai_tables::Migration),
            Box::new(m20240901_000034_create_workflow_tables::Migration),
            Box::new(m20240901_000035_create_report_tables::Migration),
            Box::new(m20240901_000036_create_iam_tables::Migration),
            Box::new(m20240901_000037_create_currencies_tables::Migration),
            Box::new(m20240901_000040_rebuild_report_tables::Migration),
            Box::new(m20240901_000041_create_identity_and_mfa_tables::Migration),
            Box::new(m20240901_000042_create_students_table::Migration),
            Box::new(m20240901_000043_create_academic_override_requests_table::Migration),
            Box::new(m20240901_000044_create_course_prerequisites_table::Migration),
            Box::new(m20240901_000045_create_gpa_records_table::Migration),
            Box::new(m20240901_000046_create_promotion_tables::Migration),
            Box::new(m20240901_000047_create_journal_entry_lines_table::Migration),
            Box::new(m20240901_000048_create_discount_templates_table::Migration),
            Box::new(m20240901_000049_update_enrollments_table::Migration),
            Box::new(m20240901_000050_extend_person_profile::Migration),
            Box::new(m20240901_000051_align_all_schema_columns::Migration),
        ]
    }
}
