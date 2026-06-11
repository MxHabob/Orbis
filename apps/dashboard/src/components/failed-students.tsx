"use client";

import NumberFlow from "@number-flow/react";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useParams } from "next/navigation";
import { useCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { useTRPC } from "@/trpc/client";

export function FailedStudents() {
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

  const failedCount = Number(data?.failedCount) || 0;

  return (
    <Card className="hidden sm:block">
      <CardHeader className="pb-2">
        <CardTitle className="font-medium text-2xl font-serif">
          <NumberFlow value={failedCount} willChange />
        </CardTitle>
      </CardHeader>

      <CardContent className="pb-5">
        <div className="flex flex-col gap-2">
          <div>Failed Students</div>
          <div className="text-sm text-muted-foreground">
            Students who failed their exams
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
