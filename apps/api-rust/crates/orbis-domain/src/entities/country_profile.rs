use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Educational stage definition within a national profile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EducationStage {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub order: u32,
    pub levels: Vec<EducationLevel>,
}

/// Specific educational grade/level within a stage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EducationLevel {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub order: u32,
    pub track: Option<String>,
}

/// Regulatory/ministry authority for an institution type in a country.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegulatoryAuthority {
    pub institution_type: String,
    pub ministry_name: String,
    pub ministry_name_ar: String,
}

/// Country-specific grading scale and policy defaults.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountryGradingPolicy {
    pub passing_percentage: rust_decimal::Decimal,
    pub honors_percentage: rust_decimal::Decimal,
    pub default_scale: String,
    pub grade_bands: Vec<CountryGradeBand>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountryGradeBand {
    pub code: String,
    pub label: String,
    pub label_ar: String,
    pub minimum_score: rust_decimal::Decimal,
}

/// Versioned Country Profile entity.
/// Defines national defaults, localization, regulatory authorities, and education stages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryProfile {
    pub id: Uuid,
    pub country_code: String,
    pub name: String,
    pub name_ar: String,
    pub default_locale: String,
    pub default_timezone: String,
    pub default_currency: String,
    pub default_calendar: String,
    pub is_active: bool,
    pub version: i32,
    pub stages: Vec<EducationStage>,
    pub grading_policy: Option<CountryGradingPolicy>,
    pub authorities: Vec<RegulatoryAuthority>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CountryProfile {
    /// Factory for the official Yemeni educational profile (YE).
    pub fn yemen() -> Self {
        use rust_decimal::Decimal;

        let basic_levels = vec![
            EducationLevel {
                code: "G01".to_string(),
                name: "First Grade Basic".to_string(),
                name_ar: "الصف الأول الأساسي".to_string(),
                order: 1,
                track: None,
            },
            EducationLevel {
                code: "G02".to_string(),
                name: "Second Grade Basic".to_string(),
                name_ar: "الصف الثاني الأساسي".to_string(),
                order: 2,
                track: None,
            },
            EducationLevel {
                code: "G03".to_string(),
                name: "Third Grade Basic".to_string(),
                name_ar: "الصف الثالث الأساسي".to_string(),
                order: 3,
                track: None,
            },
            EducationLevel {
                code: "G04".to_string(),
                name: "Fourth Grade Basic".to_string(),
                name_ar: "الصف الرابع الأساسي".to_string(),
                order: 4,
                track: None,
            },
            EducationLevel {
                code: "G05".to_string(),
                name: "Fifth Grade Basic".to_string(),
                name_ar: "الصف الخامس الأساسي".to_string(),
                order: 5,
                track: None,
            },
            EducationLevel {
                code: "G06".to_string(),
                name: "Sixth Grade Basic".to_string(),
                name_ar: "الصف السادس الأساسي".to_string(),
                order: 6,
                track: None,
            },
            EducationLevel {
                code: "G07".to_string(),
                name: "Seventh Grade Basic".to_string(),
                name_ar: "الصف السابع الأساسي".to_string(),
                order: 7,
                track: None,
            },
            EducationLevel {
                code: "G08".to_string(),
                name: "Eighth Grade Basic".to_string(),
                name_ar: "الصف الثامن الأساسي".to_string(),
                order: 8,
                track: None,
            },
            EducationLevel {
                code: "G09".to_string(),
                name: "Ninth Grade Basic".to_string(),
                name_ar: "الصف التاسع الأساسي (الشهادة الأساسية)".to_string(),
                order: 9,
                track: None,
            },
        ];

        let secondary_levels = vec![
            EducationLevel {
                code: "G10".to_string(),
                name: "First Secondary (General)".to_string(),
                name_ar: "الصف الأول الثانوي (عام)".to_string(),
                order: 10,
                track: Some("general".to_string()),
            },
            EducationLevel {
                code: "G11_SCI".to_string(),
                name: "Second Secondary (Scientific)".to_string(),
                name_ar: "الصف الثاني الثانوي (علمي)".to_string(),
                order: 11,
                track: Some("scientific".to_string()),
            },
            EducationLevel {
                code: "G11_LIT".to_string(),
                name: "Second Secondary (Literary)".to_string(),
                name_ar: "الصف الثاني الثانوي (أدبي)".to_string(),
                order: 12,
                track: Some("literary".to_string()),
            },
            EducationLevel {
                code: "G12_SCI".to_string(),
                name: "Third Secondary (Scientific)".to_string(),
                name_ar: "الصف الثالث الثانوي (علمي - الشهادة الثانوية)".to_string(),
                order: 13,
                track: Some("scientific".to_string()),
            },
            EducationLevel {
                code: "G12_LIT".to_string(),
                name: "Third Secondary (Literary)".to_string(),
                name_ar: "الصف الثالث الثانوي (أدبي - الشهادة الثانوية)".to_string(),
                order: 14,
                track: Some("literary".to_string()),
            },
        ];

        let higher_ed_levels = vec![
            EducationLevel {
                code: "UG_Y1".to_string(),
                name: "First Academic Year".to_string(),
                name_ar: "السنة الجامعية الأولى".to_string(),
                order: 1,
                track: None,
            },
            EducationLevel {
                code: "UG_Y2".to_string(),
                name: "Second Academic Year".to_string(),
                name_ar: "السنة الجامعية الثانية".to_string(),
                order: 2,
                track: None,
            },
            EducationLevel {
                code: "UG_Y3".to_string(),
                name: "Third Academic Year".to_string(),
                name_ar: "السنة الجامعية الثالثة".to_string(),
                order: 3,
                track: None,
            },
            EducationLevel {
                code: "UG_Y4".to_string(),
                name: "Fourth Academic Year (Graduation)".to_string(),
                name_ar: "السنة الجامعية الرابعة (التخرج)".to_string(),
                order: 4,
                track: None,
            },
        ];

        let vocational_levels = vec![
            EducationLevel {
                code: "VOC_Y1".to_string(),
                name: "First Training Year".to_string(),
                name_ar: "السنة التدريبية الأولى".to_string(),
                order: 1,
                track: None,
            },
            EducationLevel {
                code: "VOC_Y2".to_string(),
                name: "Second Training Year (Diploma)".to_string(),
                name_ar: "السنة التدريبية الثانية (دبلوم تقني)".to_string(),
                order: 2,
                track: None,
            },
        ];

        let stages = vec![
            EducationStage {
                code: "BASIC".to_string(),
                name: "Basic Education (Grades 1-9)".to_string(),
                name_ar: "التعليم الأساسي (الصفوف 1-9)".to_string(),
                order: 1,
                levels: basic_levels,
            },
            EducationStage {
                code: "SECONDARY".to_string(),
                name: "Secondary Education (Grades 10-12)".to_string(),
                name_ar: "التعليم الثانوي (الصفوف 10-12)".to_string(),
                order: 2,
                levels: secondary_levels,
            },
            EducationStage {
                code: "HIGHER_ED".to_string(),
                name: "Higher Education / University".to_string(),
                name_ar: "التعليم الجامعي والعالي".to_string(),
                order: 3,
                levels: higher_ed_levels,
            },
            EducationStage {
                code: "VOCATIONAL".to_string(),
                name: "Technical & Vocational Training".to_string(),
                name_ar: "التعليم الفني والتدريب المهني".to_string(),
                order: 4,
                levels: vocational_levels,
            },
        ];

        let authorities = vec![
            RegulatoryAuthority {
                institution_type: "school".to_string(),
                ministry_name: "Ministry of Education".to_string(),
                ministry_name_ar: "وزارة التربية والتعليم".to_string(),
            },
            RegulatoryAuthority {
                institution_type: "university".to_string(),
                ministry_name: "Ministry of Higher Education and Scientific Research".to_string(),
                ministry_name_ar: "وزارة التعليم العالي والبحث العلمي".to_string(),
            },
            RegulatoryAuthority {
                institution_type: "institute".to_string(),
                ministry_name: "Ministry of Technical Education and Vocational Training".to_string(),
                ministry_name_ar: "وزارة التعليم الفني والتدريب المهني".to_string(),
            },
        ];

        let grading_policy = CountryGradingPolicy {
            passing_percentage: Decimal::from(50),
            honors_percentage: Decimal::from(85),
            default_scale: "percentage".to_string(),
            grade_bands: vec![
                CountryGradeBand {
                    code: "A".to_string(),
                    label: "Excellent".to_string(),
                    label_ar: "ممتاز".to_string(),
                    minimum_score: Decimal::from(90),
                },
                CountryGradeBand {
                    code: "B".to_string(),
                    label: "Very Good".to_string(),
                    label_ar: "جيد جداً".to_string(),
                    minimum_score: Decimal::from(80),
                },
                CountryGradeBand {
                    code: "C".to_string(),
                    label: "Good".to_string(),
                    label_ar: "جيد".to_string(),
                    minimum_score: Decimal::from(65),
                },
                CountryGradeBand {
                    code: "D".to_string(),
                    label: "Pass".to_string(),
                    label_ar: "مقبول".to_string(),
                    minimum_score: Decimal::from(50),
                },
                CountryGradeBand {
                    code: "F".to_string(),
                    label: "Fail".to_string(),
                    label_ar: "راسب".to_string(),
                    minimum_score: Decimal::ZERO,
                },
            ],
        };

        let now = Utc::now();
        Self {
            id: Uuid::nil(), // standard catalog entry
            country_code: "YE".to_string(),
            name: "Yemen".to_string(),
            name_ar: "الجمهورية اليمنية".to_string(),
            default_locale: "ar".to_string(),
            default_timezone: "Asia/Aden".to_string(),
            default_currency: "YER".to_string(),
            default_calendar: "gregorian".to_string(),
            is_active: true,
            version: 1,
            stages,
            grading_policy: Some(grading_policy),
            authorities,
            metadata: serde_json::json!({
                "currency_symbol": "ر.ي",
                "phone_prefix": "+967",
                "default_academic_term_model": "two_semesters"
            }),
            created_at: now,
            updated_at: now,
        }
    }

    /// Resolve profile by country code, defaulting to Yemen.
    pub fn resolve(code: Option<&str>) -> Self {
        match code.map(|c| c.trim().to_uppercase()).as_deref() {
            Some("YE") | None => Self::yemen(),
            _ => {
                let mut p = Self::yemen();
                if let Some(c) = code {
                    p.country_code = c.to_uppercase();
                }
                p
            }
        }
    }

    /// Resolve regulatory/ministry authority for an institution type under this country profile.
    pub fn authority_for(&self, inst_type: &str) -> Option<&RegulatoryAuthority> {
        let normalized = inst_type.trim().to_lowercase();
        self.authorities.iter().find(|a| a.institution_type == normalized)
    }

    /// Filter applicable education stages for an institution type under this country profile.
    pub fn stages_for(&self, inst_type: &str) -> Vec<&EducationStage> {
        let normalized = inst_type.trim().to_lowercase();
        match normalized.as_str() {
            "school" => self
                .stages
                .iter()
                .filter(|s| s.code == "BASIC" || s.code == "SECONDARY")
                .collect(),
            "university" => self
                .stages
                .iter()
                .filter(|s| s.code == "HIGHER_ED")
                .collect(),
            "institute" | "training_center" => self
                .stages
                .iter()
                .filter(|s| s.code == "VOCATIONAL")
                .collect(),
            _ => self.stages.iter().collect(),
        }
    }
}
