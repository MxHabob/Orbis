/**
 * @orbis/sdk-core — Platform-agnostic SDK core.
 *
 * This package provides:
 * - Type definitions (mirroring Rust entities/contracts)
 * - Command interfaces (the "what" — platform adapters provide the "how")
 * - Error handling utilities
 * - Permission constants
 * - Event type constants
 *
 * Platform adapters:
 * - @orbis/sdk-desktop → Tauri IPC (invoke)
 * - @orbis/sdk (existing) → HTTP (fetch) via API Gateway
 */

// ── Types ───────────────────────────────────────────────────────
export * from './types/index.js';

// ── Commands ────────────────────────────────────────────────────
export * from './commands/index.js';

// ── Errors ──────────────────────────────────────────────────────
export { OrbisError, extractErrorMessage } from './errors.js';

// ── Permissions ─────────────────────────────────────────────────
export { Permissions } from './permissions.js';

// ── Events ──────────────────────────────────────────────────────
export { EventTypes } from './events.js';

export type JsonRecord = Record<string, any>;
