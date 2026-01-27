"use client";

import { useTRPC } from "@/trpc/client";
import {
  useMutation,
  useQueryClient,
  useSuspenseQuery,
} from "@tanstack/react-query";

export function useUsersQuery() {
  const trpc = useTRPC();
  return useSuspenseQuery(trpc.user.get.queryOptions());
}

export function useUsersMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.user.update.mutationOptions({
      onMutate: async (newData) => {
        // Cancel outgoing refetches
        await queryClient.cancelQueries({
          queryKey: trpc.user.get.queryKey(),
        });

        // Get current data
        const previousData = queryClient.getQueryData(
          trpc.user.get.queryKey(),
        );

        // Optimistically update
        queryClient.setQueryData(trpc.user.get.queryKey(), (old: any) => ({
          ...old,
          ...newData,
        }));

        return { previousData };
      },
      onError: (_, __, context) => {
        // Rollback on error
        queryClient.setQueryData(
          trpc.user.get.queryKey(),
          context?.previousData,
        );
      },
      onSettled: () => {
        // Refetch after error or success
        queryClient.invalidateQueries({
          queryKey: trpc.user.get.queryKey(),
        });
      },
    }),
  );
}
