/**
 * Academic Calendar queries.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

export function createCalendarEventsQuery(orgId: string, cycleId: string) {
  return createQuery({
    queryKey: ["calendar-events", orgId, cycleId],
    queryFn: () => commands.platform.listCalendarEvents(orgId, cycleId),
    enabled: !!orgId && !!cycleId,
    staleTime: 5 * 60 * 1000,
  });
}

export function createPendingCalendarEventsQuery(orgId: string) {
  return createQuery({
    queryKey: ["calendar-pending", orgId],
    queryFn: () => commands.platform.listPendingCalendarEvents(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}

export function createOrgPolicyQuery(orgId: string) {
  return createQuery({
    queryKey: ["org-policy", orgId],
    queryFn: () => commands.platform.getOrgPolicy(orgId),
    enabled: !!orgId,
    staleTime: 30 * 60 * 1000,
  });
}
