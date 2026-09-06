// ── Core IPC Layer ─────────────────────────────────────────────────
// Single entry point for all Tauri IPC communication.

// We export the `api` object directly from our unified client layer.
// This ensures that all frontend features use a single, strongly-typed
// source of truth whether running in Local (SQLite/Tauri) or Remote (HTTP) mode.

export { api as commands } from "$lib/api/client"; // Temporary alias to satisfy existing imports until fully migrated

export const isDesktopApp = typeof window !== 'undefined' && !!(window as any).__TAURI_INTERNALS__;
export const listenForDeepLinks = () => {};
export const createDeepLink = () => "";

export const extractErrorMessage = (e: unknown): string => {
    if (e instanceof Error) return e.message;
    if (typeof e === 'string') return e;
    return 'An unknown error occurred';
};

export class OrbisError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'OrbisError';
    }
}

export { dedup } from "./dedup";
export { prefetchRouteData } from "./prefetch";
export { loadPageBundle } from "./batch";
export type { CursorRequest, CursorPage, PageBundleResult } from "./batch";

export type { UserDto as User, SessionResponse as UserSession } from "$lib/api/types";
export * from "$lib/api/types";

