/**
 * Workflow & Approval queries.
 */
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

export function createPendingApprovalsQuery(
  orgId: () => string,
  approverRole: () => string,
) {
  return createQuery(() => ({
    queryKey: ["pending-approvals", orgId(), approverRole()],
    queryFn: () => api.listEnrollments(),
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
    queryFn: () => api.listEnrollments(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

export function createOverrideDetailQuery(requestId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollment(requestId()),
    queryFn: () => api.getEnrollment(requestId()),
    enabled: !!requestId(),
    staleTime: 30 * 1000,
  }));
}
