/**
 * Academic Calendar & Policy queries.
 * Migrated: calendar events → periods, periodId → periodId.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Periods for a given year (replaces calendar events). */
export function createCalendarEventsQuery(orgId: string, periodId: string) {
  return createQuery({
    queryKey: ["calendar-periods", orgId, periodId],
    queryFn: () => commands.platform.listCalendarEvents(orgId, periodId),
    enabled: !!orgId && !!periodId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Pending/active periods (replaces pending calendar events). */
export function createPendingCalendarEventsQuery(orgId: string) {
  return createQuery({
    queryKey: ["calendar-pending", orgId],
    queryFn: () => commands.platform.listPendingCalendarEvents(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Unified academic policy for org. */
export function createOrgPolicyQuery(orgId: string) {
  return createQuery({
    queryKey: ["org-policy", orgId],
    queryFn: () => commands.platform.getOrgPolicy(orgId),
    enabled: !!orgId,
    staleTime: 30 * 60 * 1000,
  });
}
