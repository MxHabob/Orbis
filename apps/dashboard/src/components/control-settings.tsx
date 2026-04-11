"use client";

import { ControlSecretNumbers } from "./control-secret-numbers";
import { ControlSeatingNumbers } from "./control-seating-numbers";
import { Button } from "@midday/ui/button";
import { ReportsExportSheet } from "./sheets/reports-export-sheet";
import { useReportExportParams } from "@/hooks/use-report-export-params";
import { Icons } from "@midday/ui/icons";

export function ControlSettings() {
  const { setParams } = useReportExportParams();

  return (
    <div className="flex flex-col gap-8">
      <div className="flex items-center justify-between">
        <Button variant="outline" className="gap-2 shrink-0" onClick={() => setParams({ exportReport: true })}>
           <Icons.FolderZip size={16} />
        </Button>
      </div>
      <div className="space-y-12">
        <ControlSecretNumbers />
        <ControlSeatingNumbers />
      </div>

      <ReportsExportSheet />
    </div>
  );
}
