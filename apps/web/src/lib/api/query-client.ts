/**
 * TanStack Query Client — centralized data caching for Orbis API calls.
 *
 * Provides automatic caching, deduplication, and background refetching.
 * Config is tuned for desktop performance:
 * - No refetch on window focus (desktop windows don't lose/gain focus like browser tabs)
 * - Longer stale times (local data changes predictably)
 * - Always-on network mode
 */

import { QueryClient } from "@tanstack/svelte-query";

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 2 * 60 * 1000, // 2 minutes (realtime SSE invalidates immediately on remote mutations)
      gcTime: 10 * 60 * 1000, // 10 minutes bounded retention for low-RAM workstations
      refetchOnWindowFocus: false,
      refetchOnReconnect: true,
      retry: 1,
      networkMode: "always",
    },
    mutations: {
      networkMode: "always",
    },
  },
});
