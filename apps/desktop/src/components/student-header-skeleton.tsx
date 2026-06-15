import { Skeleton } from "@orbis/ui/skeleton";

export function StudentHeaderSkeleton() {
  return (
    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 py-4 animate-in fade-in duration-500">
      <div className="flex items-center gap-4 min-w-0">
        <Skeleton className="size-6 rounded-none flex-shrink-0" />
        <Skeleton className="h-7 w-[200px]" />
      </div>

      <div className="flex items-center gap-3">
        <Skeleton className="h-9 w-full sm:w-[350px] rounded-none" />
        <Skeleton className="h-9 w-[120px] rounded-none" />
      </div>
    </div>
  );
}
