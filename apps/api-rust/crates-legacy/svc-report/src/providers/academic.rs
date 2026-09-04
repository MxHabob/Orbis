//! Academic Report Data Providers.
//!
//! Provides report definitions and data provider functions for:
//! - Student List (class roster)

use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::json;

use crate::dataset::*;
use crate::definition::*;
use crate::error::ReportError;
use crate::org_context::OrgContext;
use crate::request::*;

// ═══════════════════════════════════════════════════════════════
// Report Definitions
// ═══════════════════════════════════════════════════════════════

/// Returns all academic report definitions for registry registration.
pub fn definitions() -> Vec<ReportDefinition> {
    vec![
        student_list_definition(),
    ]
}

fn student_list_definition() -> ReportDefinition {
    ReportDefinition {
        code: "academic.student_list".into(),
        name: "Student List".into(),
        name_ar: "قائمة الطلاب".into(),
        domain: ReportDomain::Academic,
        category: "enrollment".into(),
        description: Some("List of enrolled students in a class with contact information".into()),
        description_ar: Some("قائمة الطلاب المسجلين في فصل مع معلومات الاتصال".into()),
        parameters: vec![
            ParameterDef {
                key: "period_id".into(),
                label: "Academic Period".into(),
                label_ar: "الفترة الدراسية".into(),
                param_type: ParamType::Period,
                required: true,
                default_value: None,
                options: None,
                entity_type: Some("academic_period".into()),
                placeholder: None,
                placeholder_ar: None,
                help_text: None,
                help_text_ar: None,
                sort_order: 0,
                group: None,
            },
            ParameterDef {
                key: "class_id".into(),
                label: "Class".into(),
                label_ar: "الفصل".into(),
                param_type: ParamType::Id,
                required: false,
                default_value: None,
                options: None,
                entity_type: Some("class".into()),
                placeholder: Some("All classes".into()),
                placeholder_ar: Some("كل الفصول".into()),
                help_text: Some("Leave empty to include all classes".into()),
                help_text_ar: Some("اتركه فارغاً لتضمين كل الفصول".into()),
                sort_order: 1,
                group: None,
            },
            ParameterDef {
                key: "status".into(),
                label: "Enrollment Status".into(),
                label_ar: "حالة التسجيل".into(),
                param_type: ParamType::Enum,
                required: false,
                default_value: None,
                options: Some(vec![
                    ParamOption { value: "active".into(), label: "Active".into(), label_ar: "نشط".into() },
                    ParamOption { value: "withdrawn".into(), label: "Withdrawn".into(), label_ar: "منسحب".into() },
                    ParamOption { value: "transferred".into(), label: "Transferred".into(), label_ar: "محول".into() },
                ]),
                entity_type: None,
                placeholder: Some("All statuses".into()),
                placeholder_ar: Some("كل الحالات".into()),
                help_text: None,
                help_text_ar: None,
                sort_order: 2,
                group: None,
            },
        ],
        required_permissions: vec!["academic.read".into()],
        supported_formats: vec![
            OutputFormatKind::DataOnly,
            OutputFormatKind::Csv,
            OutputFormatKind::Xlsx,
            OutputFormatKind::Json,
            OutputFormatKind::Pdf,
        ],
        default_template_id: None,
        is_system: true,
        is_active: true,
        sort_order: 10,
        version: 1,
    }
}

// ═══════════════════════════════════════════════════════════════
// Data Providers
// ═══════════════════════════════════════════════════════════════

/// Student List — enrolled students per class with contact info.
pub async fn student_list(
    db: &DatabaseConnection,
    org_id: &str,
    org: &OrgContext,
    request: &ReportRequest,
) -> Result<ReportDataset, ReportError> {
    let period_id = request.parameters.get("period_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ReportError::MissingParameter("period_id".into()))?;

    let class_filter = request.parameters.get("class_id")
        .and_then(|v| v.as_str());

    let status_filter = request.parameters.get("status")
        .and_then(|v| v.as_str());

    let now = chrono::Utc::now().to_rfc3339();

    let mut sql = String::from(
        r#"SELECT
            COUNT(*) OVER() AS total_count,
            e.id AS enrollment_id,
            e.student_number,
            e.status AS enrollment_status,
            e.enrollment_date,
            e.class_id,
            c.name AS class_name,
            c.name_ar AS class_name_ar,
            c.level AS class_level,
                        p.id AS student_id,
                        p.full_name,
                        p.name_ar,
                        u.email,
                        p.phone,
                        p.national_id,
                        p.gender AS sex,
                        p.date_of_birth AS birth_date
        FROM enrollments e
                JOIN persons p ON e.student_id = p.id
                LEFT JOIN users u ON u.person_id = p.id AND u.org_id = e.org_id
        JOIN classes c ON e.class_id = c.id
        WHERE e.org_id = $1
                    AND e.period_id = $2
                    AND p.org_id = $1"#
    );

    let mut params: Vec<sea_orm::Value> = vec![org_id.into(), period_id.into()];
    let mut param_idx = 3;

    if let Some(class_id) = class_filter {
        sql.push_str(&format!(" AND e.class_id = ${}", param_idx));
        params.push(class_id.into());
        param_idx += 1;
    }
    if let Some(status) = status_filter {
        sql.push_str(&format!(" AND e.status = ${}", param_idx));
        params.push(status.into());
        param_idx += 1;
    }

    sql.push_str(" ORDER BY c.level, c.name, e.student_number");

    let pagination = request.pagination.clone().unwrap_or_default();
    if request.pagination.is_some() {
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));
        params.push(pagination.limit().into());
        params.push(pagination.offset().into());
    }

    let stmt = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(), &sql, params,
    );

    let rows_raw = db.query_all(stmt).await.map_err(|e| ReportError::DataProviderError(e.to_string()))?;

    let mut rows = Vec::with_capacity(rows_raw.len());

    let total_count = rows_raw.first()
        .and_then(|row| row.try_get::<i64>("", "total_count").ok())
        .unwrap_or(rows_raw.len() as i64) as u64;
    let row_offset = if request.pagination.is_some() {
        pagination.offset() as usize
    } else {
        0
    };

    for (idx, row) in rows_raw.iter().enumerate() {
        rows.push(json!({
            "seq": row_offset + idx + 1,
            "studentNumber": row.try_get::<String>("", "student_number").unwrap_or_default(),
            "fullName": row.try_get::<Option<String>>("", "full_name").unwrap_or_default(),
            "nameAr": row.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
            "className": row.try_get::<String>("", "class_name").unwrap_or_default(),
            "classNameAr": row.try_get::<Option<String>>("", "class_name_ar").unwrap_or_default(),
            "classLevel": row.try_get::<String>("", "class_level").unwrap_or_default(),
            "enrollmentStatus": row.try_get::<String>("", "enrollment_status").unwrap_or_default(),
            "enrollmentDate": row.try_get::<String>("", "enrollment_date").unwrap_or_default(),
            "nationalId": row.try_get::<Option<String>>("", "national_id").unwrap_or_default(),
            "email": row.try_get::<Option<String>>("", "email").unwrap_or_default(),
            "phone": row.try_get::<Option<String>>("", "phone").unwrap_or_default(),
            "sex": row.try_get::<Option<String>>("", "sex").unwrap_or_default(),
            "birthDate": row.try_get::<Option<String>>("", "birth_date").unwrap_or_default(),
        }));
    }

    let mut param_summary = vec![
        ParameterSummaryItem {
            label: "Academic Period".into(), label_ar: "الفترة الدراسية".into(),
            value: period_id.to_string(), value_ar: period_id.to_string(),
        },
    ];
    if let Some(class_id) = class_filter {
        param_summary.push(ParameterSummaryItem {
            label: "Class".into(), label_ar: "الفصل".into(),
            value: class_id.to_string(), value_ar: class_id.to_string(),
        });
    }

    Ok(ReportDataset {
        report_code: "academic.student_list".into(),
        report_name: "Student List".into(),
        report_name_ar: "قائمة الطلاب".into(),
        org: org.clone(),
        parameter_summary: param_summary,
        columns: vec![
            ColumnDef {
                key: "seq".into(), header: "#".into(), header_ar: "#".into(),
                data_type: ColumnDataType::Sequence, width: Some("4%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: Some(AggregationType::Count), sensitive: false,
            },
            ColumnDef {
                key: "studentNumber".into(), header: "Student No.".into(), header_ar: "رقم الطالب".into(),
                data_type: ColumnDataType::Text, width: Some("10%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "fullName".into(), header: "Full Name".into(), header_ar: "الاسم الكامل".into(),
                data_type: ColumnDataType::Text, width: Some("18%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "nameAr".into(), header: "Name (AR)".into(), header_ar: "الاسم بالعربية".into(),
                data_type: ColumnDataType::Text, width: Some("18%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "className".into(), header: "Class".into(), header_ar: "الفصل".into(),
                data_type: ColumnDataType::Text, width: Some("12%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "enrollmentStatus".into(), header: "Status".into(), header_ar: "الحالة".into(),
                data_type: ColumnDataType::Status, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "nationalId".into(), header: "National ID".into(), header_ar: "رقم الهوية".into(),
                data_type: ColumnDataType::Text, width: Some("12%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: true,
            },
            ColumnDef {
                key: "phone".into(), header: "Phone".into(), header_ar: "الهاتف".into(),
                data_type: ColumnDataType::Text, width: Some("12%".into()),
                align: None, format: None, visible: false,
                total_type: None, sensitive: true,
            },
            ColumnDef {
                key: "birthDate".into(), header: "Birth Date".into(), header_ar: "تاريخ الميلاد".into(),
                data_type: ColumnDataType::Date, width: Some("10%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: false,
                total_type: None, sensitive: true,
            },
        ],
        rows,
        groups: if class_filter.is_none() {
            Some(vec![GroupDef {
                field: "className".into(),
                label: "Class".into(),
                label_ar: "الفصل".into(),
                show_subtotals: true,
                page_break: false,
            }])
        } else {
            None
        },
        aggregations: Some(Aggregations {
            totals: json!({ "seq": total_count }),
            subtotals: None,
        }),
        total_count,
        page: if request.pagination.is_some() { pagination.page } else { 0 },
        page_size: if request.pagination.is_some() { pagination.page_size } else { 0 },
        generated_at: now,
        generated_by: None,
    })
}
