//! Exam Report Data Providers.
//!
//! Provides report definitions and data provider functions for:
//! - Exam Seating Plan
//! - Exam Schedule

use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::json;

use crate::dataset::*;
use crate::definition::*;
use crate::error::ReportError;
use crate::org_context::OrgContext;
use crate::request::*;

/// Returns all exam report definitions for registry registration.
pub fn definitions() -> Vec<ReportDefinition> {
    vec![
        exam_seating_definition(),
        exam_schedule_definition(),
    ]
}

fn exam_seating_definition() -> ReportDefinition {
    ReportDefinition {
        code: "exam.seating_plan".into(),
        name: "Exam Seating Plan".into(),
        name_ar: "خطة جلوس الاختبارات".into(),
        domain: ReportDomain::Exam,
        category: "control".into(),
        description: Some("Seating arrangement for exam sessions by hall".into()),
        description_ar: Some("ترتيب الجلوس لجلسات الاختبارات حسب القاعة".into()),
        parameters: vec![
            ParameterDef {
                key: "session_id".into(),
                label: "Exam Session".into(),
                label_ar: "جلسة الاختبار".into(),
                param_type: ParamType::Id,
                required: true,
                default_value: None,
                options: None,
                entity_type: Some("exam_session".into()),
                placeholder: None,
                placeholder_ar: None,
                help_text: None,
                help_text_ar: None,
                sort_order: 0,
                group: None,
            },
        ],
        required_permissions: vec!["exam.read".into()],
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

fn exam_schedule_definition() -> ReportDefinition {
    ReportDefinition {
        code: "exam.schedule".into(),
        name: "Exam Schedule".into(),
        name_ar: "جدول الاختبارات".into(),
        domain: ReportDomain::Exam,
        category: "schedule".into(),
        description: Some("Complete exam timetable for a session".into()),
        description_ar: Some("الجدول الزمني الكامل للاختبارات للجلسة".into()),
        parameters: vec![
            ParameterDef {
                key: "session_id".into(),
                label: "Exam Session".into(),
                label_ar: "جلسة الاختبار".into(),
                param_type: ParamType::Id,
                required: true,
                default_value: None,
                options: None,
                entity_type: Some("exam_session".into()),
                placeholder: None,
                placeholder_ar: None,
                help_text: None,
                help_text_ar: None,
                sort_order: 0,
                group: None,
            },
        ],
        required_permissions: vec!["exam.read".into()],
        supported_formats: vec![
            OutputFormatKind::DataOnly,
            OutputFormatKind::Csv,
            OutputFormatKind::Xlsx,
            OutputFormatKind::Pdf,
        ],
        default_template_id: None,
        is_system: true,
        is_active: true,
        sort_order: 20,
        version: 1,
    }
}

// ═══════════════════════════════════════════════════════════════
// Data Providers
// ═══════════════════════════════════════════════════════════════

/// Exam Seating Plan — students assigned to halls with seat numbers.
pub async fn seating_plan(
    db: &DatabaseConnection,
    org_id: &str,
    org: &OrgContext,
    request: &ReportRequest,
) -> Result<ReportDataset, ReportError> {
    let session_id = request.parameters.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ReportError::MissingParameter("session_id".into()))?;

    let now = chrono::Utc::now().to_rfc3339();

    let sql = r#"SELECT
            sa.seat_number,
            sa.hall_id,
            h.name AS hall_name,
            h.name_ar AS hall_name_ar,
            u.full_name AS student_name,
            u.name_ar AS student_name_ar,
            e.student_number,
            c.name AS class_name,
            c.name_ar AS class_name_ar
        FROM seating_assignments sa
        JOIN exam_halls h ON sa.hall_id = h.id
        JOIN users u ON sa.student_id = u.id
        JOIN enrollments e ON sa.enrollment_id = e.id
        JOIN classes c ON e.class_id = c.id
        WHERE sa.session_id = $1
          AND sa.org_id = $2
        ORDER BY h.name, sa.seat_number"#;

    let stmt = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(), sql,
        vec![session_id.into(), org_id.into()],
    );

    let rows_raw = db.query_all(stmt).await.map_err(|e| ReportError::DataProviderError(e.to_string()))?;

    let mut rows = Vec::with_capacity(rows_raw.len());
    for (idx, row) in rows_raw.iter().enumerate() {
        rows.push(json!({
            "seq": idx + 1,
            "seatNumber": row.try_get::<i32>("", "seat_number").unwrap_or(0),
            "hallName": row.try_get::<String>("", "hall_name").unwrap_or_default(),
            "hallNameAr": row.try_get::<Option<String>>("", "hall_name_ar").unwrap_or_default(),
            "studentNumber": row.try_get::<String>("", "student_number").unwrap_or_default(),
            "studentName": row.try_get::<String>("", "student_name").unwrap_or_default(),
            "studentNameAr": row.try_get::<Option<String>>("", "student_name_ar").unwrap_or_default(),
            "className": row.try_get::<String>("", "class_name").unwrap_or_default(),
            "classNameAr": row.try_get::<Option<String>>("", "class_name_ar").unwrap_or_default(),
        }));
    }

    let total_count = rows.len() as u64;

    Ok(ReportDataset {
        report_code: "exam.seating_plan".into(),
        report_name: "Exam Seating Plan".into(),
        report_name_ar: "خطة جلوس الاختبارات".into(),
        org: org.clone(),
        parameter_summary: vec![ParameterSummaryItem {
            label: "Session".into(), label_ar: "الجلسة".into(),
            value: session_id.to_string(), value_ar: session_id.to_string(),
        }],
        columns: vec![
            ColumnDef {
                key: "seq".into(), header: "#".into(), header_ar: "#".into(),
                data_type: ColumnDataType::Sequence, width: Some("5%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: Some(AggregationType::Count), sensitive: false,
            },
            ColumnDef {
                key: "seatNumber".into(), header: "Seat".into(), header_ar: "المقعد".into(),
                data_type: ColumnDataType::Number, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "hallName".into(), header: "Hall".into(), header_ar: "القاعة".into(),
                data_type: ColumnDataType::Text, width: Some("15%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "studentNumber".into(), header: "Student No.".into(), header_ar: "رقم الطالب".into(),
                data_type: ColumnDataType::Text, width: Some("12%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "studentName".into(), header: "Student Name".into(), header_ar: "اسم الطالب".into(),
                data_type: ColumnDataType::Text, width: Some("30%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "className".into(), header: "Class".into(), header_ar: "الفصل".into(),
                data_type: ColumnDataType::Text, width: Some("15%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
        ],
        rows,
        groups: Some(vec![GroupDef {
            field: "hallName".into(),
            label: "Hall".into(),
            label_ar: "القاعة".into(),
            show_subtotals: true,
            page_break: true,
        }]),
        aggregations: Some(Aggregations {
            totals: json!({"seq": total_count}),
            subtotals: None,
        }),
        total_count,
        page: 0,
        page_size: 0,
        generated_at: now,
        generated_by: None,
    })
}
