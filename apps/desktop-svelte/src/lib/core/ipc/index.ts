// ── Core IPC Layer ─────────────────────────────────────────────────
// Single entry point for all Tauri IPC communication.
// Re-exports from @orbis/desktop-client (framework-agnostic).

export { commands } from "@orbis/desktop-client/commands";
export {
  isDesktopApp,
  listenForDeepLinks,
  createDeepLink,
} from "@orbis/desktop-client/platform";
export type {
  User,
  LoginResponse,
  JsonRecord,
  CommandError,
  Enrollment,
  UserSession,
} from "@orbis/desktop-client/types";

export { dedup } from "./dedup";
export { extractErrorMessage } from "./error";
export { prefetchRouteData } from "./prefetch";
