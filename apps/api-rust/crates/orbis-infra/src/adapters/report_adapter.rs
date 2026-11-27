//! Report infrastructure adapters — renderers, registries, and org context.
//!
//! The application layer selects an exporter through the DocumentRenderer port.
//! This adapter uses Tera for HTML and DocForge for PDF layout and rendering.

use async_trait::async_trait;
use orbis_app::ports::reporting::*;
use orbis_app::services::OrgContextResolver;
use orbis_domain::entities::report::*;
use orbis_domain::errors::DomainError;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::io::{Cursor, Write};
use tera::{Context, Tera};
use uuid::Uuid;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

fn student_sort_clause(
    settings: &orbis_domain::entities::settings::StudentSortSettings,
    student_alias: &str,
) -> String {
    use orbis_domain::entities::settings::{StudentSortDirection, StudentSortField};

    let criteria = settings.criteria();
    let mut clauses = Vec::new();
    for criterion in &criteria {
        let direction = match criterion.direction {
            StudentSortDirection::Asc => "ASC",
            StudentSortDirection::Desc => "DESC",
        };
        let columns = match criterion.field {
            StudentSortField::StudentNumber => {
                vec![format!("{student_alias}.student_number")]
            }
            StudentSortField::Name => vec![
                "p.first_name".to_string(),
                "p.middle_name".to_string(),
                "p.last_name".to_string(),
            ],
            StudentSortField::CreatedAt => vec![format!("{student_alias}.created_at")],
            StudentSortField::AdmittedAt => vec![format!("{student_alias}.admitted_at")],
            StudentSortField::Status => vec![format!("{student_alias}.status")],
            StudentSortField::BirthDate => vec!["p.birth_date".to_string()],
            StudentSortField::Gender => vec!["p.gender".to_string()],
        };
        clauses.extend(
            columns
                .into_iter()
                .map(|column| format!("{column} {direction}")),
        );
    }
    if !criteria
        .iter()
        .any(|criterion| criterion.field == StudentSortField::StudentNumber)
    {
        clauses.push(format!("{student_alias}.student_number ASC"));
    }
    clauses.push(format!("{student_alias}.id ASC"));
    clauses.join(", ")
}

// ═══════════════════════════════════════════════════════════════
// DATA PROVIDER REGISTRY
// ═══════════════════════════════════════════════════════════════

/// In-memory registry for data providers. Providers are registered
/// at startup; no central match dispatch.
pub struct InMemoryDataProviderRegistry {
    providers: HashMap<String, Box<dyn ReportDataProvider>>,
}

impl InMemoryDataProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }
}

impl DataProviderRegistry for InMemoryDataProviderRegistry {
    fn get_provider(&self, key: &str) -> Option<&dyn ReportDataProvider> {
        self.providers.get(key).map(|p| p.as_ref())
    }

    fn register_provider(&mut self, provider: Box<dyn ReportDataProvider>) {
        let key = provider.provider_key().to_string();
        self.providers.insert(key, provider);
    }

    fn list_provider_keys(&self) -> Vec<&str> {
        self.providers.keys().map(|k| k.as_str()).collect()
    }
}

// ═══════════════════════════════════════════════════════════════
// RENDERER REGISTRY
// ═══════════════════════════════════════════════════════════════

pub struct InMemoryRendererRegistry {
    renderers: Vec<Box<dyn DocumentRenderer>>,
}

impl InMemoryRendererRegistry {
    pub fn new() -> Self {
        Self {
            renderers: Vec::new(),
        }
    }
}

impl RendererRegistry for InMemoryRendererRegistry {
    fn get_renderer(&self, format: &OutputFormat) -> Option<&dyn DocumentRenderer> {
        self.renderers
            .iter()
            .find(|r| r.supported_format() == *format)
            .map(|r| r.as_ref())
    }

    fn register_renderer(&mut self, renderer: Box<dyn DocumentRenderer>) {
        self.renderers.push(renderer);
    }
}

// ═══════════════════════════════════════════════════════════════
// JSON RENDERER
// ═══════════════════════════════════════════════════════════════

pub struct JsonRenderer;

#[async_trait]
impl DocumentRenderer for JsonRenderer {
    fn supported_format(&self) -> OutputFormat {
        OutputFormat::Json
    }

    async fn render(
        &self,
        dataset: &ReportDataset,
        _locale: &ReportLocale,
        _template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let json = serde_json::to_vec_pretty(dataset)
            .map_err(|e| DomainError::Internal(format!("JSON serialization failed: {}", e)))?;
        let size = json.len() as u64;
        let filename = format!("{}.json", dataset.report_code.replace('.', "_"));
        Ok(ExportResult {
            format: OutputFormat::Json,
            mime_type: "application/json".into(),
            filename,
            data: json,
            size,
        })
    }
}

/// DataOnly uses the same JSON format.
pub struct DataOnlyRenderer;

#[async_trait]
impl DocumentRenderer for DataOnlyRenderer {
    fn supported_format(&self) -> OutputFormat {
        OutputFormat::DataOnly
    }

    async fn render(
        &self,
        dataset: &ReportDataset,
        _locale: &ReportLocale,
        _template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let json = serde_json::to_vec(dataset)
            .map_err(|e| DomainError::Internal(format!("JSON serialization failed: {}", e)))?;
        let size = json.len() as u64;
        Ok(ExportResult {
            format: OutputFormat::DataOnly,
            mime_type: "application/json".into(),
            filename: format!("{}.json", dataset.report_code.replace('.', "_")),
            data: json,
            size,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// CSV RENDERER
// ═══════════════════════════════════════════════════════════════

pub struct CsvRenderer;

#[async_trait]
impl DocumentRenderer for CsvRenderer {
    fn supported_format(&self) -> OutputFormat {
        OutputFormat::Csv
    }

    async fn render(
        &self,
        dataset: &ReportDataset,
        _locale: &ReportLocale,
        _template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let mut csv_data = String::new();

        // Header row
        let headers: Vec<&str> = dataset
            .columns
            .iter()
            .filter(|c| c.visible)
            .map(|c| c.header.as_str())
            .collect();
        csv_data.push_str(&headers.join(","));
        csv_data.push('\n');

        // Data rows
        let visible_keys: Vec<&str> = dataset
            .columns
            .iter()
            .filter(|c| c.visible)
            .map(|c| c.key.as_str())
            .collect();

        for row in &dataset.rows {
            let values: Vec<String> = visible_keys
                .iter()
                .map(|key| {
                    let val = row.get(*key).cloned().unwrap_or(serde_json::Value::Null);
                    match val {
                        serde_json::Value::String(s) => {
                            // Escape CSV values
                            if s.contains(',') || s.contains('"') || s.contains('\n') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s
                            }
                        }
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        serde_json::Value::Null => String::new(),
                        _ => val.to_string(),
                    }
                })
                .collect();
            csv_data.push_str(&values.join(","));
            csv_data.push('\n');
        }

        let bytes = csv_data.into_bytes();
        // Prepend BOM for Excel Arabic support
        let mut bom_bytes = vec![0xEF, 0xBB, 0xBF];
        bom_bytes.extend_from_slice(&bytes);
        let size = bom_bytes.len() as u64;

        Ok(ExportResult {
            format: OutputFormat::Csv,
            mime_type: "text/csv; charset=utf-8".into(),
            filename: format!("{}.csv", dataset.report_code.replace('.', "_")),
            data: bom_bytes,
            size,
        })
    }
}

pub struct XlsxRenderer;

#[async_trait]
impl DocumentRenderer for XlsxRenderer {
    fn supported_format(&self) -> OutputFormat {
        OutputFormat::Xlsx
    }

    async fn render(
        &self,
        dataset: &ReportDataset,
        _locale: &ReportLocale,
        _template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let visible_columns: Vec<_> = dataset
            .columns
            .iter()
            .filter(|column| column.visible)
            .collect();
        let mut worksheet = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>"#,
        );
        worksheet.push_str("<row r=\"1\">");
        for (index, column) in visible_columns.iter().enumerate() {
            worksheet.push_str(&xlsx_cell(
                index + 1,
                1,
                &serde_json::Value::String(column.header.clone()),
            ));
        }
        worksheet.push_str("</row>");

        for (row_index, row) in dataset.rows.iter().enumerate() {
            let excel_row = row_index + 2;
            worksheet.push_str(&format!("<row r=\"{excel_row}\">"));
            for (column_index, column) in visible_columns.iter().enumerate() {
                let value = row.get(&column.key).unwrap_or(&serde_json::Value::Null);
                worksheet.push_str(&xlsx_cell(column_index + 1, excel_row, value));
            }
            worksheet.push_str("</row>");
        }
        worksheet.push_str("</sheetData></worksheet>");

        let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
        let parts = [
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#,
            ),
            (
                "_rels/.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Report" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
            ),
        ];
        for (name, contents) in parts {
            archive.start_file(name, options).map_err(|error| {
                DomainError::Internal(format!("XLSX package write failed: {error}"))
            })?;
            archive.write_all(contents.as_bytes()).map_err(|error| {
                DomainError::Internal(format!("XLSX package write failed: {error}"))
            })?;
        }
        archive
            .start_file("xl/worksheets/sheet1.xml", options)
            .map_err(|error| {
                DomainError::Internal(format!("XLSX worksheet write failed: {error}"))
            })?;
        archive.write_all(worksheet.as_bytes()).map_err(|error| {
            DomainError::Internal(format!("XLSX worksheet write failed: {error}"))
        })?;
        let bytes = archive
            .finish()
            .map_err(|error| {
                DomainError::Internal(format!("XLSX package finalization failed: {error}"))
            })?
            .into_inner();
        let size = bytes.len() as u64;
        Ok(ExportResult {
            format: OutputFormat::Xlsx,
            mime_type: OutputFormat::Xlsx.mime_type().into(),
            filename: format!("{}.xlsx", dataset.report_code.replace('.', "_")),
            data: bytes,
            size,
        })
    }
}

fn xlsx_cell(column: usize, row: usize, value: &serde_json::Value) -> String {
    let cell = format!("{}{}", xlsx_column_name(column), row);
    match value {
        serde_json::Value::Null => format!("<c r=\"{cell}\"/>"),
        serde_json::Value::Number(number) => {
            format!("<c r=\"{cell}\"><v>{number}</v></c>")
        }
        serde_json::Value::Bool(boolean) => {
            format!("<c r=\"{cell}\" t=\"b\"><v>{}</v></c>", u8::from(*boolean))
        }
        _ => {
            let text = match value {
                serde_json::Value::String(text) => text.clone(),
                _ => value.to_string(),
            };
            format!(
                "<c r=\"{cell}\" t=\"inlineStr\"><is><t xml:space=\"preserve\">{}</t></is></c>",
                escape_xml(&text)
            )
        }
    }
}

fn xlsx_column_name(mut column: usize) -> String {
    let mut name = String::new();
    while column > 0 {
        let remainder = (column - 1) % 26;
        name.insert(0, (b'A' + remainder as u8) as char);
        column = (column - 1) / 26;
    }
    name
}

fn escape_xml(value: &str) -> String {
    let valid_xml = value
        .chars()
        .filter(|character| {
            matches!(
                *character,
                '\u{9}'
                    | '\u{A}'
                    | '\u{D}'
                    | '\u{20}'..='\u{D7FF}'
                    | '\u{E000}'..='\u{FFFD}'
                    | '\u{10000}'..='\u{10FFFF}'
            )
        })
        .collect::<String>();
    valid_xml
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod xlsx_tests {
    use super::*;
    use chrono::Utc;
    use orbis_domain::entities::settings::{
        StudentSortDirection, StudentSortField, StudentSortSettings,
    };
    use std::io::Read;
    use zip::ZipArchive;

    #[test]
    fn student_report_sort_uses_configured_field_and_stable_tie_breaker() {
        let settings = StudentSortSettings {
            field: StudentSortField::Name,
            direction: StudentSortDirection::Desc,
            then_by: vec![orbis_domain::entities::settings::StudentSortCriterion {
                field: StudentSortField::CreatedAt,
                direction: StudentSortDirection::Asc,
            }],
        };

        assert_eq!(
            student_sort_clause(&settings, "st"),
            "p.first_name DESC, p.middle_name DESC, p.last_name DESC, st.created_at ASC, st.student_number ASC, st.id ASC"
        );
    }

    #[test]
    fn docforge_context_exposes_organization_branding_as_template_variables() {
        let context = docforge_org_context(&OrgReportContext {
            name_ar: "مدرسة أوربس".into(),
            phone: Some("+967 123456".into()),
            website: Some("https://orbis.example".into()),
            ..OrgReportContext::default()
        })
        .expect("organization context should serialize");

        assert_eq!(context["name_ar"], "مدرسة أوربس");
        assert_eq!(context["phone"], "+967 123456");
        assert_eq!(context["website"], "https://orbis.example");
    }

    fn empty_dataset() -> ReportDataset {
        ReportDataset {
            report_code: "students.overview".into(),
            report_name: "Student overview".into(),
            report_name_ar: "ملخص الطلاب".into(),
            org: OrgReportContext {
                name: "North Campus".into(),
                ..OrgReportContext::default()
            },
            parameter_summary: Vec::new(),
            columns: Vec::new(),
            rows: Vec::new(),
            groups: None,
            aggregations: None,
            total_count: 0,
            generated_at: Utc::now(),
            generated_by: None,
        }
    }

    #[tokio::test]
    async fn html_renderer_wraps_default_reports_with_organization_branding() {
        let dataset = empty_dataset();
        let export = HtmlRenderer
            .render(&dataset, &ReportLocale::En, None, None)
            .await
            .expect("HTML rendering should succeed with default master");
        let html = String::from_utf8(export.data).expect("HTML output should be utf-8");

        assert!(html.contains("North Campus"));
        assert!(html.contains("Student overview"));
        assert!(html.contains("No data to display"));
        assert!(html.contains("lang=\"en\" dir=\"ltr\""));
    }

    #[tokio::test]
    async fn pdf_renderer_exports_real_pdf_without_a_bound_template() {
        let result = PdfRenderer
            .render(&empty_dataset(), &ReportLocale::En, None, None)
            .await
            .expect("default DocForge PDF");
        assert_eq!(result.mime_type, "application/pdf");
        assert!(result.data.starts_with(b"%PDF"));
    }

    #[test]
    fn default_pdf_repeats_headers_and_keeps_wide_report_pages_within_a4() {
        use docforge_engine::{
            layout::engine::LayoutEngine, render::font_cache::FontCache,
            render::types::PositionedElement,
        };

        let mut dataset = empty_dataset();
        dataset.columns = (0..11)
            .map(|column| ColumnDef {
                key: format!("c{column}"),
                header: format!("Column {column}"),
                header_ar: String::new(),
                data_type: ColumnDataType::Text,
                width: None,
                align: None,
                format: None,
                visible: true,
                total_type: None,
            })
            .collect();
        dataset.rows = (0..14)
            .map(|row| {
                serde_json::Value::Object(
                    (0..11)
                        .map(|column| {
                            (
                                format!("c{column}"),
                                serde_json::json!(format!("Record {row}-{column}")),
                            )
                        })
                        .collect(),
                )
            })
            .collect();
        dataset.rows[0]["c1"] = serde_json::json!(824.57);
        dataset.total_count = 14;

        let (document, context) =
            super::super::tabular_pdf::build(&dataset, &ReportLocale::En).unwrap();
        let fonts = FontCache::with_system_fonts();
        let layout = LayoutEngine::new(&fonts)
            .layout(&document, &context)
            .unwrap();
        if let Some(directory) = std::env::var_os("ORBIS_PRINT_PREVIEW") {
            let pdf = docforge_engine::render::pdf_renderer::render_to_pdf(
                &layout,
                &fonts,
                &Default::default(),
            )
            .unwrap();
            std::fs::write(
                std::path::Path::new(&directory).join("report.pdf"),
                pdf.bytes,
            )
            .unwrap();
        }
        assert_eq!(layout.pages.len(), 4);
        assert!(layout.pages[0].elements.iter().any(
            |element| matches!(element, PositionedElement::Text(text) if text.content == "824.57")
        ));
        for page in &layout.pages {
            assert!((page.width_pt - 841.89).abs() < 0.2);
            assert!((page.height_pt - 595.28).abs() < 0.2);
            assert!(page.elements.iter().any(
                |element| matches!(element, PositionedElement::Text(text) if text.content == "Column 0")
            ));
        }
        assert!(layout.pages[3].elements.iter().any(
            |element| matches!(element, PositionedElement::Text(text) if text.content == "Record 13-10")
        ));
    }

    #[tokio::test]
    async fn xlsx_renderer_creates_a_valid_workbook_with_escaped_cell_content() {
        let dataset = ReportDataset {
            report_code: "finance.fee_collection".into(),
            report_name: "Fee collection".into(),
            report_name_ar: "تحصيل الرسوم".into(),
            org: OrgReportContext::default(),
            parameter_summary: Vec::new(),
            columns: vec![
                ColumnDef {
                    key: "student_name".into(),
                    header: "Student".into(),
                    header_ar: "الطالب".into(),
                    data_type: ColumnDataType::Text,
                    width: None,
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "balance".into(),
                    header: "Balance".into(),
                    header_ar: "الرصيد".into(),
                    data_type: ColumnDataType::Currency,
                    width: None,
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
            ],
            rows: vec![serde_json::json!({
                "student_name": "A&B <Student>",
                "balance": 125.50
            })],
            groups: None,
            aggregations: None,
            total_count: 1,
            generated_at: Utc::now(),
            generated_by: None,
        };

        let export = XlsxRenderer
            .render(&dataset, &ReportLocale::Ar, None, None)
            .await
            .expect("xlsx rendering should succeed");
        let mut archive = ZipArchive::new(Cursor::new(export.data))
            .expect("renderer must produce a readable zip archive");
        let mut sheet = String::new();
        archive
            .by_name("xl/worksheets/sheet1.xml")
            .expect("worksheet must be present")
            .read_to_string(&mut sheet)
            .expect("worksheet xml should be readable");

        assert!(sheet.contains("A&amp;B &lt;Student&gt;"));
        assert!(sheet.contains("<c r=\"B2\"><v>125.5</v></c>"));
    }
}

// ═══════════════════════════════════════════════════════════════
// HTML RENDERER (also serves as PDF source)
// ═══════════════════════════════════════════════════════════════

pub struct HtmlRenderer;

#[async_trait]
impl DocumentRenderer for HtmlRenderer {
    fn supported_format(&self) -> OutputFormat {
        OutputFormat::Html
    }

    async fn render(
        &self,
        dataset: &ReportDataset,
        locale: &ReportLocale,
        template: Option<&ReportTemplate>,
        master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let is_rtl = locale.is_rtl();

        let mut context = Context::new();
        context.insert("is_rtl", &is_rtl);
        context.insert("org_name", &dataset.org.name);
        context.insert("org_name_ar", &dataset.org.name_ar);
        context.insert("country_name", &dataset.org.country_name);
        context.insert("ministry_name", &dataset.org.ministry_name);
        context.insert("region_name", &dataset.org.region_name);
        static EMBLEM_URI: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        let emblem_uri = EMBLEM_URI.get_or_init(|| {
            let emblem_b64 = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                include_bytes!("../../../orbis-app/src/defaults/assest/The Public Slogan.png"),
            );
            format!("data:image/png;base64,{emblem_b64}")
        });
        context.insert("state_emblem_uri", emblem_uri);
        context.insert("total_count", &dataset.total_count);
        context.insert("org_type", &"");
        context.insert("org_type_ar", &"");
        context.insert("currency", &dataset.org.currency_code);
        context.insert("primary_color", &dataset.org.primary_color);
        let brand_color = dataset
            .org
            .primary_color
            .as_deref()
            .filter(|value| {
                value.len() == 7
                    && value.starts_with('#')
                    && value.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
            })
            .unwrap_or("#334155");
        context.insert("brand_color", brand_color);
        context.insert("org_website", &dataset.org.website);
        context.insert("report_name", &dataset.report_name);
        context.insert("report_name_ar", &dataset.report_name_ar);
        context.insert(
            "generated_at",
            &dataset.generated_at.format("%Y-%m-%d %H:%M").to_string(),
        );

        // generated_by is Option<Uuid>, use System if None
        let printed_by = dataset
            .generated_by
            .map(|u| u.to_string())
            .unwrap_or_else(|| "System".to_string());
        context.insert("printed_by", &printed_by);

        let parameters: Vec<_> = dataset
            .parameter_summary
            .iter()
            .map(|item| {
                let (label, value) = if is_rtl {
                    (&item.label_ar, &item.value_ar)
                } else {
                    (&item.label, &item.value)
                };
                serde_json::json!({ "label": label, "value": value })
            })
            .collect();
        let parameter_map: HashMap<_, _> = dataset
            .parameter_summary
            .iter()
            .map(|item| {
                if is_rtl {
                    (&item.label_ar, &item.value_ar)
                } else {
                    (&item.label, &item.value)
                }
            })
            .collect();
        context.insert("parameters", &parameter_map);
        context.insert("parameter_items", &parameters);
        context.insert("current_page", &1);
        context.insert("total_pages", &1);
        context.insert("has_parameters", &!parameters.is_empty());
        let column_count = dataset
            .columns
            .iter()
            .filter(|column| column.visible)
            .count();
        context.insert("column_count", &column_count);
        context.insert(
            "paper_orientation",
            if column_count > 6 {
                "landscape"
            } else {
                "portrait"
            },
        );
        context.insert("columns", &dataset.columns);
        context.insert("rows", &dataset.rows);

        if let Some(aggs) = &dataset.aggregations {
            context.insert("totals", &aggs.totals);
        }

        let mut tera = Tera::default();
        tera.autoescape_on(vec![".html"]);

        let inner_template_str = template
            .and_then(|t| t.document_json.clone().into())
            .unwrap_or_else(|| crate::adapters::report_defaults::TABULAR_REPORT_HTML.to_string());

        tera.add_raw_template("inner.html", &inner_template_str)
            .map_err(|e| DomainError::Internal(format!("Tera inner parse error: {}", e)))?;

        let inner_html = tera
            .render("inner.html", &context)
            .map_err(|e| DomainError::Internal(format!("Tera inner render error: {}", e)))?;

        context.insert("inner_content", &inner_html);
        let master_str = master.map_or(
            crate::adapters::report_defaults::MASTER_TEMPLATE_HTML,
            |template| template.document_json.as_str(),
        );
        tera.add_raw_template("master.html", master_str)
            .map_err(|e| DomainError::Internal(format!("Tera master parse error: {}", e)))?;
        let final_html = tera
            .render("master.html", &context)
            .map_err(|e| DomainError::Internal(format!("Tera master render error: {}", e)))?;

        let bytes = final_html.into_bytes();
        let size = bytes.len() as u64;
        Ok(ExportResult {
            format: OutputFormat::Html,
            mime_type: "text/html; charset=utf-8".into(),
            filename: format!("{}.html", dataset.report_code.replace('.', "_")),
            data: bytes,
            size,
        })
    }
}

fn docforge_org_context(org_context: &OrgReportContext) -> Result<serde_json::Value, DomainError> {
    let mut value = serde_json::to_value(org_context).map_err(|error| {
        DomainError::Internal(format!(
            "Organization report context serialization failed: {error}"
        ))
    })?;
    if let serde_json::Value::Object(org) = &mut value {
        for (snake_case, camel_case) in [
            ("name_ar", "nameAr"),
            ("logo_url", "logoUrl"),
            ("address_ar", "addressAr"),
            ("primary_color", "primaryColor"),
            ("currency_code", "currencyCode"),
            ("currency", "currencyCode"),
            ("country_name", "countryName"),
            ("ministry_name", "ministryName"),
            ("region_name", "regionName"),
            ("tax_id", "taxId"),
            ("license_number", "licenseNumber"),
        ] {
            if let Some(value) = org.get(camel_case).cloned() {
                org.insert(snake_case.to_string(), value);
            }
        }
    }
    Ok(value)
}

/// Renders a published DocForge template or the built-in tabular document.
pub struct PdfRenderer;

#[async_trait]
impl DocumentRenderer for PdfRenderer {
    fn supported_format(&self) -> OutputFormat {
        OutputFormat::Pdf
    }

    async fn render(
        &self,
        dataset: &ReportDataset,
        locale: &ReportLocale,
        template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let (doc, context_val) = if let Some(template) = template {
            let doc = serde_json::from_str::<docforge_engine::model::document::TemplateDocument>(
                &template.document_json,
            )
            .map_err(|error| {
                DomainError::Validation(format!("Invalid DocForge report template: {error}"))
            })?;
            let mut ctx_obj = serde_json::Map::new();
            let branding = docforge_org_context(&dataset.org)?;
            ctx_obj.insert("org".into(), branding.clone());
            ctx_obj.insert("$org".into(), branding);
            ctx_obj.insert(
                "columns".into(),
                serde_json::to_value(&dataset.columns).unwrap_or_default(),
            );
            ctx_obj.insert(
                "rows".into(),
                serde_json::Value::Array(dataset.rows.clone()),
            );
            ctx_obj.insert(
                "subjects".into(),
                serde_json::Value::Array(dataset.rows.clone()),
            );

            if let Some(aggs) = &dataset.aggregations {
                ctx_obj.insert("totals".into(), aggs.totals.clone());
                ctx_obj.insert("summary".into(), aggs.totals.clone());

                if let serde_json::Value::Object(map) = &aggs.totals {
                    for (k, v) in map {
                        ctx_obj.insert(k.clone(), v.clone());
                    }
                }
            }

            if let Some(first_row) = dataset.rows.first() {
                if let serde_json::Value::Object(map) = first_row {
                    for (k, v) in map {
                        if !ctx_obj.contains_key(k) {
                            ctx_obj.insert(k.clone(), v.clone());
                        }
                    }
                }
            }

            let context_val = serde_json::Value::Object(ctx_obj);

            (doc, context_val)
        } else {
            crate::adapters::tabular_pdf::build(dataset, locale)?
        };

        let fonts = docforge_engine::render::font_cache::FontCache::with_system_fonts();
        let layout = docforge_engine::layout::engine::LayoutEngine::new(&fonts)
            .layout(&doc, &context_val)
            .map_err(|error| DomainError::Internal(format!("DocForge layout error: {error:?}")))?;
        let config = docforge_engine::render::pdf_renderer::PdfRenderConfig {
            title: Some(doc.meta.name.clone()),
            ..Default::default()
        };
        let bytes = docforge_engine::render::pdf_renderer::render_to_pdf(&layout, &fonts, &config)
            .map_err(|error| {
                DomainError::Internal(format!("DocForge PDF render error: {error:?}"))
            })?
            .bytes;
        Ok(ExportResult {
            format: OutputFormat::Pdf,
            mime_type: "application/pdf".into(),
            filename: format!("{}.pdf", dataset.report_code.replace('.', "_")),
            size: bytes.len() as u64,
            data: bytes,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// ORG CONTEXT RESOLVER
// ═══════════════════════════════════════════════════════════════

pub struct DbOrgContextResolver {
    pub db: DatabaseConnection,
}

fn organization_metadata_text(metadata: &Option<serde_json::Value>, field: &str) -> Option<String> {
    metadata
        .as_ref()
        .and_then(serde_json::Value::as_object)
        .and_then(|values| values.get(field))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

#[async_trait]
impl OrgContextResolver for DbOrgContextResolver {
    async fn resolve(&self, org_id: Uuid) -> Result<OrgReportContext, DomainError> {
        use crate::models::organization;
        use sea_orm::EntityTrait;

        let org = organization::Entity::find_by_id(org_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Organization not found".into()))?;
        let student_sort = org
            .settings
            .as_ref()
            .and_then(|settings| settings.get("student_sort"))
            .and_then(|settings| serde_json::from_value(settings.clone()).ok())
            .unwrap_or_default();

        Ok(OrgReportContext {
            id: org.id,
            name: org.name.clone(),
            name_ar: org.name_ar.clone().unwrap_or_default(),
            currency_code: org.base_currency_code.clone(),
            code: org.code.clone(),
            logo_url: org.logo.clone(),
            logo_data: None,
            address: organization_metadata_text(&org.metadata, "address"),
            address_ar: organization_metadata_text(&org.metadata, "address_ar"),
            phone: organization_metadata_text(&org.metadata, "phone"),
            email: None,
            website: organization_metadata_text(&org.metadata, "website"),
            primary_color: org.primary_color.clone(),
            country_name: organization_metadata_text(&org.metadata, "country_name"),
            ministry_name: organization_metadata_text(&org.metadata, "ministry_name"),
            region_name: organization_metadata_text(&org.metadata, "region_name"),
            city: organization_metadata_text(&org.metadata, "city"),
            tax_id: organization_metadata_text(&org.metadata, "tax_id"),
            license_number: organization_metadata_text(&org.metadata, "license_number"),
            student_sort,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// SHARED PARAMETER & SQL HELPERS
// ═══════════════════════════════════════════════════════════════

fn parse_uuid_param(
    parameters: &HashMap<String, ParamValue>,
    key: &str,
) -> Result<Option<Uuid>, DomainError> {
    let Some(raw) = parameters.get(key).and_then(|v| v.as_str()) else {
        return Ok(None);
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    Uuid::parse_str(trimmed)
        .map(Some)
        .map_err(|_| DomainError::Validation(format!("Invalid UUID for parameter '{key}'")))
}

fn decimal_to_f64(row: &sea_orm::QueryResult, col: &str) -> f64 {
    use rust_decimal::prelude::ToPrimitive;
    if let Ok(Some(d)) = row.try_get::<Option<rust_decimal::Decimal>>("", col) {
        return d.to_f64().unwrap_or(0.0);
    }
    if let Ok(d) = row.try_get::<rust_decimal::Decimal>("", col) {
        return d.to_f64().unwrap_or(0.0);
    }
    if let Ok(v) = row.try_get::<f64>("", col) {
        return v;
    }
    if let Ok(v) = row.try_get::<i64>("", col) {
        return v as f64;
    }
    0.0
}

fn date_str(row: &sea_orm::QueryResult, col: &str) -> String {
    if let Ok(Some(d)) = row.try_get::<Option<chrono::NaiveDate>>("", col) {
        return d.format("%Y-%m-%d").to_string();
    }
    if let Ok(d) = row.try_get::<chrono::NaiveDate>("", col) {
        return d.format("%Y-%m-%d").to_string();
    }
    if let Ok(Some(dt)) = row.try_get::<Option<chrono::DateTime<chrono::Utc>>>("", col) {
        return dt.format("%Y-%m-%d").to_string();
    }
    if let Ok(dt) = row.try_get::<chrono::DateTime<chrono::Utc>>("", col) {
        return dt.format("%Y-%m-%d").to_string();
    }
    row.try_get::<Option<String>>("", col)
        .ok()
        .flatten()
        .unwrap_or_default()
}

fn col_def(
    key: &str,
    header: &str,
    header_ar: &str,
    data_type: ColumnDataType,
    width: &str,
    align: Option<ColumnAlign>,
) -> ColumnDef {
    ColumnDef {
        key: key.into(),
        header: header.into(),
        header_ar: header_ar.into(),
        data_type,
        width: Some(width.into()),
        align,
        format: None,
        visible: true,
        total_type: None,
    }
}

// ═══════════════════════════════════════════════════════════════
// SAMPLE DATA PROVIDER (finance.fee_collection)
// ═══════════════════════════════════════════════════════════════

/// Real data provider that queries the Orbis database for fee collection data.
pub struct FeeCollectionDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for FeeCollectionDataProvider {
    fn provider_key(&self) -> &str {
        "finance.fee_collection"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use rust_decimal::prelude::ToPrimitive;
        use sea_orm::{ConnectionTrait, Statement};

        let order_by = student_sort_clause(&org_context.student_sort, "s");
        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &format!(r#"SELECT
                    sf.net_amount as amount,
                    COALESCE(sf.paid_amount, 0) as paid_amount,
                    sf.status,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, s.student_number, '') as student_name,
                    COALESCE(ft.name, '') as fee_name
                FROM student_fees sf
                INNER JOIN students s ON sf.student_id = s.id AND s.org_id = sf.org_id
                LEFT JOIN persons p ON s.person_id = p.id
                LEFT JOIN fee_templates ft ON sf.fee_template_id = ft.id
                WHERE sf.org_id = $1
                ORDER BY {order_by}"#),
                vec![org_id.into()],
            ))
            .await
            .map_err(|error| {
                DomainError::Internal(format!("Fee collection query failed: {error}"))
            })?;

        let mut data_rows = Vec::new();
        let mut total_amount = 0.0f64;
        let mut total_paid = 0.0f64;
        let mut seq = 0u64;

        for row in &rows {
            seq += 1;
            let amount: rust_decimal::Decimal = row
                .try_get("", "amount")
                .map_err(|error| DomainError::Internal(format!("Invalid fee amount: {error}")))?;
            let paid: rust_decimal::Decimal = row
                .try_get("", "paid_amount")
                .map_err(|error| DomainError::Internal(format!("Invalid paid amount: {error}")))?;
            let student_name: String = row
                .try_get("", "student_name")
                .map_err(|error| DomainError::Internal(format!("Invalid student name: {error}")))?;
            let fee_name: String = row
                .try_get("", "fee_name")
                .map_err(|error| DomainError::Internal(format!("Invalid fee name: {error}")))?;
            let status: String = row
                .try_get("", "status")
                .map_err(|error| DomainError::Internal(format!("Invalid fee status: {error}")))?;
            let amount = amount.to_f64().ok_or_else(|| {
                DomainError::Internal("Fee amount could not be represented".into())
            })?;
            let paid = paid.to_f64().ok_or_else(|| {
                DomainError::Internal("Paid amount could not be represented".into())
            })?;
            let balance = amount - paid;
            total_amount += amount;
            total_paid += paid;

            data_rows.push(serde_json::json!({
                "seq": seq,
                "student_name": student_name,
                "fee_name": fee_name,
                "amount": amount,
                "paid_amount": paid,
                "balance": balance,
                "status": status,
            }));
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "finance.fee_collection".into(),
            report_name: "Fee Collection Report".into(),
            report_name_ar: "تقرير تحصيل الرسوم".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                ColumnDef {
                    key: "seq".into(),
                    header: "#".into(),
                    header_ar: "#".into(),
                    data_type: ColumnDataType::Sequence,
                    width: Some("5%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "student_name".into(),
                    header: "Student".into(),
                    header_ar: "الطالب".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("25%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "fee_name".into(),
                    header: "Fee".into(),
                    header_ar: "الرسم".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("20%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "amount".into(),
                    header: "Amount".into(),
                    header_ar: "المبلغ".into(),
                    data_type: ColumnDataType::Currency,
                    width: Some("15%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "paid_amount".into(),
                    header: "Paid".into(),
                    header_ar: "المدفوع".into(),
                    data_type: ColumnDataType::Currency,
                    width: Some("15%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "balance".into(),
                    header: "Balance".into(),
                    header_ar: "الرصيد".into(),
                    data_type: ColumnDataType::Currency,
                    width: Some("15%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "status".into(),
                    header: "Status".into(),
                    header_ar: "الحالة".into(),
                    data_type: ColumnDataType::Status,
                    width: Some("10%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
            ],
            rows: data_rows,
            groups: None,
            aggregations: if total_count > 0 {
                Some(Aggregations {
                    totals: serde_json::json!({
                        "seq": "",
                        "student_name": "Total",
                        "fee_name": "",
                        "amount": total_amount,
                        "paid_amount": total_paid,
                        "balance": total_amount - total_paid,
                        "status": "",
                    }),
                    subtotals: None,
                })
            } else {
                None
            },
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// STUDENT LIST DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct StudentListDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for StudentListDataProvider {
    fn provider_key(&self) -> &str {
        "students.student_list"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let class_filter = parse_uuid_param(parameters, "class_id")?;
        let order_by = student_sort_clause(&org_context.student_sort, "s");
        let mut sql = format!(
            r#"SELECT
                    e.id, e.status,
                    COALESCE(s.student_number, '') as student_number,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, s.student_number, '') as student_name,
                    COALESCE(sec.name, '') as class_name
                FROM enrollments e
                LEFT JOIN students s ON e.student_id = s.id
                LEFT JOIN persons p ON s.person_id = p.id
                LEFT JOIN sections sec ON e.section_id = sec.id
                WHERE e.org_id = $1"#
        );
        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];
        if let Some(cid) = class_filter {
            sql.push_str(" AND e.section_id = $2");
            values.push(cid.into());
        }
        sql.push_str(&format!(" ORDER BY sec.name, {order_by}"));

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Student list query failed: {e}")))?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        for row in &rows {
            seq += 1;
            data_rows.push(serde_json::json!({
                "seq": seq,
                "student_number": row.try_get::<String>("", "student_number").unwrap_or_default(),
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "class_name": row.try_get::<String>("", "class_name").unwrap_or_default(),
                "status": row.try_get::<String>("", "status").unwrap_or_default(),
            }));
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "students.student_list".into(),
            report_name: "Student List".into(),
            report_name_ar: "قائمة الطلاب".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "6%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "student_number",
                    "Student #",
                    "الرقم الأكاديمي",
                    ColumnDataType::Text,
                    "18%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "student_name",
                    "Student Name",
                    "اسم الطالب",
                    ColumnDataType::Text,
                    "36%",
                    None,
                ),
                col_def(
                    "class_name",
                    "Class",
                    "الصف",
                    ColumnDataType::Text,
                    "24%",
                    None,
                ),
                col_def(
                    "status",
                    "Status",
                    "الحالة",
                    ColumnDataType::Status,
                    "16%",
                    Some(ColumnAlign::Center),
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// TERM RESULT / STUDENT CERTIFICATE DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct TermResultDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for TermResultDataProvider {
    fn provider_key(&self) -> &str {
        "academic.term_result"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let student_filter = parse_uuid_param(parameters, "student_id")?;
        let period_filter = parse_uuid_param(parameters, "period_id")?;
        let class_filter = parse_uuid_param(parameters, "class_id")?;
        let is_general_mode = student_filter.is_none();
        let student_order = student_sort_clause(&org_context.student_sort, "st");

        let mut sql = r#"SELECT
            g.id, g.score, COALESCE(g.max_score, 100) as max_score,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            COALESCE(s.name, '') as subject_name,
            COALESCE(s.code, '') as subject_code,
            COALESCE(c.name, '') as class_name,
            COALESCE(ap.name, '') as period_name
        FROM grades g
        LEFT JOIN students st ON g.student_id = st.id
        LEFT JOIN persons p ON st.person_id = p.id
        LEFT JOIN section_subjects cs ON g.section_subject_id = cs.id
        LEFT JOIN subjects s ON cs.subject_id = s.id
        LEFT JOIN sections c ON cs.section_id = c.id
        LEFT JOIN academic_periods ap ON g.period_id = ap.id
        WHERE g.org_id = $1"#
            .to_string();

        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];

        if let Some(sid) = student_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.student_id = ${idx}"));
            values.push(sid.into());
        }
        if let Some(pid) = period_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.period_id = ${idx}"));
            values.push(pid.into());
        }
        if let Some(cid) = class_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND cs.section_id = ${idx}"));
            values.push(cid.into());
        }

        sql.push_str(&format!(" ORDER BY c.name, {student_order}, s.name"));

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Term result query failed: {e}")))?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_score = 0.0f64;
        let mut max_total = 0.0f64;

        for row in &rows {
            seq += 1;
            let score = decimal_to_f64(row, "score");
            let max = decimal_to_f64(row, "max_score");
            let max = if max > 0.0 { max } else { 100.0 };
            total_score += score;
            max_total += max;

            data_rows.push(serde_json::json!({
                "seq": seq,
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "subject_name": row.try_get::<String>("", "subject_name").unwrap_or_default(),
                "subject_code": row.try_get::<String>("", "subject_code").unwrap_or_default(),
                "class_name": row.try_get::<String>("", "class_name").unwrap_or_default(),
                "period_name": row.try_get::<String>("", "period_name").unwrap_or_default(),
                "score": score,
                "max_score": max,
                "percentage": if max > 0.0 { (score / max) * 100.0 } else { 0.0 },
                "status": if score >= (max * 0.5) { "Pass" } else { "Fail" },
            }));
        }

        let total_count = data_rows.len() as u64;
        let mut columns = vec![col_def(
            "seq",
            "#",
            "#",
            ColumnDataType::Sequence,
            "5%",
            Some(ColumnAlign::Center),
        )];
        if is_general_mode {
            columns.push(col_def(
                "student_name",
                "Student",
                "الطالب",
                ColumnDataType::Text,
                "24%",
                None,
            ));
            columns.push(col_def(
                "class_name",
                "Class",
                "الصف",
                ColumnDataType::Text,
                "14%",
                None,
            ));
        }
        columns.push(col_def(
            "subject_name",
            "Subject",
            "المادة",
            ColumnDataType::Text,
            if is_general_mode { "22%" } else { "35%" },
            None,
        ));
        columns.push(ColumnDef {
            key: "score".into(),
            header: "Score".into(),
            header_ar: "الدرجة".into(),
            data_type: ColumnDataType::Decimal,
            width: Some(if is_general_mode { "12%" } else { "20%" }.into()),
            align: Some(ColumnAlign::End),
            format: Some("number:2".into()),
            visible: true,
            total_type: Some(AggregationType::Sum),
        });
        columns.push(ColumnDef {
            key: "max_score".into(),
            header: "Max".into(),
            header_ar: "العظمى".into(),
            data_type: ColumnDataType::Decimal,
            width: Some(if is_general_mode { "11%" } else { "20%" }.into()),
            align: Some(ColumnAlign::End),
            format: Some("number:2".into()),
            visible: true,
            total_type: Some(AggregationType::Sum),
        });
        columns.push(col_def(
            "status",
            "Result",
            "النتيجة",
            ColumnDataType::Status,
            if is_general_mode { "12%" } else { "20%" },
            Some(ColumnAlign::Center),
        ));

        Ok(ReportDataset {
            report_code: "academic.term_result".into(),
            report_name: "Term Student Certificate".into(),
            report_name_ar: "شهادة نتائج الفصل الدراسي".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns,
            rows: data_rows,
            groups: None,
            aggregations: if total_count > 0 {
                Some(Aggregations {
                    totals: serde_json::json!({
                        "seq": "",
                        "subject_name": "Total / المجموع",
                        "score": total_score,
                        "max_score": max_total,
                        "status": if total_score >= (max_total * 0.5) { "Pass" } else { "Fail" },
                    }),
                    subtotals: None,
                })
            } else {
                None
            },
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// EXAM SEATING PLAN DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct ExamSeatingPlanDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for ExamSeatingPlanDataProvider {
    fn provider_key(&self) -> &str {
        "exam.seating_plan"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    sn.id, sn.seating_number,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
                    COALESCE(r.name, '') as room_name,
                    COALESCE(es.name, '') as session_name
                FROM seating_numbers sn
                LEFT JOIN students st ON sn.student_id = st.id
                LEFT JOIN persons p ON st.person_id = p.id
                LEFT JOIN rooms r ON sn.room_id = r.id
                LEFT JOIN exam_sessions es ON sn.exam_session_id = es.id
                WHERE es.org_id = $1
                ORDER BY r.name, sn.seating_number"#,
                vec![org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Exam seating plan query failed: {e}")))?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        for row in &rows {
            seq += 1;
            data_rows.push(serde_json::json!({
                "seq": seq,
                "seating_number": row.try_get::<String>("", "seating_number").unwrap_or_default(),
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "room_name": row.try_get::<String>("", "room_name").unwrap_or_default(),
                "session_name": row.try_get::<String>("", "session_name").unwrap_or_default(),
            }));
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "exam.seating_plan".into(),
            report_name: "Exam Seating Plan".into(),
            report_name_ar: "مخطط جلوس الطلاب في الاختبارات".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                ColumnDef {
                    key: "seq".into(),
                    header: "#".into(),
                    header_ar: "#".into(),
                    data_type: ColumnDataType::Sequence,
                    width: Some("8%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "seating_number".into(),
                    header: "Seat #".into(),
                    header_ar: "رقم الجلوس".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("20%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "student_name".into(),
                    header: "Student Name".into(),
                    header_ar: "اسم الطالب".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("42%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "room_name".into(),
                    header: "Exam Room".into(),
                    header_ar: "القاعة".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("30%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// PAYMENT RECEIPT DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct PaymentReceiptDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for PaymentReceiptDataProvider {
    fn provider_key(&self) -> &str {
        "finance.payment_receipt"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let payment_filter = parse_uuid_param(parameters, "payment_id")?;
        let student_filter = parse_uuid_param(parameters, "student_id")?;

        let mut sql = r#"SELECT
            py.id, py.receipt_number, py.amount, py.currency_code, py.payment_method, py.payment_date, COALESCE(py.notes, '') as notes,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            COALESCE(ft.name, '') as fee_name
        FROM payments py
        LEFT JOIN student_fees sf ON py.fee_id = sf.id
        LEFT JOIN fee_templates ft ON sf.fee_template_id = ft.id
        LEFT JOIN students st ON sf.student_id = st.id
        LEFT JOIN persons p ON st.person_id = p.id
        WHERE sf.org_id = $1"#
            .to_string();

        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];

        if let Some(pid) = payment_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND py.id = ${idx}"));
            values.push(pid.into());
        }
        if let Some(sid) = student_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND sf.student_id = ${idx}"));
            values.push(sid.into());
        }

        sql.push_str(" ORDER BY py.payment_date DESC");

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Payment receipt query failed: {e}")))?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        for row in &rows {
            seq += 1;
            data_rows.push(serde_json::json!({
                "seq": seq,
                "receipt_number": row.try_get::<String>("", "receipt_number").unwrap_or_default(),
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "fee_name": row.try_get::<String>("", "fee_name").unwrap_or_default(),
                "amount": decimal_to_f64(row, "amount"),
                "currency": row.try_get::<Option<String>>("", "currency_code").ok().flatten()
                    .or_else(|| org_context.currency_code.clone()).unwrap_or_default(),
                "payment_method": row.try_get::<String>("", "payment_method").unwrap_or_default(),
                "payment_date": date_str(row, "payment_date"),
                "notes": row.try_get::<String>("", "notes").unwrap_or_default(),
            }));
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "finance.payment_receipt".into(),
            report_name: "Payment Receipt Voucher".into(),
            report_name_ar: "سند قبض مالي".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                ColumnDef {
                    key: "receipt_number".into(),
                    header: "Receipt #".into(),
                    header_ar: "رقم السند".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("15%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "student_name".into(),
                    header: "Student".into(),
                    header_ar: "الطالب".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("30%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "fee_name".into(),
                    header: "Fee Type".into(),
                    header_ar: "نوع الرسم".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("25%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "amount".into(),
                    header: "Amount".into(),
                    header_ar: "المبلغ".into(),
                    data_type: ColumnDataType::Currency,
                    width: Some("15%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "payment_method".into(),
                    header: "Method".into(),
                    header_ar: "طريقة الدفع".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("15%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// ANNUAL RESULT / TRANSCRIPT DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct AnnualResultDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for AnnualResultDataProvider {
    fn provider_key(&self) -> &str {
        "academic.annual_result"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let student_filter = parse_uuid_param(parameters, "student_id")?;
        let class_filter = parse_uuid_param(parameters, "class_id")?;
        let is_general_mode = student_filter.is_none();
        let student_order = student_sort_clause(&org_context.student_sort, "st");

        let mut sql = r#"SELECT
            g.id, g.score, COALESCE(g.max_score, 100) as max_score,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            COALESCE(s.name, '') as subject_name,
            COALESCE(s.code, '') as subject_code,
            COALESCE(c.name, '') as class_name,
            COALESCE(ay.name, '') as year_name
        FROM grades g
        LEFT JOIN students st ON g.student_id = st.id
        LEFT JOIN persons p ON st.person_id = p.id
        LEFT JOIN section_subjects cs ON g.section_subject_id = cs.id
        LEFT JOIN subjects s ON cs.subject_id = s.id
        LEFT JOIN sections c ON cs.section_id = c.id
        LEFT JOIN academic_periods ap ON g.period_id = ap.id
        LEFT JOIN academic_calendars ay ON ap.calendar_id = ay.id
        WHERE g.org_id = $1"#
            .to_string();

        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];

        if let Some(sid) = student_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.student_id = ${idx}"));
            values.push(sid.into());
        }
        if let Some(cid) = class_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND cs.section_id = ${idx}"));
            values.push(cid.into());
        }

        sql.push_str(&format!(" ORDER BY c.name, {student_order}, s.name"));

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Annual result query failed: {e}")))?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_score = 0.0f64;
        let mut max_total = 0.0f64;

        for row in &rows {
            seq += 1;
            let score = decimal_to_f64(row, "score");
            let max = decimal_to_f64(row, "max_score");
            let max = if max > 0.0 { max } else { 100.0 };
            total_score += score;
            max_total += max;

            data_rows.push(serde_json::json!({
                "seq": seq,
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "subject_name": row.try_get::<String>("", "subject_name").unwrap_or_default(),
                "subject_code": row.try_get::<String>("", "subject_code").unwrap_or_default(),
                "class_name": row.try_get::<String>("", "class_name").unwrap_or_default(),
                "year_name": row.try_get::<String>("", "year_name").unwrap_or_default(),
                "score": score,
                "max_score": max,
                "percentage": if max > 0.0 { (score / max) * 100.0 } else { 0.0 },
                "status": if score >= (max * 0.5) { "Pass" } else { "Fail" },
            }));
        }

        let total_count = data_rows.len() as u64;
        let mut columns = vec![col_def(
            "seq",
            "#",
            "#",
            ColumnDataType::Sequence,
            "5%",
            Some(ColumnAlign::Center),
        )];
        if is_general_mode {
            columns.push(col_def(
                "student_name",
                "Student",
                "الطالب",
                ColumnDataType::Text,
                "24%",
                None,
            ));
            columns.push(col_def(
                "class_name",
                "Class",
                "الصف",
                ColumnDataType::Text,
                "14%",
                None,
            ));
        }
        columns.push(col_def(
            "subject_name",
            "Subject",
            "المادة الدراسية",
            ColumnDataType::Text,
            if is_general_mode { "22%" } else { "35%" },
            None,
        ));
        columns.push(ColumnDef {
            key: "score".into(),
            header: "Annual Total".into(),
            header_ar: "المجموع السنوي".into(),
            data_type: ColumnDataType::Decimal,
            width: Some(if is_general_mode { "12%" } else { "20%" }.into()),
            align: Some(ColumnAlign::End),
            format: Some("number:2".into()),
            visible: true,
            total_type: Some(AggregationType::Sum),
        });
        columns.push(ColumnDef {
            key: "max_score".into(),
            header: "Max".into(),
            header_ar: "العظمى".into(),
            data_type: ColumnDataType::Decimal,
            width: Some(if is_general_mode { "11%" } else { "20%" }.into()),
            align: Some(ColumnAlign::End),
            format: Some("number:2".into()),
            visible: true,
            total_type: Some(AggregationType::Sum),
        });
        columns.push(col_def(
            "status",
            "Result",
            "النتيجة",
            ColumnDataType::Status,
            if is_general_mode { "12%" } else { "20%" },
            Some(ColumnAlign::Center),
        ));

        Ok(ReportDataset {
            report_code: "academic.annual_result".into(),
            report_name: "Annual Student Report Card".into(),
            report_name_ar: "الشهادة السنوية العامة للنتائج".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns,
            rows: data_rows,
            groups: None,
            aggregations: if total_count > 0 {
                Some(Aggregations {
                    totals: serde_json::json!({
                        "seq": "",
                        "subject_name": "Grand Total / المجموع العام",
                        "score": total_score,
                        "max_score": max_total,
                        "status": if total_score >= (max_total * 0.5) { "Pass" } else { "Fail" },
                    }),
                    subtotals: None,
                })
            } else {
                None
            },
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// ACADEMIC CERTIFICATE DATA PROVIDER (COMPREHENSIVE / BILINGUAL)
// ═══════════════════════════════════════════════════════════════

pub struct AcademicCertificateDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for AcademicCertificateDataProvider {
    fn provider_key(&self) -> &str {
        "academic.certificate"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let student_filter = parse_uuid_param(parameters, "student_id")?;
        let class_filter = parse_uuid_param(parameters, "class_id")?;
        let year_filter = parse_uuid_param(parameters, "year_id")?;
        let period_filter = parse_uuid_param(parameters, "period_id")?;
        let is_general_mode = student_filter.is_none();
        let student_order = student_sort_clause(&org_context.student_sort, "st");

        let mut sql = r#"SELECT
            g.id, g.score, COALESCE(g.max_score, 100) as max_score,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            COALESCE(sn_agg.seating_number, '') as seat_number,
            COALESCE(s.name, '') as subject_name,
            COALESCE(s.code, '') as subject_code,
            COALESCE(c.name, '') as class_name,
            COALESCE(ay.name, '') as year_name,
            COALESCE(ap.name, '') as period_name
        FROM grades g
        LEFT JOIN students st ON g.student_id = st.id
        LEFT JOIN persons p ON st.person_id = p.id
        LEFT JOIN (
            SELECT sn.student_id, MAX(sn.seating_number) as seating_number
            FROM seating_numbers sn
            GROUP BY sn.student_id
        ) sn_agg ON sn_agg.student_id = st.id
        LEFT JOIN section_subjects cs ON g.section_subject_id = cs.id
        LEFT JOIN subjects s ON cs.subject_id = s.id
        LEFT JOIN sections c ON cs.section_id = c.id
        LEFT JOIN academic_periods ap ON g.period_id = ap.id
        LEFT JOIN academic_calendars ay ON ap.calendar_id = ay.id
        WHERE g.org_id = $1"#
            .to_string();

        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];

        if let Some(sid) = student_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.student_id = ${idx}"));
            values.push(sid.into());
        }
        if let Some(cid) = class_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND cs.section_id = ${idx}"));
            values.push(cid.into());
        }
        if let Some(yid) = year_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND ap.calendar_id = ${idx}"));
            values.push(yid.into());
        }
        if let Some(pid) = period_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.period_id = ${idx}"));
            values.push(pid.into());
        }

        sql.push_str(&format!(" ORDER BY c.name, {student_order}, s.name"));

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| {
                DomainError::Internal(format!("Academic certificate query failed: {e}"))
            })?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_score = 0.0f64;
        let mut max_total = 0.0f64;
        let mut min_total = 0.0f64;

        let mut student_name = String::new();
        let mut class_name = String::new();
        let mut year_name = String::new();
        let mut seat_number = String::new();

        for row in &rows {
            seq += 1;
            let score = decimal_to_f64(row, "score");
            let max = decimal_to_f64(row, "max_score");
            let max = if max > 0.0 { max } else { 100.0 };
            let min = max * 0.5;

            total_score += score;
            max_total += max;
            min_total += min;

            let s_name = row
                .try_get::<String>("", "student_name")
                .unwrap_or_default();
            if student_name.is_empty() && !s_name.is_empty() {
                student_name = s_name.clone();
            }

            let c_name = row.try_get::<String>("", "class_name").unwrap_or_default();
            if class_name.is_empty() && !c_name.is_empty() {
                class_name = c_name.clone();
            }

            let y_name = row.try_get::<String>("", "year_name").unwrap_or_default();
            if year_name.is_empty() && !y_name.is_empty() {
                year_name = y_name;
            }

            let sn_val = row.try_get::<String>("", "seat_number").unwrap_or_default();
            if seat_number.is_empty() && !sn_val.is_empty() {
                seat_number = sn_val;
            }

            let t1_cw = (score * 0.3).round();
            let t1_ex = score - t1_cw;
            let pct = if max > 0.0 {
                (score / max) * 100.0
            } else {
                0.0
            };

            let appreciation = if pct >= 90.0 {
                "ممتاز / Excellent"
            } else if pct >= 80.0 {
                "جيد جداً / Very Good"
            } else if pct >= 65.0 {
                "جيد / Good"
            } else if pct >= 50.0 {
                "مقبول / Pass"
            } else {
                "ضعيف / Fail"
            };

            let status = if score >= min { "ناجح" } else { "راسب" };

            data_rows.push(serde_json::json!({
                "seq": seq,
                "student_name": s_name,
                "class_name": c_name,
                "subject_name": row.try_get::<String>("", "subject_name").unwrap_or_default(),
                "subject_name_ar": row.try_get::<String>("", "subject_name").unwrap_or_default(),
                "subject_code": row.try_get::<String>("", "subject_code").unwrap_or_default(),
                "max_score": max,
                "min_score": min,
                "t1_coursework": t1_cw,
                "t1_exam": t1_ex,
                "t1_total": score,
                "t2_coursework": t1_cw,
                "t2_exam": t1_ex,
                "t2_total": score,
                "annual_total": score,
                "percentage": format!("{:.1}%", pct),
                "appreciation": appreciation,
                "status": status,
            }));
        }

        let total_count = data_rows.len() as u64;
        let overall_pct = if max_total > 0.0 {
            (total_score / max_total) * 100.0
        } else {
            0.0
        };
        let final_status = if total_count == 0 {
            ""
        } else if total_score >= min_total {
            "ناجح / منتقل للصف التالي"
        } else {
            "راسب / دور ثان"
        };
        let overall_appreciation = if total_count == 0 {
            ""
        } else if overall_pct >= 90.0 {
            "ممتاز"
        } else if overall_pct >= 80.0 {
            "جيد جداً"
        } else if overall_pct >= 65.0 {
            "جيد"
        } else if overall_pct >= 50.0 {
            "مقبول"
        } else {
            "ضعيف"
        };

        Ok(ReportDataset {
            report_code: "academic.certificate".into(),
            report_name: "Official Academic Student Certificate".into(),
            report_name_ar: "شهادة درجات الطالب المعتمدة".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: {
                let mut cols = vec![col_def(
                    "seq",
                    "م",
                    "م",
                    ColumnDataType::Sequence,
                    "5%",
                    Some(ColumnAlign::Center),
                )];
                if is_general_mode {
                    cols.push(col_def(
                        "student_name",
                        "Student",
                        "الطالب",
                        ColumnDataType::Text,
                        "20%",
                        None,
                    ));
                    cols.push(col_def(
                        "class_name",
                        "Class",
                        "الصف",
                        ColumnDataType::Text,
                        "11%",
                        None,
                    ));
                }
                cols.extend([
                    col_def(
                        "subject_name",
                        "Subject / المادة",
                        "المادة الدراسية",
                        ColumnDataType::Text,
                        if is_general_mode { "18%" } else { "25%" },
                        None,
                    ),
                    ColumnDef {
                        key: "max_score".into(),
                        header: "Max / الكبرى".into(),
                        header_ar: "النهاية الكبرى".into(),
                        data_type: ColumnDataType::Decimal,
                        width: Some("9%".into()),
                        align: Some(ColumnAlign::Center),
                        format: Some("number:0".into()),
                        visible: true,
                        total_type: Some(AggregationType::Sum),
                    },
                    ColumnDef {
                        key: "min_score".into(),
                        header: "Min / الصغرى".into(),
                        header_ar: "النهاية الصغرى".into(),
                        data_type: ColumnDataType::Decimal,
                        width: Some("9%".into()),
                        align: Some(ColumnAlign::Center),
                        format: Some("number:0".into()),
                        visible: true,
                        total_type: Some(AggregationType::Sum),
                    },
                    ColumnDef {
                        key: "annual_total".into(),
                        header: "المجموع".into(),
                        header_ar: "المجموع السنوي".into(),
                        data_type: ColumnDataType::Decimal,
                        width: Some("10%".into()),
                        align: Some(ColumnAlign::Center),
                        format: Some("number:0".into()),
                        visible: true,
                        total_type: Some(AggregationType::Sum),
                    },
                    col_def(
                        "appreciation",
                        "التقدير",
                        "التقدير",
                        ColumnDataType::Text,
                        "10%",
                        Some(ColumnAlign::Center),
                    ),
                    col_def(
                        "status",
                        "النتيجة",
                        "النتيجة",
                        ColumnDataType::Status,
                        "8%",
                        Some(ColumnAlign::Center),
                    ),
                ]);
                cols
            },
            rows: data_rows,
            groups: None,
            aggregations: Some(Aggregations {
                totals: serde_json::json!({
                    "seq": "",
                    "subject_name": "المجموع العام / Total",
                    "student_name": student_name,
                    "class_name": class_name,
                    "year_name": year_name,
                    "seat_number": seat_number,
                    "max_score": max_total,
                    "min_score": min_total,
                    "annual_total": total_score,
                    "total_score": total_score,
                    "percentage": format!("{:.2}%", overall_pct),
                    "overall_percentage": format!("{:.2}%", overall_pct),
                    "status": final_status,
                    "appreciation": overall_appreciation,
                    "rank_in_class": "",
                }),
                subtotals: None,
            }),
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// EXAM ATTENDANCE SHEET DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct ExamAttendanceSheetDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for ExamAttendanceSheetDataProvider {
    fn provider_key(&self) -> &str {
        "exam.attendance_sheet"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    sn.id, sn.seating_number,
                    COALESCE(sec.secret_number, '-') as secret_number,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
                    COALESCE(r.name, '') as room_name,
                    COALESCE(es.name, '') as session_name
                FROM seating_numbers sn
                LEFT JOIN students st ON sn.student_id = st.id
                LEFT JOIN persons p ON st.person_id = p.id
                LEFT JOIN rooms r ON sn.room_id = r.id
                LEFT JOIN exam_sessions es ON sn.exam_session_id = es.id
                LEFT JOIN (
                    SELECT sch.exam_session_id, sec_num.student_id, MAX(sec_num.secret_number) as secret_number
                    FROM secret_numbers sec_num
                    INNER JOIN exam_schedules sch ON sec_num.exam_schedule_id = sch.id
                    GROUP BY sch.exam_session_id, sec_num.student_id
                ) sec ON sec.exam_session_id = sn.exam_session_id AND sec.student_id = sn.student_id
                WHERE es.org_id = $1
                ORDER BY r.name, sn.seating_number"#,
                vec![org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Exam attendance sheet query failed: {e}")))?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        for row in &rows {
            seq += 1;
            data_rows.push(serde_json::json!({
                "seq": seq,
                "seating_number": row.try_get::<String>("", "seating_number").unwrap_or_default(),
                "secret_number": row.try_get::<String>("", "secret_number").unwrap_or_default(),
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "room_name": row.try_get::<String>("", "room_name").unwrap_or_default(),
                "session_name": row.try_get::<String>("", "session_name").unwrap_or_default(),
                "signature": "_______________",
            }));
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "exam.attendance_sheet".into(),
            report_name: "Exam Attendance & Signature Roster".into(),
            report_name_ar: "كشف حضور وتوقيع الطلاب في الاختبارات".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                ColumnDef {
                    key: "seq".into(),
                    header: "#".into(),
                    header_ar: "#".into(),
                    data_type: ColumnDataType::Sequence,
                    width: Some("6%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "seating_number".into(),
                    header: "Seat #".into(),
                    header_ar: "رقم الجلوس".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("14%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "secret_number".into(),
                    header: "Secret #".into(),
                    header_ar: "الرقم السري".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("14%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "student_name".into(),
                    header: "Student Name".into(),
                    header_ar: "اسم الطالب".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("36%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "signature".into(),
                    header: "Signature".into(),
                    header_ar: "التوقيع".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("30%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// STUDENT ACCOUNT STATEMENT DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct StudentStatementDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for StudentStatementDataProvider {
    fn provider_key(&self) -> &str {
        "finance.student_statement"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let student_filter = parse_uuid_param(parameters, "student_id")?;
        let is_general_mode = student_filter.is_none();

        let mut sql = r#"SELECT
            sf.id, sf.net_amount as fee_amount, sf.status, sf.due_date,
            COALESCE(ft.name, '') as fee_name,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            COALESCE((SELECT SUM(amount) FROM payments py WHERE py.fee_id = sf.id), 0) as paid_amount
        FROM student_fees sf
        LEFT JOIN fee_templates ft ON sf.fee_template_id = ft.id
        LEFT JOIN students st ON sf.student_id = st.id
        LEFT JOIN persons p ON st.person_id = p.id
        WHERE sf.org_id = $1"#.to_string();

        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];

        if let Some(sid) = student_filter {
            sql.push_str(" AND sf.student_id = $2");
            values.push(sid.into());
        }

        sql.push_str(" ORDER BY sf.due_date");

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Student statement query failed: {e}")))?;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_due = 0.0f64;
        let mut total_paid = 0.0f64;

        for row in &rows {
            seq += 1;
            let due = decimal_to_f64(row, "fee_amount");
            let paid = decimal_to_f64(row, "paid_amount");
            let balance = due - paid;
            total_due += due;
            total_paid += paid;

            data_rows.push(serde_json::json!({
                "seq": seq,
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "fee_name": row.try_get::<String>("", "fee_name").unwrap_or_default(),
                "due_date": date_str(row, "due_date"),
                "fee_amount": due,
                "paid_amount": paid,
                "balance": balance,
                "status": row.try_get::<String>("", "status").unwrap_or_default(),
            }));
        }

        let total_count = data_rows.len() as u64;
        let mut columns = vec![col_def(
            "seq",
            "#",
            "#",
            ColumnDataType::Sequence,
            "5%",
            Some(ColumnAlign::Center),
        )];
        if is_general_mode {
            columns.push(col_def(
                "student_name",
                "Student",
                "الطالب",
                ColumnDataType::Text,
                "22%",
                None,
            ));
        }
        columns.push(col_def(
            "fee_name",
            "Fee / Description",
            "البيان / الرسم",
            ColumnDataType::Text,
            if is_general_mode { "23%" } else { "35%" },
            None,
        ));
        columns.extend([
            ColumnDef {
                key: "fee_amount".into(),
                header: "Due Amount".into(),
                header_ar: "المطلوب".into(),
                data_type: ColumnDataType::Currency,
                width: Some("13%".into()),
                align: Some(ColumnAlign::End),
                format: Some("number:2".into()),
                visible: true,
                total_type: Some(AggregationType::Sum),
            },
            ColumnDef {
                key: "paid_amount".into(),
                header: "Paid Amount".into(),
                header_ar: "المدفوع".into(),
                data_type: ColumnDataType::Currency,
                width: Some("13%".into()),
                align: Some(ColumnAlign::End),
                format: Some("number:2".into()),
                visible: true,
                total_type: Some(AggregationType::Sum),
            },
            ColumnDef {
                key: "balance".into(),
                header: "Remaining Balance".into(),
                header_ar: "المتبقي".into(),
                data_type: ColumnDataType::Currency,
                width: Some("12%".into()),
                align: Some(ColumnAlign::End),
                format: Some("number:2".into()),
                visible: true,
                total_type: Some(AggregationType::Sum),
            },
            col_def(
                "status",
                "Status",
                "الحالة",
                ColumnDataType::Status,
                "12%",
                Some(ColumnAlign::Center),
            ),
        ]);

        Ok(ReportDataset {
            report_code: "finance.student_statement".into(),
            report_name: "Student Account Statement".into(),
            report_name_ar: "كشف حساب الطالب المالي".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns,
            rows: data_rows,
            groups: None,
            aggregations: if total_count > 0 {
                Some(Aggregations {
                    totals: serde_json::json!({
                        "seq": "",
                        "fee_name": "Total / الإجمالي",
                        "fee_amount": total_due,
                        "paid_amount": total_paid,
                        "balance": total_due - total_paid,
                        "status": "",
                    }),
                    subtotals: None,
                })
            } else {
                None
            },
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// ATTENDANCE REPORTS DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct AttendanceReportDataProvider {
    pub db: DatabaseConnection,
    pub key: &'static str,
}

#[async_trait]
impl ReportDataProvider for AttendanceReportDataProvider {
    fn provider_key(&self) -> &str {
        self.key
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let class_filter = parse_uuid_param(parameters, "class_id")?;
        let student_filter = parse_uuid_param(parameters, "student_id")?;
        let order_by = student_sort_clause(&org_context.student_sort, "s");

        let (title_en, title_ar, sql_filter) = match self.key {
            "attendance.daily_absence" => (
                "Daily Absence Report",
                "تقرير الغياب والتأخر اليومي",
                " AND a.status IN ('absent', 'late', 'sick', 'excused')",
            ),
            "attendance.chronic_truancy" => (
                "Chronic Truancy & At-Risk Students",
                "تقرير الغياب المتكرر والإنذار المبكر",
                " AND a.status = 'absent'",
            ),
            "attendance.teacher_log" => (
                "Attendance Marking Compliance Log",
                "سجل رصد الحضور والغياب",
                "",
            ),
            _ => (
                "Monthly Attendance Summary",
                "الملخص الشهري للحضور والانضباط",
                "",
            ),
        };

        let mut sql = format!(
            r#"SELECT
                a.attendance_date,
                a.status,
                COALESCE(a.notes, '') as notes,
                COALESCE(s.student_number, '') as student_number,
                COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, s.student_number, '') as student_name,
                COALESCE(sec.name, '') as class_name
            FROM attendances a
            INNER JOIN students s ON a.student_id = s.id
            LEFT JOIN persons p ON s.person_id = p.id
            LEFT JOIN sections sec ON a.section_id = sec.id
            WHERE a.org_id = $1{sql_filter}"#
        );
        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];
        if let Some(cid) = class_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND a.section_id = ${idx}"));
            values.push(cid.into());
        }
        if let Some(sid) = student_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND a.student_id = ${idx}"));
            values.push(sid.into());
        }
        sql.push_str(&format!(
            " ORDER BY a.attendance_date DESC, sec.name, {order_by} LIMIT 500"
        ));

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Attendance report query failed: {e}")))?;

        let mut data_rows = Vec::new();
        for (idx, row) in rows.iter().enumerate() {
            data_rows.push(serde_json::json!({
                "seq": idx + 1,
                "attendance_date": date_str(row, "attendance_date"),
                "student_number": row.try_get::<String>("", "student_number").unwrap_or_default(),
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "class_name": row.try_get::<String>("", "class_name").unwrap_or_default(),
                "status": row.try_get::<String>("", "status").unwrap_or_default(),
                "notes": row.try_get::<String>("", "notes").unwrap_or_default(),
            }));
        }
        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: self.key.into(),
            report_name: title_en.into(),
            report_name_ar: title_ar.into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "6%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "attendance_date",
                    "Date",
                    "التاريخ",
                    ColumnDataType::Date,
                    "14%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "student_name",
                    "Student",
                    "اسم الطالب",
                    ColumnDataType::Text,
                    "28%",
                    None,
                ),
                col_def(
                    "class_name",
                    "Class",
                    "الصف",
                    ColumnDataType::Text,
                    "18%",
                    None,
                ),
                col_def(
                    "status",
                    "Status",
                    "الحالة",
                    ColumnDataType::Status,
                    "14%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "notes",
                    "Notes",
                    "ملاحظات",
                    ColumnDataType::Text,
                    "20%",
                    None,
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// BEHAVIOR REPORTS DATA PROVIDER
// ═══════════════════════════════════════════════════════════════

pub struct BehaviorReportDataProvider {
    pub db: DatabaseConnection,
    pub key: &'static str,
}

#[async_trait]
impl ReportDataProvider for BehaviorReportDataProvider {
    fn provider_key(&self) -> &str {
        self.key
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let student_filter = parse_uuid_param(parameters, "student_id")?;
        let merits_only = self.key == "behavior.positive_merits";
        let (title_en, title_ar, filter_sql) = if merits_only {
            (
                "Positive Behavior & Merits",
                "سجل التميز والمكافآت السلوكية",
                " AND b.points > 0",
            )
        } else {
            (
                "Behavior & Conduct Log",
                "سجل المخالفات والملاحظات السلوكية",
                "",
            )
        };

        let mut sql = format!(
            r#"SELECT
                b.created_at,
                b.record_type,
                b.description,
                b.points,
                COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, s.student_number, '') as student_name
            FROM behavior_records b
            INNER JOIN students s ON b.student_id = s.id
            LEFT JOIN persons p ON s.person_id = p.id
            WHERE b.org_id = $1{filter_sql}"#
        );
        let mut values: Vec<sea_orm::Value> = vec![org_id.into()];
        if let Some(sid) = student_filter {
            sql.push_str(" AND b.student_id = $2");
            values.push(sid.into());
        }
        sql.push_str(" ORDER BY b.created_at DESC LIMIT 500");

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Behavior report query failed: {e}")))?;

        let mut data_rows = Vec::new();
        for (idx, row) in rows.iter().enumerate() {
            data_rows.push(serde_json::json!({
                "seq": idx + 1,
                "date": date_str(row, "created_at"),
                "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                "record_type": row.try_get::<String>("", "record_type").unwrap_or_default(),
                "points": row.try_get::<i32>("", "points").unwrap_or(0),
                "description": row.try_get::<String>("", "description").unwrap_or_default(),
            }));
        }
        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: self.key.into(),
            report_name: title_en.into(),
            report_name_ar: title_ar.into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "6%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "date",
                    "Date",
                    "التاريخ",
                    ColumnDataType::Date,
                    "14%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "student_name",
                    "Student",
                    "اسم الطالب",
                    ColumnDataType::Text,
                    "25%",
                    None,
                ),
                col_def(
                    "record_type",
                    "Type",
                    "النوع",
                    ColumnDataType::Text,
                    "15%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "points",
                    "Points",
                    "النقاط",
                    ColumnDataType::Decimal,
                    "10%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "description",
                    "Description",
                    "الوصف",
                    ColumnDataType::Text,
                    "30%",
                    None,
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// FACILITIES, HR & SYSTEM REPORTS DATA PROVIDERS
// ═══════════════════════════════════════════════════════════════

pub struct AdminFacilitiesDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for AdminFacilitiesDataProvider {
    fn provider_key(&self) -> &str {
        "admin.facilities"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                "SELECT name, COALESCE(name_ar, name) as name_ar, room_type, capacity, is_exam_hall, floor FROM rooms WHERE org_id = $1 ORDER BY floor, name",
                vec![org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Facilities query failed: {e}")))?;

        let mut data_rows = Vec::new();
        for (idx, row) in rows.iter().enumerate() {
            let is_exam_hall: bool = row.try_get("", "is_exam_hall").unwrap_or(false);
            data_rows.push(serde_json::json!({
                "seq": idx + 1,
                "name": row.try_get::<String>("", "name_ar").unwrap_or_default(),
                "room_type": row.try_get::<String>("", "room_type").unwrap_or_default(),
                "capacity": row.try_get::<i32>("", "capacity").unwrap_or(0),
                "floor": row.try_get::<Option<i32>>("", "floor").ok().flatten().map_or_else(|| "—".to_string(), |f| f.to_string()),
                "exam_hall": if is_exam_hall { "نعم / Yes" } else { "لا / No" },
            }));
        }
        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "admin.facilities".into(),
            report_name: "Facilities & Rooms Inventory".into(),
            report_name_ar: "بيان القاعات والمرافق التعليمية".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "6%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "name",
                    "Room / Hall",
                    "القاعة / المرفق",
                    ColumnDataType::Text,
                    "30%",
                    None,
                ),
                col_def(
                    "room_type",
                    "Type",
                    "النوع",
                    ColumnDataType::Text,
                    "20%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "capacity",
                    "Capacity",
                    "السعة",
                    ColumnDataType::Decimal,
                    "14%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "floor",
                    "Floor",
                    "الدور",
                    ColumnDataType::Text,
                    "14%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "exam_hall",
                    "Exam Hall",
                    "لجنة اختبار",
                    ColumnDataType::Text,
                    "16%",
                    Some(ColumnAlign::Center),
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

pub struct HrEmployeesDataProvider {
    pub db: DatabaseConnection,
    pub key: &'static str,
}

#[async_trait]
impl ReportDataProvider for HrEmployeesDataProvider {
    fn provider_key(&self) -> &str {
        self.key
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let (title_en, title_ar) = match self.key {
            "hr.contracts" => (
                "Contracts & Employment Status",
                "تقرير العقود وحالة التوظيف",
            ),
            "hr.leave_balance" => (
                "Staff Directory & Leave Status",
                "بيان الكادر الوظيفي والإجازات",
            ),
            "hr.payroll" => ("Staff Payroll Roster", "كشف الكادر الوظيفي والرواتب"),
            _ => ("Faculty & Staff Directory", "دليل الكادر التعليمي والإداري"),
        };

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    e.employee_number,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, e.employee_number) as full_name,
                    COALESCE(e.job_title, '') as job_title,
                    COALESCE(e.department, '') as department,
                    e.employment_type,
                    e.hire_date,
                    e.status
                FROM employees e
                LEFT JOIN persons p ON e.person_id = p.id
                WHERE e.org_id = $1 AND e.is_deleted = FALSE
                ORDER BY e.department, full_name"#,
                vec![org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("HR employees query failed: {e}")))?;

        let mut data_rows = Vec::new();
        for (idx, row) in rows.iter().enumerate() {
            data_rows.push(serde_json::json!({
                "seq": idx + 1,
                "employee_number": row.try_get::<String>("", "employee_number").unwrap_or_default(),
                "full_name": row.try_get::<String>("", "full_name").unwrap_or_default(),
                "job_title": row.try_get::<String>("", "job_title").unwrap_or_default(),
                "department": row.try_get::<String>("", "department").unwrap_or_default(),
                "employment_type": row.try_get::<String>("", "employment_type").unwrap_or_default(),
                "hire_date": row.try_get::<String>("", "hire_date").unwrap_or_default(),
                "status": row.try_get::<String>("", "status").unwrap_or_default(),
            }));
        }
        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: self.key.into(),
            report_name: title_en.into(),
            report_name_ar: title_ar.into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "5%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "employee_number",
                    "Emp #",
                    "الرقم الوظيفي",
                    ColumnDataType::Text,
                    "13%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "full_name",
                    "Name",
                    "الاسم",
                    ColumnDataType::Text,
                    "24%",
                    None,
                ),
                col_def(
                    "job_title",
                    "Job Title",
                    "المسمى الوظيفي",
                    ColumnDataType::Text,
                    "18%",
                    None,
                ),
                col_def(
                    "department",
                    "Department",
                    "القسم",
                    ColumnDataType::Text,
                    "14%",
                    None,
                ),
                col_def(
                    "hire_date",
                    "Hire Date",
                    "تاريخ التعيين",
                    ColumnDataType::Date,
                    "13%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "status",
                    "Status",
                    "الحالة",
                    ColumnDataType::Status,
                    "13%",
                    Some(ColumnAlign::Center),
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

pub struct SystemAuditTrailDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for SystemAuditTrailDataProvider {
    fn provider_key(&self) -> &str {
        "system.audit_trail"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                "SELECT action, entity_type, created_at FROM report_audit_log WHERE org_id = $1 ORDER BY created_at DESC LIMIT 300",
                vec![org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Audit trail query failed: {e}")))?;

        let mut data_rows = Vec::new();
        for (idx, row) in rows.iter().enumerate() {
            data_rows.push(serde_json::json!({
                "seq": idx + 1,
                "created_at": date_str(row, "created_at"),
                "action": row.try_get::<String>("", "action").unwrap_or_default(),
                "entity_type": row.try_get::<String>("", "entity_type").unwrap_or_default(),
            }));
        }
        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "system.audit_trail".into(),
            report_name: "System Audit Trail Log".into(),
            report_name_ar: "سجل التدقيق والمراجعة الأمنية".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "8%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "created_at",
                    "Date",
                    "التاريخ",
                    ColumnDataType::Date,
                    "22%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "action",
                    "Action",
                    "الإجراء",
                    ColumnDataType::Text,
                    "40%",
                    None,
                ),
                col_def(
                    "entity_type",
                    "Entity",
                    "الكيان",
                    ColumnDataType::Text,
                    "30%",
                    None,
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

pub struct SystemAutopilotLogDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for SystemAutopilotLogDataProvider {
    fn provider_key(&self) -> &str {
        "system.autopilot_log"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                "SELECT task_type, status, risk_level, triggered_by, scheduled_at FROM autopilot_tasks WHERE org_id = $1 ORDER BY scheduled_at DESC LIMIT 300",
                vec![org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Autopilot log query failed: {e}")))?;

        let mut data_rows = Vec::new();
        for (idx, row) in rows.iter().enumerate() {
            data_rows.push(serde_json::json!({
                "seq": idx + 1,
                "scheduled_at": date_str(row, "scheduled_at"),
                "task_type": row.try_get::<String>("", "task_type").unwrap_or_default(),
                "risk_level": row.try_get::<String>("", "risk_level").unwrap_or_default(),
                "triggered_by": row.try_get::<String>("", "triggered_by").unwrap_or_default(),
                "status": row.try_get::<String>("", "status").unwrap_or_default(),
            }));
        }
        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "system.autopilot_log".into(),
            report_name: "Autopilot Execution Log".into(),
            report_name_ar: "سجل مهام الأتمتة والتنفيذ التلقائي".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "6%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "scheduled_at",
                    "Date",
                    "التاريخ",
                    ColumnDataType::Date,
                    "18%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "task_type",
                    "Task",
                    "المهمة",
                    ColumnDataType::Text,
                    "30%",
                    None,
                ),
                col_def(
                    "risk_level",
                    "Risk",
                    "المستوى",
                    ColumnDataType::Text,
                    "14%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "triggered_by",
                    "Source",
                    "المصدر",
                    ColumnDataType::Text,
                    "16%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "status",
                    "Status",
                    "الحالة",
                    ColumnDataType::Status,
                    "16%",
                    Some(ColumnAlign::Center),
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

pub struct SystemPermissionsDataProvider {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportDataProvider for SystemPermissionsDataProvider {
    fn provider_key(&self) -> &str {
        "system.permissions"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    COALESCE(r.name_ar, r.name) as role_name,
                    rp.resource,
                    rp.action
                FROM role_permissions rp
                INNER JOIN roles r ON rp.role_id = r.id
                WHERE rp.org_id = $1
                ORDER BY r.name, rp.resource, rp.action"#,
                vec![org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(format!("Permissions matrix query failed: {e}")))?;

        let mut data_rows = Vec::new();
        for (idx, row) in rows.iter().enumerate() {
            data_rows.push(serde_json::json!({
                "seq": idx + 1,
                "role_name": row.try_get::<String>("", "role_name").unwrap_or_default(),
                "resource": row.try_get::<String>("", "resource").unwrap_or_default(),
                "action": row.try_get::<String>("", "action").unwrap_or_default(),
            }));
        }
        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "system.permissions".into(),
            report_name: "Roles & Permissions Matrix".into(),
            report_name_ar: "مصفوفة الأدوار والصلاحيات".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                col_def(
                    "seq",
                    "#",
                    "#",
                    ColumnDataType::Sequence,
                    "8%",
                    Some(ColumnAlign::Center),
                ),
                col_def(
                    "role_name",
                    "Role",
                    "الدور",
                    ColumnDataType::Text,
                    "32%",
                    None,
                ),
                col_def(
                    "resource",
                    "Resource",
                    "المورد",
                    ColumnDataType::Text,
                    "35%",
                    None,
                ),
                col_def(
                    "action",
                    "Action",
                    "الصلاحية",
                    ColumnDataType::Text,
                    "25%",
                    Some(ColumnAlign::Center),
                ),
            ],
            rows: data_rows,
            groups: None,
            aggregations: None,
            total_count,
            generated_at: chrono::Utc::now(),
            generated_by: None,
        })
    }
}

pub fn register_default_providers(
    registry: &mut InMemoryDataProviderRegistry,
    db: DatabaseConnection,
) {
    registry.register_provider(Box::new(FeeCollectionDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(StudentListDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(TermResultDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(AnnualResultDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(ExamSeatingPlanDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(ExamAttendanceSheetDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(PaymentReceiptDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(StudentStatementDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(AcademicCertificateDataProvider { db: db.clone() }));
    for key in [
        "attendance.daily_absence",
        "attendance.monthly_summary",
        "attendance.chronic_truancy",
        "attendance.teacher_log",
    ] {
        registry.register_provider(Box::new(AttendanceReportDataProvider {
            db: db.clone(),
            key,
        }));
    }
    for key in ["behavior.incident_log", "behavior.positive_merits"] {
        registry.register_provider(Box::new(BehaviorReportDataProvider {
            db: db.clone(),
            key,
        }));
    }
    registry.register_provider(Box::new(AdminFacilitiesDataProvider { db: db.clone() }));
    for key in [
        "hr.faculty_list",
        "hr.contracts",
        "hr.leave_balance",
        "hr.payroll",
    ] {
        registry.register_provider(Box::new(HrEmployeesDataProvider {
            db: db.clone(),
            key,
        }));
    }
    registry.register_provider(Box::new(SystemAuditTrailDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(SystemAutopilotLogDataProvider { db: db.clone() }));
    registry.register_provider(Box::new(SystemPermissionsDataProvider { db }));
}
