<!--
  @component ClassSubjectForm
  Form for assigning a subject to a teacher for a specific class and academic cycle.
  Used in both create and edit modes via a Sheet panel.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import type { ClassSubject, JsonRecord } from "@orbis/sdk-core";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { createCyclesQuery, createClassesQuery, createSubjectsQuery } from "$lib/features/academics/queries";
import { createUsersQuery } from "$lib/features/settings/queries";
import { Skeleton } from "@orbis/ui-svelte/skeleton";

let {
  orgId,
  editData,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  editData?: ClassSubject | null;
  onSuccess?: () => void;
  onCancel?: () => void;
} = $props();

const isEditMode = $derived(!!editData);

// Form state
let classId = $state("");
let subjectId = $state("");
let teacherId = $state("");
let assistantTeacherId = $state("");
let weeklyHours = $state(4);
let cycleId = $state("");

$effect(() => {
  classId = editData ? String(editData.class_id ?? "") : "";
  subjectId = editData ? String(editData.subject_id ?? "") : "";
  teacherId = editData ? String(editData.teacher_id ?? "") : "";
  assistantTeacherId = editData
    ? String(editData.assistant_teacher_id ?? "")
    : "";
  weeklyHours = editData ? Number(editData.weekly_hours ?? 4) : 4;
  cycleId = editData ? String(editData.cycle_id ?? "") : "";
});

// TanStack Queries
const cyclesQuery = createCyclesQuery(orgId);
const classesQuery = createClassesQuery(orgId);
const subjectsQuery = createSubjectsQuery(orgId);
const usersQuery = createUsersQuery(orgId);

const cycles = $derived(($cyclesQuery.data as JsonRecord[]) ?? []);
const classes = $derived(($classesQuery.data as JsonRecord[]) ?? []);
const subjects = $derived(($subjectsQuery.data as JsonRecord[]) ?? []);
const teachers = $derived(
  (($usersQuery.data as JsonRecord[]) ?? []).filter(
    (u) => String(u.primary_role ?? u.role) === "teacher",
  ),
);

const isLoading = $derived(
  $cyclesQuery.isLoading ||
    $classesQuery.isLoading ||
    $subjectsQuery.isLoading ||
    $usersQuery.isLoading,
);
let isSaving = $state(false);

// Derived label helpers
const selectedClass = $derived(classes.find((c) => String(c.id) === classId));
const selectedSubject = $derived(
  subjects.find((s) => String(s.id) === subjectId),
);
const selectedTeacher = $derived(
  teachers.find((t) => String(t.id) === teacherId),
);
const selectedAssistant = $derived(
  teachers.find((t) => String(t.id) === assistantTeacherId),
);
const selectedCycle = $derived(cycles.find((c) => String(c.id) === cycleId));

async function handleSubmit() {
  if (!classId || !subjectId || !teacherId || !cycleId) {
    toast.error("يرجى ملء الحقول المطلوبة");
    return;
  }

  isSaving = true;
  try {
    const request: JsonRecord = {
      class_id: classId,
      subject_id: subjectId,
      cycle_id: cycleId,
      primary_teacher_id: teacherId,
      assistant_teacher_id: assistantTeacherId || null,
      weekly_hours: weeklyHours,
    };

    if (isEditMode && editData?.id) {
      await commands.academic.updateClassSubject(String(editData.id), request);
      toast.success(m.cs_update_success());
    } else {
      await commands.academic.createClassSubject(request);
      toast.success(m.cs_create_success());
    }

    onSuccess?.();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSaving = false;
  }
}
</script>

<form class="flex flex-col gap-5" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  {#if isLoading}
    <div class="flex flex-col gap-5 animate-pulse">
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-24" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-20" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-28" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-24" />
        <Skeleton class="h-9 w-full rounded-md" />
      </div>
    </div>
  {:else}
    <!-- Cycle -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.cs_form_cycle()} *</Label>
      <Select type="single" value={cycleId} onValueChange={(v) => (cycleId = v ?? "")}>
        <SelectTrigger class="h-9 text-sm">
          {selectedCycle
            ? String(selectedCycle.name ?? selectedCycle.name_ar ?? "")
            : m.cs_form_select_cycle()}
        </SelectTrigger>
        <SelectContent>
          {#each cycles as cyc (cyc.id)}
            <SelectItem value={String(cyc.id)}>
              {String(cyc.name ?? cyc.name_ar ?? cyc.id)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Class -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.cs_form_class()} *</Label>
      <Select type="single" value={classId} onValueChange={(v) => (classId = v ?? "")}>
        <SelectTrigger class="h-9 text-sm">
          {selectedClass
            ? String(selectedClass.name ?? selectedClass.name_ar ?? "")
            : m.cs_form_select_class()}
        </SelectTrigger>
        <SelectContent>
          {#each classes as cls (cls.id)}
            <SelectItem value={String(cls.id)}>
              {String(cls.name ?? cls.name_ar ?? cls.id)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Subject -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.cs_form_subject()} *</Label>
      <Select type="single" value={subjectId} onValueChange={(v) => (subjectId = v ?? "")}>
        <SelectTrigger class="h-9 text-sm">
          {selectedSubject
            ? String(selectedSubject.name ?? selectedSubject.name_ar ?? "")
            : m.cs_form_select_subject()}
        </SelectTrigger>
        <SelectContent>
          {#each subjects as subj (subj.id)}
            <SelectItem value={String(subj.id)}>
              {String(subj.name ?? subj.name_ar ?? subj.id)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Primary Teacher -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.cs_form_teacher()} *</Label>
      <Select type="single" value={teacherId} onValueChange={(v) => (teacherId = v ?? "")}>
        <SelectTrigger class="h-9 text-sm">
          {selectedTeacher
            ? String(selectedTeacher.full_name ?? selectedTeacher.email ?? "")
            : m.cs_form_select_teacher()}
        </SelectTrigger>
        <SelectContent>
          {#each teachers as t (t.id)}
            <SelectItem value={String(t.id)}>
              {String(t.full_name ?? t.email ?? t.id)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Assistant Teacher (optional) -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.cs_form_assistant()}</Label>
      <Select type="single" value={assistantTeacherId} onValueChange={(v) => (assistantTeacherId = v ?? "")}>
        <SelectTrigger class="h-9 text-sm">
          {selectedAssistant
            ? String(selectedAssistant.full_name ?? selectedAssistant.email ?? "")
            : "— لا يوجد —"}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="">— لا يوجد —</SelectItem>
          {#each teachers.filter((t) => String(t.id) !== teacherId) as t (t.id)}
            <SelectItem value={String(t.id)}>
              {String(t.full_name ?? t.email ?? t.id)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Weekly Hours -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.cs_form_hours()}</Label>
      <Input
        type="number"
        min="1"
        max="30"
        value={weeklyHours}
        oninput={(e) => (weeklyHours = parseInt((e.target as HTMLInputElement).value) || 4)}
        class="h-9 text-sm"
      />
    </div>

    <!-- Actions -->
    <div class="flex gap-2 pt-2 border-t border-border/40">
      <Button
        type="button"
        variant="outline"
        class="flex-1 h-9 text-sm"
        onclick={onCancel}
      >
        {m.cs_form_cancel()}
      </Button>
      <Button
        type="submit"
        class="flex-1 h-9 text-sm"
        disabled={isSaving || isLoading || !classId || !subjectId || !teacherId || !cycleId}
      >
        {isSaving ? m.cs_form_saving() : m.cs_form_save()}
      </Button>
    </div>
  {/if}
</form>
