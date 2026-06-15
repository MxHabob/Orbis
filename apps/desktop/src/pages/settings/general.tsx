import { Suspense } from "react";
import { AcademicSkeleton } from "@/components/tables/academic/skeleton";
import { DataTable } from "@/components/tables/academic/table";

export default function AcademicSettingsPage() {
  return (
    <div className="max-w-screen-lg">
      <Suspense fallback={<AcademicSkeleton />}>
        <DataTable />
      </Suspense>
    </div>
  );
}
