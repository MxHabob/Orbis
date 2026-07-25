//! Orbis database infrastructure layer.
//!
//! This crate owns all database connection logic, SQLite/PostgreSQL pragmas,
//! schema migrations, and bootstrap management. Domain crates depend
//! on `sea_orm::DatabaseConnection` — they never manage connections themselves.
//!
//! # Architecture (Unified Single Database)
//!
//! ```text
//! AppData/
//! ├── orbis.db              ← Single database (all tables, all years)
//! └── backups/
//!     └── pre_promotion_<year>_<ts>.db
//! ```
//!
//! All tables (organizations, users, academic data, finance, exams,
//! AI/autopilot, DocForge) live in a single database file.

pub mod connection;
pub mod migrations;
pub mod registry;
pub mod provider;
pub mod schemas;
pub mod bootstrap;
pub mod schema_migrations;
pub mod fee_assignment;

