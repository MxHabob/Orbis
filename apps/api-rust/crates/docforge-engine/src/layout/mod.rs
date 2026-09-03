// ═══════════════════════════════════════════════════════════════
// docforge-engine — Layout Engine Module
// Computes visual page layouts from evaluated template trees.
// ═══════════════════════════════════════════════════════════════

pub mod expr_eval;
pub mod engine;
pub mod multi_up;

pub use expr_eval::{evaluate_node_tree, EvaluatedNode};
pub use engine::LayoutEngine;
pub use multi_up::{MultiUpConfig, MultiUpLayout, SlotPosition, SheetSize, FillOrder};
