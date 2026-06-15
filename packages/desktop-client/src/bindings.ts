// Typed IPC layer — maintained in ./commands.ts until tauri-specta auto-export
// is enabled (requires Tauri >= 2.10 for specta compatibility).
export * from "./types";
export { commands, api } from "./commands";
