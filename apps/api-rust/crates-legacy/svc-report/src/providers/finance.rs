//! Finance Report Data Providers.
//!
//! Provides report definitions and data provider functions for:
//! - Fee Collection Summary
//! - Unpaid Fees
//! - Payment Register

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

/// Returns all finance report definitions for registry registration.
pub fn definitions() -> Vec<ReportDefinition> {
    vec![
        fee_collection_definition(),
        unpaid_fees_definition(),
        payment_register_definition(),
    ]
}

fn fee_collection_definition() -> ReportDefinition {
    ReportDefinition {
        code: "finance.fee_collection".into(),
        name: "Fee Collection Summary".into(),
        name_ar: "ملخص تحصيل الرسوم".into(),
        domain: ReportDomain::Finance,
        category: "fees".into(),
        description: Some("Summary of fee assignments, discounts, and collection status per student".into()),
        description_ar: Some("ملخص تعيينات الرسوم والخصومات وحالة التحصيل لكل طالب".into()),
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
                placeholder: Some("Select period".into()),
                placeholder_ar: Some("اختر الفترة".into()),
                help_text: None,
                help_text_ar: None,
                sort_order: 0,
                group: None,
            },
            ParameterDef {
                key: "status".into(),
                label: "Fee Status".into(),
                label_ar: "حالة الرسوم".into(),
                param_type: ParamType::Enum,
                required: false,
                default_value: None,
                options: Some(vec![
                    ParamOption { value: "pending".into(), label: "Pending".into(), label_ar: "معلق".into() },
                    ParamOption { value: "partial".into(), label: "Partially Paid".into(), label_ar: "مدفوع جزئياً".into() },
                    ParamOption { value: "paid".into(), label: "Fully Paid".into(), label_ar: "مدفوع بالكامل".into() },
                    ParamOption { value: "overdue".into(), label: "Overdue".into(), label_ar: "متأخر".into() },
                ]),
                entity_type: None,
                placeholder: Some("All statuses".into()),
                placeholder_ar: Some("كل الحالات".into()),
                help_text: None,
                help_text_ar: None,
                sort_order: 1,
                group: None,
            },
        ],
        required_permissions: vec!["finance.read".into()],
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

fn unpaid_fees_definition() -> ReportDefinition {
    ReportDefinition {
        code: "finance.unpaid_fees".into(),
        name: "Unpaid Fees Report".into(),
        name_ar: "تقرير الرسوم غير المدفوعة".into(),
        domain: ReportDomain::Finance,
        category: "fees".into(),
        description: Some("List of all students with outstanding fee balances".into()),
        description_ar: Some("قائمة بجميع الطلاب الذين لديهم أرصدة رسوم مستحقة".into()),
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
                key: "min_balance".into(),
                label: "Minimum Balance".into(),
                label_ar: "الحد الأدنى للرصيد".into(),
                param_type: ParamType::Decimal,
                required: false,
                default_value: Some(json!(0)),
                options: None,
                entity_type: None,
                placeholder: Some("0.00".into()),
                placeholder_ar: Some("0.00".into()),
                help_text: Some("Only show students with balance >= this amount".into()),
                help_text_ar: Some("عرض الطلاب الذين رصيدهم >= هذا المبلغ فقط".into()),
                sort_order: 1,
                group: None,
            },
        ],
        required_permissions: vec!["finance.read".into()],
        supported_formats: vec![
            OutputFormatKind::DataOnly,
            OutputFormatKind::Csv,
            OutputFormatKind::Xlsx,
            OutputFormatKind::Json,
        ],
        default_template_id: None,
        is_system: true,
        is_active: true,
        sort_order: 20,
        version: 1,
    }
}

fn payment_register_definition() -> ReportDefinition {
    ReportDefinition {
        code: "finance.payment_register".into(),
        name: "Payment Register".into(),
        name_ar: "سجل المدفوعات".into(),
        domain: ReportDomain::Finance,
        category: "payments".into(),
        description: Some("Chronological register of all payment transactions".into()),
        description_ar: Some("سجل زمني لجميع عمليات الدفع".into()),
        parameters: vec![
            ParameterDef {
                key: "date_range".into(),
                label: "Date Range".into(),
                label_ar: "نطاق التاريخ".into(),
                param_type: ParamType::DateRange,
                required: true,
                default_value: None,
                options: None,
                entity_type: None,
                placeholder: None,
                placeholder_ar: None,
                help_text: None,
                help_text_ar: None,
                sort_order: 0,
                group: None,
            },
            ParameterDef {
                key: "method".into(),
                label: "Payment Method".into(),
                label_ar: "طريقة الدفع".into(),
                param_type: ParamType::Enum,
                required: false,
                default_value: None,
                options: Some(vec![
                    ParamOption { value: "cash".into(), label: "Cash".into(), label_ar: "نقداً".into() },
                    ParamOption { value: "bank_transfer".into(), label: "Bank Transfer".into(), label_ar: "تحويل بنكي".into() },
                    ParamOption { value: "check".into(), label: "Check".into(), label_ar: "شيك".into() },
                    ParamOption { value: "card".into(), label: "Card".into(), label_ar: "بطاقة".into() },
                    ParamOption { value: "online".into(), label: "Online".into(), label_ar: "إلكتروني".into() },
                ]),
                entity_type: None,
                placeholder: Some("All methods".into()),
                placeholder_ar: Some("كل الطرق".into()),
                help_text: None,
                help_text_ar: None,
                sort_order: 1,
                group: None,
            },
        ],
        required_permissions: vec!["finance.read".into()],
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
        sort_order: 30,
        version: 1,
    }
}

// ═══════════════════════════════════════════════════════════════
// Data Providers
// ═══════════════════════════════════════════════════════════════

/// Fee Collection Summary — joins student_fees with enrollments and users
/// to produce a per-student fee summary with totals.
pub async fn fee_collection(
    db: &DatabaseConnection,
    org_id: &str,
    org: &OrgContext,
    request: &ReportRequest,
) -> Result<ReportDataset, ReportError> {
    let period_id = request.parameters.get("period_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ReportError::MissingParameter("period_id".into()))?;

    let status_filter = request.parameters.get("status")
        .and_then(|v| v.as_str());

    let now = chrono::Utc::now().to_rfc3339();

    // Build the query with JOIN to get student names
    let mut sql = String::from(
        r#"SELECT
            sf.id,
            sf.enrollment_id,
            e.student_number,
            u.full_name AS student_name,
            u.name_ar AS student_name_ar,
            ft.name AS fee_name,
            ft.name_ar AS fee_name_ar,
            sf.original_amount,
            sf.discount_amount,
            sf.net_amount,
            sf.status,
            sf.due_date,
            sf.installment_number
        FROM student_fees sf
        JOIN enrollments e ON sf.enrollment_id = e.id
        JOIN users u ON e.student_id = u.id
        LEFT JOIN fee_templates ft ON sf.fee_template_id = ft.id
        WHERE sf.org_id = $1
          AND sf.period_id = $2"#
    );

    let mut param_idx = 3;
    let mut params: Vec<sea_orm::Value> = vec![org_id.into(), period_id.into()];

    if let Some(status) = status_filter {
        sql.push_str(&format!(" AND sf.status = ${}", param_idx));
        params.push(status.into());
        param_idx += 1;
    }
    let _ = param_idx; // suppress unused warning

    sql.push_str(" ORDER BY u.full_name, sf.due_date");

    let stmt = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(),
        &sql,
        params,
    );

    let rows_raw = db.query_all(stmt).await.map_err(|e| ReportError::DataProviderError(e.to_string()))?;

    // Transform to JSON rows
    let mut rows = Vec::with_capacity(rows_raw.len());
    let mut total_original = 0.0f64;
    let mut total_discount = 0.0f64;
    let mut total_net = 0.0f64;

    for (idx, row) in rows_raw.iter().enumerate() {
        let original: f64 = row.try_get("", "original_amount").unwrap_or(0.0);
        let discount: f64 = row.try_get("", "discount_amount").unwrap_or(0.0);
        let net: f64 = row.try_get("", "net_amount").unwrap_or(0.0);

        total_original += original;
        total_discount += discount;
        total_net += net;

        rows.push(json!({
            "seq": idx + 1,
            "studentNumber": row.try_get::<String>("", "student_number").unwrap_or_default(),
            "studentName": row.try_get::<String>("", "student_name").unwrap_or_default(),
            "studentNameAr": row.try_get::<Option<String>>("", "student_name_ar").unwrap_or_default(),
            "feeName": row.try_get::<Option<String>>("", "fee_name").unwrap_or_default(),
            "feeNameAr": row.try_get::<Option<String>>("", "fee_name_ar").unwrap_or_default(),
            "originalAmount": original,
            "discountAmount": discount,
            "netAmount": net,
            "status": row.try_get::<String>("", "status").unwrap_or_default(),
            "dueDate": row.try_get::<String>("", "due_date").unwrap_or_default(),
        }));
    }

    let total_count = rows.len() as u64;

    // Build parameter summary
    let mut param_summary = vec![
        ParameterSummaryItem {
            label: "Academic Period".into(),
            label_ar: "الفترة الدراسية".into(),
            value: period_id.to_string(),
            value_ar: period_id.to_string(),
        },
    ];
    if let Some(status) = status_filter {
        param_summary.push(ParameterSummaryItem {
            label: "Status".into(),
            label_ar: "الحالة".into(),
            value: status.to_string(),
            value_ar: status.to_string(),
        });
    }

    Ok(ReportDataset {
        report_code: "finance.fee_collection".into(),
        report_name: "Fee Collection Summary".into(),
        report_name_ar: "ملخص تحصيل الرسوم".into(),
        org: org.clone(),
        parameter_summary: param_summary,
        columns: vec![
            ColumnDef {
                key: "seq".into(), header: "#".into(), header_ar: "#".into(),
                data_type: ColumnDataType::Sequence, width: Some("5%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
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
                key: "feeName".into(), header: "Fee Type".into(), header_ar: "نوع الرسوم".into(),
                data_type: ColumnDataType::Text, width: Some("15%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "originalAmount".into(), header: "Original".into(), header_ar: "المبلغ الأصلي".into(),
                data_type: ColumnDataType::Currency, width: Some("12%".into()),
                align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "discountAmount".into(), header: "Discount".into(), header_ar: "الخصم".into(),
                data_type: ColumnDataType::Currency, width: Some("10%".into()),
                align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "netAmount".into(), header: "Net Amount".into(), header_ar: "المبلغ الصافي".into(),
                data_type: ColumnDataType::Currency, width: Some("12%".into()),
                align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "status".into(), header: "Status".into(), header_ar: "الحالة".into(),
                data_type: ColumnDataType::Status, width: Some("8%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "dueDate".into(), header: "Due Date".into(), header_ar: "تاريخ الاستحقاق".into(),
                data_type: ColumnDataType::Date, width: Some("10%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
        ],
        rows,
        groups: None,
        aggregations: Some(Aggregations {
            totals: json!({
                "originalAmount": total_original,
                "discountAmount": total_discount,
                "netAmount": total_net,
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

/// Payment Register — chronological list of all payments in a date range.
pub async fn payment_register(
    db: &DatabaseConnection,
    org_id: &str,
    org: &OrgContext,
    request: &ReportRequest,
) -> Result<ReportDataset, ReportError> {
    let (from_date, to_date) = request.parameters.get("date_range")
        .and_then(|v| v.as_date_range())
        .ok_or_else(|| ReportError::MissingParameter("date_range".into()))?;

    let method_filter = request.parameters.get("method")
        .and_then(|v| v.as_str());

    let now = chrono::Utc::now().to_rfc3339();

    let mut sql = String::from(
        r#"SELECT
            p.id,
            p.receipt_number,
            p.amount,
            p.currency,
            p.method,
            p.payer_name,
            p.payment_type,
            p.notes,
            p.paid_at,
            p.created_by
        FROM payments p
        WHERE p.org_id = $1
          AND p.paid_at >= $2
          AND p.paid_at <= $3"#
    );

    let mut params: Vec<sea_orm::Value> = vec![org_id.into(), from_date.into(), to_date.into()];
    let mut param_idx = 4;

    if let Some(method) = method_filter {
        sql.push_str(&format!(" AND p.method = ${}", param_idx));
        params.push(method.into());
        param_idx += 1;
    }
    let _ = param_idx;

    sql.push_str(" ORDER BY p.paid_at DESC");

    let stmt = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(), &sql, params,
    );

    let rows_raw = db.query_all(stmt).await.map_err(|e| ReportError::DataProviderError(e.to_string()))?;

    let mut rows = Vec::with_capacity(rows_raw.len());
    let mut total_amount = 0.0f64;

    for (idx, row) in rows_raw.iter().enumerate() {
        let amount: f64 = row.try_get("", "amount").unwrap_or(0.0);
        total_amount += amount;

        rows.push(json!({
            "seq": idx + 1,
            "receiptNumber": row.try_get::<Option<String>>("", "receipt_number").unwrap_or_default(),
            "amount": amount,
            "currency": row.try_get::<String>("", "currency").unwrap_or_default(),
            "method": row.try_get::<String>("", "method").unwrap_or_default(),
            "payerName": row.try_get::<Option<String>>("", "payer_name").unwrap_or_default(),
            "paymentType": row.try_get::<String>("", "payment_type").unwrap_or_default(),
            "notes": row.try_get::<Option<String>>("", "notes").unwrap_or_default(),
            "paidAt": row.try_get::<String>("", "paid_at").unwrap_or_default(),
        }));
    }

    let total_count = rows.len() as u64;

    let mut param_summary = vec![
        ParameterSummaryItem {
            label: "From".into(), label_ar: "من".into(),
            value: from_date.to_string(), value_ar: from_date.to_string(),
        },
        ParameterSummaryItem {
            label: "To".into(), label_ar: "إلى".into(),
            value: to_date.to_string(), value_ar: to_date.to_string(),
        },
    ];
    if let Some(method) = method_filter {
        param_summary.push(ParameterSummaryItem {
            label: "Method".into(), label_ar: "الطريقة".into(),
            value: method.to_string(), value_ar: method.to_string(),
        });
    }

    Ok(ReportDataset {
        report_code: "finance.payment_register".into(),
        report_name: "Payment Register".into(),
        report_name_ar: "سجل المدفوعات".into(),
        org: org.clone(),
        parameter_summary: param_summary,
        columns: vec![
            ColumnDef {
                key: "seq".into(), header: "#".into(), header_ar: "#".into(),
                data_type: ColumnDataType::Sequence, width: Some("5%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "receiptNumber".into(), header: "Receipt #".into(), header_ar: "رقم السند".into(),
                data_type: ColumnDataType::Text, width: Some("12%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "payerName".into(), header: "Payer".into(), header_ar: "الدافع".into(),
                data_type: ColumnDataType::Text, width: Some("20%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "amount".into(), header: "Amount".into(), header_ar: "المبلغ".into(),
                data_type: ColumnDataType::Currency, width: Some("15%".into()),
                align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "method".into(), header: "Method".into(), header_ar: "الطريقة".into(),
                data_type: ColumnDataType::Text, width: Some("12%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "paymentType".into(), header: "Type".into(), header_ar: "النوع".into(),
                data_type: ColumnDataType::Text, width: Some("10%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "paidAt".into(), header: "Date".into(), header_ar: "التاريخ".into(),
                data_type: ColumnDataType::DateTime, width: Some("15%".into()),
                align: Some(ColumnAlign::Center), format: None, visible: true,
                total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "notes".into(), header: "Notes".into(), header_ar: "ملاحظات".into(),
                data_type: ColumnDataType::Text, width: Some("15%".into()),
                align: None, format: None, visible: false,
                total_type: None, sensitive: false,
            },
        ],
        rows,
        groups: None,
        aggregations: Some(Aggregations {
            totals: json!({ "amount": total_amount }),
            subtotals: None,
        }),
        total_count,
        page: 0,
        page_size: 0,
        generated_at: now,
        generated_by: None,
    })
}

/// Unpaid Fees — students with outstanding balances, aggregated per student.
pub async fn unpaid_fees(
    db: &DatabaseConnection,
    org_id: &str,
    org: &OrgContext,
    request: &ReportRequest,
) -> Result<ReportDataset, ReportError> {
    let period_id = request.parameters.get("period_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ReportError::MissingParameter("period_id".into()))?;

    let min_balance = request.parameters.get("min_balance")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let now = chrono::Utc::now().to_rfc3339();

    // Aggregate fees and payments per student to compute outstanding balances
    let sql = r#"SELECT
            e.student_number,
            u.full_name AS student_name,
            u.name_ar AS student_name_ar,
            u.phone,
            c.name AS class_name,
            c.name_ar AS class_name_ar,
            SUM(sf.net_amount) AS total_fees,
            COALESCE((
                SELECT SUM(p.amount)
                FROM payments p
                WHERE p.enrollment_id = e.id
                  AND p.org_id = $1
            ), 0) AS total_paid,
            SUM(sf.net_amount) - COALESCE((
                SELECT SUM(p.amount)
                FROM payments p
                WHERE p.enrollment_id = e.id
                  AND p.org_id = $1
            ), 0) AS balance
        FROM student_fees sf
        JOIN enrollments e ON sf.enrollment_id = e.id
        JOIN users u ON e.student_id = u.id
        JOIN classes c ON e.class_id = c.id
        WHERE sf.org_id = $1
          AND sf.period_id = $2
        GROUP BY e.id, e.student_number, u.full_name, u.name_ar, u.phone,
                 c.name, c.name_ar
        HAVING (SUM(sf.net_amount) - COALESCE((
            SELECT SUM(p.amount)
            FROM payments p
            WHERE p.enrollment_id = e.id
              AND p.org_id = $1
        ), 0)) > $3
        ORDER BY balance DESC"#;

    let stmt = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(),
        sql,
        vec![org_id.into(), period_id.into(), min_balance.into()],
    );

    let rows_raw = db.query_all(stmt).await.map_err(|e| ReportError::DataProviderError(e.to_string()))?;

    let mut rows = Vec::with_capacity(rows_raw.len());
    let mut grand_total_fees = 0.0f64;
    let mut grand_total_paid = 0.0f64;
    let mut grand_balance = 0.0f64;

    for (idx, row) in rows_raw.iter().enumerate() {
        let total_fees: f64 = row.try_get("", "total_fees").unwrap_or(0.0);
        let total_paid: f64 = row.try_get("", "total_paid").unwrap_or(0.0);
        let balance: f64 = row.try_get("", "balance").unwrap_or(0.0);

        grand_total_fees += total_fees;
        grand_total_paid += total_paid;
        grand_balance += balance;

        rows.push(json!({
            "seq": idx + 1,
            "studentNumber": row.try_get::<String>("", "student_number").unwrap_or_default(),
            "studentName": row.try_get::<String>("", "student_name").unwrap_or_default(),
            "studentNameAr": row.try_get::<Option<String>>("", "student_name_ar").unwrap_or_default(),
            "className": row.try_get::<String>("", "class_name").unwrap_or_default(),
            "classNameAr": row.try_get::<Option<String>>("", "class_name_ar").unwrap_or_default(),
            "phone": row.try_get::<Option<String>>("", "phone").unwrap_or_default(),
            "totalFees": total_fees,
            "totalPaid": total_paid,
            "balance": balance,
        }));
    }

    let total_count = rows.len() as u64;

    let mut param_summary = vec![
        ParameterSummaryItem {
            label: "Academic Period".into(), label_ar: "الفترة الدراسية".into(),
            value: period_id.to_string(), value_ar: period_id.to_string(),
        },
    ];
    if min_balance > 0.0 {
        param_summary.push(ParameterSummaryItem {
            label: "Minimum Balance".into(), label_ar: "الحد الأدنى للرصيد".into(),
            value: format!("{:.2}", min_balance), value_ar: format!("{:.2}", min_balance),
        });
    }

    Ok(ReportDataset {
        report_code: "finance.unpaid_fees".into(),
        report_name: "Unpaid Fees Report".into(),
        report_name_ar: "تقرير الرسوم غير المدفوعة".into(),
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
                key: "studentName".into(), header: "Student Name".into(), header_ar: "اسم الطالب".into(),
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
                key: "phone".into(), header: "Phone".into(), header_ar: "الهاتف".into(),
                data_type: ColumnDataType::Text, width: Some("12%".into()),
                align: None, format: None, visible: true,
                total_type: None, sensitive: true,
            },
            ColumnDef {
                key: "totalFees".into(), header: "Total Fees".into(), header_ar: "إجمالي الرسوم".into(),
                data_type: ColumnDataType::Currency, width: Some("12%".into()),
                align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "totalPaid".into(), header: "Paid".into(), header_ar: "المدفوع".into(),
                data_type: ColumnDataType::Currency, width: Some("12%".into()),
                align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
            },
            ColumnDef {
                key: "balance".into(), header: "Outstanding".into(), header_ar: "المتبقي".into(),
                data_type: ColumnDataType::Currency, width: Some("12%".into()),
                align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true,
                total_type: Some(AggregationType::Sum), sensitive: false,
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
                "totalFees": grand_total_fees,
                "totalPaid": grand_total_paid,
                "balance": grand_balance,
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
