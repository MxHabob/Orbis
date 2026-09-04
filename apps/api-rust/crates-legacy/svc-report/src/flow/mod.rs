//! Flow PDF Renderer — converts `ReportDataset` to paginated PDF.
//!
//! This module bridges the reporting system with DocForge's PDF pipeline.
//! It generates `LayoutResult` (positioned elements) from report data,
//! which the existing `pdf_renderer` can render to PDF bytes.
//!
//! # Architecture
//!
//! ```text
//! ReportDataset → FlowLayout → LayoutResult → pdf_renderer → PDF bytes
//!                    │
//!                    ├── Org header (logo, name, address)
//!                    ├── Report title + parameters
//!                    ├── Table header row
//!                    ├── Data rows (with auto page breaks)
//!                    ├── Group headers + subtotals
//!                    └── Grand totals + footer
//! ```

pub mod flow_layout;
