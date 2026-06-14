import type { Metadata } from "next";
import { Suspense } from "react";
import { SubjectsSkeleton } from "@/components/tables/subjects/skeleton";
import { DataTable } from "@/components/tables/subjects/table";
import { HydrateClient, prefetch, trpc } from "@/trpc/server";

export const metadata: Metadata = {
  title: "Subjects | Orbis",
};

export default async function Subjects() {
  prefetch(trpc.subjects.getAll.queryOptions());

  return (
    <div className="max-w-screen-lg">
      <HydrateClient>
        <Suspense fallback={<SubjectsSkeleton />}>
          <DataTable />
        </Suspense>
      </HydrateClient>
    </div>
  );
}
