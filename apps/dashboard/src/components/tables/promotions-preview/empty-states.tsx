"use client";

import { useI18n } from "@/locales/client";
import { UserSearch } from "lucide-react";

export function PreviewEmptyState() {
  const t = useI18n();

  return (
    <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/5 text-muted-foreground animate-in fade-in duration-500">
      <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-6 text-3xl">
        👥
      </div>
      <h2 className="font-medium text-lg text-foreground mb-2">
        {t("grades.no_students")}
      </h2>
      <p className="text-sm text-[#606060] max-w-sm text-center">
        {t("grades.no_students_description")}
      </p>
    </div>
  );
}

