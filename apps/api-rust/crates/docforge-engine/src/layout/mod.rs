// ═══════════════════════════════════════════════════════════════
// docforge-engine — Layout Engine Module
// Computes visual page layouts from evaluated template trees.
// ═══════════════════════════════════════════════════════════════

pub mod engine;
pub mod expr_eval;
pub mod multi_up;

pub use engine::LayoutEngine;
pub use expr_eval::{evaluate_node_tree, EvaluatedNode};
pub use multi_up::{FillOrder, MultiUpConfig, MultiUpLayout, SheetSize, SlotPosition};
