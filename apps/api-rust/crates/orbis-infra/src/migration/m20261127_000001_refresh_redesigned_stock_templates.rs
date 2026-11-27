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
        Ok(())
    }
}
