<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { createPeriodInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { FormField } from "@orbis/ui";
import { ChevronRight } from "@lucide/svelte";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";

let {
  orgId,
  period = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  period?: {
    id: string;
    name?: string | null;
    name_ar?: string | null;
    code?: string | null;
    period_type?: string | null;
    parent_id?: string | null;
    period_id?: string | null;
    start_date?: string | null;
    end_date?: string | null;
    grading_system?: string | null;
    passing_grade?: number | null;
    max_grade?: number | null;
  } | null;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!period);

let periodsList = $state<any[]>([]);

const form = new FormController({
  initialValues: {
    name: period?.name || "",
    name_ar: period?.name_ar || "",
    code: period?.code || "",
    period_type: period?.period_type || "year",
    parent_id: period?.parent_id || period?.period_id || "",
    start_date: period?.start_date || "",
    end_date: period?.end_date || "",
    grading_system: period?.grading_system || "percentage",
    passing_grade: period?.passing_grade ?? 50,
    max_grade: period?.max_grade ?? 100,
  },
  schema: createPeriodInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

const periodTypeLabels = $derived({
  year: m.common_cycle_form_type_year(),
  term: m.common_period_form_type_term(),
});

const gradingLabels = $derived({
  percentage: m.common_cycle_form_grading_percentage(),
  letter: m.common_cycle_form_grading_letter(),
  gpa: m.common_period_form_grading_gpa(),
});

async function loadParentOptions() {
  if (!orgId) return;
  try {
    const list = await (commands.calendar.listHierarchy
      ? commands.calendar.listHierarchy()
      : commands.calendar.listAcademicPeriods());
    periodsList = (list || []).filter(
      (p: any) => !isEdit || String(p.id) !== String(period?.id),
    );
  } catch (e) {
    console.error("Failed to load parent periods:", e);
  }
}

$effect(() => {
  if (orgId) {
    loadParentOptions();
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (payload) => {
    const request = {
      ...payload,
      name: payload.name || payload.name_ar || "",
      name_ar: payload.name_ar || payload.name || undefined,
      parent_id: payload.period_type !== "year" && payload.parent_id ? payload.parent_id : undefined,
    };
    if (isEdit && period) {
      await commands.calendar.updateAcademicPeriod(period.id, request);
      toast.success(m.common_action_save());
    } else {
      await commands.calendar.createAcademicPeriod(request);
      toast.success(m.common_period_form_success());
    }
  });
  if (submitted) onSuccess();
  else if (form.formError) toast.error(form.formError);
  else if (Object.keys(form.errors).length > 0) toast.error(m.common_period_form_error_dates_required());
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-xl font-bold text-foreground">
      {isEdit ? m.common_action_edit() : m.common_period_form_title_create()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide text-start">
    <Accordion type="multiple" value={["identity", "schedule", "grading"]} class="flex flex-col gap-2">
      <AccordionItem value="identity" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_user_form_name_en()}</AccordionTrigger>
        <AccordionContent>
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <FormField id="per_name_ar" label={m.common_cycle_form_name()} error={fieldError("name_ar")}>
        {#snippet children()}
        <Input
          id="per_name_ar"
          type="text"
          bind:value={form.values.name_ar}
          onblur={() => form.markTouched("name_ar")}
          placeholder={m.common_period_form_name_placeholder()}
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>

      <FormField id="per_name_en" label={m.common_user_form_name_en()} error={fieldError("name")} required>
        {#snippet children()}
        <Input
          id="per_name_en"
          type="text"
          bind:value={form.values.name}
          onblur={() => form.markTouched("name")}
          aria-invalid={Boolean(fieldError("name"))}
          aria-describedby={fieldError("name") ? "period_name_error" : undefined}
          placeholder={m.common_period_form_name_en_placeholder()}
          required
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>
    </div>

    <!-- Type & Parent -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <FormField id="per_type" label={m.common_word_type()} error={fieldError("period_type")} required>
        {#snippet children()}
        <Select type="single" value={form.values.period_type} onValueChange={(v) => (form.values.period_type = v ?? "year")} disabled={isEdit}>
          <SelectTrigger class="h-9 text-sm w-full">
            {periodTypeLabels[form.values.period_type as keyof typeof periodTypeLabels] || form.values.period_type}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="year">{m.common_cycle_form_type_year()}</SelectItem>
            <SelectItem value="term">{m.common_period_form_type_term()}</SelectItem>
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>

      {#if form.values.period_type !== "year"}
        <FormField id="per_parent" label={m.common_period_form_parent()} error={fieldError("parent_id")}>
          {#snippet children()}
          <Select type="single" value={form.values.parent_id} onValueChange={(v) => (form.values.parent_id = v ?? "")} disabled={isEdit}>
            <SelectTrigger class="h-9 text-sm w-full">
              {periodsList.find(p => String(p.id) === form.values.parent_id) ? (periodsList.find(p => String(p.id) === form.values.parent_id).name_ar || periodsList.find(p => String(p.id) === form.values.parent_id).name) : m.common_period_form_parent_none()}
            </SelectTrigger>
            <SelectContent class="max-h-60 overflow-y-auto">
              <SelectItem value="">{m.common_period_form_parent_none()}</SelectItem>
              {#each periodsList as p}
                <SelectItem value={String(p.id)}>
                  <span class="flex items-center gap-2">
                    {#if p.period_type !== "year"}
                      <ChevronRight size={12} class="text-muted-foreground/50" />
                    {/if}
                    {p.name_ar || p.name}
                  </span>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
          {/snippet}
        </FormField>
      {:else}
        <FormField id="per_code" label="Code" error={fieldError("code")}>
          {#snippet children()}
          <Input
            id="per_code"
            type="text"
            bind:value={form.values.code}
            placeholder="e.g. AY-2026"
            disabled={form.isSubmitting || isEdit}
          />
          {/snippet}
        </FormField>
      {/if}
    </div>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="schedule" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_year_start_date()} / {m.common_year_end_date()}</AccordionTrigger>
        <AccordionContent>

    <!-- Dates -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <FormField id="per_start" label={m.common_year_start_date()} error={fieldError("start_date")} required>
        {#snippet children()}
        <Input
          id="per_start"
          type="date"
          bind:value={form.values.start_date}
          onblur={() => form.markTouched("start_date")}
          required
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>

      <FormField id="per_end" label={m.common_year_end_date()} error={fieldError("end_date")} required>
        {#snippet children()}
        <Input
          id="per_end"
          type="date"
          bind:value={form.values.end_date}
          onblur={() => form.markTouched("end_date")}
          required
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>
    </div>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="grading" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_cycle_form_grading()}</AccordionTrigger>
        <AccordionContent>

    <!-- Grading configs -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <FormField id="per_grading" label={m.common_cycle_form_grading()} error={fieldError("grading_system")} required>
        {#snippet children()}
        <Select type="single" value={form.values.grading_system} onValueChange={(v) => (form.values.grading_system = v ?? "percentage")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {gradingLabels[form.values.grading_system as keyof typeof gradingLabels] || form.values.grading_system}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="percentage">{m.common_cycle_form_grading_percentage()}</SelectItem>
            <SelectItem value="letter">{m.common_cycle_form_grading_letter()}</SelectItem>
            <SelectItem value="gpa">{m.common_period_form_grading_gpa()}</SelectItem>
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>

      <FormField id="per_max" label="Max Grade" error={fieldError("max_grade")} required>
        {#snippet children()}
        <Input
          id="per_max"
          type="number"
          bind:value={form.values.max_grade}
          onblur={() => form.markTouched("max_grade")}
          min={1}
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>

      <FormField id="per_passing" label={m.common_cycle_form_passing_grade()} error={fieldError("passing_grade")} required>
        {#snippet children()}
        <Input
          id="per_passing"
          type="number"
          bind:value={form.values.passing_grade}
          min={0}
          max={form.values.max_grade}
          onblur={() => form.markTouched("passing_grade")}
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>
    </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </div>

  <!-- Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={form.isSubmitting}
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold px-6"
      disabled={form.isSubmitting}
    >
      {form.isSubmitting ? m.common_period_form_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
