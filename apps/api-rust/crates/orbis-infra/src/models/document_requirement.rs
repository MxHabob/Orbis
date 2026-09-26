use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use orbis_domain::entities::document_requirement::{
    DocumentRequirement as DomainDocumentRequirement,
    RequirementTargetType, DocumentCondition,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "document_requirements")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub target_type: String, // "global", "stage", "grade_level", "program"
    pub target_id: Option<Uuid>,
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    pub is_mandatory: bool,
    pub blocks_enrollment: bool,
    pub condition: String, // "all_students", "non_national_only", etc.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for DomainDocumentRequirement {
    fn from(m: Model) -> Self {
        let target_type = match m.target_type.as_str() {
            "stage" => RequirementTargetType::Stage,
            "grade_level" => RequirementTargetType::GradeLevel,
            "program" => RequirementTargetType::Program,
            _ => RequirementTargetType::Global,
        };

        let condition = match m.condition.as_str() {
            "non_national_only" => DocumentCondition::NonNationalOnly,
            "transfer_students_only" => DocumentCondition::TransferStudentsOnly,
            "special_needs_only" => DocumentCondition::SpecialNeedsOnly,
            _ => DocumentCondition::AllStudents,
        };

        Self {
            id: m.id,
            org_id: m.org_id,
            target_type,
            target_id: m.target_id,
            code: m.code,
            name_ar: m.name_ar,
            name_en: m.name_en,
            is_mandatory: m.is_mandatory,
            blocks_enrollment: m.blocks_enrollment,
            condition,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

impl From<&DomainDocumentRequirement> for ActiveModel {
    fn from(d: &DomainDocumentRequirement) -> Self {
        Self {
            id: Set(d.id),
            org_id: Set(d.org_id),
            target_type: Set(d.target_type.to_string()),
            target_id: Set(d.target_id),
            code: Set(d.code.clone()),
            name_ar: Set(d.name_ar.clone()),
            name_en: Set(d.name_en.clone()),
            is_mandatory: Set(d.is_mandatory),
            blocks_enrollment: Set(d.blocks_enrollment),
            condition: Set(d.condition.to_string()),
            created_at: Set(d.created_at),
            updated_at: Set(d.updated_at),
        }
    }
}
