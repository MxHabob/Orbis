/**
 * Finance domain queries — fees, treasuries, payments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Student fees with unpaid count. */
export function createStudentFeesQuery(orgId: string) {
  return createQuery({
    queryKey: ["student-fees", orgId],
    queryFn: () => commands.finance.getStudentFees(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Treasuries for an organization. */
export function createTreasuriesQuery(orgId: string) {
  return createQuery({
    queryKey: ["treasuries", orgId],
    queryFn: () => commands.finance.getTreasuries(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Payments for an organization. */
export function createPaymentsQuery(orgId: string) {
  return createQuery({
    queryKey: ["payments", orgId],
    queryFn: () => commands.finance.getPayments(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}
