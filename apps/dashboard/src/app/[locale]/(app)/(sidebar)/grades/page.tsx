import { AddTransactions } from "@/components/add-transactions";
import { GradeGridWrapper } from "@/components/grades/grade-grid-wrapper";
import { ScrollableContent } from "@/components/scrollable-content";
import { DataTable } from "@/components/tables/transactions/data-table";
import { Loading } from "@/components/tables/transactions/loading";
import { TransactionTabs } from "@/components/transaction-tabs";
import { TransactionsColumnVisibility } from "@/components/transactions-column-visibility";
import { TransactionsSearchFilter } from "@/components/transactions-search-filter";
import { TransactionsUploadZone } from "@/components/transactions-upload-zone";
import { GradesSearchFilter } from "@/components/grades/grades-search-filter";
import { loadGradeFilterParams } from "@/hooks/use-grade-filter-params";
import { loadSortParams } from "@/hooks/use-sort-params";
import { loadTransactionFilterParams } from "@/hooks/use-transaction-filter-params";
import { loadGradesTab } from "@/hooks/use-grades-tab";
import { HydrateClient, batchPrefetch, trpc } from "@/trpc/server";
import { getInitialTableSettings } from "@/utils/columns";
import type { Metadata } from "next";
import type { SearchParams } from "nuqs/server";
import { Suspense } from "react";
import { ClassesSelect } from "@/components/grades/classes-select";
import { SubjectsSelect } from "@/components/grades/subjects-select";
import { PeriodSelect } from "@/components/grades/period-select";
import { GradesTabs } from "@/components/grades/grades-tabs";
import { GradesColumnVisibility } from "@/components/grades/grades-column-visibility";
import { GradesActions } from "@/components/grades/grades-actions";
import { GradeDataTable } from "@/components/tables/grades/data-table";
import { GradesUploadZone } from "@/components/grades/grades-upload-zone";

export const metadata: Metadata = {
  title: "Grades | Midday",
};

type Props = {
  searchParams: Promise<SearchParams>;
};

export default async function Grades(props: Props) {
  const searchParams = await props.searchParams;

  const filter = loadGradeFilterParams(searchParams);
  const { sort } = loadSortParams(searchParams);
  const { tab } = loadGradesTab(searchParams);

  // Get unified table settings from cookie
  const initialSettings = await getInitialTableSettings("grades");

  // Build query filters for both tabs
  const allTabFilter = {
    ...filter,
    month: filter.month ?? null,
    sort,
  };

  const reviewTabFilter = {
    ...filter,
    month: filter.month ?? null,
    sort,
    fulfilled: true,
    exported: false,
    pageSize: 10000,
  };

  // Prefetch all data needed for instant experience
  // batchPrefetch([
  //   // Transaction data for both tabs
  //   trpc.transactions.get.infiniteQueryOptions(allTabFilter),
  //   trpc.transactions.get.infiniteQueryOptions(reviewTabFilter),
  //   trpc.transactions.getReviewCount.queryOptions(),
  //   // Shared data used by table rows (assign user, tags)
  //   trpc.team.members.queryOptions(),
  //   trpc.tags.get.queryOptions(),
  //   // Apps for export bar (accounting providers)
  //   trpc.apps.get.queryOptions(),
  // ]);

  return (
    <HydrateClient>
      <ScrollableContent>
        <GradesUploadZone>
          <div className="flex justify-between items-center py-6">
            <GradesSearchFilter />
            <div className="flex items-center gap-4">
              <div className="hidden md:flex items-center gap-2">
                <GradesColumnVisibility />
                <GradesActions />
                {/* <PeriodSelect />
                <SubjectsSelect />
                <ClassesSelect /> */}
              </div>
              <GradesTabs />
            </div>
          </div>

          <Suspense
            fallback={
              <Loading
                columnVisibility={initialSettings.columns}
                columnSizing={initialSettings.sizing}
                columnOrder={initialSettings.order}
              />
            }
          >
            <GradeDataTable initialSettings={initialSettings} initialTab={tab}/>
          </Suspense>
        </GradesUploadZone>
      </ScrollableContent>
    </HydrateClient>
  );
}
