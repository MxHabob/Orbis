/**
 * Application Bootstrap — one-time initialization for the Tauri desktop app.
 *
 * Called once from AppChrome.onMount (client-side only, after auth).
 * Performs non-blocking initialization:
 * 1. Resolves the academic context (year, period, phase)
 * 2. Loads workflow inbox counts (pending approvals + suggestions)
 * 3. Registers Tauri event listeners for startup automation
 *
 * Design rationale:
 * - Separated from +layout.ts (which is for SSR-compatible data loading)
 * - Separated from stores (which are UI state, not initialization logic)
 * - Idempotent — safe to call multiple times (guards with `bootstrapped` flag)
 */

import { academicContext } from "$lib/stores/academic-context.svelte";
import { workflowInbox } from "$lib/stores/workflow-inbox.svelte";
import { isDesktopApp } from "$lib/api";

let bootstrapped = false;
let unlistenCalendarSuggestions: (() => void) | null = null;

/**
 * Bootstrap the application context.
 * Fire-and-forget — does not block UI rendering.
 */
export function bootstrapApp(orgId: string): void {
  if (bootstrapped || !orgId) return;
  bootstrapped = true;

  // 1. Resolve academic context (year, active period, phase, read-only status)
  academicContext.resolveContext(orgId);

  // 2. Load workflow inbox counts (pending approvals + calendar suggestions)
  workflowInbox.loadCount();

  // 3. Register Tauri event listeners for calendar automation
  if (isDesktopApp) {
    registerTauriListeners();
  }
}

/**
 * Reset bootstrap state (e.g., on logout or org switch).
 */
export function resetBootstrap(): void {
  bootstrapped = false;
  unlistenCalendarSuggestions?.();
  unlistenCalendarSuggestions = null;
}

async function registerTauriListeners(): Promise<void> {
  try {
    const { listen } = await import("@tauri-apps/api/event");

    // Startup automation sends pending calendar suggestions
    unlistenCalendarSuggestions?.();
    unlistenCalendarSuggestions = await listen<unknown[]>(
      "calendar:pending-suggestions",
      (event) => {
        workflowInbox.addSuggestions(event.payload);
      },
    );
  } catch (e) {
    // Silently fail — event listeners are non-critical
    console.warn("Failed to register Tauri event listeners:", e);
  }
}
