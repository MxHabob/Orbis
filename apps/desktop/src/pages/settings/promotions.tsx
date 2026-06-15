import { Suspense } from "react";
import { useQuery } from "@tanstack/react-query";
import { PromotionsClient } from "@/components/promotions/promotions-client";
import { commands } from "@orbis/desktop-client/commands";
import { Spinner } from "@orbis/ui/spinner";

export default function PromotionsSettingsPage() {
  const { data, isLoading } = useQuery({
    queryKey: ["promotions-settings-init"],
    queryFn: async () => {
      const currentYear = await commands.academics.getCurrentYear("") as any;
      const years = await commands.academics.getYears("") as any[];

      const classesResponse = await commands.classes.getAll("", { academicYearId: currentYear?.id }) as any[];

      const initialData = {
        classes: (classesResponse || []).map((c: any) => ({
          id: c.id,
          name: c.name,
          division: c.division,
        })),
        years: (years || []).map((y: any) => ({ id: y.id, yearName: y.name || y.yearName })),
      };

      return { initialData, currentYearId: currentYear?.id };
    }
  });

  if (isLoading || !data) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <Spinner />
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <Suspense
        fallback={
          <div className="h-[500px] w-full bg-muted/20 animate-pulse rounded-lg" />
        }
      >
        <PromotionsClient
          initialData={data.initialData}
          currentYearId={data.currentYearId}
        />
      </Suspense>
    </div>
  );
}
