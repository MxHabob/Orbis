import { Suspense } from "react";
import { useParams, useSearchParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { AverageGrade } from "@/components/average-grade";
import { CollapsibleSummary } from "@/components/collapsible-summary";
import { ErrorBoundary } from "@/components/error-boundary";
import { ErrorFallback } from "@/components/error-fallback";
import { FailedStudents } from "@/components/failed-students";
import { PassedStudents } from "@/components/passed-students";
import { ScrollableContent } from "@/components/scrollable-content";
import { StudentHeader } from "@/components/student-header";
import { StudentHeaderSkeleton } from "@/components/student-header-skeleton";
import { StudentSummarySkeleton } from "@/components/student-summary-skeleton";
import { DataTable } from "@/components/tables/students/data-table";
import { StudentsSkeleton } from "@/components/tables/students/skeleton";
import { TotalStudents } from "@/components/total-students";
import { loadCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { loadSortParams } from "@/hooks/use-sort-params";
import { getInitialTableSettings } from "@/utils/columns";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { useTauriQuery } from "@/hooks/use-tauri-query";
import { Spinner } from "@orbis/ui/spinner";

function StudentHeaderContainer({ id }: { id: string }) {
  const { data: classData, isLoading } = useTauriQuery(
    queryKeys.classes.detail(id),
    async () => {
      const classes = await commands.classes.getAll("");
      return classes.find((c: any) => String(c.id) === id) || null;
    }
  );

  if (isLoading) return <StudentHeaderSkeleton />;

  return (
    <StudentHeader
      name={(classData as any)?.name ?? "Class Report"}
      color={(classData as any)?.color}
      id={id}
    />
  );
}

export default function ReportDetailPage() {
  const { id } = useParams<{ id: string }>();
  const [searchParams] = useSearchParams();

  const searchParamsRecord: Record<string, string> = {};
  searchParams.forEach((value, key) => {
    searchParamsRecord[key] = value;
  });

  const filter = loadCustomerFilterParams(searchParamsRecord);
  const { sort } = loadSortParams(searchParamsRecord);

  const { data: initialSettings, isLoading } = useQuery({
    queryKey: ["students-table-settings"],
    queryFn: () => getInitialTableSettings("students"),
  });

  if (!id) return null;

  if (isLoading || !initialSettings) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <Spinner />
      </div>
    );
  }

  return (
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
          <StudentHeaderContainer id={id} />
        </Suspense>

        <ErrorBoundary fallback={<ErrorFallback />}>
          <Suspense fallback={<StudentsSkeleton />}>
            <DataTable initialSettings={initialSettings} id={id} />
          </Suspense>
        </ErrorBoundary>
      </div>
    </ScrollableContent>
  );
}
