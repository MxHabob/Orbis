import { Suspense } from "react";
import { SubjectsSkeleton } from "@/components/tables/subjects/skeleton";
import { DataTable } from "@/components/tables/subjects/table";

export default function SubjectsSettingsPage() {
  return (
    <div className="max-w-screen-lg">
      <Suspense fallback={<SubjectsSkeleton />}>
        <DataTable />
      </Suspense>
    </div>
  );
}
