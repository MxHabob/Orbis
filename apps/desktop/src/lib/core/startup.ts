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

import { api } from "$lib/api/client";
import type {
  StartupPhase,
  ConnectionConfig,
} from "$lib/api/types";

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
    const config = await api.connectionGetConfig();
    const validation = await api.connectionValidate();

    if (!validation.is_valid) {
      return {
        phase: "connection_setup",
        config,
        connected: false,
        error: validation.message,
      };
    }

    // Step 2: Connect to database (auto-connect may have already done this)
    try {
      await api.connectionConnect();
    } catch {
      // If connection fails, it might already be connected (auto-connect in lib.rs)
      // Try to proceed anyway — if AppState doesn't exist, system calls will fail
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
