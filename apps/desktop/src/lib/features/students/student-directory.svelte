<!--
  @component StudentDirectory
  Main workspace for students directory. Integrates filters, table, and batch actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import MainViewport from "$lib/shell/main-viewport.svelte";
import { Button } from "@orbis/ui/button";
import {
  ArrowForward,
  ArrowUpTray,
  BookOpen,
  DocumentChartBar,
  Plus,
  Printer,
  XMark,
} from "@orbis/icons";
import { fly } from "svelte/transition";
import { quintOut } from "svelte/easing";
import { academicContext } from "$lib/stores";
import StudentsTableSkeleton from "$lib/shared/skeletons/students-table-skeleton.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import StudentTable from "$lib/features/students/student-table.svelte";
import { toast } from "@orbis/ui";
import { confirmStore } from "$lib/stores";
import { extractErrorMessage } from "$lib/api";
import { goto } from "$app/navigation";
import PeriodGateAction from "$lib/shared/components/period-gate-action.svelte";
import OverrideRequestDialog from "$lib/shared/components/override-request-dialog.svelte";
import SplitActionMenu from "$lib/shared/components/split-action-menu.svelte";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, l } from "$lib/core/i18n";
import {
  createDeleteStudentsMutation,
  createStudentDirectoryQuery,
} from "$lib/features/students";

let { orgId }: { orgId: string } = $props();
const directory = createStudentDirectoryQuery(() => orgId);
const deleteStudentsMutation = createDeleteStudentsMutation();
const students = $derived(
  directory.studentsQuery.data?.pages.flatMap((page) => page.items) ?? [],
);
const classes = $derived(directory.classesQuery.data ?? []);

let showOverrideDialog = $state(false);

let printDialogOpen = $state(false);
let isDeletingStudents = $state(false);
let selectedStudentIds = $state<Set<string>>(new Set());

async function deleteSelectedStudents() {
  const studentIds = [...selectedStudentIds];
  if (studentIds.length === 0 || isDeletingStudents) return;

  isDeletingStudents = true;
  try {
    const confirmed = await confirmStore.prompt({
      title: m.common_action_delete(),
      description: m.common_confirm_delete_record(),
      confirmText: m.common_action_delete(),
      cancelText: m.common_action_cancel(),
      variant: "destructive",
    });
    if (!confirmed) return;

    const results = await deleteStudentsMutation.mutateAsync(studentIds);
    const failedIds = new Set<string>();
    let firstError: unknown;
    for (let index = 0; index < results.length; index++) {
      const result = results[index]!;
      if (result.status === "fulfilled" && result.value) continue;
      failedIds.add(studentIds[index]!);
      if (result.status === "rejected" && firstError === undefined) {
        firstError = result.reason;
      }
    }

    selectedStudentIds = failedIds;
    if (failedIds.size > 0) {
      toast.error(
        firstError === undefined
          ? "Some selected students could not be deleted."
          : extractErrorMessage(firstError),
      );
    }
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isDeletingStudents = false;
  }
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
                isActive: directory.statusFilter === 'all',
                onclick: () => directory.setStatusFilter('all')
              },
              {
                label: m.common_filter_active(),
                isActive: directory.statusFilter === 'active',
                onclick: () => directory.setStatusFilter('active')
              },
              {
                label: m.operations_ui_inactive(),
                isActive: directory.statusFilter === 'inactive',
                onclick: () => directory.setStatusFilter('inactive')
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
              ...classes.map(c => ({
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
        <div class="flex items-center justify-end shrink-0">
          <div class="flex items-center gap-2">
            <SplitActionMenu
              label={m.common_new()}
              menuLabel={m.operations_ui_actions()}
              icon={Plus}
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
              onprimary={() => goto(localizeHref(`/${orgId}/students/new`))}
            >
              {#snippet primary()}
                {#if academicContext.effectivePeriodId}
                  <PeriodGateAction
                    gateType="enrollment"
                    periodId={academicContext.effectivePeriodId}
                    classId={directory.selectedClassId ?? undefined}
                    label={m.common_new()}
                    size="sm"
                    onAction={() => goto(localizeHref(`/${orgId}/students/new`))}
                    onDraft={() => goto(localizeHref(`/${orgId}/students/new?draft=true`))}
                    onOverride={() => { showOverrideDialog = true; }}
                  >
                    {#snippet allowed()}
                      <Button
                        size="sm"
                        class="rounded-e-none gap-1.5"
                        onclick={() => goto(localizeHref(`/${orgId}/students/new`))}
                      >
                        <Plus class="size-4" />
                        <span>{m.common_new()}</span>
                      </Button>
                    {/snippet}
                  </PeriodGateAction>
                {:else}
                  <Button
                    size="sm"
                    class="rounded-e-none gap-1.5"
                    onclick={() => goto(localizeHref(`/${orgId}/students/new`))}
                  >
                    <Plus class="size-4" />
                    <span>{m.common_new()}</span>
                  </Button>
                {/if}
              {/snippet}
            </SplitActionMenu>
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
              enrollments={directory.enrollmentsQuery.data ?? []}
              {classes}
              enableSelection={true}
              bind:selectedIds={selectedStudentIds}
              search={directory.search}
              onsearchchange={directory.updateSearch}
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
      class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-[calc(100%-2rem)] max-w-2xl pointer-events-auto select-none"
      transition:fly={{ y: 35, duration: 250, easing: quintOut }}
    >
      <div class="bg-card/95 backdrop-blur-md border border-border shadow-2xl rounded-xl px-4 py-2.5 flex flex-wrap items-center justify-between gap-3 ring-1 ring-black/5 dark:ring-white/10">
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
        <div class="flex min-w-0 flex-wrap items-center justify-end gap-2">
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
            variant="destructive"
            size="sm"
            class="h-8 px-2.5 text-xs font-medium cursor-pointer gap-1.5"
            disabled={isDeletingStudents}
            onclick={deleteSelectedStudents}
          >
            {m.common_action_delete()} ({selectedStudentIds.size})
          </Button>

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
