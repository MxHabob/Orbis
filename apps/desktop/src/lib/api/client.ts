/**
 * @module api/client
 *
 * Provides the singleton `api` proxy that delegates to the active `OrbisApiClient`
 * transport (LocalClient inside Tauri desktop shell, RemoteClient in browser).
 */

import { getApiClient, resetApiClient } from "./index";
import type { OrbisApiClient } from "./types";

export const api: OrbisApiClient = new Proxy({} as OrbisApiClient, {
  get(_target, prop) {
    const client = getApiClient();
    const val = (client as any)[prop];
    if (typeof val === "function") {
      return val.bind(client);
    }
    return val;
  },
});

export { getApiClient, resetApiClient };
export * from "./types";
