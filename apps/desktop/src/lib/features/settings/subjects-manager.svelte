<!--
@component SubjectsManager
Lists all academic subjects with OrbisGrid, CRUD via SubjectCreateSheet.
Matches the design, layout, and UX of the Rooms manager page.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { Plus, Search, Refresh } from "@orbis/icons";
import SubjectCreateSheet from "$lib/features/academics/components/subject-create-sheet.svelte";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { subjectFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

// State
let subjects = $state<any[]>([]);
let isLoading = $state(true);
let searchQuery = $state("");
let isSheetOpen = $state(false);
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

async function loadSubjects() {
  isLoading = true;
  try {
    subjects = await api.listSubjects();
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load subjects");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadSubjects();
});

const gridSource = $derived(
  subjects.map((s, i) => ({
    ...s,
    __idx: i + 1,
    name: l(s.name_ar, s.name) || s.name || s.name_ar || "—",
    name_en: s.name || "—",
    code: s.code || "—",
    credit_hours: s.credit_hours ?? s.periods_per_week ?? 3,
    status: s.is_active !== false ? "active" : "inactive",
    created_at_display: s.created_at ? formatDate(s.created_at, "datetime") : "—",
    updated_at_display: s.updated_at ? formatDate(s.updated_at, "datetime") : "—",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "name", prop: "name", header: m.common_word_subject(), name: m.common_word_subject(), width: 220, size: 220, readonly: true, pin: "left", cellClass: "font-medium" },
  { id: "name_en", prop: "name_en", header: m.common_subject_label_name_en(), name: m.common_subject_label_name_en(), width: 180, size: 180, readonly: true },
  { id: "code", prop: "code", header: m.common_subject_label_code(), name: m.common_subject_label_code(), width: 120, size: 120, readonly: true, cellClass: "font-mono font-medium" },
  { id: "credit_hours", prop: "credit_hours", header: m.common_subject_label_periods(), name: m.common_subject_label_periods(), width: 130, size: 130, readonly: true, align: "center", type: "number" },
  { id: "status", prop: "status", header: m.common_word_status(), name: m.common_word_status(), width: 100, size: 100, type: "badge", align: "center", readonly: true },
  { id: "created_at_display", prop: "created_at_display", header: l("تاريخ الإنشاء", "Created At"), name: "Created At", width: 130, size: 130, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "updated_at_display", prop: "updated_at_display", header: l("آخر تحديث", "Updated At"), name: "Updated At", width: 130, size: 130, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
];

function openCreateSheet() {
  isSheetOpen = true;
}

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "dblclick" || event.actionId === "edit") {
    isSheetOpen = true;
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide font-sans">
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <div class="relative flex-1 max-w-xs">
      <Search
        size={14}
        class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
      />
      <Input
        type="text"
        placeholder={m.settings_subjects_search_placeholder_custom()}
        bind:value={searchQuery}
        class="h-8 ps-8 text-xs bg-card border-border/60"
      />
    </div>

    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={subjectFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadSubjects} class="text-muted-foreground">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button variant="lime" class="text-xs h-8 px-4 font-semibold shrink-0" onclick={openCreateSheet}>
        <Plus size={14} class="me-1.5" />
        {m.common_exams_subjects_add()}
      </Button>
    </div>
  </div>

  <div class="flex-1 min-h-0">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      search={searchQuery}
      filter={activeFilter}
      emptyTitle={m.common_subjects_empty_title()}
      emptyDescription={m.common_subjects_empty_desc_general()}
      rtl={getLocale() === "ar"}
      readonly
      onaction={handleGridAction}
    />
  </div>
</div>

<SubjectCreateSheet
  bind:open={isSheetOpen}
  {orgId}
  onSuccess={loadSubjects}
/>
