// ═══════════════════════════════════════════════════════════════
// Orbis Integration Layer — External Adapters & Event Bridge
//
// Provides pluggable interfaces for:
//   • LLM providers (Ollama local-first, OpenAI, Anthropic)
//   • External device adapters (camera, fingerprint, IoT)
//   • Communication adapters (email, SMS, LMS)
//   • Event bridge for inbound/outbound event routing
//
// Design:
//   All external systems are behind trait abstractions.
//   Core system never depends on specific providers.
//   ADQ-3: Ollama local-first — desktop app, no internet dependency.
// ═══════════════════════════════════════════════════════════════

pub mod traits;
pub mod event_bridge;
pub mod adapters;
pub mod notification_sender;
