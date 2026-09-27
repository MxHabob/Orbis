<!--
@component SessionCreateSheet
Sheet for creating/editing exam sessions.
Includes choosing the academic period.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import { toast } from "@orbis/ui";
import { api } from "$lib/api";
import { examSessionInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let {
  open = $bindable(false),
  orgId,
  session = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  session?: any;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!session);

// Form fields state
let name = $state("");
let nameAr = $state("");
let sessionType = $state<"final" | "midterm" | "quiz" | "practical">("final");
let startDate = $state("");
let endDate = $state("");
let periodId = $state("");

const form = new FormController({
  initialValues: {
    name: "",
    name_ar: "",
    session_type: "final" as "final" | "midterm" | "quiz" | "practical",
    start_date: "",
    end_date: "",
    period_id: "",
  },
  schema: examSessionInputSchema,
});
const isSaving = $derived(form.isSubmitting);

// Reset form when session changes
$effect(() => {
  name = session?.name || "";
  nameAr = session?.name_ar || "";
  sessionType = session?.session_type || "final";
  startDate = session?.start_date || "";
  endDate = session?.end_date || "";
  periodId = session?.period_id ? String(session.period_id) : "";
});

async function handleSubmit() {
  form.values = {
    name,
    name_ar: nameAr,
    session_type: sessionType,
    start_date: startDate,
    end_date: endDate,
    period_id: periodId,
  };
  const submitted = await form.submit(async (payload) => {
    await api.createExamSession({
      name: payload.name || payload.name_ar,
      session_type: payload.session_type,
      start_date: payload.start_date,
      end_date: payload.end_date,
      period_id: payload.period_id,
    } as any);
    toast.success(m.common_exams_create_success());
    // Reset form
    name = "";
    nameAr = "";
    startDate = "";
    endDate = "";
    periodId = "";
    open = false;
    onSuccess();
  });
  if (!submitted && form.formError) toast.error(form.formError);
  else if (!submitted) {
    const validationMessage = Object.values(form.errors).flat()[0];
    toast.error(validationMessage || "بيانات الدورة غير صالحة");
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden">
    <SheetHeader class="text-start select-none border-b border-border/40 pb-4">
      <SheetTitle class="text-lg font-bold text-foreground">
        {isEdit ? m.common_action_edit() : m.common_exams_create_title()}
      </SheetTitle>
    </SheetHeader>

    <div class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 px-1">
      
      <!-- Academic Period Selector -->
      <FormField id="session-period" label="الدورة الأكاديمية" error={form.errors.period_id?.[0]} required>
        {#snippet children()}
        <PeriodSelect
          id="session-period"
          bind:value={periodId}
          placeholder={m.exams_session_select_term()}
          class="text-sm"
        />
        {/snippet}
      </FormField>

      <!-- Name Arabic -->
      <FormField id="session-name-ar" label={m.common_exams_create_name_ar()} error={form.errors.name_ar?.[0]}>
        {#snippet children()}
        <Input id="session-name-ar" bind:value={nameAr} placeholder="مثال: امتحانات نهاية الفصل الأول" class="text-xs h-9" />
        {/snippet}
      </FormField>

      <!-- Name English -->
      <FormField id="session-name" label={m.common_exams_create_name()} error={form.errors.name?.[0]}>
        {#snippet children()}
        <Input id="session-name" bind:value={name} placeholder="e.g. Final Exams Semester 1" class="text-xs h-9" />
        {/snippet}
      </FormField>

      <!-- Session Type -->
      <FormField id="session-type" label={m.common_exams_create_type()} error={form.errors.session_type?.[0]} required>
        {#snippet children()}
        <Select type="single" value={sessionType} onValueChange={(value: string | undefined) => (sessionType = (value as typeof sessionType) ?? "final")}>
          <SelectTrigger id="session-type" class="h-9 w-full text-sm">
            {sessionType === "final"
              ? m.exams_session_type_final()
              : sessionType === "midterm"
                ? m.exams_session_type_midterm()
                : sessionType === "quiz"
                  ? m.exams_session_type_quiz()
                  : m.exams_subject_type_practical()}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="final">{m.exams_session_type_final()}</SelectItem>
            <SelectItem value="midterm">{m.exams_session_type_midterm()}</SelectItem>
            <SelectItem value="quiz">{m.exams_session_type_quiz()}</SelectItem>
            <SelectItem value="practical">{m.exams_subject_type_practical()}</SelectItem>
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>

      <!-- Dates Row -->
      <div class="grid grid-cols-2 gap-3">
        <FormField id="start-date" label={m.common_year_start_date()} error={form.errors.start_date?.[0]} required>
          {#snippet children()}
          <Input id="start-date" type="date" bind:value={startDate} class="text-xs h-9" />
          {/snippet}
        </FormField>
        <FormField id="end-date" label={m.common_year_end_date()} error={form.errors.end_date?.[0]} required>
          {#snippet children()}
          <Input id="end-date" type="date" bind:value={endDate} class="text-xs h-9" />
          {/snippet}
        </FormField>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="border-t border-border/40 pt-4 flex items-center justify-end gap-2">
      <Button variant="ghost" class="h-8 px-4 text-xs cursor-pointer" onclick={() => open = false}>
        إلغاء
      </Button>
      <Button variant="default" class="h-8 px-4 text-xs cursor-pointer" disabled={isSaving} onclick={handleSubmit}>
        {#if isSaving}
          جاري الحفظ...
        {:else}
          {m.common_exams_create_submit()}
        {/if}
      </Button>
    </div>
  </SheetContent>
</Sheet>
