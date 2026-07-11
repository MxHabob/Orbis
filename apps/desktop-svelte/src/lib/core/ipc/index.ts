// ── Core IPC Layer ─────────────────────────────────────────────────
// Single entry point for all Tauri IPC communication.

export { commands } from "@orbis/sdk-desktop";
export {
  isDesktopApp,
  listenForDeepLinks,
  createDeepLink,
} from "@orbis/sdk-desktop";
export { extractErrorMessage, OrbisError } from "@orbis/sdk-core";
export type * from "@orbis/sdk-core";
export { dedup } from "./dedup";
export { prefetchRouteData } from "./prefetch";
