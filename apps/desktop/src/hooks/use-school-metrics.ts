import { useQuery } from "@tanstack/react-query";
import { useTRPC } from "@/trpc/client";
import { useMetricsFilter } from "./use-metrics-filter";

export function useSchoolMetrics() {
  const trpc = useTRPC();
  const { from, to, currency } = useMetricsFilter();

  return useQuery(
    trpc.reports.getSchoolMetrics.queryOptions({
      from,
      to,
      currency,
    }),
  );
}
