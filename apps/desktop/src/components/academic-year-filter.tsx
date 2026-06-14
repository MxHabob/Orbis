"use client";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useLocation } from "react-router-dom";
import { useClassesFilterParams } from "@/hooks/use-classes-filter-params";
import { useCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";

export function AcademicYearFilter() {
  const trpc = useTRPC();
  const { pathname } = useLocation();
  const t = useI18n();

  // Choose the right hook based on the page
  const isDetailsPage = pathname.includes("/reports/");
  const { filter: classFilter, setFilter: setClassFilter } =
    useClassesFilterParams();
  const { filter: customerFilter, setFilter: setCustomerFilter } =
    useCustomerFilterParams();

  const currentFilter = isDetailsPage ? customerFilter : classFilter;
  const setCurrentFilter = isDetailsPage ? setCustomerFilter : setClassFilter;

  const { data: years } = useSuspenseQuery(
    trpc.academic.getYears.queryOptions(),
  );

  const handleValueChange = (value: string) => {
    setCurrentFilter({ academicYearId: value === "current" ? null : value });
  };

  const currentValue = currentFilter.academicYearId || "current";

  return (
    <div className="flex items-center space-x-2">
      <Select value={currentValue} onValueChange={handleValueChange}>
        <SelectTrigger className="h-8 min-w-[130px] text-xs border-border rounded-none shadow-none bg-background focus:ring-0">
          <SelectValue placeholder={t("education.academic_year")} />
        </SelectTrigger>
        <SelectContent className="rounded-none">
          <SelectItem value="current" className="text-xs rounded-none">
            {t("education.current")}
          </SelectItem>
          {years?.map((year) => (
            <SelectItem
              key={year.id}
              value={year.id.toString()}
              className="text-xs rounded-none text-left"
            >
              <div className="flex items-center gap-2">
                <span>{year.yearName}</span>
                {year.isCurrent && (
                  <span className="text-[9px] text-muted-foreground">
                    ({t("education.current")})
                  </span>
                )}
              </div>
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
