use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Documents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Documents::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Documents::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Documents::UploadedBy).uuid())
                    .col(ColumnDef::new(Documents::EntityType).string())
                    .col(ColumnDef::new(Documents::EntityId).uuid())
                    .col(ColumnDef::new(Documents::Name).string().not_null())
                    .col(ColumnDef::new(Documents::Description).string())
                    .col(ColumnDef::new(Documents::Category).string().not_null())
                    .col(ColumnDef::new(Documents::AssetType).string().not_null())
                    .col(ColumnDef::new(Documents::SchoolYear).string())
                    .col(ColumnDef::new(Documents::Term).string())
                    .col(ColumnDef::new(Documents::Tags).string())
                    .col(ColumnDef::new(Documents::Visibility).string().not_null().default("public"))
                    .col(ColumnDef::new(Documents::Size).integer())
                    .col(ColumnDef::new(Documents::MimeType).string())
                    .col(ColumnDef::new(Documents::Extension).string())
                    .col(ColumnDef::new(Documents::StorageProvider).string().not_null())
                    .col(ColumnDef::new(Documents::Path).string().not_null())
                    .col(ColumnDef::new(Documents::Url).string())
                    .col(ColumnDef::new(Documents::ThumbnailPath).string())
                    .col(ColumnDef::new(Documents::PreviewPath).string())
                    .col(ColumnDef::new(Documents::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Documents::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Documents::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Documents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Documents {
    Table,
    Id,
    OrgId,
    UploadedBy,
    EntityType,
    EntityId,
    Name,
    Description,
    Category,
    AssetType,
    SchoolYear,
    Term,
    Tags,
    Visibility,
    Size,
    MimeType,
    Extension,
    StorageProvider,
    Path,
    Url,
    ThumbnailPath,
    PreviewPath,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
