//! Report infrastructure adapters — renderers, registries, and org context.
//!
//! DocForge stays in crates-legacy/ and is accessed ONLY through the
//! DocumentRendererPort trait. The domain/application layers never
//! reference it directly.
//!
//! Rendering decision: We use Tera HTML templates for report rendering.
//! DocForge is a layout engine for structured documents (ID cards, certificates),
//! not tabular reports. For tabular data with org headers, HTML→PDF via
//! Tera is the right fit — simpler, more maintainable, no heavy deps
//! (krilla, cosmic-text, taffy) needed in the new architecture.

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
    use std::io::Read;
    use zip::ZipArchive;

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
            .render(&dataset, None, None)
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
        template: Option<&ReportTemplate>,
        master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let is_rtl = dataset.org.name_ar.len() > dataset.org.name.len();

        let mut context = Context::new();
        context.insert("is_rtl", &is_rtl);
        context.insert("org_name", &dataset.org.name);
        context.insert("org_name_ar", &dataset.org.name_ar);
        context.insert("currency", &dataset.org.currency_code);
        context.insert("primary_color", &dataset.org.primary_color);
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

        context.insert("total_pages", &1);
        context.insert("current_page", &1);

        let mut param_map = HashMap::new();
        for p in &dataset.parameter_summary {
            param_map.insert(
                if is_rtl {
                    p.label_ar.clone()
                } else {
                    p.label.clone()
                },
                if is_rtl {
                    p.value_ar.clone()
                } else {
                    p.value.clone()
                },
            );
        }
        context.insert("parameters", &param_map);
        context.insert("has_parameters", &!param_map.is_empty());
        context.insert("columns", &dataset.columns);
        context.insert("rows", &dataset.rows);

        if let Some(aggs) = &dataset.aggregations {
            context.insert("totals", &aggs.totals);
        }

        let mut tera = Tera::default();

        let inner_template_str = template
            .and_then(|t| t.document_json.clone().into())
            .unwrap_or_else(|| crate::adapters::report_defaults::TABULAR_REPORT_HTML.to_string());

        tera.add_raw_template("inner", &inner_template_str)
            .map_err(|e| DomainError::Internal(format!("Tera inner parse error: {}", e)))?;

        let inner_html = tera
            .render("inner", &context)
            .map_err(|e| DomainError::Internal(format!("Tera inner render error: {}", e)))?;

        let final_html = if let Some(m) = master {
            context.insert("inner_content", &inner_html);
            let master_str = m.document_json.as_str();
            tera.add_raw_template("master", master_str)
                .map_err(|e| DomainError::Internal(format!("Tera master parse error: {}", e)))?;
            tera.render("master", &context)
                .map_err(|e| DomainError::Internal(format!("Tera master render error: {}", e)))?
        } else {
            inner_html
        };

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

/// PDF renderer — uses docforge-engine for visual AST templates,
/// with fallback to HTML renderer for legacy tabular Tera templates.
pub struct PdfRenderer;

#[async_trait]
impl DocumentRenderer for PdfRenderer {
    fn supported_format(&self) -> OutputFormat {
        OutputFormat::Pdf
    }

    async fn render(
        &self,
        dataset: &ReportDataset,
        template: Option<&ReportTemplate>,
        master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        if let Some(t) = template {
            if let Ok(doc) = serde_json::from_str::<docforge_engine::model::document::TemplateDocument>(&t.document_json) {
                let mut ctx_obj = serde_json::Map::new();
                ctx_obj.insert("org".into(), serde_json::to_value(&dataset.org).unwrap_or_default());
                ctx_obj.insert("columns".into(), serde_json::to_value(&dataset.columns).unwrap_or_default());
                ctx_obj.insert("rows".into(), serde_json::Value::Array(dataset.rows.clone()));
                ctx_obj.insert("subjects".into(), serde_json::Value::Array(dataset.rows.clone()));

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

                let font_candidates = [
                    ("Amiri", "C:\\Windows\\Fonts\\tahoma.ttf"),
                    ("Tahoma", "C:\\Windows\\Fonts\\tahoma.ttf"),
                    ("Arial", "C:\\Windows\\Fonts\\arial.ttf"),
                    ("DejaVu Sans", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
                ];
                let (font_family, font_path) = font_candidates
                    .iter()
                    .find(|(_, path)| std::path::Path::new(path).exists())
                    .copied()
                    .unwrap_or(("Noto Sans Arabic", ""));

                let mut fonts = docforge_engine::render::font_cache::FontCache::new(font_family);
                if !font_path.is_empty() {
                    let _ = fonts.load_font_file(
                        font_family,
                        400,
                        docforge_engine::render::font_cache::FontStyleKind::Normal,
                        font_path,
                    );
                }

                let layout_engine = docforge_engine::layout::engine::LayoutEngine::new(&fonts);
                let layout = layout_engine
                    .layout(&doc, &context_val)
                    .map_err(|e| DomainError::Internal(format!("DocForge layout error: {:?}", e)))?;

                let config = docforge_engine::render::pdf_renderer::PdfRenderConfig {
                    title: Some(doc.meta.name.clone()),
                    ..Default::default()
                };

                let pdf_out = docforge_engine::render::pdf_renderer::render_to_pdf(&layout, &fonts, &config)
                    .map_err(|e| DomainError::Internal(format!("DocForge PDF render error: {:?}", e)))?;

                let bytes = pdf_out.bytes;
                let size = bytes.len() as u64;
                return Ok(ExportResult {
                    format: OutputFormat::Pdf,
                    mime_type: "application/pdf".into(),
                    filename: format!("{}.pdf", dataset.report_code.replace('.', "_")),
                    data: bytes,
                    size,
                });
            }
        }

        let mut result = HtmlRenderer.render(dataset, template, master).await?;
        result.format = OutputFormat::Pdf;
        Ok(result)
    }
}

// ═══════════════════════════════════════════════════════════════
// ORG CONTEXT RESOLVER
// ═══════════════════════════════════════════════════════════════

pub struct DbOrgContextResolver {
    pub db: DatabaseConnection,
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

        Ok(OrgReportContext {
            id: org.id,
            name: org.name.clone(),
            name_ar: org.name_ar.clone().unwrap_or_default(),
            currency_code: org.base_currency_code.clone(),
            code: org.code.clone(),
            logo_url: org.logo.clone(),
            logo_data: None,
            address: None,
            address_ar: None,
            phone: None,
            email: None,
            website: None,
            primary_color: org.primary_color.clone(),
        })
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

        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
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
                ORDER BY student_name"#,
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
        "academic.student_list"
    }

    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        _parameters: &HashMap<String, ParamValue>,
        _locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    e.id, e.status,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, s.student_number, '') as student_name,
                    COALESCE(sec.name, '') as class_name
                FROM enrollments e
                LEFT JOIN students s ON e.student_id = s.id
                LEFT JOIN persons p ON s.person_id = p.id
                LEFT JOIN sections sec ON e.section_id = sec.id
                WHERE e.org_id = $1
                ORDER BY sec.name, student_name"#,
                vec![org_id.to_string().into()],
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        if let Ok(rows) = rows_result {
            for row in &rows {
                seq += 1;
                data_rows.push(serde_json::json!({
                    "seq": seq,
                    "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                    "class_name": row.try_get::<String>("", "class_name").unwrap_or_default(),
                    "status": row.try_get::<String>("", "status").unwrap_or_default(),
                }));
            }
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "academic.student_list".into(),
            report_name: "Student List".into(),
            report_name_ar: "قائمة الطلاب".into(),
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
                    key: "student_name".into(),
                    header: "Student Name".into(),
                    header_ar: "اسم الطالب".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("40%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "class_name".into(),
                    header: "Class".into(),
                    header_ar: "الصف".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("30%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "status".into(),
                    header: "Status".into(),
                    header_ar: "الحالة".into(),
                    data_type: ColumnDataType::Status,
                    width: Some("22%".into()),
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

        let student_filter = parameters.get("student_id").and_then(|v| v.as_str());
        let period_filter = parameters.get("period_id").and_then(|v| v.as_str());

        let mut sql = r#"SELECT
            g.id, g.score, g.max_score,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            s.name as subject_name,
            s.code as subject_code,
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

        let mut values = vec![org_id.to_string().into()];

        if let Some(sid) = student_filter {
            sql.push_str(" AND g.student_id = $2");
            values.push(sid.to_string().into());
        }
        if let Some(pid) = period_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.period_id = ${}", idx));
            values.push(pid.to_string().into());
        }

        sql.push_str(" ORDER BY s.name");

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_score = 0.0f64;
        let mut max_total = 0.0f64;

        if let Ok(rows) = rows_result {
            for row in &rows {
                seq += 1;
                let score: f64 = row.try_get::<f64>("", "score").unwrap_or(0.0);
                let max: f64 = row.try_get::<f64>("", "max_score").unwrap_or(100.0);
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
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "academic.term_result".into(),
            report_name: "Term Student Certificate".into(),
            report_name_ar: "شهادة نتائج الفصل الدراسي".into(),
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
                    key: "subject_name".into(),
                    header: "Subject".into(),
                    header_ar: "المادة".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("35%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "score".into(),
                    header: "Score".into(),
                    header_ar: "الدرجة".into(),
                    data_type: ColumnDataType::Decimal,
                    width: Some("20%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "max_score".into(),
                    header: "Max".into(),
                    header_ar: "العظمى".into(),
                    data_type: ColumnDataType::Decimal,
                    width: Some("20%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "status".into(),
                    header: "Result".into(),
                    header_ar: "النتيجة".into(),
                    data_type: ColumnDataType::Status,
                    width: Some("20%".into()),
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

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    sn.id, sn.seating_number,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
                    r.name as room_name,
                    es.name as session_name
                FROM seating_numbers sn
                LEFT JOIN students st ON sn.student_id = st.id
                LEFT JOIN persons p ON st.person_id = p.id
                LEFT JOIN rooms r ON sn.room_id = r.id
                LEFT JOIN exam_sessions es ON sn.exam_session_id = es.id
                WHERE es.org_id = $1
                ORDER BY r.name, sn.seating_number"#,
                vec![org_id.to_string().into()],
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        if let Ok(rows) = rows_result {
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

        let payment_filter = parameters.get("payment_id").and_then(|v| v.as_str());

        let mut sql = r#"SELECT
            py.id, py.receipt_number, py.amount, py.currency_code, py.payment_method, py.payment_date, py.notes,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            ft.name as fee_name
        FROM payments py
        LEFT JOIN student_fees sf ON py.fee_id = sf.id
        LEFT JOIN fee_templates ft ON sf.fee_template_id = ft.id
        LEFT JOIN students st ON sf.student_id = st.id
        LEFT JOIN persons p ON st.person_id = p.id
        WHERE sf.org_id = $1"#
            .to_string();

        let mut values = vec![org_id.to_string().into()];

        if let Some(pid) = payment_filter {
            sql.push_str(" AND py.id = $2");
            values.push(pid.to_string().into());
        }

        sql.push_str(" ORDER BY py.payment_date DESC");

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        if let Ok(rows) = rows_result {
            for row in &rows {
                seq += 1;
                data_rows.push(serde_json::json!({
                    "seq": seq,
                    "receipt_number": row.try_get::<String>("", "receipt_number").unwrap_or_default(),
                    "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                    "fee_name": row.try_get::<String>("", "fee_name").unwrap_or_default(),
                    "amount": row.try_get::<f64>("", "amount").unwrap_or(0.0),
                    "currency": row.try_get::<Option<String>>("", "currency_code").ok().flatten()
                        .or_else(|| org_context.currency_code.clone()).unwrap_or_default(),
                    "payment_method": row.try_get::<String>("", "payment_method").unwrap_or_default(),
                    "payment_date": row.try_get::<String>("", "payment_date").unwrap_or_default(),
                    "notes": row.try_get::<String>("", "notes").unwrap_or_default(),
                }));
            }
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

        let student_filter = parameters.get("student_id").and_then(|v| v.as_str());

        let mut sql = r#"SELECT
            g.id, g.score, g.max_score,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            s.name as subject_name,
            s.code as subject_code,
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

        let mut values = vec![org_id.to_string().into()];

        if let Some(sid) = student_filter {
            sql.push_str(" AND g.student_id = $2");
            values.push(sid.to_string().into());
        }

        sql.push_str(" ORDER BY s.name");

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_score = 0.0f64;
        let mut max_total = 0.0f64;

        if let Ok(rows) = rows_result {
            for row in &rows {
                seq += 1;
                let score: f64 = row.try_get::<f64>("", "score").unwrap_or(0.0);
                let max: f64 = row.try_get::<f64>("", "max_score").unwrap_or(100.0);
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
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "academic.annual_result".into(),
            report_name: "Annual Student Report Card".into(),
            report_name_ar: "الشهادة السنوية العامة للنتائج".into(),
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
                    key: "subject_name".into(),
                    header: "Subject".into(),
                    header_ar: "المادة الدراسية".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("35%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "score".into(),
                    header: "Annual Total".into(),
                    header_ar: "المجموع السنوي".into(),
                    data_type: ColumnDataType::Decimal,
                    width: Some("20%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "max_score".into(),
                    header: "Max".into(),
                    header_ar: "العظمى".into(),
                    data_type: ColumnDataType::Decimal,
                    width: Some("20%".into()),
                    align: Some(ColumnAlign::End),
                    format: Some("number:2".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "status".into(),
                    header: "Result".into(),
                    header_ar: "النتيجة".into(),
                    data_type: ColumnDataType::Status,
                    width: Some("20%".into()),
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

        let student_filter = parameters.get("student_id").and_then(|v| v.as_str());
        let class_filter = parameters.get("class_id").and_then(|v| v.as_str());
        let year_filter = parameters.get("year_id").and_then(|v| v.as_str());
        let period_filter = parameters.get("period_id").and_then(|v| v.as_str());

        let mut sql = r#"SELECT
            g.id, g.score, g.max_score,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            p.national_id as national_id,
            s.name as subject_name,
            s.code as subject_code,
            COALESCE(c.name, '') as class_name,
            COALESCE(ay.name, '') as year_name,
            COALESCE(ap.name, '') as period_name
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

        let mut values = vec![org_id.to_string().into()];

        if let Some(sid) = student_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.student_id = ${}", idx));
            values.push(sid.to_string().into());
        }
        if let Some(cid) = class_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND cs.section_id = ${}", idx));
            values.push(cid.to_string().into());
        }
        if let Some(yid) = year_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND ap.year_id = ${}", idx));
            values.push(yid.to_string().into());
        }
        if let Some(pid) = period_filter {
            let idx = values.len() + 1;
            sql.push_str(&format!(" AND g.period_id = ${}", idx));
            values.push(pid.to_string().into());
        }

        sql.push_str(" ORDER BY s.name");

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_score = 0.0f64;
        let mut max_total = 0.0f64;
        let mut min_total = 0.0f64;

        let mut student_name = String::new();
        let mut class_name = String::new();
        let mut year_name = String::new();
        let mut seat_number = String::new();

        if let Ok(rows) = rows_result {
            for row in &rows {
                seq += 1;
                let score: f64 = row.try_get::<f64>("", "score").unwrap_or(0.0);
                let max: f64 = row.try_get::<f64>("", "max_score").unwrap_or(100.0);
                let min = max * 0.5;

                total_score += score;
                max_total += max;
                min_total += min;

                let s_name = row.try_get::<String>("", "student_name").unwrap_or_default();
                if student_name.is_empty() && !s_name.is_empty() {
                    student_name = s_name.clone();
                }

                let c_name = row.try_get::<String>("", "class_name").unwrap_or_default();
                if class_name.is_empty() && !c_name.is_empty() {
                    class_name = c_name;
                }

                let y_name = row.try_get::<String>("", "year_name").unwrap_or_default();
                if year_name.is_empty() && !y_name.is_empty() {
                    year_name = y_name;
                }

                let nat_id = row.try_get::<String>("", "national_id").unwrap_or_default();
                if seat_number.is_empty() && !nat_id.is_empty() {
                    seat_number = nat_id;
                }

                let t1_cw = (score * 0.3).round();
                let t1_ex = score - t1_cw;
                let pct = if max > 0.0 { (score / max) * 100.0 } else { 0.0 };

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
        }

        let total_count = data_rows.len() as u64;
        let overall_pct = if max_total > 0.0 { (total_score / max_total) * 100.0 } else { 0.0 };
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
            columns: vec![
                ColumnDef {
                    key: "seq".into(),
                    header: "م".into(),
                    header_ar: "م".into(),
                    data_type: ColumnDataType::Sequence,
                    width: Some("5%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "subject_name".into(),
                    header: "Subject / المادة".into(),
                    header_ar: "المادة الدراسية".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("25%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "max_score".into(),
                    header: "Max / الكبرى".into(),
                    header_ar: "النهاية الكبرى".into(),
                    data_type: ColumnDataType::Decimal,
                    width: Some("10%".into()),
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
                    width: Some("10%".into()),
                    align: Some(ColumnAlign::Center),
                    format: Some("number:0".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "t1_coursework".into(),
                    header: "محصلة ف1".into(),
                    header_ar: "محصلة ف1".into(),
                    data_type: ColumnDataType::Decimal,
                    width: Some("10%".into()),
                    align: Some(ColumnAlign::Center),
                    format: Some("number:0".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "t1_exam".into(),
                    header: "اختبار ف1".into(),
                    header_ar: "اختبار ف1".into(),
                    data_type: ColumnDataType::Decimal,
                    width: Some("10%".into()),
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
                    width: Some("12%".into()),
                    align: Some(ColumnAlign::Center),
                    format: Some("number:0".into()),
                    visible: true,
                    total_type: Some(AggregationType::Sum),
                },
                ColumnDef {
                    key: "appreciation".into(),
                    header: "التقدير".into(),
                    header_ar: "التقدير".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("10%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "status".into(),
                    header: "النتيجة".into(),
                    header_ar: "النتيجة".into(),
                    data_type: ColumnDataType::Status,
                    width: Some("8%".into()),
                    align: Some(ColumnAlign::Center),
                    format: None,
                    visible: true,
                    total_type: None,
                },
            ],
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

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    sn.id, sn.seating_number,
                    COALESCE(sec.secret_number, '-') as secret_number,
                    COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
                    r.name as room_name,
                    es.name as session_name
                FROM seating_numbers sn
                LEFT JOIN students st ON sn.student_id = st.id
                LEFT JOIN persons p ON st.person_id = p.id
                LEFT JOIN rooms r ON sn.room_id = r.id
                LEFT JOIN exam_sessions es ON sn.exam_session_id = es.id
                LEFT JOIN secret_numbers sec ON sn.student_id = sec.student_id
                WHERE es.org_id = $1
                ORDER BY r.name, sn.seating_number"#,
                vec![org_id.to_string().into()],
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;

        if let Ok(rows) = rows_result {
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

        let student_filter = parameters.get("student_id").and_then(|v| v.as_str());

        let mut sql = r#"SELECT
            sf.id, sf.net_amount as fee_amount, sf.status, sf.due_date,
            ft.name as fee_name,
            COALESCE(NULLIF(TRIM(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')), ''), NULLIF(TRIM(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')), ''), p.display_name, st.student_number, '') as student_name,
            COALESCE((SELECT SUM(amount) FROM payments py WHERE py.fee_id = sf.id), 0.0) as paid_amount
        FROM student_fees sf
        LEFT JOIN fee_templates ft ON sf.fee_template_id = ft.id
        LEFT JOIN students st ON sf.student_id = st.id
        LEFT JOIN persons p ON st.person_id = p.id
        WHERE sf.org_id = $1"#.to_string();

        let mut values = vec![org_id.to_string().into()];

        if let Some(sid) = student_filter {
            sql.push_str(" AND sf.student_id = $2");
            values.push(sid.to_string().into());
        }

        sql.push_str(" ORDER BY sf.due_date");

        let rows_result = self
            .db
            .query_all(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                values,
            ))
            .await;

        let mut data_rows = Vec::new();
        let mut seq = 0u64;
        let mut total_due = 0.0f64;
        let mut total_paid = 0.0f64;

        if let Ok(rows) = rows_result {
            for row in &rows {
                seq += 1;
                let due: f64 = row.try_get::<f64>("", "fee_amount").unwrap_or(0.0);
                let paid: f64 = row.try_get::<f64>("", "paid_amount").unwrap_or(0.0);
                let balance = due - paid;
                total_due += due;
                total_paid += paid;

                data_rows.push(serde_json::json!({
                    "seq": seq,
                    "fee_name": row.try_get::<String>("", "fee_name").unwrap_or_default(),
                    "due_date": row.try_get::<String>("", "due_date").unwrap_or_default(),
                    "fee_amount": due,
                    "paid_amount": paid,
                    "balance": balance,
                    "status": row.try_get::<String>("", "status").unwrap_or_default(),
                }));
            }
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "finance.student_statement".into(),
            report_name: "Student Account Statement".into(),
            report_name_ar: "كشف حساب الطالب المالي".into(),
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
                    key: "fee_name".into(),
                    header: "Fee / Description".into(),
                    header_ar: "البيان / الرسم".into(),
                    data_type: ColumnDataType::Text,
                    width: Some("35%".into()),
                    align: None,
                    format: None,
                    visible: true,
                    total_type: None,
                },
                ColumnDef {
                    key: "fee_amount".into(),
                    header: "Due Amount".into(),
                    header_ar: "المطلوب".into(),
                    data_type: ColumnDataType::Currency,
                    width: Some("15%".into()),
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
                    width: Some("15%".into()),
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
                    width: Some("15%".into()),
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
