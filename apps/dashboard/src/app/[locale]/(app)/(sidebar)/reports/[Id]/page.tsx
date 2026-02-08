import { AverageGrade } from "@/components/average-grade";
import { CollapsibleSummary } from "@/components/collapsible-summary";
import { CustomerSummarySkeleton } from "@/components/customer-summary-skeleton";
import { StudentsHeader } from "@/components/customers-header";
import { ErrorFallback } from "@/components/error-fallback";
import { FailedStudents } from "@/components/failed-students";
import { PassedStudents } from "@/components/passed-students";
import { ScrollableContent } from "@/components/scrollable-content";
import { DataTable } from "@/components/tables/students/data-table";
import { CustomersSkeleton } from "@/components/tables/students/skeleton";
import { TotalStudents } from "@/components/total-students";
import { loadCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { loadSortParams } from "@/hooks/use-sort-params";
import { HydrateClient, batchPrefetch, trpc } from "@/trpc/server";
import { getInitialTableSettings } from "@/utils/columns";
import type { Metadata } from "next";
import { ErrorBoundary } from "next/dist/client/components/error-boundary";
import type { SearchParams } from "nuqs";
import { Suspense } from "react";

export const metadata: Metadata = {
  title: "Class Report | Midday",
};

type Props = {
  params: Promise<{ Id: string }>;
  searchParams: Promise<SearchParams>;
};

export default async function Page(props: Props) {
  const { Id } = await props.params;
  const searchParams = await props.searchParams;

  const filter = loadCustomerFilterParams(searchParams);
  const { sort } = loadSortParams(searchParams);

  const initialSettings = await getInitialTableSettings("students");

  if (!Id) return null;

  const infiniteOptions = trpc.students.getByClassId.infiniteQueryOptions?.({
    id: Id,
    q: filter.q ?? undefined,
    sort: sort ?? undefined,
  });

  // Prefetch class statistics
  batchPrefetch([
    infiniteOptions,
    trpc.grades.getClassStatistics.queryOptions({ classId: Id }),
  ]);

  return (
    <HydrateClient>
      <ScrollableContent>
        <div className="flex flex-col gap-6" data-canvas-content>
          <CollapsibleSummary>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 pt-6">
              <Suspense fallback={<CustomerSummarySkeleton />}>
                <TotalStudents />
              </Suspense>
              <Suspense fallback={<CustomerSummarySkeleton />}>
                <PassedStudents />
              </Suspense>
              <Suspense fallback={<CustomerSummarySkeleton />}>
                <FailedStudents />
              </Suspense>
              <Suspense fallback={<CustomerSummarySkeleton />}>
                <AverageGrade />
              </Suspense>
            </div>
          </CollapsibleSummary>

          <StudentsHeader id={Id} />

          <ErrorBoundary errorComponent={ErrorFallback}>
            <Suspense fallback={<CustomersSkeleton />}>
              <DataTable initialSettings={initialSettings} id={Id} />
            </Suspense>
          </ErrorBoundary>
        </div>
      </ScrollableContent>
    </HydrateClient>
  );
}