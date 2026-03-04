"use client";

import { useI18n } from "@/locales/client";
import { GraduationCap } from "lucide-react";

export function HistoryEmptyState() {
  const t = useI18n();

  return (
    <div className="flex flex-col items-center justify-center py-24 text-center border-b border-x bg-background border-border">
      <div className="size-10 bg-muted flex items-center justify-center rounded-none mb-4">
        <GraduationCap className="size-5 text-muted-foreground" />
      </div>
      <h3 className="text-xs font-bold uppercase tracking-tight mb-1">
        {(t as any)("promotions.no_history") || "سجل ترفيعات فارغ"}
      </h3>
      <p className="text-[11px] text-[#606060] max-w-[200px] leading-relaxed">
        {(t as any)("promotions.no_history_desc") || "لا يوجد سجل للترفيعات لهذا الصف حتى الآن."}
      </p>
    </div>
  );
}
