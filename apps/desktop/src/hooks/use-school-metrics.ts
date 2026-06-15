import { useQuery } from "@tanstack/react-query";
import { useTRPC } from "@/ipc/query-adapter";
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
