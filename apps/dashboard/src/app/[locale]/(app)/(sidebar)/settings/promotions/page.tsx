import { Suspense } from "react";
import type { Metadata } from "next";
import { trpc, HydrateClient, getQueryClient } from "@/trpc/server";
import { PromotionsClient } from "@/components/promotions/promotions-client";

export const metadata: Metadata = {
  title: "Promotions | Mommy",
};

export default async function PromotionsPage() {
  const queryClient = getQueryClient();

  const [currentYear, years] = await Promise.all([
    queryClient.fetchQuery(trpc.academic.getCurrentYear.queryOptions()),
    queryClient.fetchQuery(trpc.academic.getYears.queryOptions()),
  ]);

  const classesResponse = await queryClient.fetchQuery(
    trpc.classes.getAll.queryOptions({ academicYearId: currentYear?.id })
  );

  const initialData = {
    classes: (classesResponse.data || []).map((c: any) => ({
      id: c.id,
      name: c.name,
      division: c.division,
    })),
    years: (years || []).map((y: any) => ({ id: y.id, yearName: y.yearName })),
  };

  return (
    <HydrateClient>
      <div className="flex flex-col gap-4"> 
        <Suspense fallback={<div className="h-[500px] w-full bg-muted/20 animate-pulse rounded-lg" />}>
          <PromotionsClient 
            initialData={initialData} 
            currentYearId={currentYear?.id} 
          />
        </Suspense>
      </div>
    </HydrateClient>
  );
}
