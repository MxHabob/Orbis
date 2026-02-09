
import { getI18n } from "@/locales/server";
import { DataTable } from "@/components/tables/certificates/data-table";
import { Button } from "@midday/ui/button";
import { Plus } from "lucide-react";
import Link from "next/link";
import { Suspense } from "react";
import { ScrollableContent } from "@/components/scrollable-content";
import { CertificateSummaryList } from "@/components/certificates/certificate-summary-list";
import type { Metadata } from "next";
import { TableToolbar } from "@/components/tables/certificates/table-toolbar";

export const metadata: Metadata = {
  title: "Certificate Templates | Midday",
};

export default async function CertificateTemplatesPage() {
  const t = await getI18n();

  return (
    <ScrollableContent>
      <div className="flex flex-col gap-6 mt-6">
        <Suspense fallback={
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="h-[120px] bg-muted/20 animate-pulse rounded-lg" />
            <div className="h-[120px] bg-muted/20 animate-pulse rounded-lg" />
            <div className="h-[120px] bg-muted/20 animate-pulse rounded-lg" />
          </div>
        }>
          <CertificateSummaryList />
        </Suspense>
        <TableToolbar />
        <Suspense fallback={<div className="h-[400px] w-full bg-muted/20 animate-pulse rounded-lg" />}>
          <DataTable />
        </Suspense>
      </div>
    </ScrollableContent>
  );
}
