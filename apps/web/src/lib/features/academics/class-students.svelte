<!--
  @component ClassStudents
  Feature component for the Class Students view.
  Uses @orbis/svelte-grid inside a standardized Card container.
-->
<script lang="ts">
import { Button } from "@orbis/ui/button";
import { InfoCircle } from "@orbis/icons";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { extractErrorMessage } from "$lib/api";
import { createClassStudentsQuery } from "$lib/features/academics";
import { formatDate } from "$lib/core/i18n";
import {
  OrbisGrid,
  type OrbisColumnDef,
  type GridActionEvent,
} from "@orbis/svelte-grid";
import { goto } from "$app/navigation";
import { tabsStore } from "$lib/stores";
import { createClassesQuery } from "$lib/features/academics";
import StudentTransferDialog from "$lib/features/students/student-transfer-dialog.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import { confirmStore } from "$lib/stores";
import { createDeleteStudentsMutation } from "$lib/features/students";
import { toast } from "@orbis/ui";
import { formatNumber } from "$lib/core/i18n";

let { orgId, classId }: { orgId: string; classId: string } = $props();

const studentsQuery = createClassStudentsQuery(() => classId);
const classesQuery = createClassesQuery(() => orgId);
const students = $derived(studentsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
let selectedStudentIds = $state<Set<string>>(new Set());
let transferDialogOpen = $state(false);
let searchQuery = $state("");
let printDialogOpen = $state(false);
let isDeletingStudents = $state(false);
const deleteStudentsMutation = createDeleteStudentsMutation();
const isLoading = $derived(studentsQuery.isLoading);
const error = $derived(studentsQuery.error);

function studentName(student: any): string {
  return (
    student.person?.full_name_ar ||
    student.person?.full_name ||
    student.person?.display_name ||
    student.student?.student_number ||
    "-"
  );
}

const columns: OrbisColumnDef[] = [
  {
    id: "__selected",
    pin: "colPinStart",
    prop: "__selected",
    header: "",
    name: "",
    width: 48,
    size: 48,
    type: "boolean",
    align: "center",
    sort: false,
  },
  {
    id: "student_name",
    pin: "colPinStart",
    prop: "student_name",
    header: m.common_student_name(),
    name: m.common_student_name(),
    width: 260,
    size: 260,
    type: "avatar",
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "student_number",
    prop: "student_number",
    header: m.common_student_id(),
    name: m.common_student_id(),
    width: 140,
    size: 140,
    cellClass: "font-mono text-muted-foreground",
  },
  {
    id: "national_id",
    prop: "national_id",
    header: m.common_national_id(),
    name: m.common_national_id(),
    width: 150,
    size: 150,
    cellClass: "font-mono text-muted-foreground",
  },
  {
    id: "gender",
    prop: "gender",
    header: m.common_gender(),
    name: m.common_gender(),
    width: 110,
    size: 110,
    align: "center",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_status(),
    name: m.common_status(),
    width: 120,
    size: 120,
    type: "badge",
    align: "center",
  },
  {
    id: "enrolled_date",
    prop: "enrolled_date",
    header: m.common_enrolled_date(),
    name: m.common_enrolled_date(),
    width: 140,
    size: 140,
    cellClass: "font-mono text-muted-foreground",
  },
];

const gridSource = $derived(
  students.map((student) => ({
    id: student.student?.id ?? student.id,
    __selected: selectedStudentIds.has(student.student?.id ?? student.id),
    student_name: studentName(student),
    student_number: student.student?.student_number || "—",
    national_id:
      student.person?.national_id || student.person?.passport_number || "—",
    gender: student.person?.gender || "—",
    status: student.student?.status || "active",
    enrolled_date: student.enrollment?.enrollment_date
      ? formatDate(student.enrollment.enrollment_date, "date")
      : "—",
  })),
);

function handleGridAction(event: GridActionEvent) {
  if (
    event.actionId === "toggle-select" ||
    event.actionId === "select" ||
    event.actionId === "select-all"
  ) {
    const next = new Set(selectedStudentIds);
    if (event.actionId === "select-all") {
      for (const id of event.selectedIds ?? []) {
        if (event.selected) next.add(String(id));
        else next.delete(String(id));
      }
    } else if (event.row?.id) {
      const id = String(event.row.id);
      if (
        event.selected === false ||
        (event.selected === undefined && next.has(id))
      ) {
        next.delete(id);
      } else {
        next.add(id);
      }
    }
    selectedStudentIds = next;
    return;
  }
  if (event.actionId === "dblclick" && event.row.id) {
    const studentId = event.row.id;
    tabsStore.markNextNavigationAsNewTab();
    goto(localizeHref(`/${orgId}/students/${studentId}`));
  }
}

function refetch() {
  studentsQuery.refetch();
}

const selectedStudentLabels = $derived(
  students
    .filter((student) =>
      selectedStudentIds.has(student.student?.id ?? student.id),
    )
    .map(
      (student) =>
        student.person?.full_name_ar ||
        student.person?.full_name ||
        student.student?.student_number ||
        m.common_word_student(),
    ),
);

async function deleteSelectedStudents() {
  if (selectedStudentIds.size === 0 || isDeletingStudents) return;
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete(),
    description: m.common_confirm_delete_record(),
    confirmText: m.common_action_delete(),
    cancelText: m.common_action_cancel(),
    variant: "destructive",
  });
  if (!confirmed) return;

  isDeletingStudents = true;
  try {
    await deleteStudentsMutation.mutateAsync([...selectedStudentIds]);
    selectedStudentIds = new Set();
    studentsQuery.refetch();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isDeletingStudents = false;
  }
}
</script>

<div class="h-full min-h-0 w-full overflow-hidden rounded-xl border border-border/80 bg-background font-sans">
  {#if error}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-6 text-destructive">
      <InfoCircle size={24} />
      <h3 class="text-xs font-semibold">{m.common_error_occurred()}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" size="sm" class="mt-2 h-8 text-xs" onclick={refetch}>
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      height="100%"
      rtl={getLocale() === "ar"}
      readonly
      clientQuery
      bind:search={searchQuery}
      searchPlaceholder={m.operations_ui_student_search_placeholder()}
      onaction={handleGridAction}
      emptyTitle={m.common_students_empty_title()}
    />
  {/if}
  <StudentTransferDialog
    bind:open={transferDialogOpen}
    studentIds={selectedStudentIds}
    enrollments={students
      .filter((student) => selectedStudentIds.has(student.student?.id ?? student.id))
      .map((student) => student.enrollment)
      .filter((enrollment) => !!enrollment)}
    {classes}
    excludedClassId={classId}
    onsuccess={() => {
      selectedStudentIds = new Set();
      studentsQuery.refetch();
    }}
  />
  <PrintDialog
    bind:open={printDialogOpen}
    entityType="student"
    entityIds={[...selectedStudentIds]}
    entityLabels={selectedStudentLabels}
  />
  <SettingsBottomBar
    visible={selectedStudentIds.size > 0}
    isDirty={selectedStudentIds.size > 0}
    isSaving={isDeletingStudents}
    showReset={false}
    statusVariant="primary"
    message={m.operations_ui_students_selected({
      count: formatNumber(selectedStudentIds.size),
    })}
  >
    {#snippet actions()}
      {#if selectedStudentIds.size === 1}
        {@const selectedId = [...selectedStudentIds][0]}
        <Button
          variant="outline"
          size="sm"
          class="h-8 px-2.5 text-xs font-medium"
          onclick={() => {
            tabsStore.markNextNavigationAsNewTab();
            goto(localizeHref(`/${orgId}/students/${selectedId}`));
          }}
        >
          {m.operations_ui_student_profile()}
        </Button>
      {/if}
      <Button
        variant="outline"
        size="sm"
        class="h-8 px-2.5 text-xs font-medium"
        onclick={() => (transferDialogOpen = true)}
      >
        {m.common_student_layout_transfer_class()}
      </Button>
      <Button
        variant="destructive"
        size="sm"
        class="h-8 px-2.5 text-xs font-medium"
        disabled={isDeletingStudents}
        onclick={deleteSelectedStudents}
      >
        {m.common_action_delete()}
      </Button>
      <Button
        variant="lime"
        size="sm"
        class="h-8 px-3 text-xs font-semibold"
        onclick={() => (printDialogOpen = true)}
      >
        {m.common_print_templates()}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        class="h-8 px-2 text-xs font-medium text-muted-foreground"
        onclick={() => (selectedStudentIds = new Set())}
      >
        {m.common_action_cancel()}
      </Button>
    {/snippet}
  </SettingsBottomBar>
</div>
