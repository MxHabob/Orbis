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
use orbis_app::usecases::report::generate_report::OrgContextResolver;
use orbis_domain::entities::report::*;
use orbis_domain::errors::DomainError;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use sea_orm::DatabaseConnection;

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
        Self { providers: HashMap::new() }
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
        Self { renderers: Vec::new() }
    }
}

impl RendererRegistry for InMemoryRendererRegistry {
    fn get_renderer(&self, format: &OutputFormat) -> Option<&dyn DocumentRenderer> {
        self.renderers.iter().find(|r| r.supported_format() == *format).map(|r| r.as_ref())
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
    fn supported_format(&self) -> OutputFormat { OutputFormat::Json }

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
    fn supported_format(&self) -> OutputFormat { OutputFormat::DataOnly }

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
    fn supported_format(&self) -> OutputFormat { OutputFormat::Csv }

    async fn render(
        &self,
        dataset: &ReportDataset,
        _template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let mut csv_data = String::new();

        // Header row
        let headers: Vec<&str> = dataset.columns.iter()
            .filter(|c| c.visible)
            .map(|c| c.header.as_str())
            .collect();
        csv_data.push_str(&headers.join(","));
        csv_data.push('\n');

        // Data rows
        let visible_keys: Vec<&str> = dataset.columns.iter()
            .filter(|c| c.visible)
            .map(|c| c.key.as_str())
            .collect();

        for row in &dataset.rows {
            let values: Vec<String> = visible_keys.iter()
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

// ═══════════════════════════════════════════════════════════════
// HTML RENDERER (also serves as PDF source)
// ═══════════════════════════════════════════════════════════════

pub struct HtmlRenderer;

impl HtmlRenderer {
    fn render_html(dataset: &ReportDataset) -> String {
        let is_rtl = dataset.org.name_ar.len() > dataset.org.name.len();
        let dir = if is_rtl { "rtl" } else { "ltr" };
        let org_name = if is_rtl { &dataset.org.name_ar } else { &dataset.org.name };
        let report_title = if is_rtl { &dataset.report_name_ar } else { &dataset.report_name };

        let mut html = format!(r#"<!DOCTYPE html>
<html dir="{dir}" lang="{lang}">
<head>
<meta charset="UTF-8">
<title>{title}</title>
<style>
body {{ font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; margin: 0; padding: 20px; direction: {dir}; }}
.header {{ text-align: center; margin-bottom: 20px; border-bottom: 2px solid {color}; padding-bottom: 10px; }}
.header h1 {{ margin: 5px 0; color: {color}; font-size: 18px; }}
.header h2 {{ margin: 5px 0; font-size: 14px; color: #333; }}
.params {{ margin-bottom: 15px; font-size: 12px; color: #666; }}
.params span {{ margin-inline-end: 15px; }}
table {{ width: 100%; border-collapse: collapse; font-size: 11px; }}
th {{ background: {color}; color: white; padding: 6px 8px; text-align: start; font-weight: 600; }}
td {{ padding: 5px 8px; border-bottom: 1px solid #e0e0e0; }}
tr:nth-child(even) {{ background: #f8f9fa; }}
.totals td {{ font-weight: 700; border-top: 2px solid {color}; background: #f0f0f0; }}
.footer {{ margin-top: 15px; font-size: 10px; color: #999; text-align: center; }}
@media print {{ body {{ padding: 0; }} }}
</style>
</head>
<body>"#,
            dir = dir,
            lang = if is_rtl { "ar" } else { "en" },
            title = report_title,
            color = dataset.org.primary_color.as_deref().unwrap_or("#6366f1"),
        );

        // Header
        html.push_str(&format!(
            r#"<div class="header"><h1>{}</h1><h2>{}</h2></div>"#,
            org_name, report_title
        ));

        // Parameter summary
        if !dataset.parameter_summary.is_empty() {
            html.push_str(r#"<div class="params">"#);
            for p in &dataset.parameter_summary {
                let label = if is_rtl { &p.label_ar } else { &p.label };
                let value = if is_rtl { &p.value_ar } else { &p.value };
                html.push_str(&format!("<span><b>{}:</b> {}</span>", label, value));
            }
            html.push_str("</div>");
        }

        // Table
        let visible_cols: Vec<&ColumnDef> = dataset.columns.iter().filter(|c| c.visible).collect();
        html.push_str("<table><thead><tr>");
        for col in &visible_cols {
            let header = if is_rtl { &col.header_ar } else { &col.header };
            html.push_str(&format!("<th>{}</th>", header));
        }
        html.push_str("</tr></thead><tbody>");

        for row in &dataset.rows {
            html.push_str("<tr>");
            for col in &visible_cols {
                let val = row.get(&col.key)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        serde_json::Value::Null => String::new(),
                        _ => v.to_string(),
                    })
                    .unwrap_or_default();
                html.push_str(&format!("<td>{}</td>", val));
            }
            html.push_str("</tr>");
        }

        // Totals row
        if let Some(ref aggs) = dataset.aggregations {
            html.push_str(r#"<tr class="totals">"#);
            for col in &visible_cols {
                let val = aggs.totals.get(&col.key)
                    .map(|v| match v {
                        serde_json::Value::Number(n) => n.to_string(),
                        _ => v.to_string().trim_matches('"').to_string(),
                    })
                    .unwrap_or_default();
                html.push_str(&format!("<td>{}</td>", val));
            }
            html.push_str("</tr>");
        }

        html.push_str("</tbody></table>");

        // Footer
        html.push_str(&format!(
            r#"<div class="footer">Generated: {} | Total rows: {}</div>"#,
            dataset.generated_at.format("%Y-%m-%d %H:%M"),
            dataset.total_count
        ));

        html.push_str("</body></html>");
        html
    }
}

#[async_trait]
impl DocumentRenderer for HtmlRenderer {
    fn supported_format(&self) -> OutputFormat { OutputFormat::Html }

    async fn render(
        &self,
        dataset: &ReportDataset,
        _template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        let html = Self::render_html(dataset);
        let bytes = html.into_bytes();
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

/// PDF renderer — uses the HTML renderer output.
/// For desktop deployment (no browser needed), generates the
/// HTML content as a self-contained file. A proper HTML→PDF
/// adapter (e.g., wkhtmltopdf, headless Chrome) can be swapped
/// in later via the DocumentRendererPort.
pub struct PdfRenderer;

#[async_trait]
impl DocumentRenderer for PdfRenderer {
    fn supported_format(&self) -> OutputFormat { OutputFormat::Pdf }

    async fn render(
        &self,
        dataset: &ReportDataset,
        _template: Option<&ReportTemplate>,
        _master: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError> {
        // Generate print-optimized HTML as the PDF output
        // This is the correct architecture: the adapter can be swapped
        // for a real PDF pipeline without changing Domain/Application.
        let html = HtmlRenderer::render_html(dataset);
        let bytes = html.into_bytes();
        let size = bytes.len() as u64;
        Ok(ExportResult {
            format: OutputFormat::Pdf,
            mime_type: "text/html; charset=utf-8".into(),
            filename: format!("{}.html", dataset.report_code.replace('.', "_")),
            data: bytes,
            size,
        })
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
        parameters: &HashMap<String, ParamValue>,
        locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError> {
        use sea_orm::{ConnectionTrait, Statement};

        // Query real fee data from the database
        let rows_result = self.db.query_all(
            Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    sf.id, sf.amount, sf.paid_amount, sf.status,
                    p.full_name as student_name,
                    ft.name as fee_name
                FROM student_fees sf
                LEFT JOIN persons p ON sf.student_id = p.id
                LEFT JOIN fee_templates ft ON sf.fee_template_id = ft.id
                WHERE sf.org_id = $1
                ORDER BY p.full_name"#,
                vec![org_id.to_string().into()],
            )
        ).await;

        let mut data_rows = Vec::new();
        let mut total_amount = 0.0f64;
        let mut total_paid = 0.0f64;
        let mut seq = 0u64;

        if let Ok(rows) = rows_result {
            for row in &rows {
                seq += 1;
                let amount: f64 = row.try_get::<f64>("", "amount").unwrap_or(0.0);
                let paid: f64 = row.try_get::<f64>("", "paid_amount").unwrap_or(0.0);
                let balance = amount - paid;
                total_amount += amount;
                total_paid += paid;

                data_rows.push(serde_json::json!({
                    "seq": seq,
                    "student_name": row.try_get::<String>("", "student_name").unwrap_or_default(),
                    "fee_name": row.try_get::<String>("", "fee_name").unwrap_or_default(),
                    "amount": amount,
                    "paid_amount": paid,
                    "balance": balance,
                    "status": row.try_get::<String>("", "status").unwrap_or_default(),
                }));
            }
        }

        let total_count = data_rows.len() as u64;

        Ok(ReportDataset {
            report_code: "finance.fee_collection".into(),
            report_name: "Fee Collection Report".into(),
            report_name_ar: "تقرير تحصيل الرسوم".into(),
            org: org_context.clone(),
            parameter_summary: vec![],
            columns: vec![
                ColumnDef { key: "seq".into(), header: "#".into(), header_ar: "#".into(), data_type: ColumnDataType::Sequence, width: Some("5%".into()), align: Some(ColumnAlign::Center), format: None, visible: true, total_type: None },
                ColumnDef { key: "student_name".into(), header: "Student".into(), header_ar: "الطالب".into(), data_type: ColumnDataType::Text, width: Some("25%".into()), align: None, format: None, visible: true, total_type: None },
                ColumnDef { key: "fee_name".into(), header: "Fee".into(), header_ar: "الرسم".into(), data_type: ColumnDataType::Text, width: Some("20%".into()), align: None, format: None, visible: true, total_type: None },
                ColumnDef { key: "amount".into(), header: "Amount".into(), header_ar: "المبلغ".into(), data_type: ColumnDataType::Currency, width: Some("15%".into()), align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true, total_type: Some(AggregationType::Sum) },
                ColumnDef { key: "paid_amount".into(), header: "Paid".into(), header_ar: "المدفوع".into(), data_type: ColumnDataType::Currency, width: Some("15%".into()), align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true, total_type: Some(AggregationType::Sum) },
                ColumnDef { key: "balance".into(), header: "Balance".into(), header_ar: "الرصيد".into(), data_type: ColumnDataType::Currency, width: Some("15%".into()), align: Some(ColumnAlign::End), format: Some("number:2".into()), visible: true, total_type: Some(AggregationType::Sum) },
                ColumnDef { key: "status".into(), header: "Status".into(), header_ar: "الحالة".into(), data_type: ColumnDataType::Status, width: Some("10%".into()), align: Some(ColumnAlign::Center), format: None, visible: true, total_type: None },
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

        let rows_result = self.db.query_all(
            Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r#"SELECT
                    e.id, e.status,
                    p.full_name as student_name,
                    c.name as class_name
                FROM enrollments e
                LEFT JOIN persons p ON e.student_id = p.id
                LEFT JOIN classes c ON e.class_id = c.id
                WHERE e.org_id = $1
                ORDER BY c.name, p.full_name"#,
                vec![org_id.to_string().into()],
            )
        ).await;

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
                ColumnDef { key: "seq".into(), header: "#".into(), header_ar: "#".into(), data_type: ColumnDataType::Sequence, width: Some("8%".into()), align: Some(ColumnAlign::Center), format: None, visible: true, total_type: None },
                ColumnDef { key: "student_name".into(), header: "Student Name".into(), header_ar: "اسم الطالب".into(), data_type: ColumnDataType::Text, width: Some("40%".into()), align: None, format: None, visible: true, total_type: None },
                ColumnDef { key: "class_name".into(), header: "Class".into(), header_ar: "الصف".into(), data_type: ColumnDataType::Text, width: Some("30%".into()), align: None, format: None, visible: true, total_type: None },
                ColumnDef { key: "status".into(), header: "Status".into(), header_ar: "الحالة".into(), data_type: ColumnDataType::Status, width: Some("22%".into()), align: Some(ColumnAlign::Center), format: None, visible: true, total_type: None },
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
