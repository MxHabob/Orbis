<!--
@component SubjectAddSheet
Sheet for adding an exam subject to a session.
Allows selecting an existing subject or creating a new one on-the-fly.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@orbis/ui-svelte/sheet";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { FormField } from "@orbis/ui-svelte";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "@orbis/ui-svelte";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";

import { commands } from "$lib/core/ipc";
import { examSubjectInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

const examTypeLabels = {
  written: m.exams_subject_type_written(),
  oral: m.exams_subject_type_oral(),
  practical: m.exams_subject_type_practical(),
  mixed: m.exams_subject_type_mixed(),
};

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

// Selection / creation state
let subjectsList = $state<any[]>([]);
let mode = $state<"select" | "create">("select");

// Form inputs
let subjectId = $state("");
let newSubjectName = $state("");
let newSubjectNameAr = $state("");
let newSubjectCode = $state("");

let maxScore = $state(100);
let durationMinutes = $state(120);
let examType = $state("written");
let isSaving = $state(false);

const form = new FormController({
  initialValues: {
    session_id: "",
    subject_id: "",
    max_score: 100,
    duration_minutes: 120,
    exam_type: "written" as "written" | "oral" | "practical" | "mixed",
  },
  schema: examSubjectInputSchema,
});

function resetForm() {
  subjectId = "";
  newSubjectName = "";
  newSubjectNameAr = "";
  newSubjectCode = "";
  maxScore = 100;
  durationMinutes = 120;
  examType = "written";
  mode = "select";
}

// Fetch subjects catalog when sheet is opened
$effect(() => {
  if (open && orgId) {
    (async () => {
      try {
        const res = await commands.academic.listSubjects();
        subjectsList = res || [];
      } catch (e: unknown) {
        console.error("Failed to load school subjects list:", e);
      }
    })();
  }
});

async function handleSubmit() {
  let targetSubjectId = subjectId;

  if (mode === "create") {
    if (!newSubjectNameAr.trim()) {
      toast.error("يرجى كتابة اسم المادة بالعربية");
      return;
    }
    if (!newSubjectCode.trim()) {
      toast.error("يرجى كتابة رمز المادة");
      return;
    }
    isSaving = true;
    try {
      // 1. Create the subject in school catalog
      const newSub = await commands.academic.createSubject({
        name: newSubjectName || newSubjectNameAr,
        name_ar: newSubjectNameAr,
        code: newSubjectCode,
      });
      targetSubjectId = newSub.id;
      // Refresh local list
      subjectsList = [...subjectsList, newSub];
    } catch (e: any) {
      toast.error((e as any)?.message || "فشل إنشاء مادة جديدة في النظام");
      isSaving = false;
      return;
    }
  } else {
    if (!targetSubjectId) {
      toast.error("يرجى اختيار مادة من القائمة");
      return;
    }
  }

  form.values = {
      session_id: sessionId,
      subject_id: targetSubjectId,
      max_score: Number(maxScore),
      duration_minutes: Number(durationMinutes),
      exam_type: examType as "written" | "oral" | "practical" | "mixed",
  };
  const submitted = await form.submit(async (payload) => {
    await commands.exam.add(payload);
    toast.success("تمت إضافة المادة للدورة بنجاح");
  });
  if (submitted) {
    resetForm();
    open = false;
    onSuccess();
  } else if (form.formError) toast.error(form.formError);
  else toast.error("بيانات المادة غير صالحة");
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-md overflow-hidden p-0 gap-0">
    <form
      class="flex flex-col h-full overflow-hidden gap-0"
      onsubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
    >
      <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
        <SheetTitle class="text-lg font-bold text-foreground">
          {m.common_exams_subjects_add()}
        </SheetTitle>
      </SheetHeader>

      <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
        <Accordion type="multiple" value={["subject", "assessment"]} class="flex flex-col gap-4">
          <AccordionItem value="subject" class="rounded-lg">
            <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.exams_subject_new_details()}</AccordionTrigger>
            <AccordionContent>
        <!-- Mode Selection Tabs -->
        <div class="flex bg-muted/40 p-0.5 rounded-lg border border-border/40 text-[10px] font-medium shrink-0">
          <button
            type="button"
            class="flex-1 py-1.5 rounded-md transition-all cursor-pointer {mode === 'select' ? 'bg-card text-foreground font-semibold shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => mode = "select"}
          >
            مادة موجودة في المدرسة
          </button>
          <button
            type="button"
            class="flex-1 py-1.5 rounded-md transition-all cursor-pointer {mode === 'create' ? 'bg-card text-foreground font-semibold shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => mode = "create"}
          >
            إنشاء مادة جديدة كلياً
          </button>
        </div>

        {#if mode === "select"}
          <!-- Existing Subjects Selection Dropdown -->
          <FormField id="subject-select" label="اختر المادة" error={form.errors.subject_id?.[0]} required>
            {#snippet children()}
            <Select type="single" value={subjectId} onValueChange={(v: string | undefined) => (subjectId = v ?? "")}>
              <SelectTrigger class="h-9 text-xs w-full">
                {subjectsList.find(s => String(s.id) === subjectId)?.name_ar || subjectsList.find(s => String(s.id) === subjectId)?.name || "اختر مادة دراسية..."}
              </SelectTrigger>
              <SelectContent>
                {#each subjectsList as s}
                  <SelectItem value={String(s.id)}>{s.name_ar || s.name} ({s.code})</SelectItem>
                {/each}
              </SelectContent>
            </Select>
            {/snippet}
          </FormField>
        {:else}
          <!-- New Subject Form Fields -->
          <div class="flex flex-col gap-3 p-3 rounded-lg border border-dashed border-border/60 bg-muted/20">
            <span class="text-[9px] font-bold text-primary uppercase">{m.exams_subject_new_details()}</span>
            
            <FormField id="new-sub-name-ar" label="اسم المادة (عربي)" error={form.errors.subject_id?.[0]} required>
              {#snippet children()}
              <Input id="new-sub-name-ar" bind:value={newSubjectNameAr} placeholder="مثال: الرياضيات" class="text-xs h-8" />
              {/snippet}
            </FormField>

            <FormField id="new-sub-name-en" label="اسم المادة (إنجليزي)">
              {#snippet children()}
              <Input id="new-sub-name-en" bind:value={newSubjectName} placeholder="e.g. Mathematics" class="text-xs h-8" />
              {/snippet}
            </FormField>

            <FormField id="new-sub-code" label="رمز المادة" required>
              {#snippet children()}
              <Input id="new-sub-code" bind:value={newSubjectCode} placeholder="مثال: MATH" class="text-xs h-8 uppercase font-mono" />
              {/snippet}
            </FormField>
          </div>
        {/if}

            </AccordionContent>
          </AccordionItem>

          <AccordionItem value="assessment" class="rounded-lg">
            <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_exams_subjects_score()} / {m.common_bulk_col_exam_type()}</AccordionTrigger>
            <AccordionContent>

        <div class="border-t border-border/40 my-1"></div>

        <!-- Max Score -->
        <FormField id="max-score" label={m.common_exams_subjects_score()} error={form.errors.max_score?.[0]} required>
          {#snippet children()}
          <Input id="max-score" type="number" bind:value={maxScore} class="text-xs h-9 font-mono" />
          {/snippet}
        </FormField>

        <!-- Duration -->
        <FormField id="duration" label={m.common_exams_subjects_duration()} error={form.errors.duration_minutes?.[0]} required>
          {#snippet children()}
          <Input id="duration" type="number" bind:value={durationMinutes} class="text-xs h-9 font-mono" />
          {/snippet}
        </FormField>

        <!-- Exam Type -->
        <FormField id="exam-type" label={m.common_bulk_col_exam_type()} error={form.errors.exam_type?.[0]} required>
          {#snippet children()}
          <Select type="single" value={examType} onValueChange={(v: string | undefined) => (examType = v ?? "written")}>
            <SelectTrigger class="h-9 text-xs w-full">
              {examTypeLabels[examType as keyof typeof examTypeLabels] || examType}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="written">{m.exams_subject_type_written()}</SelectItem>
              <SelectItem value="oral">{m.exams_subject_type_oral()}</SelectItem>
              <SelectItem value="practical">{m.exams_subject_type_practical()}</SelectItem>
              <SelectItem value="mixed">{m.exams_subject_type_mixed()}</SelectItem>
            </SelectContent>
          </Select>
          {/snippet}
        </FormField>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>

      <!-- Footer -->
      <div class="px-6 py-3 border-t border-border/40 flex items-center justify-end gap-2 shrink-0 bg-background/50">
        <Button variant="ghost" class="h-8 px-4 text-xs cursor-pointer" onclick={() => { resetForm(); open = false; }}>
          إلغاء
        </Button>
        <Button type="submit" variant="default" class="h-8 px-4 text-xs cursor-pointer" disabled={isSaving}>
          {#if isSaving}
            جاري الحفظ...
          {:else}
            {m.common_exams_subjects_add()}
          {/if}
        </Button>
      </div>
    </form>
  </SheetContent>
</Sheet>
