import { useI18n } from "@/locales/client";

export function EmptyState() {
  const t = useI18n();
  return (
    <div className="flex flex-col items-center justify-center p-20 border-2 border-dashed rounded-2xl bg-muted/5 text-muted-foreground animate-in fade-in duration-500">
      <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-4 text-3xl">
        📊
      </div>
      <p className="font-medium text-lg">{t("grades.no_students")}</p>
      <p className="text-sm">{t("grades.no_students_description")}</p>
    </div>
  );
}

export function NoResults() {
  const t = useI18n();
  return (
    <div className="flex flex-col items-center justify-center p-20 border border-dashed rounded-xl bg-muted/5 text-muted-foreground animate-in fade-in duration-300">
      <div className="h-12 w-12 rounded-full bg-muted/20 flex items-center justify-center mb-4 text-2xl">
        🔍
      </div>
      <p className="font-semibold text-lg">{t("vault.no_results_title")}</p>
      <p className="text-sm">{t("grades.no_results_description")}</p>
    </div>
  );
}
