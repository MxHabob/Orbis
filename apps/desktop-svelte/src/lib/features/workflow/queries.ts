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
    queryFn: () => commands.enrollment_workflow.listJobs(),
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
      commands.enrollment_workflow.listJobs(),
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
    queryFn: () => commands.enrollment_workflow.getJob(requestId()),
    get enabled() {
      return !!requestId();
    },
    staleTime: 30 * 1000,
  });
}
