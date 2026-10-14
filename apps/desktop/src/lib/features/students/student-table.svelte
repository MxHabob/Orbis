<!--
@component StudentTable
Renders the list of students using OrbisGrid with exact backend contract alignment.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { StudentDetailDto, EnrollmentDto, ClassDto } from "$lib/api";
import { formatDate, l } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { studentFilterFields } from "$lib/core/filter-configs";

let {
  students = [],
  enrollments = [],
  classes = [],
  enableSelection = true,
  selectedIds = $bindable(new Set()),
  onSelectionChange,
  hasMore = false,
  onloadmore,
  orgId,
}: {
  students: StudentDetailDto[];
  enrollments?: EnrollmentDto[];
  classes?: ClassDto[];
  enableSelection?: boolean;
  selectedIds?: Set<string>;
  onSelectionChange?: (ids: Set<string>) => void;
  hasMore?: boolean;
  onloadmore?: () => void | Promise<unknown>;
  orgId: string;
} = $props();

// Map student ID to active enrollment and class
const studentClassMap = $derived.by(() => {
  const map: Record<string, string> = {};
  const classLookup = new Map(classes.map(c => [c.id, l(c.name_ar, c.name) || c.name]));
  for (const e of enrollments) {
    if (e.section_id && (e.status === "active" || e.status === "pending_approval")) {
      const clsName = classLookup.get(e.section_id) || e.section_id;
      map[e.student_id] = clsName;
    }
  }
  return map;
});

// ── Column Definitions ─────────────────────────────────────────────
const baseColumns: OrbisColumnDef[] = [
  {
    id: "full_name",
    prop: "full_name",
    header: m.common_word_student(),
    name: m.common_word_student(),
    width: 240,
    size: 240,
    type: "avatar",
    readonly: true,
    pin: "left",
  },
  {
    id: "student_number",
    prop: "student_number",
    header: m.common_table_student_number(),
    name: m.common_table_student_number(),
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "font-mono font-medium text-foreground",
  },
  {
    id: "national_id",
    prop: "national_id",
    header: m.ui_copy_national_id_passport_f63e7620(),
    name: "National ID",
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "font-mono text-muted-foreground",
  },
  {
    id: "class_name",
    prop: "class_name",
    header: m.common_table_class(),
    name: m.common_table_class(),
    width: 150,
    size: 150,
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
    id: "standing",
    prop: "standing",
    header: m.ui_copy_academic_standing_04c8e5f1(),
    name: "Standing",
    width: 140,
    size: 140,
    type: "badge",
    align: "center",
    readonly: true,
  },
  {
    id: "gender_display",
    prop: "gender_display",
    header: m.common_form_sex(),
    name: m.common_form_sex(),
    width: 100,
    size: 100,
    readonly: true,
    align: "center",
  },
  {
    id: "admitted_at_display",
    prop: "admitted_at_display",
    header: m.common_table_enrolled_at(),
    name: m.common_table_enrolled_at(),
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "text-xs font-mono text-muted-foreground",
  },
  {
    id: "created_at_display",
    prop: "created_at_display",
    header: m.ui_copy_created_at_01f6ea5f(),
    name: "Created At",
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "text-xs font-mono text-muted-foreground",
  },
  {
    id: "updated_at_display",
    prop: "updated_at_display",
    header: m.ui_copy_updated_at_ba0d0a9f(),
    name: "Updated At",
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "text-xs font-mono text-muted-foreground",
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

function formatStanding(standing: string): string {
  switch (standing) {
    case "good_standing": return m.ui_copy_good_standing_97d50066();
    case "honor_list": return m.ui_copy_honor_list_2fbe9f9c();
    case "warning_1": return m.ui_copy_warning_1_d56eb3db();
    case "warning_2": return m.ui_copy_warning_2_7d80f6e1();
    case "warning_3": return m.ui_copy_warning_3_9d03bb80();
    case "probation": return m.ui_copy_probation_fe016d19();
    case "dismissed": return m.ui_copy_dismissed_99b10f56();
    default: return standing || "—";
  }
}

let gridSource = $derived(
  (students || []).map((detail, i) => {
    const s = detail.student;
    const p = detail.person;
    const studentName = l(p.full_name_ar, p.full_name) || p.first_name || m.common_word_student();
    const isSelected = selectedIds.has(String(s.id));
    const className = studentClassMap[s.id] || m.common_status_not_enrolled();

    const rawGender = String(p.gender || "").toLowerCase();
    const genderDisplay = rawGender === "male" || rawGender === "ذكر"
      ? m.common_form_male()
      : rawGender === "female" || rawGender === "أنثى"
        ? m.common_form_female()
        : "—";

    return {
      id: s.id,
      __idx: i + 1,
      __selected: isSelected,
      full_name: studentName,
      student_number: s.student_number || "—",
      national_id: p.national_id || p.passport_number || "—",
      class_name: className,
      status: s.status || "active",
      standing: formatStanding(s.standing),
      gender_display: genderDisplay,
      admitted_at_display: s.admitted_at ? formatDate(s.admitted_at) : "—",
      created_at_display: s.created_at ? formatDate(s.created_at, "datetime") : "—",
      updated_at_display: s.updated_at ? formatDate(s.updated_at, "datetime") : "—",
      avatar_url: p.avatar_url,
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
      {hasMore}
      {onloadmore}
      emptyTitle={m.common_table_no_students()}
      emptyDescription={m.common_table_no_students_hint()}
      readonly
    />
  </div>
</div>
