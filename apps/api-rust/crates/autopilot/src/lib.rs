// ═══════════════════════════════════════════════════════════════
// Orbis Autopilot — Automated Academic Lifecycle Engine
//
// Orchestrates automated school operations through 3 modes:
//   automatic  — execute tasks without human approval
//   suggested  — suggest actions and wait for approval
//   manual     — observe and notify only
//
// Architecture:
//   Evaluator  → reads system state (calendar, phases, health)
//   Planner    → creates tasks from state + calendar events
//   Executor   → runs tasks according to execution mode
//   Resumption → catches up after app restarts / interruptions
//
// Communication:
//   Reads domain state via domain crate query repos
//   Publishes results via shared-kernel EventBus
//   Persists state in autopilot_* tables
// ═══════════════════════════════════════════════════════════════

pub mod entities;
pub mod config;
pub mod engine;
pub mod evaluator;
pub mod planner;
pub mod executor;
pub mod resumption;
pub mod diagnostics;
