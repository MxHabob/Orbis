// ═══════════════════════════════════════════════════════════════
// docforge-engine — Standalone Document Template Engine
//
// This crate provides the core rendering pipeline:
// Document Model → Expression Resolution → Layout (taffy) → PDF (krilla)
//
// ZERO Orbis dependencies. Usable in any Rust project.
// ═══════════════════════════════════════════════════════════════

pub mod model;
pub mod plugin;
pub mod error;
pub mod render;
pub mod expr;
pub mod layout;
