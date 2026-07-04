<!--
@component ClassTeachersPage
Manages teacher-subject assignments for the current class (classId from URL).
Shows assignments for this class.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import { Button } from "@orbis/ui-svelte/button";
import { Plus, BookUser } from "lucide-svelte";
import { commands } from "$lib/core/ipc";
import type { ClassSubject, JsonRecord } from "@orbis/desktop-client";
import ClassSubjectSheet from "$lib/features/academics/components/class-subject-sheet.svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

import { DataTable } from "$lib/tables";
import { classTeachers as ctDef, withIndex } from "$lib/tables/definitions";

const orgId = $derived(page.params.orgId);
const classId = $derived(page.params.classId);

// Data state
let allAssignments = $state<ClassSubject[]>([]);
let subjects = $state<JsonRecord[]>([]);
let teachers = $state<JsonRecord[]>([]);
let isLoading = $state(false);

// Sheet state
let sheetOpen = $state(false);
let editTarget = $state<ClassSubject | null>(null);

// Only show assignments for this class
const displayed = $derived(
  allAssignments.filter((a) => String(a.class_id) === classId),
);

// Lookup helpers
function getSubjectName(id: unknown) {
  const s = subjects.find((x) => String(x.id) === String(id));
  return s ? String(s.name ?? s.name_ar ?? id) : String(id);
}
function getTeacherName(id: unknown) {
  if (!id) return "—";
  const t = teachers.find((x) => String(x.id) === String(id));
  return t ? String(t.full_name ?? t.email ?? id) : String(id);
}
function getTeacherPhone(id: unknown) {
  if (!id) return "—";
  const t = teachers.find((x) => String(x.id) === String(id));
  return t ? String(t.phone ?? "—") : "—";
}
function getTeacherEmail(id: unknown) {
  if (!id) return "—";
  const t = teachers.find((x) => String(x.id) === String(id));
  return t ? String(t.email ?? "—") : "—";
}

async function loadData() {
  if (!orgId) return;
  isLoading = true;
  try {
    const [asgn, subj, usr] = await Promise.all([
      commands.schedule.getClassSubjects(),
      commands.subjects.getAll(orgId),
      commands.users.getUsers(orgId, { role: "teacher" }),
    ]);
    allAssignments = asgn;
    subjects = subj as unknown as JsonRecord[];
    teachers = usr as unknown as JsonRecord[];
  } catch (e) {
    console.error("Failed to load class teachers:", e);
  } finally {
    isLoading = false;
  }
}

onMount(loadData);

function openCreate() {
  editTarget = null;
  sheetOpen = true;
}

function onSheetSuccess() {
  loadData();
}

// Grid Source
const gridSource = $derived(
  withIndex(
    displayed.map((a) => ({
      ...a,
      teacher_name: getTeacherName(a.teacher_id),
      subject_name: getSubjectName(a.subject_id),
      role: "Teacher",
      phone: getTeacherPhone(a.teacher_id),
      email: getTeacherEmail(a.teacher_id),
    })),
  ),
);

// Columns
const columns = $derived(
  ctDef.columns({
    idx: "#",
    teacher: m.cs_col_teacher?.() || "Teacher",
    subject: m.cs_col_subject?.() || "Subject",
    role: "Role",
    phone: "Phone",
    email: "Email",
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-5 text-start h-full">
  <!-- Header -->
  <div class="flex items-center justify-between pb-2 border-b border-border/40 select-none">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-lg bg-primary/10">
        <BookUser class="h-4 w-4 text-primary" />
      </div>
      <div>
        <h2 class="text-sm font-bold text-foreground">
          {m.cs_title ? m.cs_title() : "ربط المعلمين بالمواد"}
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {m.cs_desc ? m.cs_desc() : "إدارة المعلمين والمواد المرتبطة بهذا الفصل"}
        </p>
      </div>
    </div>
    <Button onclick={openCreate} size="sm" class="h-8 gap-1.5 text-xs font-semibold">
      <Plus size={13} />
      {m.cs_add ? m.cs_add() : "ربط معلم"}
    </Button>
  </div>

  <!-- Table -->
  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      emptyIcon={BookUser}
      emptyTitle={m.cs_empty ? m.cs_empty() : "لا يوجد ربط معلمين لهذا الفصل"}
      {rtl}
      refreshable
      onRefresh={loadData}
    />
  </div>
</div>

<!-- Class-Subject Sheet (Create/Edit) -->
<ClassSubjectSheet
  bind:open={sheetOpen}
  {orgId}
  preselectedClassId={classId}
  editData={editTarget}
  onSuccess={onSheetSuccess}
/>
