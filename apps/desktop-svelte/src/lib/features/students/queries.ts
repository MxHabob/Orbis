/**
 * Students domain queries — enrollments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All enrollments for an organization. */
export function createEnrollmentsQuery(orgId: string) {
  return createQuery({
    queryKey: ["enrollments", orgId],
    queryFn: () => commands.enrollments.getAll(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000, // Student data changes more frequently — 2 min
  });
}
