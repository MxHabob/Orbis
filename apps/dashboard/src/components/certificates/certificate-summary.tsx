"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@midday/ui/card";
import { Skeleton } from "@midday/ui/skeleton";

type Props = {
  value: number | string;
  title: string;
  description: string;
};

export function CertificateSummarySkeleton() {
  return (
    <Card className="hidden sm:block">
      <CardHeader className="pb-2 flex flex-row items-center">
        <CardTitle className="font-medium text-2xl font-serif">
          <Skeleton className="h-[32px] w-12" />
        </CardTitle>
      </CardHeader>

      <CardContent className="pb-5">
        <div className="flex flex-col gap-2">
          <Skeleton className="h-[20px] w-24" />
          <Skeleton className="h-[18px] w-32" />
        </div>
      </CardContent>
    </Card>
  );
}

export function CertificateSummary({ value, title, description }: Props) {
  return (
    <Card>
      <CardHeader className="pb-2 flex flex-row items-center">
        <CardTitle className="font-medium text-2xl font-serif">
          {value}
        </CardTitle>
      </CardHeader>

      <CardContent className="pb-5">
        <div className="flex flex-col gap-2">
          <div className="text-sm font-medium">{title}</div>
          <div className="text-xs text-muted-foreground">
            {description}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
