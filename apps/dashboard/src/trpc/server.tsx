import "server-only";

import type { AppRouter } from "@orbis/api/trpc/routers/_app";
import { getCountryCode, getLocale, getTimezone } from "@orbis/location";
import { dehydrate, HydrationBoundary } from "@tanstack/react-query";
import { createTRPCClient, httpBatchLink, loggerLink } from "@trpc/client";
import {
  createTRPCOptionsProxy,
  type TRPCQueryOptions,
} from "@trpc/tanstack-react-query";
import { cookies } from "next/headers";
import { cache } from "react";
import superjson from "superjson";
import { Cookies } from "@/utils/constants";
import { makeQueryClient } from "./query-client";

// IMPORTANT: Create a stable getter for the query client that
//            will return the same client during the same request.
export const getQueryClient = cache(makeQueryClient);

// Get API URL with fallback for development
function getApiUrl(): string {
  // Use environment variable if set
  if (process.env.NEXT_PUBLIC_API_URL) {
    const url = process.env.NEXT_PUBLIC_API_URL;
    if (process.env.NODE_ENV === "development") {
      console.log(`[tRPC] Using API URL from env: ${url}`);
    }
    return url;
  }

  // Default to localhost:3002 for development (API server port from package.json)
  // API server runs on port 3002 in development mode
  if (process.env.NODE_ENV === "development" || !process.env.NODE_ENV) {
    const url = "http://localhost:3002";
    console.log(`[tRPC] Using default API URL: ${url}`);
    return url;
  }

  // Production fallback
  return "https://api.orbis.ai";
}

export const trpc = createTRPCOptionsProxy<AppRouter>({
  queryClient: getQueryClient,
  client: createTRPCClient({
    links: [
      httpBatchLink({
        url: `${getApiUrl()}/trpc`,
        transformer: superjson,
        async fetch(url, options) {
          const headers = new Headers(options?.headers);

          // Add custom headers
          headers.set("x-user-timezone", await getTimezone());
          headers.set("x-user-locale", await getLocale());
          headers.set("x-user-country", await getCountryCode());

          // Forward cookies to the API for authentication
          try {
            const cookieStore = await cookies();
            const allCookies = cookieStore.getAll();
            if (allCookies.length > 0) {
              const cookieHeader = allCookies
                .map((cookie) => `${cookie.name}=${cookie.value}`)
                .join("; ");
              headers.set("cookie", cookieHeader);
            }

            // Pass force-primary cookie as header to API for replication lag handling
            const forcePrimary = cookieStore.get(Cookies.ForcePrimary);
            if (forcePrimary?.value === "true") {
              headers.set("x-force-primary", "true");
            }
          } catch (error) {
            // Ignore cookie errors
          }

          try {
            return await fetch(url, {
              ...options,
              headers,
            });
          } catch (error) {
            // Log error for debugging in development
            if (process.env.NODE_ENV === "development") {
              console.error(
                `[tRPC Server] Failed to connect to API at ${url}:`,
                error,
              );
            }
            throw error;
          }
        },
      }),
      loggerLink({
        enabled: (opts) =>
          process.env.NODE_ENV === "development" ||
          (opts.direction === "down" && opts.result instanceof Error),
      }),
    ],
  }),
});

export function HydrateClient(props: { children: React.ReactNode }) {
  const queryClient = getQueryClient();

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      {props.children}
    </HydrationBoundary>
  );
}

export function prefetch<T extends ReturnType<TRPCQueryOptions<any>>>(
  queryOptions: T,
) {
  const queryClient = getQueryClient();

  if (queryOptions.queryKey[1]?.type === "infinite") {
    void queryClient.prefetchInfiniteQuery(queryOptions as any);
  } else {
    void queryClient.prefetchQuery(queryOptions);
  }
}

export function batchPrefetch<T extends ReturnType<TRPCQueryOptions<any>>>(
  queryOptionsArray: T[],
) {
  const queryClient = getQueryClient();

  for (const queryOptions of queryOptionsArray) {
    if (queryOptions.queryKey[1]?.type === "infinite") {
      void queryClient.prefetchInfiniteQuery(queryOptions as any);
    } else {
      void queryClient.prefetchQuery(queryOptions);
    }
  }
}

/**
 * Get a tRPC client for server-side API routes
 * Use this when you need to call mutations from API routes (e.g., webhooks, callbacks)
 * For queries, use the `trpc` proxy with `queryOptions` instead
 */
export async function getTRPCClient() {
  return createTRPCClient<AppRouter>({
    links: [
      httpBatchLink({
        url: `${getApiUrl()}/trpc`,
        transformer: superjson,
        async fetch(url, options) {
          const headers = new Headers(options?.headers);

          // Add custom headers
          headers.set("x-user-timezone", await getTimezone());
          headers.set("x-user-locale", await getLocale());
          headers.set("x-user-country", await getCountryCode());

          // Forward cookies to the API for authentication
          try {
            const cookieStore = await cookies();
            const allCookies = cookieStore.getAll();
            if (allCookies.length > 0) {
              const cookieHeader = allCookies
                .map((cookie) => `${cookie.name}=${cookie.value}`)
                .join("; ");
              headers.set("cookie", cookieHeader);
            }

            // Pass force-primary cookie as header to API for replication lag handling
            const forcePrimary = cookieStore.get(Cookies.ForcePrimary);
            if (forcePrimary?.value === "true") {
              headers.set("x-force-primary", "true");
            }
          } catch (error) {
            // Ignore cookie errors
          }

          try {
            return await fetch(url, {
              ...options,
              headers,
            });
          } catch (error) {
            // Log error for debugging in development
            if (process.env.NODE_ENV === "development") {
              console.error(
                `[tRPC Client] Failed to connect to API at ${url}:`,
                error,
              );
            }
            throw error;
          }
        },
      }),
    ],
  });
}
