<!--
@component YearCreateSheet
Slide-over Sheet for creating or editing an Academic Year.
-->
<script lang="ts">
  import { api, extractErrorMessage } from "$lib/api";
  import * as m from "$lib/paraglide/messages";
  import { academicContext } from "$lib/stores";
  import type { AcademicYearDto } from "@orbis/sdk";
  import { toast } from "@orbis/ui";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Label } from "@orbis/ui/label";
  import {
    Sheet,
    SheetContent,
    SheetDescription,
    SheetFooter,
    SheetHeader,
    SheetTitle,
  } from "@orbis/ui/sheet";
  import { Calendar, Sparkles } from "@orbis/icons";

  let {
    open = $bindable(false),
    orgId,
    year = null,
    existingYears = [],
    onSuccess,
  }: {
    open: boolean;
    orgId: string;
    year?: AcademicYearDto | null;
    existingYears?: AcademicYearDto[];
    onSuccess?: () => void;
  } = $props();

  const isEdit = $derived(!!year);

  let name = $state("");
  let nameAr = $state("");
  let startDate = $state("");
  let endDate = $state("");
  let notes = $state("");
  let isSubmitting = $state(false);
  let validationError = $state<string | null>(null);

  $effect(() => {
    if (open) {
      if (year) {
        name = year.name || "";
        nameAr = year.name_ar || "";
        startDate = year.start_date ? year.start_date.slice(0, 10) : "";
        endDate = year.end_date ? year.end_date.slice(0, 10) : "";
        notes = year.notes || "";
      } else {
        const currentYear = new Date().getFullYear();
        name = `Academic Year ${currentYear}/${currentYear + 1}`;
        nameAr = `العام الدراسي ${currentYear}/${currentYear + 1}`;
        startDate = `${currentYear}-09-01`;
        endDate = `${currentYear + 1}-06-30`;
        notes = "";
      }
      validationError = null;
    }
  });

  function applyHijriPreset() {
    const currentYear = new Date().getFullYear();
    const hijriEstimate = currentYear - 579;
    name = `${hijriEstimate}/${hijriEstimate + 1} AH`;
    nameAr = `العام الدراسي ${hijriEstimate}/${hijriEstimate + 1} هـ`;
    startDate = `${currentYear}-08-25`;
    endDate = `${currentYear + 1}-06-15`;
  }

  function validate(): boolean {
    validationError = null;
    if (!name.trim()) {
      validationError = m.year_sheet_validation_name();
      return false;
    }
    if (!startDate || !endDate) {
      validationError = m.year_sheet_validation_dates();
      return false;
    }
    const s = new Date(startDate);
    const e = new Date(endDate);
    if (s >= e) {
      validationError = m.year_sheet_validation_order();
      return false;
    }

    for (const y of existingYears) {
      if (isEdit && y.id === year?.id) continue;
      const ys = new Date(y.start_date);
      const ye = new Date(y.end_date);
      if (s < ye && e > ys) {
        validationError = m.year_sheet_validation_overlap();
        return false;
      }
    }

    return true;
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!validate()) return;

    isSubmitting = true;
    try {
      if (isEdit && year) {
        await api.updateAcademicYear(year.id, {
          name: name.trim(),
          name_ar: nameAr.trim() || undefined,
          start_date: new Date(startDate).toISOString(),
          end_date: new Date(endDate).toISOString(),
          notes: notes.trim() || undefined,
        });
        toast.success(m.year_sheet_updated_success());
      } else {
        await api.createAcademicYear({
          name: name.trim(),
          name_ar: nameAr.trim() || undefined,
          start_date: new Date(startDate).toISOString(),
          end_date: new Date(endDate).toISOString(),
        });
        toast.success(m.year_sheet_created_success());
      }

      await academicContext.resolveContext(orgId);
      open = false;
      onSuccess?.();
    } catch (err: unknown) {
      toast.error(extractErrorMessage(err));
    } finally {
      isSubmitting = false;
    }
  }
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-md overflow-hidden p-0 gap-0 font-sans">
    <form onsubmit={handleSubmit} class="flex flex-col h-full">
      <SheetHeader class="px-5 py-4 border-b border-border/40 shrink-0">
        <div class="flex items-center justify-between">
          <div>
            <SheetTitle class="text-sm font-bold text-foreground">
              {isEdit ? m.year_sheet_edit_title() : m.year_sheet_create_title()}
            </SheetTitle>
            <SheetDescription class="text-xs text-muted-foreground mt-0.5">
              {m.year_sheet_desc()}
            </SheetDescription>
          </div>
          {#if !isEdit}
            <Button
              type="button"
              variant="outline"
              size="sm"
              class="h-7 text-[11px] gap-1 px-2 text-primary border-primary/20 hover:bg-primary/5"
              onclick={applyHijriPreset}
            >
              <Sparkles size={12} />
              {m.year_sheet_hijri_preset()}
            </Button>
          {/if}
        </div>
      </SheetHeader>

      <div class="flex-1 overflow-y-auto p-5 space-y-4">
        {#if validationError}
          <div class="p-2.5 rounded-md bg-destructive/10 text-destructive text-xs border border-destructive/20 font-medium">
            {validationError}
          </div>
        {/if}

        <div class="space-y-3.5">
          <div class="space-y-1">
            <Label class="text-xs font-medium text-foreground">
              {m.year_sheet_name_en()} <span class="text-destructive">*</span>
            </Label>
            <Input
              bind:value={name}
              placeholder="e.g. 2026/2027 or 1447/1448 AH"
              required
              class="text-xs h-8 bg-card border-border/60"
            />
          </div>

          <div class="space-y-1">
            <Label class="text-xs font-medium text-foreground">
              {m.year_sheet_name_ar()}
            </Label>
            <Input
              bind:value={nameAr}
              placeholder="مثال: العام الدراسي 1447/1448 هـ"
              dir="rtl"
              class="text-xs h-8 bg-card border-border/60"
            />
          </div>

          <div class="grid grid-cols-2 gap-2.5">
            <div class="space-y-1">
              <Label class="text-xs font-medium text-foreground">
                {m.year_sheet_start_date()} <span class="text-destructive">*</span>
              </Label>
              <Input
                type="date"
                bind:value={startDate}
                required
                class="text-xs h-8 bg-card border-border/60 font-mono"
              />
            </div>

            <div class="space-y-1">
              <Label class="text-xs font-medium text-foreground">
                {m.year_sheet_end_date()} <span class="text-destructive">*</span>
              </Label>
              <Input
                type="date"
                bind:value={endDate}
                required
                class="text-xs h-8 bg-card border-border/60 font-mono"
              />
            </div>
          </div>

          <div class="space-y-1">
            <Label class="text-xs font-medium text-foreground">
              {m.year_sheet_notes()}
            </Label>
            <Input
              bind:value={notes}
              placeholder={m.year_sheet_notes_placeholder()}
              class="text-xs h-8 bg-card border-border/60"
            />
          </div>
        </div>
      </div>

      <SheetFooter class="p-3 border-t border-border/40 bg-muted/10 flex items-center justify-end gap-2 shrink-0">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          class="text-xs h-8 px-3"
          onclick={() => (open = false)}
          disabled={isSubmitting}
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          size="sm"
          class="text-xs h-8 px-4 font-semibold"
          disabled={isSubmitting}
        >
          {isSubmitting
            ? m.year_sheet_saving()
            : isEdit
              ? m.year_sheet_update_btn()
              : m.year_sheet_create_btn()}
        </Button>
      </SheetFooter>
    </form>
  </SheetContent>
</Sheet>
