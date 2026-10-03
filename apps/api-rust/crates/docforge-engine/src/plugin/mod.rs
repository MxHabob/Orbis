// ═══════════════════════════════════════════════════════════════
// docforge-engine — Plugin Traits
//
// These traits define the extension points for consumers.
// Orbis implements these in docforge-orbis; any other project
// can implement them for their own data sources and storage.
// ═══════════════════════════════════════════════════════════════

pub mod asset_store;
pub mod batch;
pub mod data_source;
pub mod formatter;
pub mod output_format;

pub use asset_store::*;
pub use batch::*;
pub use data_source::*;
pub use formatter::*;
pub use output_format::*;
