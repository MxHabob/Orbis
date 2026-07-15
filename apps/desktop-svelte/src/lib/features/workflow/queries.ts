import * as m from "$lib/paraglide/messages";
/**
 * Workflow & Approval queries.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

export function createPendingApprovalsQuery(
  orgId: () => string,
  approverRole: () => string,
) {
  return createQuery({
    get queryKey() {
      return ["pending-approvals", orgId(), approverRole()];
    },
    queryFn: () => commands.platform.getPendingApprovals(approverRole()),
    get enabled() {
      return !!orgId() && !!approverRole();
    },
    staleTime: 60 * 1000,
  });
}

export function createOverrideRequestsQuery(
  orgId: () => string,
  params?: () => Record<string, unknown>,
) {
  return createQuery({
    get queryKey() {
      return ["override-requests", orgId(), params ? params() : {}];
    },
    queryFn: () =>
      commands.platform.listOverrideRequests(params ? params() : undefined),
    get enabled() {
      return !!orgId();
    },
    staleTime: 60 * 1000,
  });
}

export function createOverrideDetailQuery(requestId: () => string) {
  return createQuery({
    get queryKey() {
      return ["override-detail", requestId()];
    },
    queryFn: () => commands.platform.getOverrideRequest(requestId()),
    get enabled() {
      return !!requestId();
    },
    staleTime: 30 * 1000,
  });
}
