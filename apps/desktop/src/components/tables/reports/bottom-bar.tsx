import { useI18n } from "@/locales/client";
import { useReportStore } from "@/store/report";
import { BottomBar as CoreBottomBar } from "../core/bottom-bar";

export function BottomBar() {
  const { rowSelection, setRowSelection } = useReportStore();
  const t = useI18n();

  const selectedIds = Object.keys(rowSelection).filter(
    (id) => rowSelection[id],
  );
  const count = selectedIds.length;

  if (count === 0) return null;

  return (
    <CoreBottomBar
      selectedCount={count}
      onDeselect={() => setRowSelection({})}
      selectedLabel={(count) => (
        <span className="font-medium">
          {t("reports.selected_classes", { count })}
        </span>
      )}
      deselectLabel={t("common.deselect_all")}
    />
  );
}
