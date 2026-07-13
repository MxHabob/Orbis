/**
 * Workflow & Approval queries.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

export function createPendingApprovalsQuery(
  orgId: string,
  approverRole: string,
) {
  return createQuery({
    queryKey: ["pending-approvals", orgId, approverRole],
    queryFn: () => commands.platform.getPendingApprovals(orgId, approverRole),
    enabled: !!orgId && !!approverRole,
    staleTime: 60 * 1000,
  });
}

export function createOverrideRequestsQuery(
  orgId: string,
  params?: Record<string, unknown>,
) {
  return createQuery({
    queryKey: ["override-requests", orgId, params ?? {}],
    queryFn: () => commands.platform.listOverrideRequests(orgId, params),
    enabled: !!orgId,
    staleTime: 60 * 1000,
  });
}

export function createOverrideDetailQuery(requestId: string) {
  return createQuery({
    queryKey: ["override-detail", requestId],
    queryFn: () => commands.platform.getOverrideRequest(requestId),
    enabled: !!requestId,
    staleTime: 30 * 1000,
  });
}
