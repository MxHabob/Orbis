"use client";

import { useTRPC } from "@/trpc/client";
import { useSuspenseQuery } from "@tanstack/react-query";
import { CertificateSummary } from "./certificate-summary";
import { formatDate } from "date-fns";

export function CertificateSummaryList() {
  const trpc = useTRPC();
  const { data } = useSuspenseQuery(
    trpc.certificateTemplates.summary.queryOptions()
  );

  if (!data) return null;

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
      <CertificateSummary 
        value={data.total} 
        title="Total Templates" 
        description="Total certificate templates created" 
      />
      <CertificateSummary 
        value={data.defaultCount} 
        title="Default Templates" 
        description="Templates set as default for new students" 
      />
      <CertificateSummary 
        value={data.lastUpdated ? formatDate(data.lastUpdated, "MMM d, yyyy") : "Never"} 
        title="Last Updated" 
        description="Most recent template modification" 
      />
    </div>
  );
}
