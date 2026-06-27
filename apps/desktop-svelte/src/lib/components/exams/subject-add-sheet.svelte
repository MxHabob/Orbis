<!--
@component SubjectAddSheet
Sheet for adding an exam subject to a session.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  sessionId,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  sessionId: string;
  onSuccess: () => void;
} = $props();

let subjectId = $state("");
let maxScore = $state(100);
let durationMinutes = $state(120);
let examType = $state("written");
let isSaving = $state(false);

function resetForm() {
  subjectId = "";
  maxScore = 100;
  durationMinutes = 120;
  examType = "written";
}

async function handleSubmit() {
  if (!subjectId.trim()) {
    toast.error("يجب اختيار المادة");
    return;
  }

  isSaving = true;
  try {
    // TODO: Wire to IPC commands
    toast.success("تمت إضافة المادة بنجاح");
    resetForm();
    open = false;
    onSuccess();
  } catch (e) {
    toast.error("Failed to add subject");
  } finally {
    isSaving = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-md overflow-hidden">
    <SheetHeader class="text-start select-none border-b border-border/40 pb-4">
      <SheetTitle class="text-lg font-bold text-foreground">
        {m.exams_subjects_add()}
      </SheetTitle>
      <SheetDescription class="text-xs text-muted-foreground mt-1">
        أضف مادة امتحانية للدورة
      </SheetDescription>
    </SheetHeader>

    <div class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 px-1">
      <!-- Subject ID / Name -->
      <div class="flex flex-col gap-1.5">
        <label for="subject-id" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.exams_subjects_name()}
        </label>
        <Input id="subject-id" bind:value={subjectId} placeholder="اختر أو اكتب اسم المادة" class="text-xs h-9" />
      </div>

      <!-- Max Score -->
      <div class="flex flex-col gap-1.5">
        <label for="max-score" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.exams_subjects_score()}
        </label>
        <Input id="max-score" type="number" bind:value={maxScore} class="text-xs h-9 font-mono" />
      </div>

      <!-- Duration -->
      <div class="flex flex-col gap-1.5">
        <label for="duration" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.exams_subjects_duration()}
        </label>
        <Input id="duration" type="number" bind:value={durationMinutes} class="text-xs h-9 font-mono" />
      </div>

      <!-- Exam Type -->
      <div class="flex flex-col gap-1.5">
        <label for="exam-type" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.exams_subjects_type()}
        </label>
        <select
          id="exam-type"
          bind:value={examType}
          class="h-9 px-3 text-xs rounded-lg border border-border bg-background text-foreground focus:ring-1 focus:ring-primary"
        >
          <option value="written">تحريري</option>
          <option value="oral">شفوي</option>
          <option value="practical">عملي</option>
          <option value="mixed">مختلط</option>
        </select>
      </div>
    </div>

    <!-- Footer -->
    <div class="border-t border-border/40 pt-4 flex items-center justify-end gap-2">
      <Button variant="ghost" class="h-8 px-4 text-xs cursor-pointer" onclick={() => open = false}>
        إلغاء
      </Button>
      <Button variant="default" class="h-8 px-4 text-xs cursor-pointer" disabled={isSaving} onclick={handleSubmit}>
        {#if isSaving}
          جاري الحفظ...
        {:else}
          {m.exams_subjects_add()}
        {/if}
      </Button>
    </div>
  </SheetContent>
</Sheet>
