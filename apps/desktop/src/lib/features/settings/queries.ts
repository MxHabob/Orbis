/**
 * Settings domain queries and mutations.
 * Wraps OrbisApiClient with TanStack Query for automatic caching and state updates.
 */
import {
  createQuery,
  createInfiniteQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys, type OrgDto, type UserDto } from "$lib/api";

/** Get the organization details. */
export function createOrganizationQuery(orgId: () => string) {
  return createQuery<OrgDto>(() => ({
    queryKey: queryKeys.org.current(),
    queryFn: async (): Promise<OrgDto> => {
      return api.getCurrentOrg();
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
      api.updateCurrentOrg(variables.request),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.all,
      });
    },
  }));
}

/** Get all users for an organization. */
export function createUsersQuery(orgId: () => string) {
  return createQuery<UserDto[]>(() => ({
    queryKey: queryKeys.auth.users(),
    queryFn: () => api.listUsers(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Single user query shared by detail layouts and their tabs. */
export function createUserQuery(orgId: () => string, userId: () => string) {
  return createQuery<UserDto>(() => ({
    queryKey: queryKeys.auth.user(userId()),
    queryFn: () => api.getUser(userId()),
    enabled: !!orgId() && !!userId(),
    staleTime: 5 * 60 * 1000,
  }));
}

import type { UniversalFilter } from "@orbis/ui/filter-builder";
import type { OrgUnitDto, CreateOrgUnitRequest, UpdateOrgUnitRequest, OrganizationSettingsDto, UpdateOrgRequest } from "$lib/api";

/** Paginated users query using infinite scrolling. */
export function createUsersPaginatedQuery(
  orgId: () => string,
  limit: number = 50,
  filter?: () => UniversalFilter | undefined,
) {
  return createInfiniteQuery(() => ({
    queryKey: ["users_paginated", orgId(), limit, filter?.()],
    queryFn: async () => {
      const items = await api.listUsers();
      return {
        items,
        count: items.length,
        has_more: false,
        next_cursor: null,
      };
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

/** Get all organizational units / branches for an organization. */
export function createOrgUnitsQuery(orgId: () => string) {
  return createQuery<OrgUnitDto[]>(() => ({
    queryKey: queryKeys.org.units(orgId()),
    queryFn: () => api.listOrgUnits(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Get a single organizational unit. */
export function createOrgUnitQuery(orgId: () => string, unitId: () => string) {
  return createQuery<OrgUnitDto>(() => ({
    queryKey: queryKeys.org.unit(unitId()),
    queryFn: () => api.getOrgUnit(unitId()),
    enabled: !!orgId() && !!unitId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Create org unit mutation. */
export function createCreateOrgUnitMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateOrgUnitRequest) => api.createOrgUnit(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.all,
      });
    },
  }));
}

/** Update org unit mutation. */
export function createUpdateOrgUnitMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { id: string; req: UpdateOrgUnitRequest }) =>
      api.updateOrgUnit(variables.id, variables.req),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.all,
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.unit(variables.id),
      });
    },
  }));
}

/** Delete org unit mutation. */
export function createDeleteOrgUnitMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteOrgUnit(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.all,
      });
    },
  }));
}

/** Get organization settings. */
export function createOrgSettingsQuery(orgId: () => string) {
  return createQuery<OrganizationSettingsDto>(() => ({
    queryKey: queryKeys.org.settings(orgId()),
    queryFn: () => api.getSettings(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Update organization settings mutation. */
export function createUpdateOrgSettingsMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (settings: OrganizationSettingsDto) => api.updateSettings(settings),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.all,
      });
    },
  }));
}
