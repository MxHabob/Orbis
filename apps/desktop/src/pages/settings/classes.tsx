import { Suspense } from "react";
import { ClassesSkeleton } from "@/components/tables/classes/skeleton";
import { DataTable } from "@/components/tables/classes/table";

export default function ClassesSettingsPage() {
  return (
    <div className="max-w-screen-lg">
      <Suspense fallback={<ClassesSkeleton />}>
        <DataTable />
      </Suspense>
    </div>
  );
}
