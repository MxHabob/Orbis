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
import { academicContext } from "$lib/stores";
import StudentsTableSkeleton from "$lib/shared/skeletons/students-table-skeleton.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import StudentTable from "$lib/features/students/student-table.svelte";
import { toast } from "@orbis/ui";
import { goto } from "$app/navigation";
import PeriodGateAction from "$lib/shared/components/period-gate-action.svelte";
import OverrideRequestDialog from "$lib/shared/components/override-request-dialog.svelte";
import ActionDropdownMenu from "$lib/shared/components/action-dropdown-menu.svelte";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, l } from "$lib/core/i18n";
import { createStudentDirectoryQuery } from "$lib/features/students";

let { orgId }: { orgId: string } = $props();
const directory = createStudentDirectoryQuery(() => orgId);
const students = $derived(
  directory.studentsQuery.data?.pages.flatMap((page) => page.items) ?? [],
);
const classes = $derived(directory.classesQuery.data ?? []);

let showOverrideDialog = $state(false);

// Local search input with debounce
let localSearch = $state("");

let printDialogOpen = $state(false);
let selectedStudentIds = $state<Set<string>>(new Set());
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
                isActive: directory.statusFilter === 'all',
                onclick: () => directory.setStatusFilter('all')
              },
              {
                label: m.common_filter_active(),
                isActive: directory.statusFilter === 'active',
                onclick: () => directory.setStatusFilter('active')
              },
              {
                label: m.common_filter_withdrawn(),
                isActive: directory.statusFilter === 'withdrawn',
                onclick: () => directory.setStatusFilter('withdrawn')
              }
            ]
          },
          {
            title: m.common_filter_classes(),
            items: [
              {
                label: m.common_filter_all_classes(),
                isActive: directory.selectedClassId === null,
                onclick: () => directory.setClassFilter(null)
              },
              ...directory.classes.map(c => ({
                key: String(c.id),
                label: String(l(c.name_ar, c.name) || c.name || c.id),
                isActive: directory.selectedClassId === c.id,
                onclick: () => directory.setClassFilter(String(c.id))
              }))
            ]
          }
        ]}
        actions={[
          {
            label: m.common_filter_clear(),
            icon: XMark,
            onclick: directory.clearFilters,
            disabled: !directory.hasActiveFilters
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
              oninput={() => directory.updateSearch(localSearch)}
            />
          </div>

          <div class="flex items-center gap-2">
            <ActionDropdownMenu
              actions={[
                {
                  id: "bulk-input",
                  label: m.common_bulk_hub_title(),
                  icon: ArrowUpTray,
                  onSelect: () => goto(localizeHref(`/${orgId}/settings/bulk-input`)),
                },
                {
                  id: "export",
                  label: m.common_export(),
                  icon: DocumentChartBar,
                  onSelect: () => directory.exportStudents(selectedStudentIds),
                },
              ]}
            />
            {#if academicContext.effectivePeriodId}
              <PeriodGateAction
                gateType="enrollment"
                periodId={academicContext.effectivePeriodId}
                classId={directory.selectedClassId ?? undefined}
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
          {#if directory.studentsQuery.isLoading || directory.classesQuery.isLoading}
            <StudentsTableSkeleton />
          {:else if directory.error}
            <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3 bg-card border border-border rounded-lg">
              <p class="text-xs text-destructive font-semibold">{directory.errorMessage}</p>
              <Button variant="outline" class="text-xs" onclick={directory.retry}>{m.common_retry()}</Button>
            </div>
          {:else}
            <StudentTable
              {students}
              {classes}
              enableSelection={true}
              bind:selectedIds={selectedStudentIds}
              hasMore={directory.hasMore}
              onloadmore={directory.loadMoreStudents}
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
              {m.operations_ui_students_selected({ count: formatNumber(selectedStudentIds.size) })}
            </span>
            {#if selectedStudentIds.size === 1}
              {@const singleStudent = students.find(s => selectedStudentIds.has(s.student?.id || s.person?.id))}
              {#if singleStudent}
                <span class="truncate text-[11px] text-muted-foreground">
                  {(singleStudent.person?.first_name && singleStudent.person?.last_name ? `${singleStudent.person.first_name} ${singleStudent.person.last_name}` : '') || ''}
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
                <span>{m.operations_ui_student_profile()}</span>
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
    entityId={directory.selectedClassId ?? ""}
    riskLevel="medium"
    onSuccess={() => { toast.success(m.common_override_submitted()); }}
  />
</div>
