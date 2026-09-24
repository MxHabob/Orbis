<!--
  @component ClassStudents
  Feature component for the Class Students view.
  Uses @orbis/svelte-grid for scalable, high-performance rendering.
-->
<script lang="ts">
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { InfoCircle, Refresh, Users } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { extractErrorMessage } from "$lib/api";
import { createClassStudentsQuery } from "$lib/features/academics";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { goto } from "$app/navigation";

let { orgId, classId }: { orgId: string; classId: string } = $props();

const studentsQuery = createClassStudentsQuery(() => classId);
const students = $derived(studentsQuery.data ?? []);
const isLoading = $derived(studentsQuery.isLoading);
const error = $derived(studentsQuery.error);

function studentName(student: any): string {
  return (
    student.name_ar ||
    student.full_name ||
    student.name ||
    student.student_id ||
    "-"
  );
}

const columns: OrbisColumnDef[] = [
  {
    id: "student_name",
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
    ...student,
    student_name: studentName(student),
    student_number: student.student_number || student.student_id || student.id,
    national_id: student.national_id || "—",
    gender: student.gender || "—",
    status: student.status || "active",
    enrolled_date: student.created_at
      ? new Date(student.created_at).toLocaleDateString()
      : "—",
  })),
);

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "dblclick" && event.row.id) {
    const studentId = event.row.student_id || event.row.id;
    goto(localizeHref(`/${orgId}/students/${studentId}`));
  }
}

function refetch() {
  studentsQuery.refetch();
}
</script>

<div class="flex flex-col gap-4 text-start h-full font-sans">
  <div class="flex items-start justify-between gap-3 border-b border-border/60 pb-3">
    <div>
      <div class="flex items-center gap-2 mb-1.5">
        <a
          href={localizeHref(`/${orgId}/academics/classes/${classId}`)}
          class="flex items-center gap-1 text-[11px] font-bold text-muted-foreground hover:text-primary transition-colors"
        >
          <span>&larr;</span>
          <span>{m.common_nav_dashboard()}</span>
        </a>
      </div>
      <div class="flex items-center gap-2">
        <Users size={17} class="text-primary" />
        <h2 class="text-sm font-bold">
          {m.academics_class_detail_tab_students()}
        </h2>
        <Badge variant="secondary" class="text-[9px] py-0 px-1.5 h-4.5 rounded font-sans ml-2">
          {students.length}
        </Badge>
      </div>
      <p class="mt-1 text-[10px] text-muted-foreground">
        {m.academics_class_students_desc()}
      </p>
    </div>
    <div class="flex gap-2">
      <Button variant="outline" size="sm" class="h-8 gap-1.5 text-xs font-semibold" onclick={refetch}>
        <Refresh size={12} class={isLoading ? "animate-spin" : ""} />
        {m.common_retry()}
      </Button>
    </div>
  </div>

  {#if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2 font-sans">
      <InfoCircle size={24} />
      <h3 class="font-semibold text-xs">{m.common_error_occurred()}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" size="sm" class="h-8 mt-2 text-xs" onclick={refetch}>
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <div class="flex-1 w-full overflow-hidden border border-border/70 rounded-xl bg-card">
      <OrbisGrid
        {columns}
        source={gridSource}
        loading={isLoading}
        height="100%"
        rowSize={44}
        readonly
        onaction={handleGridAction}
        emptyTitle={m.common_students_empty_title()}
      />
    </div>
  {/if}
</div>
