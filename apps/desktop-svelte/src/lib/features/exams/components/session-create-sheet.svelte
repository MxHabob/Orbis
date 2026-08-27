<!--
@component SessionCreateSheet
Sheet for creating/editing exam sessions.
Includes choosing the academic period.
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
import { Select, SelectContent, SelectItem, SelectTrigger } from "@orbis/ui-svelte/select";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import { toast } from "svelte-sonner";
import { commands } from "$lib/core/ipc";
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
let periodsList = $state<any[]>([]);

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
  periodId = session?.period_id || "";
});

// Load academic periods on open
$effect(() => {
  if (open && orgId) {
    commands.calendar
      .listAcademicPeriods()
      .then((res: any) => {
        periodsList = res || [];
        if (!periodId && periodsList.length > 0) {
          periodId = periodsList[0].id;
        }
      })
      .catch((e) => {
        console.error("Failed to load periods:", e);
      });
  }
});

async function handleSubmit() {
  form.values = { name, name_ar: nameAr, session_type: sessionType, start_date: startDate, end_date: endDate, period_id: periodId };
  const submitted = await form.submit(async (payload) => {
    if (isEdit && session) {
      await commands.exam.updateSession(
        session.id,
        payload.name || payload.name_ar,
        session.status || null,
        payload.start_date,
        payload.end_date,
      );
    } else {
      await commands.exam.createSession({
        name: payload.name || payload.name_ar,

        session_type: payload.session_type,
        start_date: payload.start_date,
        end_date: payload.end_date,
        period_id: payload.period_id,
      });
    }
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
  else if (!submitted) toast.error("بيانات الدورة غير صالحة");
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
      <div class="flex flex-col gap-1.5">
        <label for="session-period" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          الدورة الأكاديمية
        </label>
        <PeriodSelect
          id="session-period"
          bind:value={periodId}
          periods={periodsList}
          placeholder={m.exams_session_select_term()}
          class="text-sm"
        />
      </div>

      <!-- Name Arabic -->
      <div class="flex flex-col gap-1.5">
        <label for="session-name-ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.common_exams_create_name_ar()}
        </label>
        <Input id="session-name-ar" bind:value={nameAr} placeholder="مثال: امتحانات نهاية الفصل الأول" class="text-xs h-9" />
      </div>

      <!-- Name English -->
      <div class="flex flex-col gap-1.5">
        <label for="session-name" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.common_exams_create_name()}
        </label>
        <Input id="session-name" bind:value={name} placeholder="e.g. Final Exams Semester 1" class="text-xs h-9" />
      </div>

      <!-- Session Type -->
      <div class="flex flex-col gap-1.5">
        <label for="session-type" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.common_exams_create_type()}
        </label>
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
      </div>

      <!-- Dates Row -->
      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <label for="start-date" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
            {m.common_year_start_date()}
          </label>
          <Input id="start-date" type="date" bind:value={startDate} class="text-xs h-9" />
        </div>
        <div class="flex flex-col gap-1.5">
          <label for="end-date" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
            {m.common_year_end_date()}
          </label>
          <Input id="end-date" type="date" bind:value={endDate} class="text-xs h-9" />
        </div>
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
