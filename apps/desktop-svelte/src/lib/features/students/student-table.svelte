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
import { onDestroy } from "svelte";

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

const columns = $derived.by<OrbisColumnDef[]>(() => [
  {
    prop: "full_name",
    name: m.table_student(),
    size: 320,
    pin: "colPinStart",
    cellRenderer: ({ h, row, rowIndex }) => {
      const name = String(row.full_name || row.student_name || row.name_ar || "");
      const src = row.avatar as string | undefined;
      const initials = createInitials(name);

      const innerEl = src
        ? h("img", {
            class: "orbis-r-avatar__img",
            src,
            alt: name,
            loading: "lazy",
          })
        : h("span", { class: "orbis-r-avatar__fallback-text" }, initials);

      const avatarEl = h(
        "span",
        {
          class: `orbis-r-avatar__root orbis-r-avatar--md ${
            src
              ? "orbis-r-avatar__root--has-img"
              : "orbis-r-avatar__root--has-fallback"
          }`,
        },
        [innerEl],
      );

      const nameLines = [
        h(
          "span",
          { class: "text-xs font-bold text-foreground truncate block" },
          name,
        ),
      ];

      const textContainer = h(
          "div",
          { class: "flex flex-col text-start min-w-0" },
          nameLines,
        );

      return h(
        "div",
        {
          class: "flex items-center gap-3 cursor-pointer w-full h-full px-4 text-start",
          "data-grid-action": "select",
          "data-row-index": String(rowIndex),
        },
        [avatarEl, textContainer],
      );
    },
  },
  {
    prop: "student_number",
    name: m.table_student_number(),
    size: 150,
    cellRenderer: ({ h, row }) => {
      const num = String(row.student_number || row.id || "-");
      return h(
        "span",
        {
          class:
            "inline-flex items-center rounded-md border border-border px-2 py-0.5 text-[10px] font-mono font-medium text-foreground bg-background",
        },
        num,
      );
    },
  },
  {
    prop: "class_name",
    name: m.table_class(),
    size: 150,
  },
  {
    prop: "status",
    name: m.table_status(),
    size: 120,
    cellRenderer: ({ h, row }) => {
      const status = String(row.status || "").toLowerCase();
      if (status === "active") {
        return h(
          "span",
          { class: "badge-brand-success py-0.5 px-2 text-[10px]" },
          m.status_active(),
        );
      } else if (status === "withdrawn") {
        return h(
          "span",
          { class: "badge-brand-destructive py-0.5 px-2 text-[10px]" },
          m.status_withdrawn(),
        );
      } else {
        return h(
          "span",
          { class: "badge-brand-default py-0.5 px-2 text-[10px]" },
          String(row.status || m.status_undefined()),
        );
      }
    },
  },
  {
    prop: "sex",
    name: m.form_sex(),
    size: 100,
    cellRenderer: ({ h, row }) => {
      const sex = String(row.sex || "").toLowerCase();
      let label = String(row.sex || "-");
      if (sex === "male") label = m.form_male();
      else if (sex === "female") label = m.form_female();
      return h("span", { class: "capitalize text-xs" }, label);
    },
  },
  {
    prop: "enrollment_date",
    name: m.table_enrolled_at(),
    size: 150,
    cellRenderer: ({ h, row }) => {
      const date = row.enrollment_date as string | undefined;
      const label = date ? formatDate(date) : "-";
      return h("span", { class: "text-xs text-muted-foreground" }, label);
    },
  },
]);

let gridSource = $derived(
  (students || []).map((student, i) => ({
    ...student,
    __idx: i + 1,
    class_name: getClassName(String(student.class_id)),
    __selected: selectedIds.has(String(student.id)),
  })),
);

let activeColumns = $derived.by(() => {
  const cols = [...columns];
  if (enableSelection) {
    cols.unshift({
      prop: "__selected",
      name: "",
      size: 40,
      readonly: true,
      pin: "colPinStart",
      cellProperties: (row) => ({
        class: row.__selected ? "orbis-cell-selected" : "orbis-cell-checkbox",
      }),
      cellRenderer: ({ h, row, rowIndex }) => {
        const isSelected = !!row.__selected;
        return h(
          "span",
          {
            class: `cursor-pointer block w-full h-full flex items-center justify-center`,
            "data-grid-action": "toggle-select",
            "data-row-index": String(rowIndex),
          },
          [
            h("input", {
              type: "checkbox",
              checked: isSelected,
              class: "cursor-pointer",
              onclick: (e: any) => e.preventDefault(),
            }),
          ],
        );
      },
    });
  }
  return cols;
});

let clickTimeout: ReturnType<typeof setTimeout> | null = null;

onDestroy(() => {
  if (clickTimeout) {
    clearTimeout(clickTimeout);
  }
});

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "select") {
    if (clickTimeout) {
      clearTimeout(clickTimeout);
      clickTimeout = null;
    }
    clickTimeout = setTimeout(() => {
      studentsStore.selectedId = String(event.row.id);
      clickTimeout = null;
    }, 220);
  } else if (event.actionId === "dblclick") {
    if (clickTimeout) {
      clearTimeout(clickTimeout);
      clickTimeout = null;
    }
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
