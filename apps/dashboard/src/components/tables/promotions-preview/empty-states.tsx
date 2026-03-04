"use client";

import { useI18n } from "@/locales/client";
import { UserSearch } from "lucide-react";

export function PreviewEmptyState() {
  const t = useI18n();

  return (
    <div className="flex flex-col items-center justify-center py-24 text-center border-b border-x bg-background border-border">
      <div className="size-10 bg-muted flex items-center justify-center rounded-none mb-4">
        <UserSearch className="size-5 text-muted-foreground" />
      </div>
      <h3 className="text-xs font-bold uppercase tracking-tight mb-1">
        {t("grades.no_students")}
      </h3>
      <p className="text-[11px] text-[#606060] max-w-[200px] leading-relaxed">
        {(t as any)("promotions.no_preview_desc") || "لا توجد بيانات متاحة للمعاينة في هذا الصف."}
      </p>
    </div>
  );
}
