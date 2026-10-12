use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub org_unit_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub uploaded_by: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub calendar_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub tags: Option<String>,
    pub visibility: String,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub checksum_sha256: Option<String>,
    pub storage_provider: String,
    pub file_path: String,
    pub storage_key: Option<String>,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub is_archived: bool,
    pub archived_at: Option<DateTimeWithTimeZone>,
    pub archived_by: Option<Uuid>,
    pub archive_reason: Option<String>,
    pub is_legal_hold: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub deleted_by: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
