use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "enrollments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    #[sea_orm(unique)]
    pub student_number: String,
    pub status: String,
    pub study_mode: Option<String>,
    pub is_attendance_exempt: Option<bool>,
    pub is_grade_exempt: Option<bool>,
    pub enrollment_date: DateTimeWithTimeZone,
    pub enrollment_type: Option<String>,
    pub withdrawal_date: Option<DateTimeWithTimeZone>,
    pub withdrawal_reason: Option<String>,
    pub meta: Option<Json>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::StudentId",
        to = "super::student::Column::Id"
    )]
    Student,
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
