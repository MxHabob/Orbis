// ═══════════════════════════════════════════════════════════════
// docforge-engine — Plugin Traits
//
// These traits define the extension points for consumers.
// Orbis implements these in docforge-orbis; any other project
// can implement them for their own data sources and storage.
// ═══════════════════════════════════════════════════════════════

pub mod data_source;
pub mod asset_store;
pub mod batch;
pub mod output_format;
pub mod formatter;

pub use data_source::*;
pub use asset_store::*;
pub use batch::*;
pub use output_format::*;
pub use formatter::*;
