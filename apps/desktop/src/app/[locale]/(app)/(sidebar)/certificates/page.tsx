import type { Metadata } from "next";
import { Suspense } from "react";
import { ScrollableContent } from "@/components/scrollable-content";
import { DataTable } from "@/components/tables/certificates/data-table";
import { TableToolbar } from "@/components/tables/certificates/table-toolbar";
import { getI18n } from "@/locales/server";

export const metadata: Metadata = {
  title: "Certificate Templates | Orbis",
};

export default async function CertificateTemplatesPage() {
  const t = await getI18n();

  return (
    <ScrollableContent>
      <div className="flex flex-col gap-2 mt-4">
        <TableToolbar />
        <Suspense
          fallback={
            <div className="h-[400px] w-full bg-muted/20 animate-pulse rounded-lg" />
          }
        >
          <DataTable />
        </Suspense>
      </div>
    </ScrollableContent>
  );
}
