import { Suspense } from "react";
import { useSearchParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { ErrorBoundary } from "@/components/error-boundary";
import { ErrorFallback } from "@/components/error-fallback";
import { ReportsHeader } from "@/components/reports-header";
import { ReportsHeaderSkeleton } from "@/components/reports-header-skeleton";
import { ScrollableContent } from "@/components/scrollable-content";
import { DataTable } from "@/components/tables/reports/data-table";
import { ReportsSkeleton } from "@/components/tables/reports/skeleton";
import { loadClassesFilterParams } from "@/hooks/use-classes-filter-params";
import { loadSortParams } from "@/hooks/use-sort-params";
import { getInitialTableSettings } from "@/utils/columns";
import { Spinner } from "@orbis/ui/spinner";

export default function ReportsPage() {
  const [searchParams] = useSearchParams();
  
  const searchParamsRecord: Record<string, string> = {};
  searchParams.forEach((value, key) => {
    searchParamsRecord[key] = value;
  });

  const filter = loadClassesFilterParams(searchParamsRecord);
  const { sort } = loadSortParams(searchParamsRecord);

  const { data: initialSettings, isLoading } = useQuery({
    queryKey: ["reports-page-settings"],
    queryFn: () => getInitialTableSettings("reports"),
  });

  if (isLoading || !initialSettings) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <Spinner />
      </div>
    );
  }

  return (
    <ScrollableContent>
      <div className="flex flex-col gap-6">
        <ErrorBoundary fallback={<ErrorFallback />}>
          <Suspense
            fallback={
              <div className="flex flex-col gap-6">
                <ReportsHeaderSkeleton />
                <ReportsSkeleton />
              </div>
            }
          >
            <ReportsHeader />
            <DataTable initialSettings={initialSettings} />
          </Suspense>
        </ErrorBoundary>
      </div>
    </ScrollableContent>
  );
}
