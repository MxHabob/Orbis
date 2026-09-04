// ═══════════════════════════════════════════════════════════════
// Orbis Autopilot — Automated Academic Organization Engine
//
// Orchestrates automated school operations through 3 modes:
//   automatic  — execute tasks without human approval
//   suggested  — suggest actions and wait for approval
//   manual     — observe and notify only
//
// Architecture (Unified):
//   AcademicEvaluator → reads academic_periods dates directly
//   Planner           → creates tasks from ProposedTask list
//   AcademicExecutor  → performs real DB mutations for 13 task types
//   Resumption        → catches up after app restarts / interruptions
//
// Communication:
//   Reads domain state via single DB (no registry_db + year_db)
//   Publishes results via shared-kernel EventBus
//   Persists state in autopilot_* tables
//   Reads policies from academic_policies table
// ═══════════════════════════════════════════════════════════════

pub mod entities;
pub mod config;
pub mod engine;
pub mod ops;

pub mod diagnostics;
pub mod academic_evaluator;
pub mod academic_executor;
pub mod resumption;
pub mod scheduler;

pub mod subscriber;
pub mod finance_executor;
pub mod exam_executor;

pub mod executor;
