import * as m from "$lib/paraglide/messages";
/**
 * Workflow Inbox Store — lightweight UI state for pending actions.
 */
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";

class WorkflowInboxStore {
  pendingApprovals = $state(0);
  pendingSuggestions = $state(0);
  isLoaded = $state(false);
  totalCount = $derived(this.pendingApprovals + this.pendingSuggestions);
  hasActions = $derived(this.totalCount > 0);

  async loadCount(): Promise<void> {
    try {
      const pending = await commands.enrollment_workflow.listJobs();
      this.pendingApprovals = Array.isArray(pending) ? pending.length : 0;
      this.isLoaded = true;
    } catch (e: unknown) {
      console.warn("Failed to load workflow inbox:", extractErrorMessage(e));
      this.isLoaded = true;
    }
  }

  addSuggestions(payload: unknown[]): void {
    this.pendingSuggestions = Array.isArray(payload) ? payload.length : 0;
  }

  decrementApprovals(): void {
    if (this.pendingApprovals > 0) this.pendingApprovals--;
  }
  decrementSuggestions(): void {
    if (this.pendingSuggestions > 0) this.pendingSuggestions--;
  }
  reset(): void {
    this.pendingApprovals = 0;
    this.pendingSuggestions = 0;
    this.isLoaded = false;
  }
}

export const workflowInbox = new WorkflowInboxStore();
