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
  return createQuery(() => ({
    queryKey: ["pending-approvals", orgId(), approverRole()],
    queryFn: () => commands.enrollment_workflow.listJobs(),
    enabled: !!orgId() && !!approverRole(),
    staleTime: 60 * 1000,
  }));
}

export function createOverrideRequestsQuery(
  orgId: () => string,
  params?: () => Record<string, unknown>,
) {
  return createQuery(() => ({
    queryKey: ["override-requests", orgId(), params ? params() : {}],
    queryFn: () =>
      commands.enrollment_workflow.listJobs(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

export function createOverrideDetailQuery(requestId: () => string) {
  return createQuery(() => ({
    queryKey: ["override-detail", requestId()],
    queryFn: () => commands.enrollment_workflow.getJob(requestId()),
    enabled: !!requestId(),
    staleTime: 30 * 1000,
  }));
}
