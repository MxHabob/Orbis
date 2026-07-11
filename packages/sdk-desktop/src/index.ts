/**
 * @orbis/sdk-desktop — Tauri IPC adapter implementing OrbisCommands.
 *
 * This package:
 * 1. Implements OrbisCommands from @orbis/sdk-core via Tauri invoke()
 * 2. Provides platform utilities (deep links, filesystem, etc.)
 * 3. Re-exports all types from @orbis/sdk-core
 *
 * Usage in desktop app:
 *   import { commands } from "@orbis/sdk-desktop";
 *   const classes = await commands.academic.listClasses(orgId);
 */

export { commands } from "./commands";
export { isDesktopApp, listenForDeepLinks, createDeepLink } from "./platform";
export { OrbisError, extractErrorMessage } from "@orbis/sdk-core";
export type { OrbisCommands } from "@orbis/sdk-core";

// Re-export all types so consumers don't need to import from sdk-core directly
export type * from "@orbis/sdk-core";
