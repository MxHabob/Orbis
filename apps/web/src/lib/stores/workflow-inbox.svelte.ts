import * as m from "$lib/paraglide/messages";
/**
 * Workflow Inbox Store — lightweight UI state for pending actions.
 */
import { api } from "$lib/api/client";
import { extractErrorMessage } from "$lib/api";
import type { OverrideRequestDto } from "$lib/api/types";

class WorkflowInboxStore {
  pendingApprovals = $state(0);
  pendingSuggestions = $state(0);
  isLoaded = $state(false);
  myOverrideRequests = $state<OverrideRequestDto[]>([]);
  myOverrideRequestsLoading = $state(false);
  myOverrideRequestsError = $state(false);
  totalCount = $derived(this.pendingApprovals + this.pendingSuggestions);
  hasActions = $derived(this.totalCount > 0);
  private requestsLoadId = 0;
  private requestsUserId = "";

  async loadCount(): Promise<void> {
    try {
      const dashboard = await api.getDashboard();
      this.pendingApprovals = dashboard ? 0 : 0;
      this.isLoaded = true;
    } catch (e: unknown) {
      console.warn("Failed to load workflow inbox:", extractErrorMessage(e));
      this.isLoaded = true;
    }
  }

  async loadMyOverrideRequests(userId: string): Promise<void> {
    const requestId = ++this.requestsLoadId;
    if (!userId) {
      this.myOverrideRequests = [];
      this.myOverrideRequestsError = true;
      this.myOverrideRequestsLoading = false;
      return;
    }

    if (this.requestsUserId !== userId) {
      this.myOverrideRequests = [];
      this.requestsUserId = userId;
    }

    this.myOverrideRequestsLoading = true;
    this.myOverrideRequestsError = false;
    try {
      const { items: requests } = await api.listOverrides({
        mine: true,
        page: 1,
        page_size: 20,
      });
      if (requestId !== this.requestsLoadId) return;
      this.myOverrideRequests = requests;
    } catch (e: unknown) {
      if (requestId !== this.requestsLoadId) return;
      console.warn(
        "Failed to load personal override requests:",
        extractErrorMessage(e),
      );
      this.myOverrideRequests = [];
      this.myOverrideRequestsError = true;
    } finally {
      if (requestId === this.requestsLoadId) {
        this.myOverrideRequestsLoading = false;
      }
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
    this.requestsLoadId++;
    this.requestsUserId = "";
    this.myOverrideRequests = [];
    this.myOverrideRequestsLoading = false;
    this.myOverrideRequestsError = false;
  }
}

export const workflowInbox = new WorkflowInboxStore();
