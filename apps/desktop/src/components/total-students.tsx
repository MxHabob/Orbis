"use client";

import NumberFlow from "@number-flow/react";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useParams } from "react-router-dom";
import { useCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { useTRPC } from "@/ipc/query-adapter";

export function TotalStudents() {
  const params = useParams();
  const classId = params.Id as string;
  const trpc = useTRPC();
  const { filter } = useCustomerFilterParams();
  const academicYearId = filter.academicYearId
    ? Number(filter.academicYearId)
    : undefined;

  const { data } = useSuspenseQuery(
    trpc.grades.getClassStatistics.queryOptions({
      classId,
      academicYearId,
    }),
  );

  const totalCount = Number(data?.totalCount) || 0;

  return (
    <Card className="hidden sm:block">
      <CardHeader className="pb-2">
        <CardTitle className="font-medium text-2xl font-serif">
          <NumberFlow value={totalCount} willChange />
        </CardTitle>
      </CardHeader>

      <CardContent className="pb-5">
        <div className="flex flex-col gap-2">
          <div>Total Students</div>
          <div className="text-sm text-muted-foreground">
            All students in this class
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
