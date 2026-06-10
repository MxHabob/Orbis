"use client";

import { Skeleton } from "@orbis/ui/skeleton";

export function ReportsHeaderSkeleton() {
  return (
    <div className="flex items-center justify-between py-4 animate-in fade-in duration-500">
      <div className="flex items-center gap-4">
        <Skeleton className="h-9 w-[250px] rounded-none" />
        <Skeleton className="h-8 w-[130px] rounded-none" />
      </div>

      <div className="hidden sm:flex space-x-2">
        <Skeleton className="h-9 w-[120px] rounded-none" />
      </div>
    </div>
  );
}
