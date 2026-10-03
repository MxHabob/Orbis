use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YearFormat {
    Full,  // e.g. "2026"
    Short, // e.g. "26"
    None,
}

impl Default for YearFormat {
    fn default() -> Self {
        YearFormat::Full
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NumberingSchema {
    pub prefix: String,
    pub year_format: YearFormat,
    pub sequence_digits: u8,
    pub separator: String,
}

impl Default for NumberingSchema {
    fn default() -> Self {
        Self {
            prefix: "STU".to_string(),
            year_format: YearFormat::Full,
            sequence_digits: 4,
            separator: "-".to_string(),
        }
    }
}

impl NumberingSchema {
    pub fn format_number(&self, year: i32, seq: u64) -> String {
        let mut parts = Vec::new();
        if !self.prefix.trim().is_empty() {
            parts.push(self.prefix.trim().to_uppercase());
        }

        match self.year_format {
            YearFormat::Full => {
                parts.push(year.to_string());
            }
            YearFormat::Short => {
                let short_yr = format!("{:02}", (year % 100).abs());
                parts.push(short_yr);
            }
            YearFormat::None => {}
        }

        let seq_str = format!("{:0width$}", seq, width = self.sequence_digits as usize);
        parts.push(seq_str);

        parts.join(&self.separator)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AdmissionPolicy {
    pub requires_approval: bool,
    pub allow_no_guardian: bool,
    pub allow_duplicate_national_id: bool,
    pub auto_assign_fees: bool,
    pub max_class_capacity: i32,
    pub allow_late_enrollment: bool,
    pub allow_waitlist: bool,
    pub sibling_priority_enabled: bool,
}

impl Default for AdmissionPolicy {
    fn default() -> Self {
        Self {
            requires_approval: false,
            allow_no_guardian: false,
            allow_duplicate_national_id: false,
            auto_assign_fees: true,
            max_class_capacity: 30,
            allow_late_enrollment: false,
            allow_waitlist: true,
            sibling_priority_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormFieldConfig {
    pub key: String,
    pub name_ar: String,
    pub name_en: String,
    pub is_active: bool,
    pub is_required: bool,
    pub prevent_duplicate: bool,
    pub supports_duplicate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdmissionConfiguration {
    #[serde(default)]
    pub numbering: NumberingSchema,
    #[serde(default)]
    pub policy: AdmissionPolicy,
    #[serde(default)]
    pub fields: Vec<FormFieldConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbering_formatting() {
        let schema = NumberingSchema {
            prefix: "STD".to_string(),
            year_format: YearFormat::Full,
            sequence_digits: 4,
            separator: "-".to_string(),
        };
        assert_eq!(schema.format_number(2026, 42), "STD-2026-0042");

        let schema_short = NumberingSchema {
            prefix: "UNI".to_string(),
            year_format: YearFormat::Short,
            sequence_digits: 5,
            separator: "/".to_string(),
        };
        assert_eq!(schema_short.format_number(2026, 7), "UNI/26/00007");
    }
}
