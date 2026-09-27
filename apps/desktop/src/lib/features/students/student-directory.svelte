<!--
  @component StudentDirectory
  Main workspace for students directory. Integrates filters, table, and batch actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import MainViewport from "$lib/shell/main-viewport.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { ArrowForward, ArrowUpTray, BookOpen, DocumentChartBar, Plus, Printer, Search, XMark } from "@orbis/icons";
import { fly } from "svelte/transition";
import { quintOut } from "svelte/easing";
import { studentsStore, createEnrollmentsQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import { academicContext } from "$lib/stores";
import StudentsTableSkeleton from "$lib/shared/skeletons/students-table-skeleton.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import StudentTable from "$lib/features/students/student-table.svelte";
import { toast } from "@orbis/ui";
import { goto } from "$app/navigation";
import PeriodGateAction from "$lib/shared/components/period-gate-action.svelte";
import OverrideRequestDialog from "$lib/shared/components/override-request-dialog.svelte";

import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, formatDate, l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import { debounce } from "$lib/shared/utils/debounce";
import { exportToCsv } from "$lib/shared/utils/export";

let { orgId }: { orgId: string } = $props();

// TanStack Queries
const studentsQuery = createStudentsQuery(() => orgId);
const classesQuery = createClassesQuery(() => orgId);

// Derived data from queries
const students = $derived(studentsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const filteredStudents = $derived(studentsStore.filterList(students));

let showOverrideDialog = $state(false);
const isLoading = $derived(
  studentsQuery.isLoading || classesQuery.isLoading,
);
const error = $derived(studentsQuery.error || classesQuery.error);

// Helper to count students matching a criteria
function getCountByClass(classId: string): number {
  return students.filter((s) => s.enrollment?.class_id === classId).length;
}

function getCountByStatus(status: string): number {
  if (status === "all") return students.length;
  return students.filter(
    (s) => String(s.student?.status || "").toLowerCase() === status,
  ).length;
}

// Local search input with debounce
let localSearch = $state("");
const updateSearch = debounce((q: string) => {
  studentsStore.searchQuery = q;
}, 150);

// Clear active filters
function clearFilters() {
  studentsStore.selectedClassId = null;
  studentsStore.statusFilter = "active";
  studentsStore.searchQuery = "";
}

const hasActiveFilters = $derived(
  studentsStore.selectedClassId !== null ||
    studentsStore.statusFilter !== "active" ||
    studentsStore.searchQuery !== "",
);

let printDialogOpen = $state(false);
let selectedStudentIds = $state<Set<string>>(new Set());

function handleExportStudents() {
  exportToCsv(
    `Students_Roster_${orgId}`,
    filteredStudents,
    [
      { header: "رقم الطالب", accessor: (s) => s.student?.student_number || "—" },
      {
        header: "اسم الطالب",
        accessor: (s) =>
          (s.person?.first_name && s.person?.last_name
            ? `${s.person.first_name} ${s.person.last_name}`
            : "") ||
          s.person?.name_ar ||
          s.person?.name_en ||
          s.person?.full_name ||
          "طالب",
      },
      {
        header: "الفصل",
        accessor: (s) =>
          classes.find((c) => c.id === s.enrollment?.class_id)?.name ||
          s.enrollment?.class_id ||
          "—",
      },
      { header: "الحالة", accessor: (s) => s.student?.status || "active" },
      {
        header: "تاريخ القبول",
        accessor: (s) =>
          s.student?.admitted_at ? formatDate(s.student.admitted_at, "date") : "—",
      },
    ],
  );
  toast.success("تم تصدير قائمة الطلاب بنجاح (CSV)");
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <MainViewport
    name="students-workspace"
    leftWidth={{ default: 240, min: 200 }}
  >
    <!-- Left Panel: Sidebar Filters -->
    {#snippet left()}
      <SidebarNav
        title={m.students_filter_panel()}
        groups={[
          {
            title: m.common_filter_academic_status(),
            items: [
              {
                label: m.common_filter_all(),
                badge: formatNumber(getCountByStatus('all')),
                isActive: studentsStore.statusFilter === 'all',
                onclick: () => studentsStore.statusFilter = 'all'
              },
              {
                label: m.common_filter_active(),
                badge: formatNumber(getCountByStatus('active')),
                isActive: studentsStore.statusFilter === 'active',
                onclick: () => studentsStore.statusFilter = 'active'
              },
              {
                label: m.common_filter_withdrawn(),
                badge: formatNumber(getCountByStatus('withdrawn')),
                isActive: studentsStore.statusFilter === 'withdrawn',
                onclick: () => studentsStore.statusFilter = 'withdrawn'
              }
            ]
          },
          {
            title: m.common_filter_classes(),
            items: [
              {
                label: m.common_filter_all_classes(),
                badge: formatNumber(students.length),
                isActive: studentsStore.selectedClassId === null,
                onclick: () => studentsStore.selectedClassId = null
              },
              ...classes.map(c => ({
                label: String(l(c.name_ar, c.name) || c.name || c.id),
                badge: formatNumber(getCountByClass(String(c.id))),
                isActive: studentsStore.selectedClassId === c.id,
                onclick: () => studentsStore.selectedClassId = String(c.id)
              }))
            ]
          }
        ]}
        actions={[
          {
            label: m.common_filter_clear(),
            icon: XMark,
            onclick: clearFilters,
            disabled: !hasActiveFilters
          }
        ]}
      />
    {/snippet}

    <!-- Middle Panel: Directory datagrid -->
    {#snippet middle()}
      <div class="h-full flex flex-col p-6 overflow-hidden gap-4">
        <div class="flex items-center justify-between shrink-0">
          <div class="relative w-full max-w-sm shrink-0">
            <Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="text"
              placeholder={m.students_search_placeholder()}
              class="input-sunken ps-9 text-xs py-2"
              bind:value={localSearch}
              oninput={() => updateSearch(localSearch)}
            />
          </div>

          <div class="flex items-center gap-2">
            <Button
              variant="outline"
              class="text-xs h-9 px-3"
              onclick={() => goto(localizeHref(`/${orgId}/settings/bulk-input`))}
            >
              <ArrowUpTray size={13} class="me-1.5" />
              {m.common_bulk_hub_title()}
            </Button>
            <Button
              variant="outline"
              class="text-xs h-9 px-3"
              onclick={handleExportStudents}
            >
              <DocumentChartBar size={13} class="me-1.5" />
              {m.common_export()}
            </Button>
            {#if academicContext.effectivePeriodId}
              <PeriodGateAction
                gateType="enrollment"
                periodId={academicContext.effectivePeriodId}
                classId={studentsStore.selectedClassId ?? undefined}
                label={m.common_new()}
                size="lg"
                onAction={() => goto(localizeHref(`/${orgId}/students/new`))}
                onDraft={() => goto(localizeHref(`/${orgId}/students/new?draft=true`))}
                onOverride={() => { showOverrideDialog = true; }}
              />
            {:else}
              <Button 
                variant="lime" 
                class="text-xs h-9 px-3" 
                onclick={() => goto(localizeHref(`/${orgId}/students/new`))}
              >
                <Plus size={14} class="me-1.5" />
                {m.common_new()}
              </Button>
            {/if}
          </div>
        </div>

        <!-- Student Grid -->
        <div class="flex-1 min-h-0 flex flex-col">
          {#if isLoading}
            <StudentsTableSkeleton />
          {:else if error}
            <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3 bg-card border border-border rounded-lg">
              <p class="text-xs text-destructive font-semibold">{m.students_load_error({ error: extractErrorMessage(error) })}</p>
              <Button variant="outline" class="text-xs" onclick={() => { studentsQuery.refetch(); classesQuery.refetch(); }}>{m.common_retry()}</Button>
            </div>
          {:else}
            <StudentTable
              students={filteredStudents}
              classes={classes}
              enableSelection={true}
              bind:selectedIds={selectedStudentIds}
              {orgId}
            />
          {/if}
        </div>
      </div>
    {/snippet}
  </MainViewport>

  <!-- Floating Bottom Action Bar for Selected Students -->
  {#if selectedStudentIds.size > 0}
    <div
      class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-[calc(100%-2rem)] max-w-lg pointer-events-auto select-none"
      transition:fly={{ y: 35, duration: 250, easing: quintOut }}
    >
      <div class="bg-card/95 backdrop-blur-md border border-border shadow-2xl rounded-xl px-4 py-2.5 flex items-center justify-between gap-3 ring-1 ring-black/5 dark:ring-white/10">
        <!-- Selection Counter & Info -->
        <div class="flex min-w-0 items-center gap-2.5 text-start">
          <span class="flex size-7 shrink-0 items-center justify-center rounded-full bg-primary/15 text-primary">
            <BookOpen size={15} strokeWidth={2.5} />
          </span>
          <div class="flex flex-col min-w-0">
            <span class="truncate text-xs font-bold text-foreground">
              {formatNumber(selectedStudentIds.size)} {selectedStudentIds.size === 1 ? m.common_word_student() : 'طلاب'} محدد
            </span>
            {#if selectedStudentIds.size === 1}
              {@const singleStudent = students.find(s => selectedStudentIds.has(s.student?.id || s.person?.id))}
              {#if singleStudent}
                <span class="truncate text-[11px] text-muted-foreground">
                  {(singleStudent.person?.first_name && singleStudent.person?.last_name ? `${singleStudent.person.first_name} ${singleStudent.person.last_name}` : '') || singleStudent.person?.name_ar || singleStudent.person?.name_en || ''}
                </span>
              {/if}
            {/if}
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex shrink-0 items-center gap-2">
          {#if selectedStudentIds.size === 1}
            {@const singleStudent = students.find(s => selectedStudentIds.has(s.student?.id || s.person?.id))}
            {#if singleStudent}
              <Button
                type="button"
                variant="outline"
                size="sm"
                class="h-8 px-2.5 text-xs font-medium cursor-pointer gap-1.5"
                onclick={() => goto(localizeHref(`/${orgId}/students/${singleStudent.student?.id}`))}
              >
                <ArrowForward size={13} />
                <span>الملف الشخصي</span>
              </Button>
            {/if}
          {/if}

          <Button
            type="button"
            variant="lime"
            size="sm"
            class="h-8 px-3 text-xs font-semibold shadow-xs cursor-pointer gap-1.5"
            onclick={() => printDialogOpen = true}
          >
            <Printer size={14} />
            <span>{m.common_print_templates()} ({selectedStudentIds.size})</span>
          </Button>

          <Button
            type="button"
            variant="ghost"
            size="sm"
            class="h-8 px-2 text-xs font-medium text-muted-foreground hover:text-foreground cursor-pointer"
            onclick={() => selectedStudentIds = new Set()}
          >
            {m.common_action_cancel()}
          </Button>
        </div>
      </div>
    </div>
  {/if}

  <PrintDialog
    bind:open={printDialogOpen}
    entityType="student"
    entityIds={Array.from(selectedStudentIds)}
  />

  <OverrideRequestDialog
    bind:open={showOverrideDialog}
    {orgId}
    overrideType="late_enrollment"
    entityType="enrollment"
    entityId={studentsStore.selectedClassId ?? ""}
    riskLevel="medium"
    onSuccess={() => { toast.success(m.common_override_submitted()); }}
  />
</div>
