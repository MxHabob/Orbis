import { Suspense } from "react";
import type { Metadata } from "next";
import { GradeGridHeader } from "@/components/grades/grade-grid-header";
import { GradeGridWrapper } from "@/components/grades/grade-grid-wrapper";
import { ScrollableContent } from "@/components/scrollable-content";
import { getClassesV3 } from "@midday/db/queries/classes";
import { getAllSubjects } from "@midday/db/queries/subjects";
import { getAcademicYears, getTermsByYear, getCurrentAcademicYear } from "@midday/db/queries/academic-calendar";
import { GradesSkeleton } from "@/components/tables/grades/skeleton";
import { getInitialTableSettings } from "@/utils/columns";
import { ErrorBoundary } from "next/dist/client/components/error-boundary";
import { ErrorFallback } from "@/components/error-fallback";

export const metadata: Metadata = {
  title: "Grade | Entity",
};

export default async function GradesPage() {
  const currentYear = await getCurrentAcademicYear();
  const years = await getAcademicYears();
  const classes = await getClassesV3(currentYear?.id);
  const subjects = await getAllSubjects();
  const terms = currentYear ? await getTermsByYear(currentYear.id) : [];
  const initialSettings = await getInitialTableSettings("grades");

  const initialData = {
    classes: classes.map((c: any) => ({
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
      <ScrollableContent>
        <div className="flex flex-col gap-6">

          <GradeGridHeader initialData={initialData} defaults={defaults} />

          <ErrorBoundary errorComponent={ErrorFallback}>
            <Suspense fallback={<GradesSkeleton />}>
              <GradeGridWrapper initialSettings={initialSettings} />
            </Suspense>
          </ErrorBoundary>
        </div>
      </ScrollableContent>
  );
}
