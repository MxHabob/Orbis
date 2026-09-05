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
