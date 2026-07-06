/**
 * Workflow & Approval queries — TanStack Query wrappers for override IPC.
 *
 * Full data loading for the Approvals page.
 * The global workflowInbox store only holds counts for the Title Bar badge.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Override requests pending the current user's role approval. */
export function createPendingApprovalsQuery(
  orgId: string,
  approverRole: string,
) {
  return createQuery({
    queryKey: ["pending-approvals", orgId, approverRole],
    queryFn: () => commands.overrides.getPending(orgId, approverRole),
    enabled: !!orgId && !!approverRole,
    staleTime: 60 * 1000, // 1 minute — approvals change frequently
  });
}

/** All override requests with optional filtering. */
export function createOverrideRequestsQuery(
  orgId: string,
  params?: Record<string, unknown>,
) {
  return createQuery({
    queryKey: ["override-requests", orgId, params ?? {}],
    queryFn: () => commands.overrides.list(orgId, params),
    enabled: !!orgId,
    staleTime: 60 * 1000,
  });
}

/** Single override request by ID. */
export function createOverrideDetailQuery(requestId: string) {
  return createQuery({
    queryKey: ["override-detail", requestId],
    queryFn: () => commands.overrides.get(requestId),
    enabled: !!requestId,
    staleTime: 30 * 1000,
  });
}
