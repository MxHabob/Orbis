//! Orbis database infrastructure layer.
//!
//! This crate owns all database connection logic, SQLite pragmas,
//! schema migrations, and registry management. Domain crates depend
//! on `sea_orm::DatabaseConnection` — they never manage connections themselves.
//!
//! # Architecture
//!
//! ```text
//! AppData/
//! ├── orbis_registry.db          ← Central index (academic_years, etc.)
//! ├── years/
//! │   ├── 2024-2025_active.db    ← Active year (full read-write)
//! │   ├── 2023-2024_archived.db  ← Archived year (read-only)
//! │   └── ...
//! └── backups/
//!     └── pre_promotion_<year>_<ts>.db
//! ```

pub mod connection;
pub mod migrations;
pub mod registry;
pub mod provider;
pub mod schemas;
pub mod bootstrap;
pub mod schema_migrations;
pub mod fee_assignment;
