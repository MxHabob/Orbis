//! CSV Exporter — converts `ReportDataset` to UTF-8 CSV with BOM.
//!
//! Features:
//! - UTF-8 BOM prefix for Arabic text compatibility in Excel
//! - Bilingual column headers (selected by locale)
//! - Totals row appended when aggregations are present
//! - Respects column visibility

use crate::dataset::{ExportResult, ReportDataset, ColumnDataType};
use crate::error::ReportError;
use crate::request::ReportLocale;

/// Export a `ReportDataset` to CSV format.
///
/// Returns an `ExportResult` containing the CSV bytes with UTF-8 BOM,
/// appropriate MIME type, and a suggested filename.
pub fn export(
    dataset: &ReportDataset,
    locale: &ReportLocale,
) -> Result<ExportResult, ReportError> {
    let mut output = Vec::new();

    // UTF-8 BOM — ensures Excel opens the file with correct encoding
    output.extend_from_slice(b"\xEF\xBB\xBF");

    // Collect visible columns
    let visible_cols: Vec<_> = dataset.columns.iter()
        .filter(|c| c.visible)
        .collect();

    // Write header row
    let headers: Vec<String> = visible_cols.iter().map(|col| {
        let header = match locale {
            ReportLocale::Ar | ReportLocale::ArEn => &col.header_ar,
            ReportLocale::En | ReportLocale::EnAr => &col.header,
        };
        csv_escape(header)
    }).collect();

    output.extend_from_slice(headers.join(",").as_bytes());
    output.push(b'\n');

    // Write data rows
    for row in &dataset.rows {
        let cells: Vec<String> = visible_cols.iter().map(|col| {
            let val = &row[&col.key];
            format_csv_cell(val, &col.data_type)
        }).collect();

        output.extend_from_slice(cells.join(",").as_bytes());
        output.push(b'\n');
    }

    // Write totals row if aggregations present
    if let Some(ref agg) = dataset.aggregations {
        let total_cells: Vec<String> = visible_cols.iter().enumerate().map(|(i, col)| {
            if i == 0 {
                // First column shows "Total" label
                let label = match locale {
                    ReportLocale::Ar | ReportLocale::ArEn => "المجموع",
                    ReportLocale::En | ReportLocale::EnAr => "Total",
                };
                csv_escape(label)
            } else if let Some(val) = agg.totals.get(&col.key) {
                format_csv_cell(val, &col.data_type)
            } else {
                String::new()
            }
        }).collect();

        output.extend_from_slice(total_cells.join(",").as_bytes());
        output.push(b'\n');
    }

    let size = output.len() as u64;
    let filename = format!("{}.csv", dataset.report_code.replace('.', "_"));

    Ok(ExportResult {
        format: "csv".into(),
        mime_type: "text/csv; charset=utf-8".into(),
        filename,
        data: output,
        size,
    })
}

/// Escape a string value for CSV (RFC 4180).
fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Format a JSON value for a CSV cell based on column data type.
fn format_csv_cell(val: &serde_json::Value, data_type: &ColumnDataType) -> String {
    match val {
        serde_json::Value::Null => String::new(),
        serde_json::Value::String(s) => csv_escape(s),
        serde_json::Value::Number(n) => {
            match data_type {
                ColumnDataType::Currency | ColumnDataType::Decimal | ColumnDataType::Percentage => {
                    // Format with 2 decimal places for currency/decimal
                    if let Some(f) = n.as_f64() {
                        format!("{:.2}", f)
                    } else {
                        n.to_string()
                    }
                }
                _ => n.to_string(),
            }
        }
        serde_json::Value::Bool(b) => {
            if *b { "✓".to_string() } else { "✗".to_string() }
        }
        // Arrays and objects — serialize as JSON string
        _ => csv_escape(&val.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::*;
    use crate::org_context::OrgContext;
    use serde_json::json;

    fn sample_dataset() -> ReportDataset {
        ReportDataset {
            report_code: "test.report".into(),
            report_name: "Test".into(),
            report_name_ar: "اختبار".into(),
            org: OrgContext::default(),
            parameter_summary: vec![],
            columns: vec![
                ColumnDef {
                    key: "name".into(), header: "Name".into(), header_ar: "الاسم".into(),
                    data_type: ColumnDataType::Text, width: None, align: None,
                    format: None, visible: true, total_type: None, sensitive: false,
                },
                ColumnDef {
                    key: "amount".into(), header: "Amount".into(), header_ar: "المبلغ".into(),
                    data_type: ColumnDataType::Currency, width: None, align: None,
                    format: None, visible: true, total_type: Some(AggregationType::Sum),
                    sensitive: false,
                },
                ColumnDef {
                    key: "hidden".into(), header: "Hidden".into(), header_ar: "مخفي".into(),
                    data_type: ColumnDataType::Text, width: None, align: None,
                    format: None, visible: false, total_type: None, sensitive: false,
                },
            ],
            rows: vec![
                json!({"name": "Ahmad", "amount": 1500.50, "hidden": "secret"}),
                json!({"name": "Fatima", "amount": 2000.00, "hidden": "secret2"}),
            ],
            groups: None,
            aggregations: Some(Aggregations {
                totals: json!({"amount": 3500.50}),
                subtotals: None,
            }),
            total_count: 2,
            page: 0,
            page_size: 0,
            generated_at: "2026-01-01T00:00:00Z".into(),
            generated_by: None,
        }
    }

    #[test]
    fn test_csv_export_arabic() {
        let dataset = sample_dataset();
        let result = export(&dataset, &ReportLocale::Ar).unwrap();

        let content = String::from_utf8(result.data).unwrap();
        // Should start with BOM
        assert!(content.starts_with('\u{FEFF}'));
        // Should have Arabic headers
        assert!(content.contains("الاسم"));
        assert!(content.contains("المبلغ"));
        // Should NOT contain hidden column
        assert!(!content.contains("مخفي"));
        assert!(!content.contains("secret"));
        // Should have data
        assert!(content.contains("Ahmad"));
        assert!(content.contains("1500.50"));
        // Should have totals
        assert!(content.contains("المجموع"));
        assert!(content.contains("3500.50"));
    }

    #[test]
    fn test_csv_export_english() {
        let dataset = sample_dataset();
        let result = export(&dataset, &ReportLocale::En).unwrap();

        let content = String::from_utf8(result.data).unwrap();
        assert!(content.contains("Name"));
        assert!(content.contains("Amount"));
        assert!(content.contains("Total"));
    }

    #[test]
    fn test_csv_escape_special() {
        assert_eq!(csv_escape("hello"), "hello");
        assert_eq!(csv_escape("hello, world"), "\"hello, world\"");
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
    }
}
