"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";
import { useMetricsFilter } from "@/hooks/use-metrics-filter";
import { useTRPC } from "@/trpc/client";

/**
 * Prefetches metrics data in the background after initial page load.
 * Delays to avoid batching with initial tRPC requests.
 */
export function usePrefetchMetrics() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { from, to, currency } = useMetricsFilter();
  const hasPrefetched = useRef(false);

  useEffect(() => {
    if (hasPrefetched.current) return;

    const prefetchMetrics = () => {
      hasPrefetched.current = true;

      // Prefetch the new school metrics
      queryClient.prefetchQuery(
        trpc.reports.getSchoolMetrics.queryOptions({
          from,
          to,
          currency,
        }),
      );
    };

    // Wait 1s to avoid batching with initial requests
    const timeoutId = setTimeout(prefetchMetrics, 1000);

    return () => clearTimeout(timeoutId);
  }, [queryClient, trpc, from, to, currency]);
}
