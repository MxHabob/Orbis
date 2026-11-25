use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm_migration::sea_query::Expr;

use crate::{
    models::report::report_template,
    repositories::template_catalog::get_master_template_definitions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for (code, name, name_ar, _, _, size, orientation, doc) in get_master_template_definitions()
        {
            report_template::Entity::update_many()
                .filter(report_template::Column::Code.eq(code))
                .filter(report_template::Column::IsSystem.eq(true))
                .filter(report_template::Column::Status.eq("published"))
                .filter(report_template::Column::Version.eq(1))
                .filter(report_template::Column::PublishedVersion.eq(1))
                .filter(
                    Expr::col(report_template::Column::UpdatedAt)
                        .equals(report_template::Column::CreatedAt),
                )
                .filter(report_template::Column::DeletedAt.is_null())
                .col_expr(report_template::Column::Name, Expr::value(name))
                .col_expr(report_template::Column::NameAr, Expr::value(name_ar))
                .col_expr(report_template::Column::PageSize, Expr::value(size))
                .col_expr(
                    report_template::Column::Orientation,
                    Expr::value(orientation),
                )
                .col_expr(
                    report_template::Column::MarginsJson,
                    Expr::value(doc["meta"]["pageSetup"]["margins"].to_string()),
                )
                .col_expr(
                    report_template::Column::DocumentJson,
                    Expr::value(doc.to_string()),
                )
                .exec(manager.get_connection())
                .await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Keep published documents intact when rolling back the database migration.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm_migration::sea_orm::{ConnectionTrait, Database, Statement};

    #[tokio::test]
    async fn refresh_updates_stock_rows_and_preserves_edited_templates() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE report_templates (
                code TEXT, name TEXT, name_ar TEXT, page_size TEXT, orientation TEXT,
                margins_json TEXT, document_json TEXT, is_system BOOLEAN, is_default BOOLEAN,
                status TEXT, version INTEGER, published_version INTEGER, deleted_at TEXT,
                created_at TEXT, updated_at TEXT
            );
            INSERT INTO report_templates
                (code, name, page_size, is_system, is_default, status, version,
                 published_version, created_at, updated_at)
            VALUES
                ('finance.payment-receipt', 'Stock', 'A4', TRUE, FALSE, 'published', 1, 1, '1', '1'),
                ('finance.payment-receipt', 'Edited', 'A4', TRUE, TRUE, 'published', 1, 1, '1', '2');",
        )
        .await
        .unwrap();
        Migration.up(&SchemaManager::new(&db)).await.unwrap();
        let rows = db
            .query_all(Statement::from_string(
                db.get_database_backend(),
                "SELECT name, page_size, is_default FROM report_templates ORDER BY is_default",
            ))
            .await
            .unwrap();
        assert_eq!(rows[0].try_get_by_index::<String>(1).unwrap(), "A5");
        assert!(!rows[0].try_get_by_index::<bool>(2).unwrap());
        assert_eq!(rows[1].try_get_by_index::<String>(0).unwrap(), "Edited");
        assert_eq!(rows[1].try_get_by_index::<String>(1).unwrap(), "A4");
        assert!(rows[1].try_get_by_index::<bool>(2).unwrap());
    }
}
