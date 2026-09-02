// ═══════════════════════════════════════════════════════════════
// docforge-orbis — Orbis Integration Adapter
//
// This crate bridges the standalone docforge-engine with Orbis:
// - Implements DataSource for Orbis entities (students, enrollments, etc.)
// - Implements AssetStore using SQLite + local filesystem
// - Provides TemplateService for CRUD operations
// - Integrates with Orbis permission system
// - Provides audit logging
//
// This crate DOES depend on core-domain and docforge-engine.
// ═══════════════════════════════════════════════════════════════

pub mod asset_store;
pub mod audit;
pub mod batch;
pub mod batch_multi_up;
pub mod datasource;
pub mod event_handler;
pub mod projection;
pub mod projection_handler;
pub mod schema;
pub mod schema_registry;
pub mod table_presets;
pub mod template_service;
