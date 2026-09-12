/**
 * @module api/client
 *
 * Provides the singleton `api` proxy that delegates to the active `OrbisApiClient`
 * transport (LocalClient inside Tauri desktop shell, RemoteClient in browser).
 */

import { getApiClient, resetApiClient } from "./index";
import type { OrbisApiClient } from "./types";

export const api: OrbisApiClient = new Proxy({} as OrbisApiClient, {
  get(_target, prop: string | symbol) {
    if (typeof prop !== "string") return undefined;
    const client = getApiClient() as any;
    const val = client[prop];
    if (typeof val === "function") {
      return val.bind(client);
    }
    return val;
  },
});

export { getApiClient, resetApiClient };
export * from "./types";
