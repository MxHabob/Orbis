"use client";

import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import { AcademicYearForm } from "../forms/academic-year-form";
import { AcademicTermForm } from "../forms/academic-term-form";
import { useI18n } from "@/locales/client";
import { useAcademicParams } from "@/hooks/use-academic-params";
import { useTRPC } from "@/trpc/client";
import { useQuery } from "@tanstack/react-query";

export function AcademicSheets() {
  const t = useI18n();
  const trpc = useTRPC();
  const { editYearId, editTermId, parentYearId, createYear, createTerm, setParams } = useAcademicParams();

  const { data: yearData } = useQuery({
    ...trpc.academic.getYearById.queryOptions({ id: parseInt(editYearId!) }),
    enabled: !!editYearId && !createTerm,
  });

  const isYearOpen = !!createYear || !!editYearId;
  const isTermOpen = !!createTerm || !!editTermId;

  return (
    <>
      <Sheet open={isYearOpen} onOpenChange={() => setParams({ createYear: null, editYearId: null })}>
        <SheetContent>
          <SheetHeader className="pb-6">
            <SheetTitle>
              {editYearId ? t("settings.academic_year_title") : t("settings.academic_year_title")}
            </SheetTitle>
          </SheetHeader>
          <AcademicYearForm 
            initialData={yearData} 
            onSuccess={() => setParams({ createYear: null, editYearId: null })} 
          />
        </SheetContent>
      </Sheet>

      <Sheet open={isTermOpen} onOpenChange={() => setParams({ createTerm: null, editTermId: null, parentYearId: null })}>
        <SheetContent>
          <SheetHeader className="pb-6">
            <SheetTitle>
              {editTermId ? t("reports.half_yearly") : t("reports.half_yearly")}
            </SheetTitle>
          </SheetHeader>
          <AcademicTermForm 
            yearId={parentYearId ? parseInt(parentYearId) : undefined}
            termId={editTermId ? parseInt(editTermId) : undefined}
            onSuccess={() => setParams({ createTerm: null, editTermId: null, parentYearId: null })} 
          />
        </SheetContent>
      </Sheet>
    </>
  );
}
