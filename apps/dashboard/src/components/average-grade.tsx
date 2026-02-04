"use client";

import { useTRPC } from "@/trpc/client";
import { Card, CardContent, CardHeader, CardTitle } from "@midday/ui/card";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useParams } from "next/navigation";

export function AverageGrade() {
  const params = useParams();
  const classId = params.Id as string;
  const trpc = useTRPC();
  
  const { data } = useSuspenseQuery(
    trpc.grades.getClassStatistics.queryOptions({ classId }),
  );

  const averageGrade = data?.averageGrade ?? 0;
  const formattedGrade = averageGrade.toFixed(1);

  return (
    <Card className="hidden sm:block">
      <CardHeader className="pb-2">
        <CardTitle className="font-medium text-2xl font-serif">
          {formattedGrade}%
        </CardTitle>
      </CardHeader>

      <CardContent className="pb-5">
        <div className="flex flex-col gap-2">
          <div>Average Grade</div>
          <div className="text-sm text-muted-foreground">
            Class average performance
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
