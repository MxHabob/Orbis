import { Suspense } from "react";
import type { Metadata } from "next";
import { GradeGridHeader } from "@/components/grades/grade-grid-header";
import { GradeGridWrapper } from "@/components/grades/grade-grid-wrapper";
import { ScrollableContent } from "@/components/scrollable-content";
import { GradesSkeleton } from "@/components/tables/grades/skeleton";
import { getInitialTableSettings } from "@/utils/columns";
import { ErrorBoundary } from "next/dist/client/components/error-boundary";
import { ErrorFallback } from "@/components/error-fallback";
import { GradesUploadZone } from "@/components/grades/grades-upload-zone";
import { trpc, prefetch, HydrateClient, getQueryClient } from "@/trpc/server";
import { loadGradeFilterParams } from "@/hooks/use-grade-filter-params";
import type { SearchParams } from "nuqs";

export const metadata: Metadata = {
  title: "Grade | Entity",
};

type Props = {
  searchParams: Promise<SearchParams>;
};

export default async function GradesPage(props: Props) {
  const searchParams = await props.searchParams;
  const filter = loadGradeFilterParams(searchParams);
  const queryClient = getQueryClient();

  const [currentYear, years, subjects, initialSettings] = await Promise.all([
    queryClient.fetchQuery(trpc.academic.getCurrentYear.queryOptions()),
    queryClient.fetchQuery(trpc.academic.getYears.queryOptions()),
    queryClient.fetchQuery(trpc.subjects.getAll.queryOptions()),
    getInitialTableSettings("grades"),
  ]);

  const [classesResponse, terms] = await Promise.all([
    queryClient.fetchQuery(trpc.classes.getAll.queryOptions({ academicYearId: currentYear?.id })),
    currentYear 
      ? queryClient.fetchQuery(trpc.academic.getTerms.queryOptions({ yearId: currentYear.id })) 
      : Promise.resolve([]),
  ]);

  const { selectedClassId, selectedSubjectId, termId, month, yearId } = filter;
  if (selectedClassId && selectedSubjectId && termId && month && yearId) {
    prefetch(
      trpc.grades.getGridData.queryOptions({ 
        classId: selectedClassId, 
        subjectId: selectedSubjectId, 
        termId, 
        month, 
        academicYearId: yearId 
      })
    );
  }

  const initialData = {
    classes: classesResponse.data.map((c: any) => ({
      id: c.id,
      name: c.name,
      division: c.division,
    })),
    subjects: subjects.map((s: any) => ({ id: s.id, name: s.name })),
    terms: terms.map((t: any) => ({
      id: t.id,
      name: t.name,
      academicYearId: t.academicYearId,
    })),
    years: years.map((y: any) => ({ id: y.id, yearName: y.yearName })),
  };

  const defaults = {
    yearId: currentYear?.id,
    termId: terms[0]?.id,
    month: new Date().getMonth() + 1,
  };

  return (
    <HydrateClient>
      <ScrollableContent>
        <GradesUploadZone>
          <div className="flex flex-col gap-6" data-canvas-content>
            <GradeGridHeader initialData={initialData} defaults={defaults} />
            <ErrorBoundary errorComponent={ErrorFallback}>
              <Suspense fallback={<GradesSkeleton />}>
                <GradeGridWrapper initialSettings={initialSettings} />
              </Suspense>
            </ErrorBoundary>
          </div>
        </GradesUploadZone>
      </ScrollableContent>
    </HydrateClient>
  );
}
