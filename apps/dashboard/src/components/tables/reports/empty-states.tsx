"use client";

import { OpenReportsSheet } from "@/components/open-report-sheet";
import { Button } from "@midday/ui/button";
import { FileQuestion } from "lucide-react";

export function EmptyState() {
  return (
    <div className="flex items-center justify-center flex-col border border-border h-[calc(100vh-350px)]">
      <div className="flex flex-col items-center mt-6">
        <FileQuestion className="h-8 w-8 mb-4 text-muted-foreground" />
        <p className="text-sm text-muted-foreground mb-6 max-w-[240px] text-center">
          No classes found. Create your first class to start viewing reports.
        </p>
        <OpenReportsSheet />
      </div>
    </div>
  );
}

export function NoResults() {
  return (
    <div className="flex items-center justify-center flex-col border border-border h-[calc(100vh-350px)]">
      <div className="flex flex-col items-center mt-6">
        <FileQuestion className="h-8 w-8 mb-4 text-muted-foreground" />
        <p className="text-sm text-muted-foreground mb-4 max-w-[240px] text-center">
          No classes match your filters.
        </p>
        <Button variant="outline" onClick={() => window.location.reload()}>
          Clear filters
        </Button>
      </div>
    </div>
  );
}
