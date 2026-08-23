/**
 * Students domain queries — enrollments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery, createInfiniteQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All enrollments for an organization. */
export function createEnrollmentsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["enrollments", orgId()],
    queryFn: () => commands.academic.listEnrollments(),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000, // Student data changes more frequently — 2 min
  }));
}

/** Paginated enrollments query using Cursor-based infinite scrolling. */
export function createEnrollmentsPaginatedQuery(
  orgId: () => string,
  limit: number = 50,
) {
  return createInfiniteQuery(() => ({
    queryKey: ["enrollments_paginated", orgId(), limit],
    queryFn: async ({ pageParam }) => {
      return commands.academic.listEnrollmentsPaginated(pageParam as string | null, limit);
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: any) => {
      // If there's a next_cursor and has_more is true, return next_cursor
      if (lastPage?.has_more && lastPage?.next_cursor) {
        return lastPage.next_cursor;
      }
      return undefined;
    },
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}
