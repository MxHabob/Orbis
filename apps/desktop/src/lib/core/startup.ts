/**
 * @module startup
 *
 * Startup orchestrator — determines the correct startup phase by checking:
 * 1. Connection config validity
 * 2. Database connectivity
 * 3. Organization existence (bootstrap status)
 *
 * Uses the unified `api` client (OrbisApiClient) — same interface for
 * both LocalClient (Tauri IPC) and RemoteClient (HTTP).
 */

import { api, configureApiClient } from "$lib/api";
import type { StartupPhase, ConnectionConfig } from "$lib/api/types";

export interface StartupState {
  phase: StartupPhase;
  config?: ConnectionConfig;
  error?: string;
  connected: boolean;
}

/**
 * Run the startup sequence and determine the current phase.
 *
 * This is the single entry point called by the root page on mount.
 */
export async function resolveStartupPhase(): Promise<StartupState> {
  try {
    // Step 1: Load connection config
    let config = await api.connectionGetConfig();
    if (config.mode === "api") {
      const baseUrl = config.api?.server_url;
      if (!baseUrl) {
        return {
          phase: "connection_setup",
          config,
          connected: false,
          error: "The saved API connection does not have a server URL.",
        };
      }
      configureApiClient("api", baseUrl);
      config = await api.connectionGetConfig();
    }
    const validation = await api.connectionValidate();

    if (!validation.is_valid) {
      return {
        phase: "connection_setup",
        config,
        connected: false,
        error: validation.message,
      };
    }

    try {
      await api.connectionConnect();
    } catch (e) {
      return {
        phase: "connection_setup",
        config,
        connected: false,
        error: e instanceof Error ? e.message : String(e),
      };
    }

    // Step 3: Check organization existence
    let hasOrg: boolean;
    try {
      hasOrg = await api.systemHasOrganization();
    } catch {
      // AppState not available — need connection setup
      return {
        phase: "connection_setup",
        config,
        connected: false,
        error: "Database not connected",
      };
    }

    if (!hasOrg) {
      return {
        phase: "bootstrap",
        config,
        connected: true,
      };
    }

    return {
      phase: "login", // Organization exists — proceed to auth
      config,
      connected: true,
    };
  } catch (e) {
    return {
      phase: "error",
      connected: false,
      error: e instanceof Error ? e.message : String(e),
    };
  }
}
