pub mod report_definition {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "report_definitions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub org_id: Uuid,
        pub code: String,
        pub name: String,
        pub name_ar: String,
        pub domain: String,
        pub category: String,
        pub description: Option<String>,
        pub description_ar: Option<String>,
        #[sea_orm(column_type = "Text")]
        pub parameters_json: String,
        #[sea_orm(column_type = "Text")]
        pub required_permissions: String,
        #[sea_orm(column_type = "Text")]
        pub supported_formats: String,
        pub data_provider_key: String,
        pub is_system: bool,
        pub is_active: bool,
        pub sort_order: i32,
        pub version: i32,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod report_template {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "report_templates")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub org_id: Uuid,
        pub code: Option<String>,
        pub name: String,
        pub name_ar: Option<String>,
        pub description: Option<String>,
        pub description_ar: Option<String>,
        pub document_type: String,
        pub entity_type: String,
        pub master_template_id: Option<Uuid>,
        pub status: String,
        pub version: i32,
        pub published_version: Option<i32>,
        pub published_at: Option<DateTimeWithTimeZone>,
        pub published_by: Option<Uuid>,
        pub page_size: String,
        pub page_width_mm: Option<f64>,
        pub page_height_mm: Option<f64>,
        pub orientation: String,
        #[sea_orm(column_type = "Text")]
        pub margins_json: String,
        #[sea_orm(column_type = "Text")]
        pub document_json: String,
        pub locale: String,
        pub tags: Option<String>,
        pub is_system: bool,
        pub is_default: bool,
        pub created_by: Uuid,
        pub deleted_at: Option<DateTimeWithTimeZone>,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod report_template_version {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "report_template_versions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub template_id: Uuid,
        pub version: i32,
        #[sea_orm(column_type = "Text")]
        pub document_json: String,
        pub page_size: Option<String>,
        pub page_width_mm: Option<f64>,
        pub page_height_mm: Option<f64>,
        pub orientation: Option<String>,
        #[sea_orm(column_type = "Text", nullable)]
        pub margins_json: Option<String>,
        pub change_summary: Option<String>,
        pub created_by: Uuid,
        pub created_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod report_template_binding {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "report_template_bindings")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub report_definition_id: Uuid,
        pub template_id: Uuid,
        pub locale: String,
        pub format: String,
        pub is_default: bool,
        pub created_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod report_job {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "report_jobs")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub org_id: Uuid,
        pub report_definition_id: Uuid,
        pub template_id: Option<Uuid>,
        pub requested_by: Uuid,
        pub report_code: String,
        pub parameters: Json,
        pub output_format: String,
        pub locale: String,
        pub status: String,
        pub output_path: Option<String>,
        pub output_size_bytes: Option<i64>,
        #[sea_orm(column_type = "Text", nullable)]
        pub error_message: Option<String>,
        pub created_at: DateTimeWithTimeZone,
        pub started_at: Option<DateTimeWithTimeZone>,
        pub completed_at: Option<DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod report_audit_log {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "report_audit_log")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub org_id: Uuid,
        pub actor_id: Uuid,
        pub action: String,
        pub entity_type: String,
        pub entity_id: Uuid,
        pub details: Option<Json>,
        pub created_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
