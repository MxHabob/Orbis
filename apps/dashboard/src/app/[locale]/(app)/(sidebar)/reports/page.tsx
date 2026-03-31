import { ErrorFallback } from "@/components/error-fallback";
import { ReportsHeader } from "@/components/reports-header";
import { ReportsHeaderSkeleton } from "@/components/reports-header-skeleton";
import { ScrollableContent } from "@/components/scrollable-content";
import { DataTable } from "@/components/tables/reports/data-table";
import { ReportsSkeleton } from "@/components/tables/reports/skeleton";
import { loadClassesFilterParams } from "@/hooks/use-classes-filter-params";
import { loadSortParams } from "@/hooks/use-sort-params";
import { HydrateClient, batchPrefetch, trpc } from "@/trpc/server";
import { getInitialTableSettings } from "@/utils/columns";
import type { Metadata } from "next";
import { ErrorBoundary } from "next/dist/client/components/error-boundary";
import type { SearchParams } from "nuqs";
import { Suspense } from "react";

export const metadata: Metadata = {
  title: "Reports | Entity",
};

type Props = {
  searchParams: Promise<SearchParams>;
};

export default async function Reports(props: Props) {
  const searchParams = await props.searchParams;

  const filter = loadClassesFilterParams(searchParams);
  const { sort } = loadSortParams(searchParams);

  // Get unified table settings from cookie
  const initialSettings = await getInitialTableSettings("reports");

  batchPrefetch([
    trpc.classes.getAll.infiniteQueryOptions({
      ...filter,
      academicYearId: filter.academicYearId ? Number(filter.academicYearId) : undefined,
      sort,
    }),
  ]);

  return (
    <HydrateClient>
      <ScrollableContent>
        <div className="flex flex-col gap-6">
          <ErrorBoundary errorComponent={ErrorFallback}>
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
    </HydrateClient>
  );
}
