<!--
  @page Class Subjects — Subject-Teacher Assignment Matrix
  Lists all class-subject-teacher assignments with filter by class/cycle.
  CRUD via side sheet panels.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Plus,
  BookUser,
  Edit2,
  Trash2,
  GraduationCap,
  BookOpen,
  User,
} from "lucide-svelte";
import { commands } from "$lib/services/ipc";
import type { ClassSubject, JsonRecord } from "@orbis/desktop-client";
import ClassSubjectSheet from "$lib/components/sheets/class-subject-sheet.svelte";
import * as m from "$lib/paraglide/messages";

const orgId = $derived(page.params.orgId ?? "");

// Data state
let allAssignments = $state<ClassSubject[]>([]);
let classes = $state<JsonRecord[]>([]);
let subjects = $state<JsonRecord[]>([]);
let teachers = $state<JsonRecord[]>([]);
let cycles = $state<JsonRecord[]>([]);
let isLoading = $state(false);

// Filters
let filterClassId = $state("");
let filterCycleId = $state("");

// Sheet state
let sheetOpen = $state(false);
let editTarget = $state<ClassSubject | null>(null);

// Filtered view
const displayed = $derived(
  allAssignments.filter((a) => {
    const classMatch = !filterClassId || String(a.class_id) === filterClassId;
    const cycleMatch = !filterCycleId || String(a.cycle_id) === filterCycleId;
    return classMatch && cycleMatch;
  }),
);

// Lookup helpers
function getClassName(id: unknown) {
  const c = classes.find((x) => String(x.id) === String(id));
  return c ? String(c.name ?? c.name_ar ?? id) : String(id);
}
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
    const [asgn, cls, subj, usr, cyc] = await Promise.all([
      commands.schedule.getClassSubjects(),
      commands.classes.getAll(orgId),
      commands.subjects.getAll(orgId),
      commands.users.getUsers(orgId, { role: "teacher" }),
      commands.academics.getCycles(orgId),
    ]);
    allAssignments = asgn;
    classes = cls as unknown as JsonRecord[];
    subjects = subj as unknown as JsonRecord[];
    teachers = usr as unknown as JsonRecord[];
    cycles = cyc;
  } catch (e) {
    console.error("Failed to load class subjects:", e);
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
  if (!confirm(m.cs_delete_confirm())) return;
  try {
    await commands.schedule.deleteClassSubject(String(a.id));
    allAssignments = allAssignments.filter((x) => x.id !== a.id);
    toast.success(m.cs_delete_success());
  } catch (e: unknown) {
    toast.error(`فشل الحذف: ${e instanceof Error ? e.message : String(e)}`);
  }
}

function onSheetSuccess() {
  loadData();
}
</script>

<svelte:head>
  <title>{m.cs_title()}</title>
</svelte:head>

<div class="h-full w-full flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="shrink-0 flex items-center justify-between px-6 py-4 border-b border-border/40 bg-card/15">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-lg bg-primary/10">
        <BookUser class="h-5 w-5 text-primary" />
      </div>
      <div>
        <h1 class="text-base font-bold text-foreground">{m.cs_title()}</h1>
        <p class="text-[11px] text-muted-foreground">{m.cs_desc()}</p>
      </div>
    </div>
    <Button onclick={openCreate} size="sm" class="h-8 gap-1.5 text-xs">
      <Plus size={13} />
      {m.cs_add()}
    </Button>
  </div>

  <!-- Filters Bar -->
  <div class="shrink-0 flex items-center gap-3 px-6 py-3 border-b border-border/30 bg-background/50">
    <!-- Filter by Cycle -->
    <div class="flex items-center gap-2">
      <span class="text-xs text-muted-foreground font-medium">{m.cs_filter_cycle()}:</span>
      <Select type="single" value={filterCycleId} onValueChange={(v) => (filterCycleId = v ?? "")}>
        <SelectTrigger class="h-7 text-xs w-40 border-border/50">
          {filterCycleId
            ? String(cycles.find((c) => String(c.id) === filterCycleId)?.name ??
                cycles.find((c) => String(c.id) === filterCycleId)?.name_ar ??
                filterCycleId)
            : m.cs_filter_all_cycles()}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="">{m.cs_filter_all_cycles()}</SelectItem>
          {#each cycles as cyc (cyc.id)}
            <SelectItem value={String(cyc.id)}>
              {String(cyc.name ?? cyc.name_ar ?? cyc.id)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Filter by Class -->
    <div class="flex items-center gap-2">
      <span class="text-xs text-muted-foreground font-medium">{m.cs_filter_class()}:</span>
      <Select type="single" value={filterClassId} onValueChange={(v) => (filterClassId = v ?? "")}>
        <SelectTrigger class="h-7 text-xs w-40 border-border/50">
          {filterClassId
            ? String(classes.find((c) => String(c.id) === filterClassId)?.name ??
                classes.find((c) => String(c.id) === filterClassId)?.name_ar ??
                filterClassId)
            : m.cs_filter_all_classes()}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="">{m.cs_filter_all_classes()}</SelectItem>
          {#each classes as cls (cls.id)}
            <SelectItem value={String(cls.id)}>
              {String(cls.name ?? cls.name_ar ?? cls.id)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    {#if displayed.length > 0}
      <span class="ms-auto text-[10px] text-muted-foreground font-mono">
        {displayed.length} ربط
      </span>
    {/if}
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto scrollbar-hide">
    {#if isLoading}
      <div class="p-6 flex flex-col gap-3">
        {#each Array(6) as _, i (i)}
          <Skeleton class="h-14 w-full rounded-xl" />
        {/each}
      </div>
    {:else if displayed.length === 0}
      <!-- Empty State -->
      <div class="h-full flex flex-col items-center justify-center gap-4 text-center p-8">
        <div class="p-5 rounded-2xl bg-primary/5 border border-primary/10">
          <BookUser class="h-10 w-10 text-primary/40" />
        </div>
        <div class="space-y-1">
          <p class="text-sm font-semibold text-foreground">{m.cs_empty()}</p>
          <p class="text-xs text-muted-foreground max-w-xs">{m.cs_empty_desc()}</p>
        </div>
        <Button onclick={openCreate} size="sm" class="mt-2 gap-1.5 text-xs">
          <Plus size={13} />
          {m.cs_add()}
        </Button>
      </div>
    {:else}
      <!-- Table -->
      <div class="p-4">
        <div class="rounded-xl border border-border/40 overflow-hidden">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-border/40 bg-muted/20">
                <th class="text-start px-4 py-3 text-[11px] font-semibold text-muted-foreground">
                  <div class="flex items-center gap-1.5">
                    <GraduationCap size={12} />
                    {m.cs_col_class()}
                  </div>
                </th>
                <th class="text-start px-4 py-3 text-[11px] font-semibold text-muted-foreground">
                  <div class="flex items-center gap-1.5">
                    <BookOpen size={12} />
                    {m.cs_col_subject()}
                  </div>
                </th>
                <th class="text-start px-4 py-3 text-[11px] font-semibold text-muted-foreground">
                  <div class="flex items-center gap-1.5">
                    <User size={12} />
                    {m.cs_col_teacher()}
                  </div>
                </th>
                <th class="text-start px-4 py-3 text-[11px] font-semibold text-muted-foreground">
                  {m.cs_col_assistant()}
                </th>
                <th class="text-start px-4 py-3 text-[11px] font-semibold text-muted-foreground">
                  {m.cs_col_hours()}
                </th>
                <th class="text-end px-4 py-3 text-[11px] font-semibold text-muted-foreground">
                  {m.cs_col_actions()}
                </th>
              </tr>
            </thead>
            <tbody>
              {#each displayed as asgn, i (asgn.id ?? i)}
                <tr
                  class="border-b border-border/20 last:border-0 hover:bg-accent/30 transition-colors group"
                >
                  <td class="px-4 py-3">
                    <span class="font-semibold text-foreground text-xs">
                      {getClassName(asgn.class_id)}
                    </span>
                  </td>
                  <td class="px-4 py-3">
                    <span class="text-xs text-foreground">{getSubjectName(asgn.subject_id)}</span>
                  </td>
                  <td class="px-4 py-3">
                    <span class="text-xs text-foreground">{getTeacherName(asgn.teacher_id)}</span>
                  </td>
                  <td class="px-4 py-3">
                    <span class="text-xs text-muted-foreground">
                      {getTeacherName(asgn.assistant_teacher_id)}
                    </span>
                  </td>
                  <td class="px-4 py-3">
                    <span class="font-mono text-xs text-muted-foreground">
                      {asgn.weekly_hours ?? "—"}
                    </span>
                  </td>
                  <td class="px-4 py-3">
                    <div class="flex items-center justify-end gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-7 w-7 hover:bg-primary/10 hover:text-primary"
                        onclick={() => openEdit(asgn)}
                        title="تعديل"
                      >
                        <Edit2 size={13} />
                      </Button>
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-7 w-7 hover:bg-destructive/10 hover:text-destructive"
                        onclick={() => handleDelete(asgn)}
                        title="حذف"
                      >
                        <Trash2 size={13} />
                      </Button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Create/Edit Sheet -->
<ClassSubjectSheet
  bind:open={sheetOpen}
  {orgId}
  editData={editTarget}
  onSuccess={onSheetSuccess}
/>
