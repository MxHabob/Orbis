//! Semantic Resolver — entity context resolution from schema introspection.
//!
//! Built at startup from hardcoded entity metadata (not from DB).
//! Provides AI with rich context about entities and their relationships.
//!
//! ADR: `entity_metadata` table was eliminated — metadata is built in-memory
//! from schema introspection + hardcoded enrichments.

use std::collections::HashMap;
use serde::Serialize;
use tracing::info;

use shared_kernel::errors::AppError;

/// Schema information for a domain entity.
#[derive(Debug, Clone, Serialize)]
pub struct EntitySchema {
    /// Entity name (table name).
    pub name: String,
    /// Human-readable description (for AI prompts).
    pub description: String,
    /// Description in Arabic.
    pub description_ar: Option<String>,
    /// Fields that contain PII (for data masking / security).
    pub pii_fields: Vec<String>,
    /// Relationships to other entities.
    pub relations: Vec<EntityRelation>,
    /// Domain this entity belongs to.
    pub domain: String,
    /// Whether this entity contains student data.
    pub is_student_data: bool,
    /// Available actions on this entity.
    pub actions: Vec<String>,
}

/// A relationship between entities.
#[derive(Debug, Clone, Serialize)]
pub struct EntityRelation {
    pub relation_type: RelationType,
    pub target_entity: String,
    pub foreign_key: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum RelationType {
    HasMany,
    BelongsTo,
    HasOne,
}

/// Resolved context for a specific entity instance.
#[derive(Debug, Clone, Serialize)]
pub struct EntityContext {
    pub entity_type: String,
    pub entity_id: String,
    pub schema: EntitySchema,
    pub depth: u8,
}

/// In-memory entity graph for AI context resolution.
pub struct SemanticResolver {
    entities: HashMap<String, EntitySchema>,
}

impl SemanticResolver {
    /// Build the resolver from hardcoded schema enrichments.
    /// Called once at startup.
    pub fn build() -> Self {
        let mut entities = HashMap::new();

        // ── IAM ──────────────────────────────────────────────────
        entities.insert("users".into(), EntitySchema {
            name: "users".into(),
            description: "School users — students, teachers, staff, and administrators".into(),
            description_ar: Some("مستخدمي المدرسة — طلاب، معلمون، موظفون، ومدراء".into()),
            pii_fields: vec!["email".into(), "phone".into(), "national_id".into(), "full_name".into()],
            relations: vec![
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "enrollments".into(),
                    foreign_key: "student_id".into(),
                    description: "Student enrollments in classes".into(),
                },
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "behavior_records".into(),
                    foreign_key: "student_id".into(),
                    description: "Behavior incidents for this student".into(),
                },
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "user_roles".into(),
                    foreign_key: "user_id".into(),
                    description: "Roles assigned to this user".into(),
                },
            ],
            domain: "iam".into(),
            is_student_data: true,
            actions: vec!["iam.user.get".into(), "iam.user.list".into(), "iam.user.update_role".into()],
        });

        // ── Academic ─────────────────────────────────────────────
        entities.insert("enrollments".into(), EntitySchema {
            name: "enrollments".into(),
            description: "Student-class enrollment records with status tracking".into(),
            description_ar: Some("سجلات تسجيل الطلاب في الفصول مع تتبع الحالة".into()),
            pii_fields: vec![],
            relations: vec![
                EntityRelation {
                    relation_type: RelationType::BelongsTo,
                    target_entity: "users".into(),
                    foreign_key: "student_id".into(),
                    description: "The enrolled student".into(),
                },
                EntityRelation {
                    relation_type: RelationType::BelongsTo,
                    target_entity: "classes".into(),
                    foreign_key: "class_id".into(),
                    description: "The class enrolled in".into(),
                },
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "grades".into(),
                    foreign_key: "enrollment_id".into(),
                    description: "Grades for this enrollment".into(),
                },
            ],
            domain: "academic".into(),
            is_student_data: true,
            actions: vec!["academic.enrollment.create".into(), "academic.enrollment.transfer".into()],
        });

        entities.insert("classes".into(), EntitySchema {
            name: "classes".into(),
            description: "School classes/sections with level and capacity".into(),
            description_ar: Some("فصول المدرسة مع المستوى والسعة".into()),
            pii_fields: vec![],
            relations: vec![
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "enrollments".into(),
                    foreign_key: "class_id".into(),
                    description: "Students enrolled in this class".into(),
                },
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "class_subjects".into(),
                    foreign_key: "class_id".into(),
                    description: "Subjects taught in this class".into(),
                },
            ],
            domain: "academic".into(),
            is_student_data: false,
            actions: vec!["academic.class.list".into()],
        });

        // ── Assessment ───────────────────────────────────────────
        entities.insert("grades".into(), EntitySchema {
            name: "grades".into(),
            description: "Student grade records for assessments".into(),
            description_ar: Some("سجلات درجات الطلاب للتقييمات".into()),
            pii_fields: vec![],
            relations: vec![
                EntityRelation {
                    relation_type: RelationType::BelongsTo,
                    target_entity: "enrollments".into(),
                    foreign_key: "enrollment_id".into(),
                    description: "The enrollment this grade belongs to".into(),
                },
                EntityRelation {
                    relation_type: RelationType::BelongsTo,
                    target_entity: "assessments".into(),
                    foreign_key: "assessment_id".into(),
                    description: "The assessment this grade is for".into(),
                },
            ],
            domain: "assessment".into(),
            is_student_data: true,
            actions: vec!["assessment.grade.list".into(), "assessment.grade.enter".into()],
        });

        entities.insert("attendance_daily".into(), EntitySchema {
            name: "attendance_daily".into(),
            description: "Daily attendance records for students".into(),
            description_ar: Some("سجلات الحضور اليومي للطلاب".into()),
            pii_fields: vec![],
            relations: vec![
                EntityRelation {
                    relation_type: RelationType::BelongsTo,
                    target_entity: "enrollments".into(),
                    foreign_key: "enrollment_id".into(),
                    description: "The enrollment this attendance is for".into(),
                },
            ],
            domain: "assessment".into(),
            is_student_data: true,
            actions: vec!["assessment.attendance.list".into(), "assessment.attendance.record".into()],
        });

        // ── Finance ──────────────────────────────────────────────
        entities.insert("student_fees".into(), EntitySchema {
            name: "student_fees".into(),
            description: "Fees assigned to individual students".into(),
            description_ar: Some("الرسوم المخصصة للطلاب".into()),
            pii_fields: vec![],
            relations: vec![
                EntityRelation {
                    relation_type: RelationType::BelongsTo,
                    target_entity: "users".into(),
                    foreign_key: "student_id".into(),
                    description: "The student this fee is for".into(),
                },
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "payments".into(),
                    foreign_key: "student_fee_id".into(),
                    description: "Payments made for this fee".into(),
                },
            ],
            domain: "finance".into(),
            is_student_data: true,
            actions: vec!["finance.fee.list".into()],
        });

        // ── Lifeperiod ────────────────────────────────────────────
        entities.insert("academic_periods".into(), EntitySchema {
            name: "academic_periods".into(),
            description: "Academic year/semester periods in the registry".into(),
            description_ar: Some("الدورات الأكاديمية (سنوات/فصول) في السجل".into()),
            pii_fields: vec![],
            relations: vec![
                EntityRelation {
                    relation_type: RelationType::HasMany,
                    target_entity: "academic_calendar_events".into(),
                    foreign_key: "period_id".into(),
                    description: "Calendar events in this period".into(),
                },
            ],
            domain: "lifecycle".into(),
            is_student_data: false,
            actions: vec!["lifecycle.period.list".into(), "lifecycle.period.lock".into()],
        });

        info!(
            entity_count = entities.len(),
            "Semantic resolver built"
        );

        Self { entities }
    }

    /// Get schema for an entity type.
    pub fn get_schema(&self, entity_type: &str) -> Option<&EntitySchema> {
        self.entities.get(entity_type)
    }

    /// Resolve context for a specific entity instance.
    pub fn resolve_context(
        &self,
        entity_type: &str,
        entity_id: &str,
        depth: u8,
    ) -> Result<EntityContext, AppError> {
        let schema = self.entities.get(entity_type)
            .ok_or_else(|| AppError::not_found("entity_schema", entity_type))?;

        Ok(EntityContext {
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            schema: schema.clone(),
            depth,
        })
    }

    /// List all known entity types.
    pub fn entity_types(&self) -> Vec<&str> {
        self.entities.keys().map(|s| s.as_str()).collect()
    }

    /// Find entities that contain PII (for data masking).
    pub fn pii_entities(&self) -> Vec<&EntitySchema> {
        self.entities.values()
            .filter(|e| !e.pii_fields.is_empty())
            .collect()
    }

    /// Find entities related to a given entity type.
    pub fn related_entities(&self, entity_type: &str) -> Vec<&EntityRelation> {
        self.entities.get(entity_type)
            .map(|e| e.relations.iter().collect())
            .unwrap_or_default()
    }
}

