import type { Metadata } from "next";
import { Suspense } from "react";
import { ClassesSkeleton } from "@/components/tables/classes/skeleton";
import { DataTable } from "@/components/tables/classes/table";
import { HydrateClient, prefetch, trpc } from "@/trpc/server";

export const metadata: Metadata = {
  title: "Classes | Orbis",
};

export default async function Classes() {
  prefetch(trpc.classes.getAll.queryOptions());

  return (
    <div className="max-w-screen-lg">
      <HydrateClient>
        <Suspense fallback={<ClassesSkeleton />}>
          <DataTable />
        </Suspense>
      </HydrateClient>
    </div>
  );
}
