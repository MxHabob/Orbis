import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import { Skeleton } from "@orbis/ui/skeleton";

export function StudentSummarySkeleton() {
  return (
    <Card className="hidden sm:block animate-in fade-in duration-500">
      <CardHeader className="pb-2">
        <CardTitle className="font-medium text-2xl font-serif">
          <Skeleton className="h-[32px] w-24" />
        </CardTitle>
      </CardHeader>

      <CardContent className="pb-5">
        <div className="flex flex-col gap-2">
          <Skeleton className="h-[20px] w-28" />
          <Skeleton className="h-[14px] w-44 opacity-60" />
        </div>
      </CardContent>
    </Card>
  );
}
