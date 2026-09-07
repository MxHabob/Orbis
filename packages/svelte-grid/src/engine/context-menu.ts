export type MenuContext = "cell" | "row" | "selection" | "column" | "filtered";

export interface ContextMenuItem {
  id: string;
  label: string;
  danger?: boolean;
  disabled?: boolean;
}

export function contextMenuItems(args: {
  context: MenuContext;
  hasSelection: boolean;
  canEdit: boolean;
  filtered: boolean;
}): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];
  const { context, hasSelection, canEdit, filtered } = args;

  if (context === "cell" || context === "row" || context === "selection") {
    items.push({ id: "copy", label: "Copy" });
    if (context === "row") items.push({ id: "copy-row", label: "Copy Row" });
    if (hasSelection) items.push({ id: "copy-selection", label: "Copy Selection" });
    if (canEdit) {
      items.push({ id: "paste", label: "Paste" });
      items.push({ id: "edit", label: "Edit" });
      items.push({ id: "duplicate", label: "Duplicate" });
      items.push({ id: "delete", label: "Delete", danger: true });
    }
    items.push({ id: "select-row", label: "Select Row" });
    items.push({ id: "select-all", label: "Select All" });
  }

  if (context === "cell") {
    items.push({ id: "filter-equals", label: "Filter by this value" });
    items.push({ id: "filter-exclude", label: "Exclude this value" });
  }

  if (context === "column" || context === "cell") {
    items.push({ id: "sort-asc", label: "Sort Ascending" });
    items.push({ id: "sort-desc", label: "Sort Descending" });
    items.push({ id: "group-by", label: "Group by this column" });
    items.push({ id: "hide-column", label: "Hide Column" });
    items.push({ id: "pin-column", label: "Pin Column" });
  }

  items.push({ id: "export-visible", label: "Export Visible" });
  if (hasSelection) items.push({ id: "export-selection", label: "Export Selection" });
  if (filtered) items.push({ id: "export-filtered", label: "Export Filtered Data" });
  items.push({ id: "refresh", label: "Refresh" });
  return items;
}
