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
    queryFn: () => commands.academic.listEnriched(),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000, // Student data changes more frequently — 2 min
  }));
}

/** Single enriched enrollment for detail pages. */
export function createEnrollmentQuery(
  orgId: () => string,
  enrollmentId: () => string,
) {
  return createQuery(() => ({
    queryKey: ["enrollment", orgId(), enrollmentId()],
    queryFn: () => commands.academic.getEnriched(enrollmentId()),
    enabled: !!orgId() && !!enrollmentId(),
    staleTime: 2 * 60 * 1000,
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
      return commands.academic.listPaginatedEnriched(pageParam as string | null, limit);
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
