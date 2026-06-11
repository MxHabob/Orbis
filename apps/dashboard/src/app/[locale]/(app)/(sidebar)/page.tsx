import type { Metadata } from "next";
import { cookies } from "next/headers";
import type { SearchParams } from "nuqs";
import { Widgets } from "@/components/widgets";
import { loadSortParams } from "@/hooks/use-sort-params";
import { loadTrackerFilterParams } from "@/hooks/use-tracker-filter-params";
import { getQueryClient, HydrateClient, trpc } from "@/trpc/server";
import { Cookies } from "@/utils/constants";

export const metadata: Metadata = {
  title: "Overview | Orbis",
};

type Props = {
  searchParams: Promise<SearchParams>;
};

export default async function Page(props: Props) {
  const searchParams = await props.searchParams;
  const filter = loadTrackerFilterParams(searchParams);
  const { sort } = loadSortParams(searchParams);
  const weeklyCalendar =
    (await cookies()).get(Cookies.WeeklyCalendar)?.value === "true";

  const queryClient = getQueryClient();

  const widgetPreferences = await queryClient.fetchQuery(
    trpc.widgets.getWidgetPreferences.queryOptions(),
  );

  return (
    <HydrateClient>
      <Widgets
        initialPreferences={widgetPreferences}
        weeklyCalendar={weeklyCalendar}
      />
    </HydrateClient>
  );
}
