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
  return createQuery<Organization>(() => ({
    queryKey: ["organization", orgId()],
    queryFn: async (): Promise<Organization> => {
      const organization = await commands.organization.getOrganization();
      return organization as Organization;
    },
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Update organization details mutation. */
export function createUpdateOrganizationMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { orgId: string; request: any }) =>
      commands.organization.updateOrganization({
        ...variables.request,
        org_id: variables.orgId,
      }),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["organization", variables.orgId],
      });
    },
  }));
}

/** Get all users for an organization. */
export function createUsersQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["users", orgId()],
    queryFn: () => commands.iam.listUsers(null),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Single user query shared by detail layouts and their tabs. */
export function createUserQuery(orgId: () => string, userId: () => string) {
  return createQuery(() => ({
    queryKey: ["user", orgId(), userId()],
    queryFn: () => commands.iam.getUser(userId()),
    enabled: !!orgId() && !!userId(),
    staleTime: 5 * 60 * 1000,
  }));
}

import type { UniversalFilter } from "@orbis/ui/filter-builder";

/** Paginated users query using Cursor-based infinite scrolling. */
export function createUsersPaginatedQuery(
  orgId: () => string,
  limit: number = 50,
  filter?: () => UniversalFilter | undefined,
) {
  // @ts-ignore - auto imported
  return createInfiniteQuery(() => ({
    queryKey: ["users_paginated", orgId(), limit, filter?.()],
    queryFn: async ({ pageParam }) => {
      return commands.iam.listUsersPaginated(
        { cursor: pageParam, limit },
        filter?.() ?? null,
      );
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: any) => {
      if (lastPage?.has_more && lastPage?.next_cursor) {
        return lastPage.next_cursor;
      }
      return undefined;
    },
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}
