<!--
@component StudentTable
Renders the list of students using the DataTable infrastructure.
Matches the column layout and styling of the React dashboard precisely.
-->
<script lang="ts">
import { DataTable } from "$lib/tables";
import type { Enrollment } from "$lib/core/ipc";
import * as m from "$lib/paraglide/messages";
import { formatDate } from "$lib/core/i18n";
import { createInitials } from "$lib/tables";
import type { OrbisColumnDef, GridActionEvent } from "@orbis/svelte-grid";
import { GraduationCap } from "lucide-svelte";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { studentsStore } from "$lib/features/students";

let {
  students,
  classes = [],
  enableSelection = false,
  selectedIds = $bindable(new Set()),
  onSelectionChange,
  orgId,
}: {
  students: Enrollment[];
  classes?: any[];
  enableSelection?: boolean;
  selectedIds?: Set<string>;
  onSelectionChange?: (ids: Set<string>) => void;
  orgId: string;
} = $props();

// ── Helpers ────────────────────────────────────────────────────────

function getClassName(classId: string): string {
  const cls = classes.find((c) => c.id === classId);
  return cls
    ? String(cls.name || cls.name_ar || classId)
    : m.table_not_enrolled();
}

// ── Column Definitions ─────────────────────────────────────────────

import { AvatarCell, BadgeCell } from "$lib/tables";

// ── Column Definitions ─────────────────────────────────────────────

const columns = $derived.by<OrbisColumnDef[]>(() => [
  {
    id: "full_name",
    prop: "full_name",
    header: m.table_student(),
    name: m.table_student(),
    width: 320,
    size: 320,
    pin: "colPinStart",
    cell: AvatarCell,
  },
  {
    id: "student_number",
    prop: "student_number",
    header: m.table_student_number(),
    name: m.table_student_number(),
    width: 150,
    size: 150,
    cellClass:
      "inline-flex items-center rounded-md border border-border px-2 py-0.5 text-[10px] font-mono font-medium text-foreground bg-background",
  },
  {
    id: "class_name",
    prop: "class_name",
    header: m.table_class(),
    name: m.table_class(),
    width: 150,
    size: 150,
  },
  {
    id: "status",
    prop: "status",
    header: m.table_status(),
    name: m.table_status(),
    width: 120,
    size: 120,
    cell: BadgeCell,
  },
  {
    id: "sex_display",
    prop: "sex_display",
    header: m.form_sex(),
    name: m.form_sex(),
    width: 100,
    size: 100,
  },
  {
    id: "enrollment_date_display",
    prop: "enrollment_date_display",
    header: m.table_enrolled_at(),
    name: m.table_enrolled_at(),
    width: 150,
    size: 150,
    cellClass: "text-xs text-muted-foreground",
  },
]);

let gridSource = $derived(
  (students || []).map((student, i) => ({
    ...student,
    __idx: i + 1,
    class_name: getClassName(String(student.class_id)),
    sex_display:
      student.sex === "male"
        ? m.form_male()
        : student.sex === "female"
          ? m.form_female()
          : "-",
    enrollment_date_display: student.enrollment_date
      ? formatDate(student.enrollment_date)
      : "-",
    __selected: selectedIds.has(String(student.id)),
  })),
);

let activeColumns = $derived.by(() => {
  const cols = [...columns];
  if (enableSelection) {
    cols.unshift({
      id: "__selected",
      prop: "__selected",
      header: "",
      name: "",
      width: 40,
      size: 40,
      readonly: true,
      pin: "colPinStart",
      editor: "checkbox",
    });
  }
  return cols;
});

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "dblclick") {
    goto(localizeHref(`/${orgId}/students/${event.row.id}`));
  } else if (event.actionId === "toggle-select") {
    const id = String(event.row.id);
    const newSet = new Set(selectedIds);
    if (newSet.has(id)) {
      newSet.delete(id);
    } else {
      newSet.add(id);
    }
    selectedIds = newSet;
    onSelectionChange?.(newSet);
  }
}
</script>

<DataTable
  columns={activeColumns}
  source={gridSource}
  height="100%"
  rowSize={48}
  rtl={getLocale() === "ar"}
  onaction={handleGridAction}
  emptyIcon={GraduationCap}
  emptyTitle={m.table_no_students()}
  emptyDescription={m.table_no_students_hint()}
/>
