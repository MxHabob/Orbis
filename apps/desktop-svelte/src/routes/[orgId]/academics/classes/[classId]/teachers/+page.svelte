<!--
@component ClassTeachersPage
Manages teacher-subject assignments for the current class (classId from URL).
Moved from academics/class-subjects/ with context-first navigation.
Shows only assignments for this class — no class filter needed.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { Button } from "@orbis/ui-svelte/button";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from "@orbis/ui-svelte/table";
  import { Plus, BookUser, Edit2, Trash2, User, BookOpen } from "lucide-svelte";
  import { commands } from "$lib/services/ipc";
  import type { ClassSubject, JsonRecord } from "@orbis/desktop-client";
  import ClassSubjectSheet from "$lib/components/sheets/class-subject-sheet.svelte";
  import * as m from "$lib/paraglide/messages";
  import { fade } from "svelte/transition";

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

  function openEdit(a: ClassSubject) {
    editTarget = a;
    sheetOpen = true;
  }

  async function handleDelete(a: ClassSubject) {
    if (
      !confirm(
        m.cs_delete_confirm
          ? m.cs_delete_confirm()
          : "هل تريد حذف هذا الربط؟",
      )
    )
      return;
    try {
      await commands.schedule.deleteClassSubject(String(a.id));
      allAssignments = allAssignments.filter((x) => x.id !== a.id);
      toast.success(
        m.cs_delete_success ? m.cs_delete_success() : "تم الحذف بنجاح",
      );
    } catch (e: unknown) {
      toast.error((e as any)?.message || "فشل الحذف");
    }
  }

  function onSheetSuccess() {
    loadData();
  }
</script>

<div class="flex flex-col gap-5 text-start">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-lg bg-primary/10">
        <BookUser class="h-4 w-4 text-primary" />
      </div>
      <div>
        <h2 class="text-sm font-bold text-foreground">
          {m.cs_title ? m.cs_title() : "ربط المعلمين بالمواد"}
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {m.cs_desc
            ? m.cs_desc()
            : "إدارة المعلمين والمواد المرتبطة بهذا الفصل"}
        </p>
      </div>
    </div>
    <Button onclick={openCreate} size="sm" class="h-8 gap-1.5 text-xs">
      <Plus size={13} />
      {m.cs_add ? m.cs_add() : "ربط معلم"}
    </Button>
  </div>

  <!-- Assignments Table -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      {#each Array(3) as _}
        <Skeleton class="h-14 w-full rounded-lg" />
      {/each}
    </div>
  {:else if displayed.length === 0}
    <div
      class="flex flex-col items-center justify-center py-12 text-muted-foreground border border-border border-dashed rounded-xl gap-3"
    >
      <BookUser size={40} class="text-muted-foreground/30" />
      <p class="text-xs font-semibold">
        {m.cs_empty ? m.cs_empty() : "لا يوجد ربط معلمين لهذا الفصل"}
      </p>
      <Button variant="lime" size="sm" class="text-xs mt-2" onclick={openCreate}>
        {m.cs_add ? m.cs_add() : "ربط معلم"}
      </Button>
    </div>
  {:else}
    <div class="border border-border rounded-xl overflow-hidden" in:fade={{ duration: 200 }}>
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/30">
            <TableHead class="text-xs font-bold">
              <span class="flex items-center gap-1.5">
                <BookOpen size={12} />
                {m.cs_col_subject ? m.cs_col_subject() : "المادة"}
              </span>
            </TableHead>
            <TableHead class="text-xs font-bold">
              <span class="flex items-center gap-1.5">
                <User size={12} />
                {m.cs_col_teacher ? m.cs_col_teacher() : "المعلم"}
              </span>
            </TableHead>
            <TableHead class="text-xs font-bold w-24 text-center">
              {m.cs_col_actions ? m.cs_col_actions() : "إجراءات"}
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#each displayed as assignment (assignment.id)}
            <TableRow class="hover:bg-accent/30 transition-colors">
              <TableCell class="text-xs font-medium">
                {getSubjectName(assignment.subject_id)}
              </TableCell>
              <TableCell class="text-xs">
                {getTeacherName(assignment.teacher_id)}
              </TableCell>
              <TableCell class="text-center">
                <div class="flex items-center justify-center gap-1">
                  <Button
                    variant="ghost"
                    size="icon-xs"
                    onclick={() => openEdit(assignment)}
                  >
                    <Edit2 size={12} />
                  </Button>
                  <Button
                    variant="ghost"
                    size="icon-xs"
                    class="text-destructive/60 hover:text-destructive hover:bg-destructive/10"
                    onclick={() => handleDelete(assignment)}
                  >
                    <Trash2 size={12} />
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          {/each}
        </TableBody>
      </Table>
    </div>
  {/if}
</div>

<!-- Class-Subject Sheet (Create/Edit) -->
<ClassSubjectSheet
  bind:open={sheetOpen}
  {orgId}
  preselectedClassId={classId}
  editData={editTarget}
  onSuccess={onSheetSuccess}
/>
