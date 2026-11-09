use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "notifications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub recipient_user_id: Uuid,
    pub category: String,
    pub priority: String,
    pub title: String,
    pub title_ar: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub body_ar: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub action_route: Option<String>,
    pub source_event: String,
    #[sea_orm(unique)]
    pub correlation_id: String,
    pub schema_version: i32,
    pub is_read: bool,
    pub read_at: Option<DateTimeUtc>,
    pub is_dismissed: bool,
    pub dismissed_at: Option<DateTimeUtc>,
    pub expires_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RecipientUserId",
        to = "super::user::Column::Id"
    )]
    Recipient,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipient.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
