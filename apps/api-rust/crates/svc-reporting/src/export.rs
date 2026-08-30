use crate::contracts::report::ReportDataset;
use csv::WriterBuilder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("CSV Error: {0}")]
    Csv(#[from] csv::Error),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Internal Error: {0}")]
    Internal(String),
}

pub fn export_to_csv(dataset: &ReportDataset) -> Result<Vec<u8>, ExportError> {
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(vec![]);
    
    // Write headers
    let headers: Vec<String> = dataset.columns.iter().map(|c| c.label.clone()).collect();
    wtr.write_record(&headers)?;
    
    // Write rows
    for row in &dataset.rows {
        let mut row_values = Vec::new();
        for col in &dataset.columns {
            let value = row.get(&col.id);
            let str_val = match value {
                Some(v) => match v {
                    serde_json::Value::Null => "".to_string(),
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => v.to_string(),
                },
                None => "".to_string(),
            };
            row_values.push(str_val);
        }
        wtr.write_record(&row_values)?;
    }
    
    // Write summary if exists
    if let Some(summary) = &dataset.summary {
        let mut row_values = Vec::new();
        for col in &dataset.columns {
            let value = summary.get(&col.id);
            let str_val = match value {
                Some(v) => match v {
                    serde_json::Value::Null => "".to_string(),
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    _ => v.to_string(),
                },
                None => "".to_string(),
            };
            row_values.push(str_val);
        }
        wtr.write_record(&row_values)?;
    }
    
    wtr.flush()?;
    Ok(wtr.into_inner().unwrap())
}
