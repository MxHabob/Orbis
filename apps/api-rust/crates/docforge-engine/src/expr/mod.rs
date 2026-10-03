// ═══════════════════════════════════════════════════════════════
// docforge-engine — Expressions Module
// Exposes parsing, evaluation and formatting capabilities.
// ═══════════════════════════════════════════════════════════════

pub mod evaluator;
pub mod formatters;
pub mod parser;

pub use evaluator::{evaluate_condition, evaluate_expression, resolve_path};
pub use formatters::format_value;
pub use parser::{parse_expression, FormatterCall, Token};
