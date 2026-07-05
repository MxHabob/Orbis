/**
 * Academic Calendar queries — TanStack Query wrappers for calendar IPC.
 *
 * Provides cached, deduplicated access to calendar events and policies.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All calendar events for a specific cycle. */
export function createCalendarEventsQuery(orgId: string, cycleId: string) {
  return createQuery({
    queryKey: ["calendar-events", orgId, cycleId],
    queryFn: () => commands.calendar.getEvents(orgId, cycleId),
    enabled: !!orgId && !!cycleId,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

/** Pending calendar events that need attention. */
export function createPendingCalendarEventsQuery(orgId: string) {
  return createQuery({
    queryKey: ["calendar-pending", orgId],
    queryFn: () => commands.calendar.getPending(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000, // 2 minutes — pending events change more often
  });
}

/** Organization academic policy (execution modes, enrollment rules, etc.). */
export function createOrgPolicyQuery(orgId: string) {
  return createQuery({
    queryKey: ["org-policy", orgId],
    queryFn: () => commands.calendar.getPolicy(orgId),
    enabled: !!orgId,
    staleTime: 30 * 60 * 1000, // 30 minutes — policies rarely change
  });
}
