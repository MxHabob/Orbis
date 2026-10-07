<!--
@component YearForm
Standard form for creating or editing an Academic Year.
Uses FormController, BilingualNameFields, and shared UI components.
-->
<script lang="ts">
  import { untrack } from "svelte";
  import { api, extractErrorMessage } from "$lib/api";
  import * as m from "$lib/paraglide/messages";
  import { academicContext } from "$lib/stores";
  import type { AcademicYearDto } from "@orbis/sdk";
  import { toast } from "@orbis/ui";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { FormField } from "@orbis/ui";
  import { SheetHeader, SheetTitle, SheetDescription } from "@orbis/ui/sheet";
  import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
  import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
  import { FormController } from "$lib/shared/forms/form-controller.svelte";
  import { Sparkles, Calendar } from "@orbis/icons";

  let {
    orgId,
    year = null,
    existingYears = [],
    onSuccess,
    onCancel,
  }: {
    orgId: string;
    year?: AcademicYearDto | null;
    existingYears?: AcademicYearDto[];
    onSuccess: () => void;
    onCancel: () => void;
  } = $props();

  const isEdit = $derived(!!year);

  // Preset next school year dates by default if new
  const currentYearNum = new Date().getFullYear();
  const defaultStartDate = `${currentYearNum}-09-01`;
  const defaultEndDate = `${currentYearNum + 1}-06-30`;

  const form = untrack(
    () =>
      new FormController({
        initialValues: {
          name: year?.name || `Academic Year ${currentYearNum}/${currentYearNum + 1}`,
          name_ar: year?.name_ar || `العام الدراسي ${currentYearNum}/${currentYearNum + 1}`,
          start_date: year?.start_date ? year.start_date.slice(0, 10) : defaultStartDate,
          end_date: year?.end_date ? year.end_date.slice(0, 10) : defaultEndDate,
          notes: year?.notes || "",
        },
      }),
  );

  function fieldError(field: string): string | undefined {
    return form.errors[field]?.[0];
  }

  function applyHijriPreset() {
    const currentYear = new Date().getFullYear();
    const hijriEstimate = currentYear - 579;
    form.values.name = `${hijriEstimate}/${hijriEstimate + 1} AH`;
    form.values.name_ar = `العام الدراسي ${hijriEstimate}/${hijriEstimate + 1} هـ`;
    form.values.start_date = `${currentYear}-08-25`;
    form.values.end_date = `${currentYear + 1}-06-15`;
  }

  function applyGregorianPreset() {
    const currentYear = new Date().getFullYear();
    form.values.name = `Academic Year ${currentYear}/${currentYear + 1}`;
    form.values.name_ar = `العام الدراسي ${currentYear}/${currentYear + 1}`;
    form.values.start_date = `${currentYear}-09-01`;
    form.values.end_date = `${currentYear + 1}-06-30`;
  }

  function validateCustom(): string | null {
    const s = new Date(form.values.start_date);
    const e = new Date(form.values.end_date);
    if (s >= e) {
      return m.year_sheet_validation_order();
    }
    for (const y of existingYears) {
      if (isEdit && y.id === year?.id) continue;
      const ys = new Date(y.start_date);
      const ye = new Date(y.end_date);
      if (s < ye && e > ys) {
        return m.year_sheet_validation_overlap();
      }
    }
    return null;
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    const customErr = validateCustom();
    if (customErr) {
      toast.error(customErr);
      return;
    }

    const submitted = await form.submit(async (payload) => {
      const name = payload.name || payload.name_ar;
      if (!name) throw new Error(m.year_sheet_validation_name());

      if (isEdit && year?.id) {
        await api.updateAcademicYear(year.id, {
          name,
          name_ar: payload.name_ar || undefined,
          start_date: new Date(payload.start_date).toISOString(),
          end_date: new Date(payload.end_date).toISOString(),
          notes: payload.notes || undefined,
        });
        toast.success(m.year_sheet_updated_success());
      } else {
        await api.createAcademicYear({
          name,
          name_ar: payload.name_ar || undefined,
          start_date: new Date(payload.start_date).toISOString(),
          end_date: new Date(payload.end_date).toISOString(),
        });
        toast.success(m.year_sheet_created_success());
      }
      await academicContext.resolveContext(orgId);
    });

    if (submitted) {
      onSuccess();
    } else if (form.formError) {
      toast.error(form.formError);
    }
  }
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0 font-sans">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <div class="flex items-center justify-between">
      <div>
        <SheetTitle class="text-base font-bold text-foreground">
          {isEdit ? m.year_sheet_edit_title() : m.year_sheet_create_title()}
        </SheetTitle>
        <SheetDescription class="text-xs text-muted-foreground mt-0.5">
          {m.year_sheet_desc()}
        </SheetDescription>
      </div>
      {#if !isEdit}
        <div class="flex items-center gap-1.5">
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="h-7 text-[11px] gap-1 px-2 text-primary border-primary/20 hover:bg-primary/5"
            onclick={applyHijriPreset}
          >
            <Sparkles size={11} />
            {m.year_sheet_hijri_preset()}
          </Button>
          <Button
            type="button"
            variant="ghost"
            size="sm"
            class="h-7 text-[11px] gap-1 px-2 text-muted-foreground hover:bg-muted"
            onclick={applyGregorianPreset}
          >
            <Calendar size={11} />
            {m.academic_years_filter_all()}
          </Button>
        </div>
      {/if}
    </div>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide text-start">
    <!-- Bilingual Name Fields -->
    <BilingualNameFields
      arId="year_name_ar"
      enId="year_name_en"
      arLabel={m.year_sheet_name_ar()}
      enLabel={m.year_sheet_name_en()}
      bind:arValue={form.values.name_ar}
      bind:enValue={form.values.name}
      arPlaceholder={m.year_sheet_name_ar_placeholder()}
      enPlaceholder="e.g. Academic Year 2026/2027"
      arError={fieldError("name_ar")}
      enError={fieldError("name")}
      onArBlur={() => form.markTouched("name_ar")}
      onEnBlur={() => form.markTouched("name")}
      kind="label"
      requestSuggestion={suggestLocalTranslation}
      arRequired
      disabled={form.isSubmitting}
      loadingMessage={m.bilingual_entry_loading()}
      failureMessage={m.bilingual_entry_failure()}
    />

    <!-- Date Range Fields -->
    <div class="grid grid-cols-2 gap-3">
      <FormField id="year_start_date" label={m.year_sheet_start_date()} error={fieldError("start_date")} required>
        {#snippet children()}
          <Input
            id="year_start_date"
            type="date"
            bind:value={form.values.start_date}
            onblur={() => form.markTouched("start_date")}
            aria-invalid={Boolean(fieldError("start_date"))}
            class="text-xs h-8 bg-card border-border/60 font-mono"
            required
          />
        {/snippet}
      </FormField>

      <FormField id="year_end_date" label={m.year_sheet_end_date()} error={fieldError("end_date")} required>
        {#snippet children()}
          <Input
            id="year_end_date"
            type="date"
            bind:value={form.values.end_date}
            onblur={() => form.markTouched("end_date")}
            aria-invalid={Boolean(fieldError("end_date"))}
            class="text-xs h-8 bg-card border-border/60 font-mono"
            required
          />
        {/snippet}
      </FormField>
    </div>

    <!-- Notes -->
    <FormField id="year_notes" label={m.year_sheet_notes()}>
      {#snippet children()}
        <Input
          id="year_notes"
          type="text"
          bind:value={form.values.notes}
          placeholder={m.year_sheet_notes_placeholder()}
          class="text-xs h-8 bg-card border-border/60"
        />
      {/snippet}
    </FormField>
  </div>

  <!-- Form Footer Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={form.isSubmitting}
      class="text-xs h-8 px-3"
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="text-xs h-8 px-4 font-semibold"
      disabled={form.isSubmitting}
    >
      {form.isSubmitting
        ? m.year_sheet_saving()
        : isEdit
          ? m.year_sheet_update_btn()
          : m.year_sheet_create_btn()}
    </Button>
  </div>
</form>
