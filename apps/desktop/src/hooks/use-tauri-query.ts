import { useQuery, useMutation, useQueryClient, UseQueryOptions, UseMutationOptions } from "@tanstack/react-query";

/**
 * Custom hook wrapper for executing Tauri IPC commands with TanStack Query.
 */
export function useTauriQuery<TQueryFnData = unknown, TError = unknown, TData = TQueryFnData>(
  queryKey: readonly unknown[],
  queryFn: () => Promise<TQueryFnData>,
  options?: Omit<UseQueryOptions<TQueryFnData, TError, TData>, "queryKey" | "queryFn">
) {
  return useQuery<TQueryFnData, TError, TData>({
    queryKey,
    queryFn,
    ...options,
  });
}

/**
 * Custom hook wrapper for executing Tauri IPC mutations with TanStack Query.
 */
export function useTauriMutation<TData = unknown, TError = unknown, TVariables = void, TContext = unknown>(
  mutationFn: (variables: TVariables) => Promise<TData>,
  options?: UseMutationOptions<TData, TError, TVariables, TContext>
) {
  const queryClient = useQueryClient();
  return useMutation<TData, TError, TVariables, TContext>({
    mutationFn,
    ...options,
  });
}
