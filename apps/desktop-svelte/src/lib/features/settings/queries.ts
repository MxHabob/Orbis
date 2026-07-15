import * as m from "$lib/paraglide/messages";
/**
 * Settings domain queries and mutations.
 * Wraps IPC commands with TanStack Query for automatic caching and state updates.
 */
import {
  createQuery,
  createInfiniteQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import type { Organization } from "$lib/core/ipc";

/** Get the organization details. */
export function createOrganizationQuery(orgId: () => string) {
  return createQuery<Organization>({
    get queryKey() {
      return ["organization", orgId()];
    },
    queryFn: async () => await commands.organization.getOrganization(),
    get enabled() {
      return !!orgId();
    },
    staleTime: 5 * 60 * 1000,
  });
}

/** Update organization details mutation. */
export function createUpdateOrganizationMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: { orgId: string; request: any }) =>
      commands.organization.updateOrganization(
        variables.orgId,
        variables.request,
      ),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["organization", variables.orgId],
      });
    },
  });
}

/** Get all users for an organization. */
export function createUsersQuery(orgId: () => string) {
  return createQuery({
    get queryKey() {
      return ["users", orgId()];
    },
    queryFn: () => commands.iam.listUsers(),
    get enabled() {
      return !!orgId();
    },
    staleTime: 5 * 60 * 1000,
  });
}

/** Paginated users query using Cursor-based infinite scrolling. */
export function createUsersPaginatedQuery(
  orgId: () => string,
  limit: number = 50,
  filter?: { role?: string; status?: string },
) {
  // @ts-ignore - auto imported
  return createInfiniteQuery({
    get queryKey() {
      return ["users_paginated", orgId(), limit, filter];
    },
    queryFn: async ({ pageParam }) => {
      return commands.iam.listUsersPaginated(pageParam, limit, filter);
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage) => {
      if (lastPage.has_more && lastPage.next_cursor) {
        return lastPage.next_cursor;
      }
      return undefined;
    },
    get enabled() {
      return !!orgId();
    },
    staleTime: 5 * 60 * 1000,
  });
}
