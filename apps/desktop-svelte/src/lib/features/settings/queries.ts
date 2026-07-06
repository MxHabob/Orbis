/**
 * Settings domain queries and mutations.
 * Wraps IPC commands with TanStack Query for automatic caching and state updates.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Get the organization details. */
export function createOrganizationQuery(orgId: string) {
  return createQuery({
    queryKey: ["organization", orgId],
    queryFn: () => commands.users.getOrganization(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000, // settings don't change very often — 5 min
  });
}

/** Update organization details mutation. */
export function createUpdateOrganizationMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: { orgId: string; request: any }) =>
      commands.users.updateOrganization(variables.orgId, variables.request),
    onSuccess: (_, variables) => {
      // Invalidate organization query to trigger updates in UI
      queryClient.invalidateQueries({
        queryKey: ["organization", variables.orgId],
      });
    },
  });
}

/** Get all users for an organization. */
export function createUsersQuery(orgId: string) {
  return createQuery({
    queryKey: ["users", orgId],
    queryFn: () => commands.users.getUsers(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}
