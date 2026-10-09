pub use sea_orm_migration::prelude::*;

pub mod m20240101_000001_initial_schema;
pub mod m20240102_000001_backfill_admin_permissions;
pub mod m20240103_000001_normalized_schema;
pub mod m20240104_000001_employee_groups_schema;
pub mod m20240105_000001_registration_schema;
pub mod m20240106_000001_room_floor_name_ar;
pub mod m20240107_000001_academic_period_kind_assessment_enrichment;
pub mod m20240108_000001_storage_and_archive_schema;
pub mod m20240109_000001_organization_format_preferences;
pub mod m20240110_000001_account_name_ar_and_subtype;
pub mod m20240111_000001_backfill_standard_subject_arabic_names;
pub mod m20240112_000001_license_quota_locks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_initial_schema::Migration),
            Box::new(m20240102_000001_backfill_admin_permissions::Migration),
            Box::new(m20240103_000001_normalized_schema::Migration),
            Box::new(m20240104_000001_employee_groups_schema::Migration),
            Box::new(m20240105_000001_registration_schema::Migration),
            Box::new(m20240106_000001_room_floor_name_ar::Migration),
            Box::new(m20240107_000001_academic_period_kind_assessment_enrichment::Migration),
            Box::new(m20240108_000001_storage_and_archive_schema::Migration),
            Box::new(m20240109_000001_organization_format_preferences::Migration),
            Box::new(m20240110_000001_account_name_ar_and_subtype::Migration),
            Box::new(m20240111_000001_backfill_standard_subject_arabic_names::Migration),
            Box::new(m20240112_000001_license_quota_locks::Migration),
        ]
    }
}
