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

pub mod schema;
pub mod template_service;
pub mod asset_store;
pub mod audit;
pub mod datasource;
pub mod batch;
pub mod batch_multi_up;
pub mod projection;
pub mod schema_registry;
pub mod event_handler;
pub mod projection_handler;
