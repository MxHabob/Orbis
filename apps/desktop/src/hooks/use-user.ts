import {
  useMutation,
  useQueryClient,
  useSuspenseQuery,
} from "@tanstack/react-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { getAccessToken, getUserId } from "@/lib/session";
import type { JsonRecord } from "@orbis/desktop-client/types";

export function useUserQuery() {
  const token = getAccessToken();
  return useSuspenseQuery({
    queryKey: queryKeys.user.me(),
    queryFn: async () => {
      if (!token) return null;
      return commands.auth.validateToken(token);
    },
  });
}

export function useUserMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (newData: JsonRecord) => {
      const userId = String(newData.id ?? getUserId());
      return commands.users.updateUser(userId, newData);
    },
    onMutate: async (newData) => {
      await queryClient.cancelQueries({ queryKey: queryKeys.user.me() });
      const previousData = queryClient.getQueryData(queryKeys.user.me());
      queryClient.setQueryData(queryKeys.user.me(), (old: unknown) => ({
        ...(old as JsonRecord),
        ...newData,
      }));
      return { previousData };
    },
    onError: (_, __, context) => {
      queryClient.setQueryData(
        queryKeys.user.me(),
        context?.previousData,
      );
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.user.me() });
    },
  });
}
