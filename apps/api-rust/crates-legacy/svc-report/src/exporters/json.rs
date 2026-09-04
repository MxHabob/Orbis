//! JSON Exporter — serializes `ReportDataset` directly to JSON.
//!
//! The simplest exporter: the dataset is already serde-serializable,
//! so this is essentially `serde_json::to_vec_pretty`.

use crate::dataset::{ExportResult, ReportDataset};
use crate::error::ReportError;

/// Export a `ReportDataset` to pretty-printed JSON.
pub fn export(dataset: &ReportDataset) -> Result<ExportResult, ReportError> {
    let data = serde_json::to_vec_pretty(dataset)
        .map_err(|e| ReportError::Internal(format!("JSON serialization failed: {}", e)))?;

    let size = data.len() as u64;
    let filename = format!("{}.json", dataset.report_code.replace('.', "_"));

    Ok(ExportResult {
        format: "json".into(),
        mime_type: "application/json; charset=utf-8".into(),
        filename,
        data,
        size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::*;
    use crate::org_context::OrgContext;
    use serde_json::json;

    #[test]
    fn test_json_export() {
        let dataset = ReportDataset {
            report_code: "test.report".into(),
            report_name: "Test".into(),
            report_name_ar: "اختبار".into(),
            org: OrgContext::default(),
            parameter_summary: vec![],
            columns: vec![ColumnDef {
                key: "name".into(), header: "Name".into(), header_ar: "الاسم".into(),
                data_type: ColumnDataType::Text, width: None, align: None,
                format: None, visible: true, total_type: None, sensitive: false,
            }],
            rows: vec![json!({"name": "Ahmad"})],
            groups: None,
            aggregations: None,
            total_count: 1,
            page: 0,
            page_size: 0,
            generated_at: "2026-01-01T00:00:00Z".into(),
            generated_by: None,
        };

        let result = export(&dataset).unwrap();
        assert_eq!(result.format, "json");
        assert_eq!(result.mime_type, "application/json; charset=utf-8");

        let parsed: serde_json::Value = serde_json::from_slice(&result.data).unwrap();
        assert_eq!(parsed["reportCode"], "test.report");
        assert_eq!(parsed["rows"][0]["name"], "Ahmad");
    }
}
