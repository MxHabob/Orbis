use sea_orm::Schema;
use sea_orm_migration::prelude::*;

use crate::models::{
    academic_calendar, academic_calendar_period, academic_period_type, course, course_enrollment,
    course_offering,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        let tables = [
            schema.create_table_from_entity(academic_calendar::Entity),
            schema.create_table_from_entity(academic_period_type::Entity),
            schema.create_table_from_entity(academic_calendar_period::Entity),
            schema.create_table_from_entity(course::Entity),
            schema.create_table_from_entity(course_offering::Entity),
            schema.create_table_from_entity(course_enrollment::Entity),
        ];

        for mut statement in tables {
            manager
                .create_table(statement.if_not_exists().to_owned())
                .await?;
        }

        for (name, table, columns) in [
            (
                "uq_academic_calendars_org_code",
                "academic_calendars",
                vec!["org_id", "code"],
            ),
            (
                "uq_academic_period_types_org_code",
                "academic_period_types",
                vec!["org_id", "code"],
            ),
            ("uq_courses_org_code", "courses", vec!["org_id", "code"]),
            (
                "uq_course_enrollments_attempt",
                "course_enrollments",
                vec!["org_id", "offering_id", "student_id", "attempt_number"],
            ),
        ] {
            let mut index = Index::create();
            index.name(name).table(Alias::new(table));
            for column in columns {
                index.col(Alias::new(column));
            }
            manager
                .create_index(index.unique().if_not_exists().to_owned())
                .await?;
        }

        for (name, table, column) in [
            ("idx_academic_calendars_org", "academic_calendars", "org_id"),
            (
                "idx_academic_calendar_periods_calendar",
                "academic_calendar_periods",
                "calendar_id",
            ),
            (
                "idx_academic_calendar_periods_parent",
                "academic_calendar_periods",
                "parent_id",
            ),
            ("idx_courses_org", "courses", "org_id"),
            (
                "idx_course_offerings_course_period",
                "course_offerings",
                "course_id",
            ),
            (
                "idx_course_enrollments_student",
                "course_enrollments",
                "student_id",
            ),
        ] {
            manager
                .create_index(
                    Index::create()
                        .name(name)
                        .table(Alias::new(table))
                        .col(Alias::new(column))
                        .if_not_exists()
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in [
            "course_enrollments",
            "course_offerings",
            "courses",
            "academic_calendar_periods",
            "academic_period_types",
            "academic_calendars",
        ] {
            manager
                .drop_table(
                    Table::drop()
                        .table(Alias::new(table))
                        .if_exists()
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Database, DbBackend, Schema, sea_query::PostgresQueryBuilder};

    #[test]
    fn generates_postgres_schema_for_academic_foundations() {
        let schema = Schema::new(DbBackend::Postgres);
        let statement = schema.create_table_from_entity(academic_calendar_period::Entity);
        let sql = statement.to_string(PostgresQueryBuilder);

        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("academic_calendar_periods"));
        assert!(sql.contains("calendar_id"));
        assert!(sql.contains("parent_id"));
    }

    #[tokio::test]
    async fn creates_academic_foundation_tables_on_sqlite() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("SQLite test database should connect");
        let manager = SchemaManager::new(&db);

        for table_name in ["organizations", "students"] {
            manager
                .create_table(
                    Table::create()
                        .table(Alias::new(table_name))
                        .col(
                            ColumnDef::new(Alias::new("id"))
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .to_owned(),
                )
                .await
                .expect("base table should be created");
        }

        Migration
            .up(&manager)
            .await
            .expect("academic foundation migration should succeed");
        Migration
            .up(&manager)
            .await
            .expect("academic foundation migration should be idempotent");

        for table_name in [
            "academic_calendars",
            "academic_period_types",
            "academic_calendar_periods",
            "courses",
            "course_offerings",
            "course_enrollments",
        ] {
            assert!(
                manager
                    .has_table(table_name)
                    .await
                    .expect("table lookup should succeed"),
                "expected {table_name} to exist"
            );
        }
    }
}
