//! Orbis Report Domain — Report Definitions, Parameters, Datasets, Exporters.
//!
//! This crate owns the **reporting platform** abstraction layer:
//! - **Report Definitions**: A registry of all available reports with metadata,
//!   parameter schemas, required permissions, and supported output formats.
//! - **Report Parameters**: A type-safe, domain-specific parameter model with
//!   declarative schemas for UI generation and backend validation.
//! - **Report Datasets**: A universal, format-agnostic contract (`ReportDataset`)
//!   carrying columns, rows, groups, and aggregations — consumed by every
//!   output target (DataTable, PDF, XLSX, CSV, JSON, HTML).
//! - **Organization Context**: Enriched org metadata (logo, address, stamps)
//!   injected into report headers.
//! - **Report Registry**: In-memory + DB-backed catalog for discovering and
//!   validating available reports.
//!
//! # Architecture
//!
//! ```text
//! ReportRequest (code + params + format)
//!        │
//!        ▼
//! ReportRegistry ──► ReportDefinition (metadata + param schema + permissions)
//!        │
//!        ▼
//! Validation (params against schema)
//!        │
//!        ▼
//! Data Provider (domain crate, e.g. svc-finance::reports)
//!        │
//!        ▼
//! ReportDataset (columns + rows + groups + aggregations)
//!        │
//!        ├──► DataTable (UI)
//!        ├──► PDF (flow renderer)
//!        ├──► XLSX exporter
//!        ├──► CSV exporter
//!        └──► JSON / HTML
//! ```
//!
//! # Key Design Decisions
//!
//! 1. Reports are NOT generic SQL builders — each report has a hand-written,
//!    type-safe data provider in its domain crate.
//! 2. `ReportDataset` is the single contract — all output formats consume it.
//! 3. Business calculations stay in domain services — providers call existing
//!    ops, never recalculate.
//! 4. Report parameters are declarative schemas — UI generates forms
//!    automatically, backend validates type-safely.

pub mod definition;
pub mod request;
pub mod dataset;
pub mod org_context;
pub mod validation;
pub mod registry;
pub mod error;
pub mod providers;
pub mod exporters;
pub mod service;
pub mod flow;
