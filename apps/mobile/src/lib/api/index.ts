/**
 * @module @orbis/mobile-api
 *
 * Central API Layer for the Orbis Mobile Application.
 * Exports:
 * - Strongly-typed domain models & DTOs
 * - OrbisMobileApiClient singleton & factory
 * - React Authentication context & useAuth hook
 * - Query Keys factory for caching
 * - Secure persistence utilities
 * - Error classes & helpers
 */

export * from "./types";
export * from "./storage";
export * from "./client";
export * from "./query-keys";
export * from "./auth-context";
