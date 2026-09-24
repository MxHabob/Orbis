<!--
@component StudentTable
Renders the list of students using OrbisGrid directly.
Matches the column layout and styling of the React dashboard precisely.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { Enrollment } from "$lib/api";
import { formatDate, l } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { studentFilterFields } from "$lib/core/filter-configs";

let {
  students,
  classes = [],
  enableSelection = true,
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
    ? String(l(cls.name_ar, cls.name) || classId)
    : m.common_status_not_enrolled();
}

// ── Column Definitions ─────────────────────────────────────────────
const baseColumns: OrbisColumnDef[] = [
  {
    id: "full_name",
    prop: "full_name",
    header: m.common_word_student(),
    name: m.common_word_student(),
    width: 280,
    size: 280,
    type: "avatar",
    readonly: true,
    pin: "left",
  },
  {
    id: "student_number",
    prop: "student_number",
    header: m.common_table_student_number(),
    name: m.common_table_student_number(),
    width: 150,
    size: 150,
    readonly: true,
    cellClass: "font-mono text-muted-foreground",
  },
  {
    id: "class_name",
    prop: "class_name",
    header: m.common_table_class(),
    name: m.common_table_class(),
    width: 160,
    size: 160,
    readonly: true,
  },
  {
    id: "status",
    prop: "status",
    header: m.common_word_status(),
    name: m.common_word_status(),
    width: 120,
    size: 120,
    type: "badge",
    align: "center",
    readonly: true,
  },
  {
    id: "sex_display",
    prop: "sex_display",
    header: m.common_form_sex(),
    name: m.common_form_sex(),
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
  },
  {
    id: "enrollment_date_display",
    prop: "enrollment_date_display",
    header: m.common_table_enrolled_at(),
    name: m.common_table_enrolled_at(),
    width: 150,
    size: 150,
    readonly: true,
    cellClass: "text-xs text-muted-foreground",
  },
];

const columns = $derived.by<OrbisColumnDef[]>(() => {
  const cols = [...baseColumns];
  if (enableSelection) {
    cols.unshift({
      id: "__selected",
      prop: "__selected",
      header: "",
      name: "",
      width: 48,
      size: 48,
      type: "boolean",
      align: "center",
      sort: false,
    });
  }
  return cols;
});

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

let gridSource = $derived(
  (students || []).map((student, i) => {
    const rawSex = String(
      student.sex || (student as any).gender || "",
    ).toLowerCase();
    const studentName =
      student.full_name ||
      (student as any).name ||
      (student as any).name_ar ||
      (student as any).name_en ||
      m.common_word_student();
    const isSelected = selectedIds.has(String(student.id));

    return {
      ...student,
      __idx: i + 1,
      __selected: isSelected,
      full_name: studentName,
      class_name: getClassName(String(student.class_id)),
      sex_display:
        rawSex === "male" || rawSex === "ذكر"
          ? m.common_form_male()
          : rawSex === "female" || rawSex === "أنثى"
            ? m.common_form_female()
            : "-",
      enrollment_date_display: student.enrollment_date
        ? formatDate(student.enrollment_date)
        : "-",
    };
  }),
);

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "toggle-select" || event.actionId === "select") {
    const id = String(event.row?.id || "");
    if (!id) return;
    const newSet = new Set(selectedIds);
    if (newSet.has(id)) {
      newSet.delete(id);
    } else {
      newSet.add(id);
    }
    selectedIds = newSet;
    onSelectionChange?.(newSet);
  } else if (event.actionId === "dblclick") {
    if (event.row?.id) {
      goto(localizeHref(`/${orgId}/students/${event.row.id}`));
    }
  }
}
</script>

<div class="h-full w-full flex flex-col gap-3">
  <div class="flex items-center justify-end shrink-0">
    <FilterTrigger
      bind:filter={activeFilter}
      fields={studentFilterFields}
    />
  </div>

  <div class="flex-1 min-h-0">
    <OrbisGrid
      {columns}
      source={gridSource}
      height="100%"
      rowSize={44}
      rtl={getLocale() === "ar"}
      clientQuery
      filter={activeFilter}
      onaction={handleGridAction}
      emptyTitle={m.common_table_no_students()}
      emptyDescription={m.common_table_no_students_hint()}
      readonly
    />
  </div>
</div>
