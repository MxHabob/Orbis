/**
 * TanStack Query Client — centralized data caching for Tauri IPC calls.
 *
 * This replaces manual data fetching in stores with automatic caching,
 * deduplication, and background refetching.
 *
 * Config is tuned for Tauri desktop apps:
 * - No refetch on window focus (Tauri windows don't lose/gain focus like browser tabs)
 * - Longer stale times (local SQLite data changes less frequently)
 * - Always-on network mode (IPC is always available)
 */
import { QueryClient } from "@tanstack/svelte-query";

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes — data considered fresh
      gcTime: 30 * 60 * 1000, // 30 minutes — keep unused data in memory
      refetchOnWindowFocus: false, // Tauri — no need
      refetchOnReconnect: false, // IPC — always connected
      retry: 1, // Single retry for transient failures
      networkMode: "always", // Tauri IPC is always available
    },
    mutations: {
      networkMode: "always",
    },
  },
});
