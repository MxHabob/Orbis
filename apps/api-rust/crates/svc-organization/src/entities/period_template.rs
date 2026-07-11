use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Period template — predefined academic structure for quick year setup.
///
/// The `structure` field is a JSON array describing the hierarchy of periods
/// to create when a new academic year is initialized from this template.
///
/// Example structure for Saudi K-12 (4 quarters):
/// ```json
/// [
///   {
///     "name": "الفصل الأول", "type": "semester", "weight": 50,
///     "offset_start_days": 0, "offset_end_days": 90,
///     "children": [
///       { "name": "الفترة الأولى", "type": "quarter", "weight": 25 },
///       { "name": "الفترة الثانية", "type": "quarter", "weight": 25 }
///     ]
///   },
///   { "name": "الفصل الثاني", "type": "semester", "weight": 50, ... }
/// ]
/// ```
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "period_templates")]
#[schema(as = PeriodTemplate)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub institution_type: String, // school, university, institute, custom
    pub structure: String,        // JSON array of period definitions
    pub is_default: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organization,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
