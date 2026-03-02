"use client";

import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@midday/ui/sheet";
import { AcademicYearForm } from "../forms/academic-year-form";
import { AcademicTermForm } from "../forms/academic-term-form";
import { useI18n } from "@/locales/client";
import { useAcademicParams } from "@/hooks/use-academic-params";
import { useTRPC } from "@/trpc/client";
import { useQuery } from "@tanstack/react-query";

export function AcademicSheets() {
  const t = useI18n();
  const trpc = useTRPC();
  const { yearId, termId, createYear, createTerm, setParams } = useAcademicParams();

  const { data: yearData } = useQuery({
    ...trpc.academic.getYearById.queryOptions({ id: parseInt(yearId!) }),
    enabled: !!yearId && !createTerm,
  });

  const isYearOpen = !!createYear || (!!yearId && !createTerm);
  const isTermOpen = !!createTerm || !!termId;

  return (
    <>
      <Sheet open={isYearOpen} onOpenChange={() => setParams({ createYear: null, yearId: null })}>
        <SheetContent>
          <SheetHeader className="pb-6">
            <SheetTitle>
              {yearId ? t("settings.academic_year_title") : t("settings.academic_year_title")}
            </SheetTitle>
          </SheetHeader>
          <AcademicYearForm 
            initialData={yearData} 
            onSuccess={() => setParams({ createYear: null, yearId: null })} 
          />
        </SheetContent>
      </Sheet>

      <Sheet open={isTermOpen} onOpenChange={() => setParams({ createTerm: null, termId: null, yearId: null })}>
        <SheetContent>
          <SheetHeader className="pb-6">
            <SheetTitle>
              {termId ? t("reports.half_yearly") : t("reports.half_yearly")}
            </SheetTitle>
          </SheetHeader>
          <AcademicTermForm 
            yearId={parseInt(yearId!)}
            termId={parseInt(termId!)}
            onSuccess={() => setParams({ createTerm: null, termId: null, yearId: null })} 
          />
        </SheetContent>
      </Sheet>
    </>
  );
}
