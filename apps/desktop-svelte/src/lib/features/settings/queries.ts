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
    queryFn: () => commands.organization.getOrganization(orgId),
    enabled: !!orgId,
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
export function createUsersQuery(orgId: string) {
  return createQuery({
    queryKey: ["users", orgId],
    queryFn: () => commands.iam.listUsers(),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}
