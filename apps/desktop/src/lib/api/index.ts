/**
 * @module api
 *
 * Factory for the OrbisApiClient. Automatically selects the correct transport:
 *   - LocalClient (Tauri IPC) when running inside the Tauri desktop shell
 *   - RemoteClient (HTTP fetch) when running in a browser or web preview
 *
 * Usage:
 *   import { getApiClient } from '$lib/api';
 *   const api = getApiClient();
 *   const org = await api.getCurrentOrg();
 */

export type { OrbisApiClient } from './types';
export * from './types';
export { queryKeys } from './query-keys';
export { queryClient } from './query-client';
export { api } from './client';

export const isDesktopApp = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
export const listenForDeepLinks = () => {};
export const createDeepLink = () => "";

export const extractErrorMessage = (e: unknown): string => {
  if (e instanceof Error) return e.message;
  if (typeof e === 'string') return e;
  if (e && typeof e === 'object' && 'message' in e) return String((e as any).message);
  return 'An unknown error occurred';
};

export class OrbisError extends Error {
  details?: unknown;
  code?: string;

  constructor(message: string, details?: unknown, code?: string) {
    super(message);
    this.name = 'OrbisError';
    this.details = details;
    this.code = code;
  }

  static fromUnknown(e: unknown): OrbisError {
    if (e instanceof OrbisError) return e;
    if (e instanceof Error) return new OrbisError(e.message, (e as any).details);
    if (typeof e === 'string') return new OrbisError(e);
    if (e && typeof e === 'object') {
      const msg = (e as any).message || 'An unknown error occurred';
      return new OrbisError(String(msg), (e as any).details, (e as any).code);
    }
    return new OrbisError('An unknown error occurred');
  }
}

export { dedup } from "./dedup";
export { prefetchRouteData } from "./prefetch";
export { loadPageBundle } from "./batch";
export type { CursorRequest, CursorPage, PageBundleResult } from "./batch";

export type {
  UserDto as User,
  SessionResponse as UserSession,
  EnrollmentDto as Enrollment,
} from "./types";

import type { OrbisApiClient } from './types';
import { LocalClient } from './local-client';
import { RemoteClient } from './remote-client';

let _instance: OrbisApiClient | null = null;


/**
 * Detect whether we are running inside a Tauri webview.
 */
function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

/**
 * Returns a singleton OrbisApiClient instance.
 *
 * - Inside Tauri → LocalClient (IPC)
 * - In browser   → RemoteClient (HTTP to default server URL)
 */
export function getApiClient(): OrbisApiClient {
  if (_instance) return _instance;

  if (isTauri()) {
    _instance = new LocalClient();
  } else {
    const baseUrl = import.meta.env?.VITE_API_URL ?? 'http://localhost:3000';
    _instance = new RemoteClient(baseUrl);
  }

  return _instance;
}

/**
 * Reset the singleton (useful for tests or logout flows).
 */
export function resetApiClient(): void {
  _instance = null;
}
