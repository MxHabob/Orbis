import { Suspense } from "react";
import type { Metadata } from "next";
import { AcademicSkeleton } from "@/components/tables/academic/skeleton";
import { DataTable } from "@/components/tables/academic/table";
import { HydrateClient, prefetch, trpc } from "@/trpc/server";

export const metadata: Metadata = {
  title: "Settings | Entity",
};

export default async function AcademicSettingsPage() {
  prefetch(trpc.academic.getYears.queryOptions());

  return (
    <div className="max-w-screen-lg">
      <HydrateClient>
        <Suspense fallback={<AcademicSkeleton />}>
          <DataTable />
        </Suspense>
      </HydrateClient>
    </div>
  );
}
