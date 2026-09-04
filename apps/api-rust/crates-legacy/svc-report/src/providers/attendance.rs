//! Attendance Report Data Providers.
//!
//! Provides report definitions and data provider functions for:
//! - Attendance Summary (per class, per period)
//! - Daily Attendance Register

use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::json;

use crate::dataset::*;
use crate::definition::*;
use crate::error::ReportError;
use crate::org_context::OrgContext;
use crate::request::*;

/// Returns all attendance report definitions.
pub fn definitions() -> Vec<ReportDefinition> {
    vec![
        attendance_summary_definition(),
    ]
}

fn attendance_summary_definition() -> ReportDefinition {
    ReportDefinition {
        code: "attendance.summary".into(),
        name: "Attendance Summary".into(),
        name_ar: "ملخص الحضور والغياب".into(),
        domain: ReportDomain::Academic,
        category: "attendance".into(),
        description: Some("Student attendance summary by class for a period".into()),
        description_ar: Some("ملخص حضور وغياب الطلاب حسب الفصل للفترة الدراسية".into()),
        parameters: vec![
            ParameterDef {
                key: "period_id".into(),
                label: "Academic Period".into(),
                label_ar: "الفترة الدراسية".into(),
                param_type: ParamType::Id,
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
                help_text: None,
                help_text_ar: None,
                sort_order: 1,
                group: None,
            },
        ],
        required_permissions: vec!["attendance.read".into()],
        supported_formats: vec![
            OutputFormatKind::DataOnly,
            OutputFormatKind::Csv,
            OutputFormatKind::Xlsx,
            OutputFormatKind::Pdf,
        ],
        default_template_id: None,
        is_system: true,
        is_active: true,
        sort_order: 10,
        version: 1,
    }
}

/// Attendance Summary — aggregated attendance counts per student.
pub async fn attendance_summary(
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

    let now = chrono::Utc::now().to_rfc3339();

    let (sql, params) = if let Some(class_id) = class_filter {
        (r#"SELECT
                e.student_number,
                u.full_name AS student_name,
                u.name_ar AS student_name_ar,
                c.name AS class_name,
                c.name_ar AS class_name_ar,
                COUNT(a.id) AS total_days,
                SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) AS present,
                SUM(CASE WHEN a.status = 'absent' THEN 1 ELSE 0 END) AS absent,
                SUM(CASE WHEN a.status = 'late' THEN 1 ELSE 0 END) AS late,
                SUM(CASE WHEN a.status = 'excused' THEN 1 ELSE 0 END) AS excused
            FROM attendance a
            JOIN enrollments e ON a.enrollment_id = e.id
            JOIN users u ON e.student_id = u.id
            JOIN classes c ON e.class_id = c.id
            WHERE a.org_id = $1
              AND e.period_id = $2
              AND e.class_id = $3
            GROUP BY e.id, e.student_number, u.full_name, u.name_ar, c.name, c.name_ar
            ORDER BY c.name, u.full_name"#.to_string(),
        vec![org_id.into(), period_id.into(), class_id.into()])
    } else {
        (r#"SELECT
                e.student_number,
                u.full_name AS student_name,
                u.name_ar AS student_name_ar,
                c.name AS class_name,
                c.name_ar AS class_name_ar,
                COUNT(a.id) AS total_days,
                SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) AS present,
                SUM(CASE WHEN a.status = 'absent' THEN 1 ELSE 0 END) AS absent,
                SUM(CASE WHEN a.status = 'late' THEN 1 ELSE 0 END) AS late,
                SUM(CASE WHEN a.status = 'excused' THEN 1 ELSE 0 END) AS excused
            FROM attendance a
            JOIN enrollments e ON a.enrollment_id = e.id
            JOIN users u ON e.student_id = u.id
            JOIN classes c ON e.class_id = c.id
            WHERE a.org_id = $1
              AND e.period_id = $2
            GROUP BY e.id, e.student_number, u.full_name, u.name_ar, c.name, c.name_ar
            ORDER BY c.name, u.full_name"#.to_string(),
        vec![org_id.into(), period_id.into()])
    };

    let stmt = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(), &sql, params,
    );

    let rows_raw = db.query_all(stmt).await.map_err(|e| ReportError::DataProviderError(e.to_string()))?;

    let mut rows = Vec::with_capacity(rows_raw.len());
    let mut total_present: i64 = 0;
    let mut total_absent: i64 = 0;
    let mut total_late: i64 = 0;
    let mut total_excused: i64 = 0;

    for (idx, row) in rows_raw.iter().enumerate() {
        let present: i64 = row.try_get("", "present").unwrap_or(0);
        let absent: i64 = row.try_get("", "absent").unwrap_or(0);
        let late: i64 = row.try_get("", "late").unwrap_or(0);
        let excused: i64 = row.try_get("", "excused").unwrap_or(0);
        let total_days: i64 = row.try_get("", "total_days").unwrap_or(0);

        let rate = if total_days > 0 {
            (present as f64 / total_days as f64) * 100.0
        } else { 0.0 };

        total_present += present;
        total_absent += absent;
        total_late += late;
        total_excused += excused;

        rows.push(json!({
            "seq": idx + 1,
            "studentNumber": row.try_get::<String>("", "student_number").unwrap_or_default(),
            "studentName": row.try_get::<String>("", "student_name").unwrap_or_default(),
            "studentNameAr": row.try_get::<Option<String>>("", "student_name_ar").unwrap_or_default(),
            "className": row.try_get::<String>("", "class_name").unwrap_or_default(),
            "classNameAr": row.try_get::<Option<String>>("", "class_name_ar").unwrap_or_default(),
            "totalDays": total_days,
            "present": present,
            "absent": absent,
            "late": late,
            "excused": excused,
            "rate": (rate * 10.0).round() / 10.0,
        }));
    }

    let total_count = rows.len() as u64;

    Ok(ReportDataset {
        report_code: "attendance.summary".into(),
        report_name: "Attendance Summary".into(),
        report_name_ar: "ملخص الحضور والغياب".into(),
        org: org.clone(),
        parameter_summary: vec![ParameterSummaryItem {
            label: "Period".into(), label_ar: "الفترة".into(),
            value: period_id.to_string(), value_ar: period_id.to_string(),
        }],
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
                key: "studentName".into(), header: "Student Name".into(), header_ar: "اسم الطالب".into(),
                data_type: ColumnDataType::Text, width: Some("20%".into()),
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
                key: "totalDays".into(), header: "Days".into(), header_ar: "الأيام".into(),
                data_type: ColumnDataType::Number, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "present".into(), header: "Present".into(), header_ar: "حاضر".into(),
                data_type: ColumnDataType::Number, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "absent".into(), header: "Absent".into(), header_ar: "غائب".into(),
                data_type: ColumnDataType::Number, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "late".into(), header: "Late".into(), header_ar: "متأخر".into(),
                data_type: ColumnDataType::Number, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "excused".into(), header: "Excused".into(), header_ar: "مستأذن".into(),
                data_type: ColumnDataType::Number, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "rate".into(), header: "Rate %".into(), header_ar: "النسبة %".into(),
                data_type: ColumnDataType::Percentage, width: Some("10%".into()),
                align: Some(ColumnAlign::Center), format: Some("number:1".into()), visible: true,
                total_type: None, sensitive: false,
            },
        ],
        rows,
        groups: Some(vec![GroupDef {
            field: "className".into(),
            label: "Class".into(),
            label_ar: "الفصل".into(),
            show_subtotals: true,
            page_break: false,
        }]),
        aggregations: Some(Aggregations {
            totals: json!({
                "seq": total_count,
                "present": total_present,
                "absent": total_absent,
                "late": total_late,
                "excused": total_excused,
            }),
            subtotals: None,
        }),
        total_count,
        page: 0,
        page_size: 0,
        generated_at: now,
        generated_by: None,
    })
}
