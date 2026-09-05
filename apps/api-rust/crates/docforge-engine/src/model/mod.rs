// ═══════════════════════════════════════════════════════════════
// docforge-engine — Document Model (Rust mirror of TypeScript types)
//
// These types are the Rust equivalent of @docforge/core's TypeScript
// types. They are used for:
// 1. Deserializing template JSON from the database
// 2. Processing the document tree in the rendering pipeline
// 3. Generating Typst source code
// ═══════════════════════════════════════════════════════════════

pub mod document;
pub mod nodes;

pub use document::*;
pub use nodes::*;
