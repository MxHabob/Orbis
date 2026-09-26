/**
 * User Groups domain queries & mutations — RBAC & User Groups Management.
 * Wraps OrbisApiClient with TanStack Query and queryKeys factory.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type CreateGroupRequest,
  type UpdateGroupRequest,
  type BulkAssignGroupRequest,
} from "$lib/api";

/** List all user groups for an organization. */
export function createGroupsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.group.list(orgId()),
    queryFn: () => api.listGroups(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

/** Single group detail query (GroupDetail). */
export function createGroupQuery(groupId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.group.detail(groupId()),
    queryFn: () => api.getGroup(groupId()),
    enabled: !!groupId(),
    staleTime: 60 * 1000,
  }));
}

/** Get members of a specific user group. */
export function createGroupMembersQuery(groupId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.group.members(groupId()),
    queryFn: () => api.getGroupMembers(groupId()),
    enabled: !!groupId(),
    staleTime: 30 * 1000,
  }));
}

/** Create new user group mutation. */
export function createCreateGroupMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateGroupRequest) => api.createGroup(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

/** Update user group mutation. */
export function createUpdateGroupMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateGroupRequest }) =>
      api.updateGroup(id, req),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.group.detail(variables.id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

/** Delete user group mutation. */
export function createDeleteGroupMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteGroup(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

/** Add member to user group mutation. */
export function createAddGroupMemberMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ groupId, userId }: { groupId: string; userId: string }) =>
      api.addGroupMember(groupId, userId),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.group.members(variables.groupId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.detail(variables.groupId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

/** Remove member from user group mutation. */
export function createRemoveGroupMemberMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ groupId, userId }: { groupId: string; userId: string }) =>
      api.removeGroupMember(groupId, userId),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.group.members(variables.groupId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.detail(variables.groupId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

/** Bulk assign users to user group mutation. */
export function createBulkAssignGroupMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: BulkAssignGroupRequest) => api.bulkAssignGroup(req),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.group.members(variables.group_id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.detail(variables.group_id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

/** Set permissions for user group mutation. */
export function createSetGroupPermissionsMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, permissions }: { id: string; permissions: string[] }) =>
      api.setGroupPermissions(id, permissions),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.group.detail(variables.id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}
