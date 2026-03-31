import { SubjectsSkeleton } from "@/components/tables/subjects/skeleton";
import { DataTable } from "@/components/tables/subjects/table";
import { HydrateClient, prefetch, trpc } from "@/trpc/server";
import type { Metadata } from "next";
import { Suspense } from "react";

export const metadata: Metadata = {
  title: "Subjects | Mommy",
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