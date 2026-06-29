// ═══════════════════════════════════════════════════════════════
// docforge-engine — Expressions Module
// Exposes parsing, evaluation and formatting capabilities.
// ═══════════════════════════════════════════════════════════════

pub mod parser;
pub mod formatters;
pub mod evaluator;

pub use parser::{parse_expression, Token, FormatterCall};
pub use formatters::format_value;
pub use evaluator::{evaluate_expression, evaluate_condition, resolve_path};
