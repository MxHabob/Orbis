<!--
  @component StudentDirectory
  Main workspace for students directory. Integrates filters, table, and batch actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import MainViewport from "$lib/shell/main-viewport.svelte";
import { Button } from "@orbis/ui/button";
import { ArrowUpTray, DocumentChartBar, Plus, XMark } from "@orbis/icons";
import { academicContext } from "$lib/stores";
import StudentsTableSkeleton from "$lib/shared/skeletons/students-table-skeleton.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import StudentTable from "$lib/features/students/student-table.svelte";
import StudentTransferDialog from "$lib/features/students/student-transfer-dialog.svelte";
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
let transferDialogOpen = $state(false);
let selectedStudentIds = $state<Set<string>>(new Set());
const selectedStudent = $derived(
  selectedStudentIds.size === 1
    ? students.find((student) =>
        selectedStudentIds.has(student.student?.id || student.person?.id),
      )
    : undefined,
);
const selectedStudentLabels = $derived(
  [...selectedStudentIds].map((id) => {
    const row = students.find(
      (student) => (student.student?.id || student.person?.id) === id,
    );
    return (
      l(row?.person?.full_name_ar, row?.person?.full_name) ||
      row?.person?.first_name ||
      row?.student?.student_number ||
      m.operations_ui_student_fallback()
    );
  }),
);

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
      >
        {#snippet headerAction()}
          <Button
            variant="ghost"
            size="icon"
            aria-label={m.common_filter_clear()}
            title={m.common_filter_clear()}
            onclick={() => directory.clearFilters()}
            disabled={!directory.hasActiveFilters}
          >
            <XMark size={14} />
          </Button>
        {/snippet}
      </SidebarNav>
    {/snippet}

    <!-- Middle Panel: Directory datagrid -->
    {#snippet middle()}
      <div class="h-full flex flex-col p-6 overflow-hidden gap-4">
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
            >
              {#snippet extraToolbarActions()}
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
              {/snippet}
            </StudentTable>
          {/if}
        </div>
      </div>
    {/snippet}
  </MainViewport>

  <SettingsBottomBar
    visible={selectedStudentIds.size > 0}
    isDirty={selectedStudentIds.size > 0}
    isSaving={isDeletingStudents}
    showReset={false}
    statusVariant="primary"
    message={m.operations_ui_students_selected({ count: formatNumber(selectedStudentIds.size) })}
  >
    {#snippet actions()}
      {#if selectedStudent?.student?.id}
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="h-8 px-2.5 text-xs font-medium"
          onclick={() => goto(localizeHref(`/${orgId}/students/${selectedStudent.student?.id}`))}
        >
          {m.operations_ui_student_profile()}
        </Button>
      {/if}

      <Button
        type="button"
        variant="outline"
        size="sm"
        class="h-8 px-2.5 text-xs font-medium"
        onclick={() => (transferDialogOpen = true)}
      >
        {m.common_student_layout_transfer_class()}
      </Button>

      <Button
        type="button"
        variant="destructive"
        size="sm"
        class="h-8 px-2.5 text-xs font-medium"
        disabled={isDeletingStudents}
        onclick={deleteSelectedStudents}
      >
        {m.common_action_delete()}
      </Button>

      <Button
        type="button"
        variant="lime"
        size="sm"
        class="h-8 px-3 text-xs font-semibold"
        onclick={() => printDialogOpen = true}
      >
        {m.common_print_templates()}
      </Button>

      <Button
        type="button"
        variant="ghost"
        size="sm"
        class="h-8 px-2 text-xs font-medium text-muted-foreground hover:text-foreground"
        onclick={() => selectedStudentIds = new Set()}
      >
        {m.common_action_cancel()}
      </Button>
    {/snippet}
  </SettingsBottomBar>

  <StudentTransferDialog
    bind:open={transferDialogOpen}
    studentIds={selectedStudentIds}
    enrollments={directory.enrollmentsQuery.data ?? []}
    {classes}
    onsuccess={() => {
      selectedStudentIds = new Set();
      directory.retry();
    }}
  />

  <PrintDialog
    bind:open={printDialogOpen}
    entityType="student"
    entityIds={Array.from(selectedStudentIds)}
    entityLabels={selectedStudentLabels}
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
