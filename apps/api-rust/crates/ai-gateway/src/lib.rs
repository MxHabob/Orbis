// ═══════════════════════════════════════════════════════════════
// Orbis AI Gateway — Unified AI Agent Interface
//
// Architecture (ADR-002, ADR-003):
//   Domain crates are PURE — they know nothing about AI.
//   All AI metadata, action registration, and execution happen here.
//
// Components:
//   ActionRegistry    → In-memory catalog of callable actions (ADR-005)
//   ActionExecutor    → Sandboxed execution pipeline with 7-step flow
//   AgentSession      → AI agent lifecycle + session management
//   SemanticResolver  → Entity context resolution from schema introspection
//   InsightEngine     → Pattern detection + data quality monitoring
//   Explainability    → Decision explanation framework
//
// Communication:
//   Reads domain data via domain crate query repos
//   Executes mutations via domain crate command repos
//   Publishes audit entries via shared-kernel EventBus
// ═══════════════════════════════════════════════════════════════

pub mod entities;
pub mod action_registry;
pub mod action_executor;
pub mod agent;
pub mod semantic_resolver;
pub mod insight_engine;
pub mod data_quality;
pub mod explainability;
pub mod integration;

