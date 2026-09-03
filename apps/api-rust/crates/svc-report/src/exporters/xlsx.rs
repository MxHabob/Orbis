//! XLSX Exporter — converts `ReportDataset` to a styled Excel spreadsheet.
//!
//! Features:
//! - RTL worksheet direction for Arabic locale
//! - Styled header row with background color and bold font
//! - Number/currency formatting with 2 decimal places
//! - Auto-fit column widths based on header length
//! - Totals row with bold formatting and SUM-like values
//! - Report title and parameter summary in header rows
//! - Bilingual column headers (selected by locale)

use rust_xlsxwriter::{Format, Workbook, Worksheet, FormatAlign, FormatBorder, Color};

use crate::dataset::{ExportResult, ReportDataset, ColumnDataType};
use crate::error::ReportError;
use crate::request::ReportLocale;

/// Export a `ReportDataset` to XLSX format.
pub fn export(
    dataset: &ReportDataset,
    locale: &ReportLocale,
) -> Result<ExportResult, ReportError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Set RTL for Arabic locales
    if locale.is_rtl() {
        worksheet.set_right_to_left(true);
    }

    // ── Formats ─────────────────────────────────────────────────
    let title_format = Format::new()
        .set_bold()
        .set_font_size(14.0)
        .set_font_color(Color::RGB(0x1a1a2e));

    let subtitle_format = Format::new()
        .set_font_size(10.0)
        .set_font_color(Color::RGB(0x666666))
        .set_italic();

    let header_format = Format::new()
        .set_bold()
        .set_font_size(10.0)
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x2c5f2d))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0x1a3d1a));

    let cell_format = Format::new()
        .set_font_size(10.0)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0xdddddd));

    let number_format = Format::new()
        .set_font_size(10.0)
        .set_num_format("#,##0.00")
        .set_align(FormatAlign::Right)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0xdddddd));

    let total_label_format = Format::new()
        .set_bold()
        .set_font_size(10.0)
        .set_background_color(Color::RGB(0xe8f5e9))
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0x999999));

    let total_number_format = Format::new()
        .set_bold()
        .set_font_size(10.0)
        .set_num_format("#,##0.00")
        .set_align(FormatAlign::Right)
        .set_background_color(Color::RGB(0xe8f5e9))
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0x999999));

    // ── Visible columns ─────────────────────────────────────────
    let visible_cols: Vec<_> = dataset.columns.iter()
        .filter(|c| c.visible)
        .collect();

    let col_count = visible_cols.len();

    // ── Title rows ──────────────────────────────────────────────
    let mut current_row: u32 = 0;

    // Report title
    let title = match locale {
        ReportLocale::Ar | ReportLocale::ArEn => &dataset.report_name_ar,
        ReportLocale::En | ReportLocale::EnAr => &dataset.report_name,
    };
    worksheet.write_string(current_row, 0, title).ok();
    worksheet.set_row_format(current_row, &title_format).ok();

    // Merge title across columns
    if col_count > 1 {
        worksheet.merge_range(
            current_row, 0, current_row, (col_count - 1) as u16,
            title, &title_format,
        ).ok();
    }
    current_row += 1;

    // Organization name
    let org_name = match locale {
        ReportLocale::Ar | ReportLocale::ArEn => &dataset.org.name_ar,
        ReportLocale::En | ReportLocale::EnAr => &dataset.org.name,
    };
    if !org_name.is_empty() {
        worksheet.write_string(current_row, 0, org_name).ok();
        worksheet.set_row_format(current_row, &subtitle_format).ok();
        current_row += 1;
    }

    // Parameter summary
    for param in &dataset.parameter_summary {
        let (label, value) = match locale {
            ReportLocale::Ar | ReportLocale::ArEn => (&param.label_ar, &param.value_ar),
            ReportLocale::En | ReportLocale::EnAr => (&param.label, &param.value),
        };
        let text = format!("{}: {}", label, value);
        worksheet.write_string(current_row, 0, &text).ok();
        worksheet.set_row_format(current_row, &subtitle_format).ok();
        current_row += 1;
    }

    // Blank row before data
    current_row += 1;

    // ── Header row ──────────────────────────────────────────────
    for (col_idx, col) in visible_cols.iter().enumerate() {
        let header = match locale {
            ReportLocale::Ar | ReportLocale::ArEn => &col.header_ar,
            ReportLocale::En | ReportLocale::EnAr => &col.header,
        };
        worksheet.write_string_with_format(
            current_row, col_idx as u16, header, &header_format,
        ).ok();

        // Set column width based on header length (approximate)
        let width = (header.chars().count() as f64 * 1.5).max(10.0).min(40.0);
        worksheet.set_column_width(col_idx as u16, width).ok();
    }
    current_row += 1;

    // ── Data rows ───────────────────────────────────────────────
    for row_data in &dataset.rows {
        for (col_idx, col) in visible_cols.iter().enumerate() {
            let val = &row_data[&col.key];
            write_cell(worksheet, current_row, col_idx as u16, val, &col.data_type, &cell_format, &number_format);
        }
        current_row += 1;
    }

    // ── Totals row ──────────────────────────────────────────────
    if let Some(ref agg) = dataset.aggregations {
        for (col_idx, col) in visible_cols.iter().enumerate() {
            if col_idx == 0 {
                let label = match locale {
                    ReportLocale::Ar | ReportLocale::ArEn => "المجموع",
                    ReportLocale::En | ReportLocale::EnAr => "Total",
                };
                worksheet.write_string_with_format(
                    current_row, 0, label, &total_label_format,
                ).ok();
            } else if let Some(val) = agg.totals.get(&col.key) {
                if let Some(n) = val.as_f64() {
                    worksheet.write_number_with_format(
                        current_row, col_idx as u16, n, &total_number_format,
                    ).ok();
                } else if let Some(n) = val.as_u64() {
                    worksheet.write_number_with_format(
                        current_row, col_idx as u16, n as f64, &total_number_format,
                    ).ok();
                } else {
                    worksheet.write_string_with_format(
                        current_row, col_idx as u16, &val.to_string(), &total_label_format,
                    ).ok();
                }
            } else {
                worksheet.write_blank(current_row, col_idx as u16, &total_label_format).ok();
            }
        }
    }

    // ── Generation metadata ─────────────────────────────────────
    current_row += 2;
    let gen_label = match locale {
        ReportLocale::Ar | ReportLocale::ArEn => "تاريخ الإنشاء",
        ReportLocale::En | ReportLocale::EnAr => "Generated at",
    };
    worksheet.write_string(current_row, 0, &format!("{}: {}", gen_label, &dataset.generated_at)).ok();

    // ── Save ────────────────────────────────────────────────────
    let data = workbook.save_to_buffer()
        .map_err(|e| ReportError::Internal(format!("XLSX generation failed: {}", e)))?;

    let size = data.len() as u64;
    let filename = format!("{}.xlsx", dataset.report_code.replace('.', "_"));

    Ok(ExportResult {
        format: "xlsx".into(),
        mime_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".into(),
        filename,
        data,
        size,
    })
}

/// Write a single cell value to the worksheet.
fn write_cell(
    ws: &mut Worksheet,
    row: u32,
    col: u16,
    val: &serde_json::Value,
    data_type: &ColumnDataType,
    cell_fmt: &Format,
    num_fmt: &Format,
) {
    match val {
        serde_json::Value::Null => {
            ws.write_blank(row, col, cell_fmt).ok();
        }
        serde_json::Value::String(s) => {
            ws.write_string_with_format(row, col, s, cell_fmt).ok();
        }
        serde_json::Value::Number(n) => {
            let fmt = match data_type {
                ColumnDataType::Currency | ColumnDataType::Decimal | ColumnDataType::Percentage => num_fmt,
                _ => cell_fmt,
            };
            if let Some(f) = n.as_f64() {
                ws.write_number_with_format(row, col, f, fmt).ok();
            } else if let Some(i) = n.as_i64() {
                ws.write_number_with_format(row, col, i as f64, fmt).ok();
            }
        }
        serde_json::Value::Bool(b) => {
            let symbol = if *b { "✓" } else { "✗" };
            ws.write_string_with_format(row, col, symbol, cell_fmt).ok();
        }
        _ => {
            ws.write_string_with_format(row, col, &val.to_string(), cell_fmt).ok();
        }
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
            report_name: "Test Report".into(),
            report_name_ar: "تقرير اختبار".into(),
            org: OrgContext {
                name: "Test School".into(),
                name_ar: "مدرسة اختبار".into(),
                ..Default::default()
            },
            parameter_summary: vec![ParameterSummaryItem {
                label: "Period".into(), label_ar: "الفترة".into(),
                value: "2025-2026".into(), value_ar: "2025-2026".into(),
            }],
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
            ],
            rows: vec![
                json!({"name": "Ahmad", "amount": 1500.50}),
                json!({"name": "Fatima", "amount": 2000.00}),
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
    fn test_xlsx_export_produces_valid_file() {
        let dataset = sample_dataset();
        let result = export(&dataset, &ReportLocale::Ar).unwrap();

        assert_eq!(result.format, "xlsx");
        assert!(result.size > 0);
        assert_eq!(result.filename, "test_report.xlsx");
        // XLSX files start with PK (ZIP magic bytes)
        assert_eq!(&result.data[0..2], b"PK");
    }

    #[test]
    fn test_xlsx_export_english() {
        let dataset = sample_dataset();
        let result = export(&dataset, &ReportLocale::En).unwrap();
        assert!(result.size > 0);
    }
}
