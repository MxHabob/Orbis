/**
 * Students domain queries — enrollments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery, createInfiniteQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All enrollments for an organization. */
export function createEnrollmentsQuery(orgId: () => string) {
  return createQuery({
    get queryKey() { return ["enrollments", orgId()]; },
    queryFn: () => commands.academic.listEnrollments(),
    get enabled() { return !!orgId(); },
    staleTime: 2 * 60 * 1000, // Student data changes more frequently — 2 min
  });
}

/** Paginated enrollments query using Cursor-based infinite scrolling. */
export function createEnrollmentsPaginatedQuery(orgId: () => string, limit: number = 50) {
  return createInfiniteQuery({
    get queryKey() { return ["enrollments_paginated", orgId(), limit]; },
    queryFn: async ({ pageParam }) => {
      return commands.academic.listEnrollmentsPaginated(pageParam, limit);
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage) => {
      // If there's a next_cursor and has_more is true, return next_cursor
      if (lastPage.has_more && lastPage.next_cursor) {
        return lastPage.next_cursor;
      }
      return undefined;
    },
    get enabled() { return !!orgId(); },
    staleTime: 2 * 60 * 1000,
  });
}
