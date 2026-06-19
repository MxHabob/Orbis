// ── IPC Service Layer ────────────────────────────────────────────────
// Re-exports from @orbis/desktop-client (framework-agnostic).
// This is the single entry point for all Tauri IPC communication.

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
} from "@orbis/desktop-client/types";
