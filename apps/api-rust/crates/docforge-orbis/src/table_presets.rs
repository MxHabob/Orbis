//! Table Presets — reusable table configurations for common document types.
//!
//! These factory functions return `TableProps` with pre-configured column
//! definitions for standard educational document tables. They are NOT
//! hard-coded into templates — they serve as default starting points
//! that institutions can customize via the template editor.
//!
//! Each preset returns a standalone `TableProps` that can be serialized
//! into a DocForge template's `TableNode.props`.

use docforge_engine::model::nodes::{TableColumnDef, TableProps, TableStyle};

/// Grades Table preset — subjects as rows, assessment results as columns.
///
/// Default school layout:
///   المادة | أعمال السنة | الاختبار | المجموع | التقدير | الترتيب
pub fn grades_table() -> TableProps {
    TableProps {
        columns: vec![
            TableColumnDef {
                id: "subject".into(),
                header: "Subject".into(),
                header_ar: Some("المادة".into()),
                field: "subjectName".into(),
                width: Some("30%".into()),
                alignment: Some("right".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "coursework".into(),
                header: "Coursework".into(),
                header_ar: Some("أعمال السنة".into()),
                field: "courseworkScore".into(),
                width: Some("12%".into()),
                alignment: Some("center".into()),
                format: Some("number:1".into()),
                is_visible: true,
                total_type: Some("sum".into()),
            },
            TableColumnDef {
                id: "exam".into(),
                header: "Exam".into(),
                header_ar: Some("الاختبار".into()),
                field: "examScore".into(),
                width: Some("12%".into()),
                alignment: Some("center".into()),
                format: Some("number:1".into()),
                is_visible: true,
                total_type: Some("sum".into()),
            },
            TableColumnDef {
                id: "total".into(),
                header: "Total".into(),
                header_ar: Some("المجموع".into()),
                field: "totalScore".into(),
                width: Some("12%".into()),
                alignment: Some("center".into()),
                format: Some("number:1".into()),
                is_visible: true,
                total_type: Some("sum".into()),
            },
            TableColumnDef {
                id: "grade".into(),
                header: "Grade".into(),
                header_ar: Some("التقدير".into()),
                field: "gradeLabel".into(),
                width: Some("15%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "rank".into(),
                header: "Rank".into(),
                header_ar: Some("الترتيب".into()),
                field: "rank".into(),
                width: Some("10%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
        ],
        data_source: Some("term_certificate.subjects".into()),
        rows: None,
        show_header: true,
        show_totals: true,
        show_subtotals: false,
        group_by: None,
        sort_by: None,
        repeating_header: true,
        style: Some(TableStyle {
            border_color: Some("#333333".into()),
            border_width: Some(1.0),
            header_bg_color: Some("#2c5f2d".into()),
            header_text_color: Some("#ffffff".into()),
            row_bg_color: Some("#ffffff".into()),
            alt_row_bg_color: Some("#f5f5f5".into()),
            font_size: Some(10.0),
            row_height: Some(24.0),
            padding: Some(4.0),
        }),
        conditional_formatting: Some(vec![docforge_engine::model::nodes::ConditionalFormat {
            field: "isPassing".into(),
            operator: "eq".into(),
            value: serde_json::json!(false),
            style: serde_json::json!({ "color": "#cc0000", "fontWeight": "bold" }),
        }]),
    }
}

/// Attendance Table preset — student names × dates grid.
pub fn attendance_table() -> TableProps {
    TableProps {
        columns: vec![
            TableColumnDef {
                id: "seq".into(),
                header: "#".into(),
                header_ar: Some("م".into()),
                field: "sequenceNumber".into(),
                width: Some("8%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "name".into(),
                header: "Student Name".into(),
                header_ar: Some("اسم الطالب".into()),
                field: "studentName".into(),
                width: Some("30%".into()),
                alignment: Some("right".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            // Date columns are injected dynamically by the template
        ],
        data_source: Some("enrollment.students".into()),
        rows: None,
        show_header: true,
        show_totals: false,
        show_subtotals: false,
        group_by: None,
        sort_by: Some("studentName".into()),
        repeating_header: true,
        style: Some(TableStyle {
            border_color: Some("#666666".into()),
            border_width: Some(0.5),
            header_bg_color: Some("#4a86c8".into()),
            header_text_color: Some("#ffffff".into()),
            row_bg_color: Some("#ffffff".into()),
            alt_row_bg_color: Some("#f0f4f8".into()),
            font_size: Some(9.0),
            row_height: Some(22.0),
            padding: Some(3.0),
        }),
        conditional_formatting: None,
    }
}

/// Exam Seating Table preset — seating plan for exam halls.
pub fn exam_seating_table() -> TableProps {
    TableProps {
        columns: vec![
            TableColumnDef {
                id: "seat".into(),
                header: "Seat No.".into(),
                header_ar: Some("رقم الجلوس".into()),
                field: "seatingNumber".into(),
                width: Some("15%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "name".into(),
                header: "Student Name".into(),
                header_ar: Some("اسم الطالب".into()),
                field: "studentNameAr".into(),
                width: Some("35%".into()),
                alignment: Some("right".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "level".into(),
                header: "Level".into(),
                header_ar: Some("المستوى".into()),
                field: "level".into(),
                width: Some("15%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "division".into(),
                header: "Division".into(),
                header_ar: Some("الشعبة".into()),
                field: "division".into(),
                width: Some("15%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
        ],
        data_source: Some("exam_session.students".into()),
        rows: None,
        show_header: true,
        show_totals: false,
        show_subtotals: false,
        group_by: Some("hallId".into()),
        sort_by: Some("seatingNumber".into()),
        repeating_header: true,
        style: Some(TableStyle {
            border_color: Some("#333333".into()),
            border_width: Some(1.0),
            header_bg_color: Some("#1a1a2e".into()),
            header_text_color: Some("#ffffff".into()),
            row_bg_color: Some("#ffffff".into()),
            alt_row_bg_color: Some("#eef2f7".into()),
            font_size: Some(10.0),
            row_height: Some(24.0),
            padding: Some(4.0),
        }),
        conditional_formatting: None,
    }
}

/// Committee/Proctor Table preset — exam staff listing.
pub fn committee_table() -> TableProps {
    TableProps {
        columns: vec![
            TableColumnDef {
                id: "seq".into(),
                header: "#".into(),
                header_ar: Some("م".into()),
                field: "_index".into(),
                width: Some("8%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: Some("count".into()),
            },
            TableColumnDef {
                id: "name".into(),
                header: "Name".into(),
                header_ar: Some("الاسم".into()),
                field: "nameAr".into(),
                width: Some("35%".into()),
                alignment: Some("right".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "role".into(),
                header: "Role".into(),
                header_ar: Some("الدور".into()),
                field: "role".into(),
                width: Some("25%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "signature".into(),
                header: "Signature".into(),
                header_ar: Some("التوقيع".into()),
                field: "_empty".into(),
                width: Some("25%".into()),
                alignment: Some("center".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
        ],
        data_source: Some("exam_session.proctors".into()),
        rows: None,
        show_header: true,
        show_totals: true,
        show_subtotals: false,
        group_by: None,
        sort_by: Some("role".into()),
        repeating_header: false,
        style: Some(TableStyle {
            border_color: Some("#333333".into()),
            border_width: Some(1.0),
            header_bg_color: Some("#2c5f2d".into()),
            header_text_color: Some("#ffffff".into()),
            row_bg_color: Some("#ffffff".into()),
            alt_row_bg_color: Some("#f5f5f5".into()),
            font_size: Some(11.0),
            row_height: Some(28.0),
            padding: Some(4.0),
        }),
        conditional_formatting: None,
    }
}

/// Financial Table preset — transactions with amounts.
pub fn financial_table() -> TableProps {
    TableProps {
        columns: vec![
            TableColumnDef {
                id: "date".into(),
                header: "Date".into(),
                header_ar: Some("التاريخ".into()),
                field: "date".into(),
                width: Some("15%".into()),
                alignment: Some("center".into()),
                format: Some("date".into()),
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "description".into(),
                header: "Description".into(),
                header_ar: Some("البيان".into()),
                field: "description".into(),
                width: Some("35%".into()),
                alignment: Some("right".into()),
                format: None,
                is_visible: true,
                total_type: None,
            },
            TableColumnDef {
                id: "amount".into(),
                header: "Amount".into(),
                header_ar: Some("المبلغ".into()),
                field: "amount".into(),
                width: Some("15%".into()),
                alignment: Some("center".into()),
                format: Some("number:2".into()),
                is_visible: true,
                total_type: Some("sum".into()),
            },
            TableColumnDef {
                id: "balance".into(),
                header: "Balance".into(),
                header_ar: Some("الرصيد".into()),
                field: "balance".into(),
                width: Some("15%".into()),
                alignment: Some("center".into()),
                format: Some("number:2".into()),
                is_visible: true,
                total_type: None,
            },
        ],
        data_source: Some("payment.transactions".into()),
        rows: None,
        show_header: true,
        show_totals: true,
        show_subtotals: false,
        group_by: None,
        sort_by: Some("date".into()),
        repeating_header: true,
        style: Some(TableStyle {
            border_color: Some("#666666".into()),
            border_width: Some(0.5),
            header_bg_color: Some("#34495e".into()),
            header_text_color: Some("#ffffff".into()),
            row_bg_color: Some("#ffffff".into()),
            alt_row_bg_color: Some("#f9f9f9".into()),
            font_size: Some(10.0),
            row_height: Some(22.0),
            padding: Some(4.0),
        }),
        conditional_formatting: None,
    }
}

/// Get a table preset by name.
pub fn get_preset(name: &str) -> Option<TableProps> {
    match name {
        "grades" | "grades_table" => Some(grades_table()),
        "attendance" | "attendance_table" => Some(attendance_table()),
        "exam_seating" | "seating_table" => Some(exam_seating_table()),
        "committee" | "committee_table" | "proctors" => Some(committee_table()),
        "financial" | "financial_table" | "payments" => Some(financial_table()),
        _ => None,
    }
}

/// List all available preset names.
pub fn list_presets() -> Vec<&'static str> {
    vec![
        "grades",
        "attendance",
        "exam_seating",
        "committee",
        "financial",
    ]
}
