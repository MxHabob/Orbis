import { AverageGrade } from "@/components/average-grade";
import { CollapsibleSummary } from "@/components/collapsible-summary";
import { StudentSummarySkeleton } from "@/components/student-summary-skeleton";
import { StudentHeader } from "@/components/student-header";
import { StudentHeaderSkeleton } from "@/components/student-header-skeleton";
import { ErrorFallback } from "@/components/error-fallback";
import { ErrorBoundary } from "@/components/error-boundary";
import { FailedStudents } from "@/components/failed-students";
import { PassedStudents } from "@/components/passed-students";
import { ScrollableContent } from "@/components/scrollable-content";
import { DataTable } from "@/components/tables/students/data-table";
import { StudentsSkeleton } from "@/components/tables/students/skeleton";
import { TotalStudents } from "@/components/total-students";
import { loadCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { loadSortParams } from "@/hooks/use-sort-params";
import { HydrateClient, batchPrefetch, trpc, getQueryClient } from "@/trpc/server";
import { getInitialTableSettings } from "@/utils/columns";
import type { Metadata } from "next";
import type { SearchParams } from "nuqs";
import { Suspense } from "react";

export const metadata: Metadata = {
  title: "Class Report | Orbis",
};

type Props = {
  params: Promise<{ Id: string }>;
  searchParams: Promise<SearchParams>;
};

// Async component for fetching class data without blocking the full page rendering
async function StudentHeaderContainer({ id }: { id: string }) {
  const classData = await getQueryClient().fetchQuery(
    trpc.classes.getById.queryOptions({ id: Number(id) })
  );

  return (
    <StudentHeader 
      name={classData?.name ?? "Class Report"} 
      color={classData?.color}
      id={id}
    />
  );
}

export default async function Page(props: Props) {
  const { Id } = await props.params;
  const searchParams = await props.searchParams;

  const filter = loadCustomerFilterParams(searchParams);
  const { sort } = loadSortParams(searchParams);

  const initialSettings = await getInitialTableSettings("students");

  if (!Id) return null;

  const academicYearId = filter.academicYearId ? Number(filter.academicYearId) : undefined;

  const infiniteOptions = trpc.students.getByClassId.infiniteQueryOptions({
    id: Id,
    q: filter.q ?? undefined,
    sort: sort ?? undefined,
    academicYearId,
  });

  const statisticsOptions = trpc.grades.getClassStatistics.queryOptions({ 
    classId: Id,
    academicYearId
  });

  // Prefetch class statistics, student list, and class name
  batchPrefetch([
    infiniteOptions, 
    statisticsOptions,
    trpc.classes.getById.queryOptions({ id: Number(Id) })
  ].filter(Boolean) as any);

  return (
    <HydrateClient>
      <ScrollableContent>
        <div className="flex flex-col gap-6" data-canvas-content>
          <CollapsibleSummary>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
              <Suspense fallback={<StudentSummarySkeleton />}>
                <TotalStudents />
              </Suspense>
              <Suspense fallback={<StudentSummarySkeleton />}>
                <PassedStudents />
              </Suspense>
              <Suspense fallback={<StudentSummarySkeleton />}>
                <FailedStudents />
              </Suspense>
              <Suspense fallback={<StudentSummarySkeleton />}>
                <AverageGrade />
              </Suspense>
            </div>
          </CollapsibleSummary>

          <Suspense fallback={<StudentHeaderSkeleton />}>
            <StudentHeaderContainer id={Id} />
          </Suspense>
            
          <ErrorBoundary fallback={<ErrorFallback />}>
            <Suspense fallback={<StudentsSkeleton />}>
              <DataTable initialSettings={initialSettings} id={Id} />
            </Suspense>
          </ErrorBoundary>
        </div>
      </ScrollableContent>
    </HydrateClient>
  );
}
