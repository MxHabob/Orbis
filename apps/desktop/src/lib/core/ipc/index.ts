// ── Core IPC Layer ─────────────────────────────────────────────────
// Single entry point for all Tauri IPC communication.

// We export a mocked commands object temporarily to satisfy existing imports.
// Eventually, these should be replaced with `api` from `$lib/api/client.ts`.
import { invoke } from "@tauri-apps/api/core";

export const commands: any = new Proxy({}, {
  get(target: any, domain: string) {
    return new Proxy({}, {
      get(target: any, operation: string) {
        return async (...args: any[]) => {
          const tauriCommand = `${domain}_${operation.replace(/[A-Z]/g, letter => '_' + letter.toLowerCase())}`;
          const payload = args[0] || {};
          return await invoke(tauriCommand, { ...payload });
        };
      }
    });
  }
});

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
