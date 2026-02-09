import { OpenTrackerSheet } from "@/components/open-tracker-sheet";
import { ScrollableContent } from "@/components/scrollable-content";
import { DataTable } from "@/components/tables/tracker";
import { Loading } from "@/components/tables/tracker/loading";
import { TrackerCalendar } from "@/components/tracker-calendar";
import { TrackerSearchFilter } from "@/components/tracker-search-filter";
import { Widgets } from "@/components/widgets";
import { loadSortParams } from "@/hooks/use-sort-params";
import { loadTrackerFilterParams } from "@/hooks/use-tracker-filter-params";
import { getQueryClient, HydrateClient, trpc } from "@/trpc/server";
import { Cookies } from "@/utils/constants";
import type { Metadata } from "next";
import { cookies } from "next/headers";
import type { SearchParams } from "nuqs";
import { Suspense } from "react";

export const metadata: Metadata = {
  title: "Overview | Entity",
};

type Props = {
  searchParams: Promise<SearchParams>;
};

export default async function Page(props: Props) {
  const searchParams = await props.searchParams;
  const filter = loadTrackerFilterParams(searchParams);
  const { sort } = loadSortParams(searchParams);
  const weeklyCalendar = (await cookies()).get(Cookies.WeeklyCalendar)?.value === "true";


    const queryClient = getQueryClient();

  const widgetPreferences = await queryClient.fetchQuery(
    trpc.widgets.getWidgetPreferences.queryOptions(),
  );

  return (
    <HydrateClient>
      <Widgets initialPreferences={widgetPreferences} weeklyCalendar={weeklyCalendar} />
    </HydrateClient>
  );
}