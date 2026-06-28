// ═══════════════════════════════════════════════════════════════
// docforge-engine — Standalone Document Template Engine
//
// This crate provides the core rendering pipeline:
// Document Model → Expression Resolution → Typst Code Gen → PDF
//
// ZERO Orbis dependencies. Usable in any Rust project.
// ═══════════════════════════════════════════════════════════════

pub mod model;
pub mod plugin;
pub mod error;
