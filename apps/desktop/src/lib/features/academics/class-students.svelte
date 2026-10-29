<!--
  @component ClassStudents
  Feature component for the Class Students view.
  Uses @orbis/svelte-grid inside a standardized Card container.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { InfoCircle, Refresh, Users } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
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

let { orgId, classId }: { orgId: string; classId: string } = $props();

const studentsQuery = createClassStudentsQuery(() => classId);
const students = $derived(studentsQuery.data ?? []);
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
    id: student.student?.id ?? student.id,
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
  if (event.actionId === "dblclick" && event.row.id) {
    const studentId = event.row.id;
    goto(localizeHref(`/${orgId}/students/${studentId}`));
  }
}

function refetch() {
  studentsQuery.refetch();
}
</script>

<div class="flex flex-col gap-4 text-start h-full font-sans">
  <Card class="flex-1 flex flex-col min-h-0 overflow-hidden">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary shrink-0">
          <Users class="w-4 h-4" />
        </div>
        <div class="flex flex-col text-start">
          <div class="flex items-center gap-2">
            <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
              {m.academics_class_detail_tab_students()}
            </CardTitle>
            <Badge variant="secondary" class="text-[9px] py-0 px-1.5 h-4.5 rounded font-mono">
              {students.length}
            </Badge>
          </div>
          <CardDescription class="text-[10px]">
            {m.academics_class_students_desc()}
          </CardDescription>
        </div>
      </div>

      <CardAction>
        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            class="h-8 gap-1.5 text-xs font-semibold"
            onclick={refetch}
          >
            {#if isLoading}<LoadingIndicator  size={12} />{:else}<Refresh size={12}   />{/if}
            {m.common_retry()}
          </Button>
        </div>
      </CardAction>
    </CardHeader>

    <CardContent class="p-0 flex-1 min-h-0 flex flex-col overflow-hidden">
      {#if error}
        <div class="flex flex-1 flex-col items-center justify-center py-16 text-destructive gap-2 font-sans p-6">
          <InfoCircle size={24} />
          <h3 class="font-semibold text-xs">{m.common_error_occurred()}</h3>
          <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
          <Button variant="outline" size="sm" class="h-8 mt-2 text-xs" onclick={refetch}>
            {m.common_retry()}
          </Button>
        </div>
      {:else}
        <div class="flex-1 min-h-0 w-full h-full">
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
    </CardContent>
  </Card>
</div>
