/**
 * Students domain queries — enrollments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All enrollments for an organization. */
export function createEnrollmentsQuery(orgId: () => string) {
  return createQuery({
    get queryKey() { return ["enrollments", orgId()]; },
    queryFn: () => commands.academic.listEnrollments(orgId()),
    get enabled() { return !!orgId(); },
    staleTime: 2 * 60 * 1000, // Student data changes more frequently — 2 min
  });
}
