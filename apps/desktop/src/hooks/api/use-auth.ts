import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import {
  clearSession,
  getAccessToken,
  getOrgId,
  getUserId,
  setSession,
} from "@/lib/session";
import type { User } from "@orbis/desktop-client/types";

export function useSession() {
  const token = getAccessToken();

  return useQuery({
    queryKey: queryKeys.auth.session(),
    queryFn: async () => {
      if (!token) return null;
      const user = await commands.auth.validateToken(token);
      localStorage.setItem("session_user", JSON.stringify(user));
      return { user };
    },
    staleTime: 60_000,
    retry: false,
  });
}

export function useLogin() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (params: { email: string; password: string }) => {
      const response = await commands.auth.login(params.email, params.password);
      setSession(response.user as Record<string, unknown>, response.access_token);
      return response;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.auth.all() });
    },
  });
}

export function useLogout() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async () => {
      const token = getAccessToken();
      if (token) {
        try {
          await commands.auth.logout(token);
        } catch {
          /* ignore */
        }
      }
      clearSession();
    },
    onSuccess: () => {
      queryClient.clear();
    },
  });
}

export function useCurrentUser() {
  return useQuery({
    queryKey: queryKeys.user.me(),
    queryFn: async () => {
      const token = getAccessToken();
      if (!token) return null;
      return commands.auth.validateToken(token);
    },
  });
}

export function useUpdateUser() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: JsonRecord & { id?: string }) =>
      commands.users.updateUser(String(data.id ?? getUserId()), data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.user.me() });
    },
  });
}

type JsonRecord = Record<string, unknown>;

export function useOrganization(orgId?: string) {
  const id = orgId ?? getOrgId();
  return useQuery({
    queryKey: queryKeys.organization.detail(id),
    queryFn: () => commands.users.getOrganization(id),
    enabled: !!id,
  });
}

export function useUsers(params?: JsonRecord) {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.user.list(orgId, params),
    queryFn: () => commands.users.getUsers(orgId, params),
    enabled: !!orgId,
  });
}

export type { User };
