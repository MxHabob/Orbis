use shared_kernel::errors::AppError;
use crate::contracts::report::{
    ReportDefinition, ReportParameter, ReportColumn,
    ParameterType, DataType,
};
use lazy_static::lazy_static;

/// Each report is tagged with a module so we can filter by section.
/// The module tag is stored here, NOT inside ReportDefinition (which is the
/// public contract sent to the frontend).
struct RegistryEntry {
    pub module: String,
    pub definition: ReportDefinition,
}

lazy_static! {
    static ref REPORTS: Vec<RegistryEntry> = vec![
        // ── Academics ──────────────────────────────────────────
        RegistryEntry {
            module: "academics".to_string(),
            definition: ReportDefinition {
                id: "student_performance".to_string(),
                name: "أداء الطلاب".to_string(),
                description: Some("تحليل شامل لدرجات ومعدلات الطلاب حسب الفصل الدراسي".to_string()),
                parameters: vec![
                    ReportParameter {
                        id: "class_id".to_string(),
                        label: "الفصل الدراسي".to_string(),
                        param_type: ParameterType::EntityRef { entity_type: "class".to_string() },
                        required: true,
                    },
                ],
                columns: vec![
                    ReportColumn {
                        id: "student_name".to_string(),
                        label: "اسم الطالب".to_string(),
                        data_type: DataType::String,
                        width_weight: 3,
                        is_summarized: false,
                    },
                    ReportColumn {
                        id: "gpa".to_string(),
                        label: "المعدل (GPA)".to_string(),
                        data_type: DataType::Number,
                        width_weight: 1,
                        is_summarized: true,
                    },
                    ReportColumn {
                        id: "status".to_string(),
                        label: "الحالة".to_string(),
                        data_type: DataType::String,
                        width_weight: 1,
                        is_summarized: false,
                    },
                ],
                default_master_template: None,
            },
        },
        RegistryEntry {
            module: "academics".to_string(),
            definition: ReportDefinition {
                id: "attendance_summary".to_string(),
                name: "ملخص الحضور والغياب".to_string(),
                description: Some("تقرير بنسب غياب وحضور الطلاب خلال فترة محددة".to_string()),
                parameters: vec![
                    ReportParameter {
                        id: "date_range".to_string(),
                        label: "الفترة الزمنية".to_string(),
                        param_type: ParameterType::DateRange,
                        required: true,
                    },
                ],
                columns: vec![
                    ReportColumn {
                        id: "student_name".to_string(),
                        label: "اسم الطالب".to_string(),
                        data_type: DataType::String,
                        width_weight: 3,
                        is_summarized: false,
                    },
                    ReportColumn {
                        id: "absent_days".to_string(),
                        label: "أيام الغياب".to_string(),
                        data_type: DataType::Number,
                        width_weight: 1,
                        is_summarized: true,
                    },
                    ReportColumn {
                        id: "present_days".to_string(),
                        label: "أيام الحضور".to_string(),
                        data_type: DataType::Number,
                        width_weight: 1,
                        is_summarized: true,
                    },
                    ReportColumn {
                        id: "rate".to_string(),
                        label: "نسبة الحضور %".to_string(),
                        data_type: DataType::Number,
                        width_weight: 1,
                        is_summarized: false,
                    },
                ],
                default_master_template: None,
            },
        },

        // ── Finance ────────────────────────────────────────────
        RegistryEntry {
            module: "finance".to_string(),
            definition: ReportDefinition {
                id: "trial_balance".to_string(),
                name: "ميزان المراجعة".to_string(),
                description: Some("ميزان مراجعة شامل لجميع الحسابات".to_string()),
                parameters: vec![
                    ReportParameter {
                        id: "as_of_date".to_string(),
                        label: "حتى تاريخ".to_string(),
                        param_type: ParameterType::Date,
                        required: true,
                    },
                ],
                columns: vec![
                    ReportColumn {
                        id: "account_name".to_string(),
                        label: "اسم الحساب".to_string(),
                        data_type: DataType::String,
                        width_weight: 3,
                        is_summarized: false,
                    },
                    ReportColumn {
                        id: "debit".to_string(),
                        label: "مدين".to_string(),
                        data_type: DataType::Currency,
                        width_weight: 2,
                        is_summarized: true,
                    },
                    ReportColumn {
                        id: "credit".to_string(),
                        label: "دائن".to_string(),
                        data_type: DataType::Currency,
                        width_weight: 2,
                        is_summarized: true,
                    },
                ],
                default_master_template: None,
            },
        },
    ];
}

pub fn get_definition(id: &str) -> Result<ReportDefinition, AppError> {
    REPORTS
        .iter()
        .find(|e| e.definition.id == id)
        .map(|e| e.definition.clone())
        .ok_or_else(|| AppError::validation(&format!("Report '{}' not found", id)))
}

pub fn list_by_module(module: &str) -> Vec<ReportDefinition> {
    REPORTS
        .iter()
        .filter(|e| e.module == module)
        .map(|e| e.definition.clone())
        .collect()
}
