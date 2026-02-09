
import { useQuery } from "@tanstack/react-query";
import { getSchoolMetricsAction } from "@/actions/get-school-metrics-action";

export function useSchoolMetrics() {
  return useQuery({
    queryKey: ["school-metrics"],
    queryFn: async () => {
      const result = await getSchoolMetricsAction();
      if (result?.data) {
        return result.data;
      }
      return null;
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
  });
}
