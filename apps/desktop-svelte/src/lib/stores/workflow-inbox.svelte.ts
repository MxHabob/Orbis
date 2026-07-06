/**
 * Workflow Inbox Store — lightweight UI state for pending actions.
 *
 * Stores only COUNTS (not full objects) for performance:
 * - Title Bar shows a badge with the total count
 * - Full data is loaded via TanStack Query on the Approvals page
 *
 * This store is deliberately separate from academicContext because:
 * - Workflow/Approval is cross-module (not just academic)
 * - Future: Finance approvals, HR approvals will use the same inbox
 * - Prevents academicContext from becoming a God Object
 *
 * Uses Svelte 5 Runes ($state, $derived).
 */

import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";

class WorkflowInboxStore {
  /** Number of override requests pending the current user's approval. */
  pendingApprovals = $state(0);

  /** Number of calendar suggestions from startup automation. */
  pendingSuggestions = $state(0);

  /** Whether the inbox has been loaded. */
  isLoaded = $state(false);

  /** Total action items requiring attention. */
  totalCount = $derived(this.pendingApprovals + this.pendingSuggestions);

  /** Whether there are any pending actions. */
  hasActions = $derived(this.totalCount > 0);

  /**
   * Load pending counts from the backend.
   * Fire-and-forget — does not block UI.
   */
  async loadCount(orgId: string): Promise<void> {
    try {
      // Load pending approvals count
      // We use the 'list' endpoint with status filter — counts the results
      // This is lightweight since we only need the count
      const pending = await commands.overrides.list(orgId, {
        status: "pending",
      });
      this.pendingApprovals = Array.isArray(pending) ? pending.length : 0;

      // Calendar suggestions are loaded via Tauri events from startup_automation
      // (handled by addSuggestions below)

      this.isLoaded = true;
    } catch (e: unknown) {
      console.warn("Failed to load workflow inbox:", extractErrorMessage(e));
      this.isLoaded = true; // Mark as loaded even on error to avoid retries
    }
  }

  /**
   * Add calendar suggestions from Tauri startup automation events.
   * Called when 'calendar:pending-suggestions' event is received.
   */
  addSuggestions(payload: unknown[]): void {
    this.pendingSuggestions = Array.isArray(payload) ? payload.length : 0;
  }

  /**
   * Decrement approval count after processing one.
   * Optimistic update — avoids re-fetching the entire list.
   */
  decrementApprovals(): void {
    if (this.pendingApprovals > 0) {
      this.pendingApprovals--;
    }
  }

  /**
   * Decrement suggestion count after executing one.
   */
  decrementSuggestions(): void {
    if (this.pendingSuggestions > 0) {
      this.pendingSuggestions--;
    }
  }

  /** Reset all counts (e.g., on logout). */
  reset(): void {
    this.pendingApprovals = 0;
    this.pendingSuggestions = 0;
    this.isLoaded = false;
  }
}

export const workflowInbox = new WorkflowInboxStore();
