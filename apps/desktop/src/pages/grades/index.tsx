import { Suspense } from "react";
import { useQuery } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { ErrorBoundary } from "@/components/error-boundary";
import { ErrorFallback } from "@/components/error-fallback";
import { GradeGridHeader } from "@/components/grades/grade-grid-header";
import { GradeGridWrapper } from "@/components/grades/grade-grid-wrapper";
import { GradesUploadZone } from "@/components/grades/grades-upload-zone";
import { ScrollableContent } from "@/components/scrollable-content";
import { GradesSkeleton } from "@/components/tables/grades/skeleton";
import { loadGradeFilterParams } from "@/hooks/use-grade-filter-params";
import { getInitialTableSettings } from "@/utils/columns";
import { commands } from "@orbis/desktop-client/commands";
import { Spinner } from "@orbis/ui/spinner";

export default function GradesPage() {
  const [searchParams] = useSearchParams();
  
  const searchParamsRecord: Record<string, string> = {};
  searchParams.forEach((value, key) => {
    searchParamsRecord[key] = value;
  });

  const filter = loadGradeFilterParams(searchParamsRecord);

  const { data, isLoading } = useQuery({
    queryKey: ["grades-page-init", filter.yearId],
    queryFn: async () => {
      const currentYear = await commands.academics.getCurrentYear("") as any;
      const years = await commands.academics.getYears("") as any[];
      const subjects = await commands.subjects.getAll("") as any[];
      const initialSettings = await getInitialTableSettings("grades");

      const activeYearId = filter.yearId || currentYear?.id;

      const classesResponse = await commands.classes.getAll("", { academicYearId: activeYearId }) as any[];
      const terms = activeYearId
        ? await commands.academics.getTerms("", activeYearId) as any[]
        : [];

      const initialData = {
        classes: (classesResponse || []).map((c: any) => ({
          id: c.id,
          name: c.name,
          division: c.division,
        })),
        subjects: (subjects || []).map((s: any) => ({ id: s.id, name: s.name })),
        terms: (terms || []).map((t: any) => ({
          id: t.id,
          name: t.name,
          academicYearId: t.parent_id || t.academicYearId,
        })),
        years: (years || []).map((y: any) => ({
          id: y.id,
          yearName: y.name || y.yearName,
          isCurrent: y.is_current || y.isCurrent,
        })),
      };

      const defaults = {
        yearId: currentYear?.id,
        termId: terms[0]?.id,
        month: new Date().getMonth() + 1,
      };

      return { initialData, defaults, initialSettings };
    }
  });

  if (isLoading || !data) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <Spinner />
      </div>
    );
  }

  const { initialData, defaults, initialSettings } = data;

  return (
    <ScrollableContent>
      <GradesUploadZone>
        <div className="flex flex-col gap-6" data-canvas-content>
          <GradeGridHeader initialData={initialData} defaults={defaults} />
          <ErrorBoundary fallback={<ErrorFallback />}>
            <Suspense fallback={<GradesSkeleton />}>
              <GradeGridWrapper initialSettings={initialSettings} />
            </Suspense>
          </ErrorBoundary>
        </div>
      </GradesUploadZone>
    </ScrollableContent>
  );
}
