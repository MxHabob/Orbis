"use client";

import { useTRPC } from "@/trpc/client";
import { Card, CardContent, CardHeader, CardTitle } from "@midday/ui/card";
import NumberFlow from "@number-flow/react";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useParams } from "next/navigation";

export function TotalStudents() {
  const params = useParams();
  const classId = params.Id as string;
  const trpc = useTRPC();
  
  const { data } = useSuspenseQuery(
    trpc.grades.getClassStatistics.queryOptions({ classId }),
  );

  const totalCount = data?.totalCount ?? 0;

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
