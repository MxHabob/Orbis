use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
// Assessment Component — a single gradable element within a scheme
// ═══════════════════════════════════════════════════════════════

/// The semantic kind of an assessment component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentComponentKind {
    Coursework,
    Assignment,
    Quiz,
    Oral,
    Attendance,
    Participation,
    Midterm,
    FinalExam,
    Practical,
    Lab,
    Project,
    Competency,
    Fieldwork,
    Other,
}

/// A single assessment component (e.g., Assignments, Oral, Attendance, Written Test, Practical Exam).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentComponent {
    /// Stable identifier for this component within the scheme.
    pub code: String,
    /// Display name (English).
    pub name: String,
    /// Display name (Arabic).
    pub name_ar: Option<String>,
    /// Maximum possible score for this component.
    pub max_score: Decimal,
    /// Weight of this component in the overall score (0.0–1.0).
    /// For school monthly assessments: each component contributes its raw score,
    /// so weight can be 1.0 and the sum is the total.
    pub weight: Decimal,
    /// Display order within the scheme.
    pub sort_order: i32,
    /// Whether this component is required (vs. optional/bonus).
    pub is_required: bool,
    /// Semantic kind of this assessment component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_type: Option<AssessmentComponentKind>,
    /// Minimum required score to pass this component (if component-level gatekeeping applies).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score: Option<Decimal>,
}

// ═══════════════════════════════════════════════════════════════
// Assessment Scope — which academic level this scheme applies to
// ═══════════════════════════════════════════════════════════════

/// The academic level at which this assessment scheme operates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentLevel {
    /// Monthly period assessment (school: 100 points per month)
    MonthlyPeriod,
    /// Term-level aggregation (school: monthly contribution + term exam = 50)
    Term,
    /// Annual aggregation (school: Term 1 + Term 2 = 100)
    Annual,
    /// University coursework assessment
    Coursework,
    /// Midterm examination
    Midterm,
    /// Final examination
    Final,
    /// Module/block assessment (institute)
    ModuleAssessment,
}

impl std::fmt::Display for AssessmentLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssessmentLevel::MonthlyPeriod => write!(f, "monthly_period"),
            AssessmentLevel::Term => write!(f, "term"),
            AssessmentLevel::Annual => write!(f, "annual"),
            AssessmentLevel::Coursework => write!(f, "coursework"),
            AssessmentLevel::Midterm => write!(f, "midterm"),
            AssessmentLevel::Final => write!(f, "final"),
            AssessmentLevel::ModuleAssessment => write!(f, "module_assessment"),
        }
    }
}

impl AssessmentLevel {
    pub fn from_str(s: &str) -> Self {
        match s {
            "monthly_period" | "monthly" => Self::MonthlyPeriod,
            "term" | "semester" => Self::Term,
            "annual" | "yearly" => Self::Annual,
            "coursework" => Self::Coursework,
            "midterm" => Self::Midterm,
            "final" => Self::Final,
            "module_assessment" | "module" => Self::ModuleAssessment,
            _ => Self::MonthlyPeriod,
        }
    }
}

impl Default for AssessmentLevel {
    fn default() -> Self {
        AssessmentLevel::MonthlyPeriod
    }
}

// ═══════════════════════════════════════════════════════════════
// Institution Scope — which institution type this scheme targets
// ═══════════════════════════════════════════════════════════════

/// Which institution types this scheme is applicable to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstitutionScope {
    School,
    University,
    Institute,
    All,
}

impl std::fmt::Display for InstitutionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstitutionScope::School => write!(f, "school"),
            InstitutionScope::University => write!(f, "university"),
            InstitutionScope::Institute => write!(f, "institute"),
            InstitutionScope::All => write!(f, "all"),
        }
    }
}

impl InstitutionScope {
    pub fn from_str(s: &str) -> Self {
        match s {
            "school" => Self::School,
            "university" => Self::University,
            "institute" => Self::Institute,
            _ => Self::All,
        }
    }
}

impl Default for InstitutionScope {
    fn default() -> Self {
        InstitutionScope::All
    }
}

// ═══════════════════════════════════════════════════════════════
// Assessment Scheme — the core entity
// ═══════════════════════════════════════════════════════════════

/// Defines how students are assessed at a given academic level.
///
/// For SCHOOL monthly assessment (default):
///   Components: Assignments(20) + Oral(20) + Attendance(20) + Written Test(40) = 100
///
/// For SCHOOL term aggregation:
///   monthly_contribution_weight = 0.20 (monthly avg normalized to 20/100)
///   term_exam_weight = 0.30 (exam out of 30)
///   Total = 50/50
///
/// For UNIVERSITY coursework:
///   Components: Quizzes + Homework + Participation + Midterm + Final
///   With configurable weights summing to 1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentScheme {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    /// Maximum total score for this scheme.
    pub max_score: Decimal,
    /// Custom AST formula for non-standard calculations.
    pub formula: Option<serde_json::Value>,
    /// The academic level this scheme targets.
    pub level: AssessmentLevel,
    /// Which institution type(s) this scheme is designed for.
    pub institution_scope: InstitutionScope,
    /// Typed component definitions.
    pub components: Vec<AssessmentComponent>,
    /// Whether this is the default scheme for its level and scope.
    pub is_default: bool,
    /// Whether this is a system-created scheme (not user-editable).
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AssessmentScheme {
    /// Validate that component max_scores sum to the scheme max_score.
    pub fn validate_components(&self) -> Result<(), String> {
        if self.components.is_empty() {
            return Ok(()); // Schemes without explicit components use formula
        }
        let sum: Decimal = self.components.iter().map(|c| c.max_score).sum();
        if sum != self.max_score {
            return Err(format!(
                "Component max_scores sum ({}) does not equal scheme max_score ({})",
                sum, self.max_score
            ));
        }
        Ok(())
    }

    /// Create a generic default periodic assessment scheme (100 points).
    pub fn default_school_monthly(org_id: Uuid) -> Self {
        Self::default_periodic_monthly(org_id)
    }

    /// Create a generic default periodic assessment scheme (100 points).
    pub fn default_periodic_monthly(org_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            name: "Periodic Assessment".to_string(),
            name_ar: Some("التقييم الدوري".to_string()),
            description: Some(
                "Baseline periodic assessment covering coursework, participation, attendance, and written evaluation."
                    .to_string(),
            ),
            max_score: Decimal::from(100),
            formula: None,
            level: AssessmentLevel::MonthlyPeriod,
            institution_scope: InstitutionScope::All,
            components: vec![
                AssessmentComponent {
                    code: "assignments".to_string(),
                    name: "Assignments & Coursework".to_string(),
                    name_ar: Some("الواجبات والأعمال الدراسية".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 1,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Assignment),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "oral".to_string(),
                    name: "Participation & Oral".to_string(),
                    name_ar: Some("المشاركة والشفهي".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 2,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Oral),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "attendance".to_string(),
                    name: "Attendance & Conduct".to_string(),
                    name_ar: Some("الحضور والسلوك".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 3,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Attendance),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "written_test".to_string(),
                    name: "Written Evaluation".to_string(),
                    name_ar: Some("التقييم التحريري".to_string()),
                    max_score: Decimal::from(40),
                    weight: Decimal::ONE,
                    sort_order: 4,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Quiz),
                    min_score: None,
                },
            ],
            is_default: true,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a generic default period-end examination scheme (30 points).
    pub fn default_school_term_exam(org_id: Uuid) -> Self {
        Self::default_periodic_term_exam(org_id)
    }

    /// Create a generic default period-end examination scheme (30 points).
    pub fn default_periodic_term_exam(org_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            name: "Periodic Examination".to_string(),
            name_ar: Some("اختبار دوري".to_string()),
            description: Some(
                "End-of-period examination for institutions using a periodic/term-based assessment model."
                    .to_string(),
            ),
            max_score: Decimal::from(30),
            formula: None,
            level: AssessmentLevel::Term,
            institution_scope: InstitutionScope::All,
            components: vec![AssessmentComponent {
                code: "term_exam".to_string(),
                name: "Period Examination".to_string(),
                name_ar: Some("اختبار الفترة".to_string()),
                max_score: Decimal::from(30),
                weight: Decimal::ONE,
                sort_order: 1,
                is_required: true,
                component_type: Some(AssessmentComponentKind::FinalExam),
                min_score: Some(Decimal::from(10)),
            }],
            is_default: true,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a default university coursework scheme.
    pub fn default_university_coursework(org_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            name: "University Course Assessment".to_string(),
            name_ar: Some("تقييم المقرر الجامعي".to_string()),
            description: Some(
                "Standard university assessment: Coursework(40) + Midterm(20) + Final(40) = 100"
                    .to_string(),
            ),
            max_score: Decimal::from(100),
            formula: None,
            level: AssessmentLevel::Coursework,
            institution_scope: InstitutionScope::University,
            components: vec![
                AssessmentComponent {
                    code: "quizzes".to_string(),
                    name: "Quizzes & Homework".to_string(),
                    name_ar: Some("الاختبارات القصيرة والواجبات".to_string()),
                    max_score: Decimal::from(15),
                    weight: Decimal::ONE,
                    sort_order: 1,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Quiz),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "participation".to_string(),
                    name: "Participation".to_string(),
                    name_ar: Some("المشاركة".to_string()),
                    max_score: Decimal::from(10),
                    weight: Decimal::ONE,
                    sort_order: 2,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Participation),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "project".to_string(),
                    name: "Project / Lab".to_string(),
                    name_ar: Some("المشروع / المختبر".to_string()),
                    max_score: Decimal::from(15),
                    weight: Decimal::ONE,
                    sort_order: 3,
                    is_required: false,
                    component_type: Some(AssessmentComponentKind::Project),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "midterm".to_string(),
                    name: "Midterm Exam".to_string(),
                    name_ar: Some("اختبار نصف الفصل".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 4,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Midterm),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "final_exam".to_string(),
                    name: "Final Exam".to_string(),
                    name_ar: Some("الاختبار النهائي".to_string()),
                    max_score: Decimal::from(40),
                    weight: Decimal::ONE,
                    sort_order: 5,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::FinalExam),
                    min_score: Some(Decimal::from(20)),
                },
            ],
            is_default: true,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a default institute module assessment scheme.
    pub fn default_institute_module(org_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            name: "Module Assessment".to_string(),
            name_ar: Some("تقييم الوحدة التدريبية".to_string()),
            description: Some(
                "Institute module assessment: Theory(40) + Practical(40) + Attendance(20) = 100"
                    .to_string(),
            ),
            max_score: Decimal::from(100),
            formula: None,
            level: AssessmentLevel::ModuleAssessment,
            institution_scope: InstitutionScope::Institute,
            components: vec![
                AssessmentComponent {
                    code: "theory".to_string(),
                    name: "Theory".to_string(),
                    name_ar: Some("الجانب النظري".to_string()),
                    max_score: Decimal::from(40),
                    weight: Decimal::ONE,
                    sort_order: 1,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Coursework),
                    min_score: Some(Decimal::from(20)),
                },
                AssessmentComponent {
                    code: "practical".to_string(),
                    name: "Practical".to_string(),
                    name_ar: Some("الجانب العملي".to_string()),
                    max_score: Decimal::from(40),
                    weight: Decimal::ONE,
                    sort_order: 2,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Practical),
                    min_score: Some(Decimal::from(24)),
                },
                AssessmentComponent {
                    code: "attendance".to_string(),
                    name: "Attendance".to_string(),
                    name_ar: Some("الحضور".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 3,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Attendance),
                    min_score: None,
                },
            ],
            is_default: true,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a vocational competency assessment scheme (100 points, practical mastery gate).
    pub fn default_competency_vocational(org_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            name: "Vocational Competency Assessment".to_string(),
            name_ar: Some("تقييم الجدارات المهنية والتدريب العملي".to_string()),
            description: Some(
                "Competency-based evaluation: Theory(30) + Practical Skills(50, min 35) + Safety & Standards(20, min 15) = 100"
                    .to_string(),
            ),
            max_score: Decimal::from(100),
            formula: None,
            level: AssessmentLevel::ModuleAssessment,
            institution_scope: InstitutionScope::Institute,
            components: vec![
                AssessmentComponent {
                    code: "theory".to_string(),
                    name: "Theory Knowledge".to_string(),
                    name_ar: Some("المعارف النظرية".to_string()),
                    max_score: Decimal::from(30),
                    weight: Decimal::ONE,
                    sort_order: 1,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Coursework),
                    min_score: Some(Decimal::from(15)),
                },
                AssessmentComponent {
                    code: "practical_skills".to_string(),
                    name: "Practical Skill Execution".to_string(),
                    name_ar: Some("التطبيق العملي والمهاري".to_string()),
                    max_score: Decimal::from(50),
                    weight: Decimal::ONE,
                    sort_order: 2,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Practical),
                    min_score: Some(Decimal::from(35)),
                },
                AssessmentComponent {
                    code: "safety_standards".to_string(),
                    name: "Safety & Professional Standards".to_string(),
                    name_ar: Some("معايير السلامة المهنية والانضباط".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 3,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Competency),
                    min_score: Some(Decimal::from(15)),
                },
            ],
            is_default: false,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create standard Yemeni school term aggregation scheme (20 coursework + 30 exam = 50).
    pub fn default_yemen_school_term_aggregation(org_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            name: "School Term Evaluation (50 pts)".to_string(),
            name_ar: Some("محصلة الفصل الدراسي المدرسي (50 درجة)".to_string()),
            description: Some(
                "Yemeni standard school term: Monthly Coursework Average(20) + Term Exam(30) = 50"
                    .to_string(),
            ),
            max_score: Decimal::from(50),
            formula: None,
            level: AssessmentLevel::Term,
            institution_scope: InstitutionScope::School,
            components: vec![
                AssessmentComponent {
                    code: "coursework_contribution".to_string(),
                    name: "Monthly Coursework Contribution".to_string(),
                    name_ar: Some("محصلة الأعمال الشهرية".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 1,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Coursework),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "term_exam".to_string(),
                    name: "Term Final Examination".to_string(),
                    name_ar: Some("اختبار نهاية الفصل".to_string()),
                    max_score: Decimal::from(30),
                    weight: Decimal::ONE,
                    sort_order: 2,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::FinalExam),
                    min_score: Some(Decimal::from(10)),
                },
            ],
            is_default: true,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create credit-hour university course assessment scheme (100 points).
    pub fn default_university_credit(org_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            name: "University Credit-Hour Course Assessment".to_string(),
            name_ar: Some("تقييم المقرر بالساعات المعتمدة (100 درجة)".to_string()),
            description: Some(
                "University assessment: Coursework(20) + Midterm(20) + Practical/Project(20) + Final Exam(40) = 100"
                    .to_string(),
            ),
            max_score: Decimal::from(100),
            formula: None,
            level: AssessmentLevel::Coursework,
            institution_scope: InstitutionScope::University,
            components: vec![
                AssessmentComponent {
                    code: "coursework".to_string(),
                    name: "Assignments & Quizzes".to_string(),
                    name_ar: Some("الواجبات والاختبارات القصيرة".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 1,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Coursework),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "midterm".to_string(),
                    name: "Midterm Examination".to_string(),
                    name_ar: Some("الاختبار النصفي".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 2,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::Midterm),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "project_lab".to_string(),
                    name: "Project / Laboratory Work".to_string(),
                    name_ar: Some("المشروع / التطبيق العملي".to_string()),
                    max_score: Decimal::from(20),
                    weight: Decimal::ONE,
                    sort_order: 3,
                    is_required: false,
                    component_type: Some(AssessmentComponentKind::Lab),
                    min_score: None,
                },
                AssessmentComponent {
                    code: "final_exam".to_string(),
                    name: "Comprehensive Final Examination".to_string(),
                    name_ar: Some("الاختبار النهائي الشامل".to_string()),
                    max_score: Decimal::from(40),
                    weight: Decimal::ONE,
                    sort_order: 4,
                    is_required: true,
                    component_type: Some(AssessmentComponentKind::FinalExam),
                    min_score: Some(Decimal::from(20)),
                },
            ],
            is_default: true,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AssessmentLevel, AssessmentScheme, InstitutionScope};
    use uuid::Uuid;

    #[test]
    fn generic_periodic_defaults_are_institution_neutral() {
        let monthly = AssessmentScheme::default_periodic_monthly(Uuid::new_v4());
        let term = AssessmentScheme::default_periodic_term_exam(Uuid::new_v4());

        assert_eq!(monthly.institution_scope, InstitutionScope::All);
        assert_eq!(term.institution_scope, InstitutionScope::All);
        assert_eq!(monthly.level, AssessmentLevel::MonthlyPeriod);
        assert_eq!(term.level, AssessmentLevel::Term);
        assert!(!monthly.name.to_lowercase().contains("school"));
        assert!(!term.name.to_lowercase().contains("school"));
        assert!(!monthly.description.as_ref().unwrap().to_lowercase().contains("kindergarten"));
        assert!(!term.description.as_ref().unwrap().to_lowercase().contains("kindergarten"));
    }
}
