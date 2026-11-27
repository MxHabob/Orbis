import type { QueryClient } from "@tanstack/svelte-query";
import { queryKeys, type UserDto } from "$lib/api";

export function isUserActive(user: Partial<UserDto> | null | undefined): boolean {
  if (!user) return true;
  if (user.is_active !== undefined && user.is_active !== null) {
    return Boolean(user.is_active);
  }
  return user.status !== "inactive";
}

export async function invalidateUserQueries(
  queryClient: QueryClient,
  orgId: string,
  userId: string,
): Promise<void> {
  await Promise.all([
    queryClient.invalidateQueries({ queryKey: queryKeys.auth.user(userId) }),
    queryClient.invalidateQueries({ queryKey: queryKeys.auth.users() }),
    queryClient.invalidateQueries({ queryKey: ["users_paginated", orgId] }),
  ]);
}
