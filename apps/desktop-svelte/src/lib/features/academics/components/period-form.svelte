<script lang="ts">
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { SheetHeader, SheetTitle } from "@orbis/ui-svelte/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import { extractErrorMessage } from "$lib/core/ipc";
import * as m from "$lib/paraglide/messages";

let {
  orgId,
  period = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  period?: any;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!period);

// Form state
let isSubmitting = $state(false);
let years = $state<any[]>([]);

let name = $state(period?.name || "");
let nameAr = $state(period?.name_ar || "");
let code = $state(period?.code || "");
let periodType = $state(period?.period_type || period?.period_type || "year");
let parentId = $state(period?.parent_id || period?.year_id || "");
let startDate = $state(period?.start_date || "");
let endDate = $state(period?.end_date || "");
let gradingSystem = $state(period?.grading_system || "percentage");
let passingGrade = $state<number>(period?.passing_grade || 50);
let maxGrade = $state<number>(period?.max_grade || 100);

const periodTypeLabels = $derived({
  year: m.period_form_type_year(),
  term: m.period_form_type_term(),
});

const gradingLabels = $derived({
  percentage: m.period_form_grading_percentage(),
  letter: m.period_form_grading_letter(),
  gpa: m.period_form_grading_gpa(),
});

async function loadParentYears() {
  if (!orgId) return;
  try {
    const list = await commands.platform.listYears(orgId);
    years = list.filter(
      (y: any) => !isEdit || String(y.id) !== String(period.id),
    );
  } catch (e) {
    console.error("Failed to load parent years:", e);
  }
}

$effect(() => {
  if (orgId) {
    loadParentYears();
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  if (!name && !nameAr) {
    toast.error("Name is required");
    return;
  }
  if (!startDate || !endDate) {
    toast.error("Dates are required");
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name || nameAr,
      name_ar: nameAr || name,
      code: code || undefined,
      period_type: periodType,
      year_id: periodType === "term" && parentId ? parentId : undefined,
      start_date: startDate,
      end_date: endDate,
      grading_system: gradingSystem,
      passing_grade: passingGrade ? Number(passingGrade) : 50,
      max_grade: maxGrade ? Number(maxGrade) : 100,
    };
    await commands.academic.createPeriod(request);
    toast.success(m.period_form_success());
    onSuccess();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-xl font-bold text-foreground">
      {isEdit ? m.period_form_title_edit() : m.period_form_title_create()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide text-start">
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-2">
        <label for="per_name_ar" class="text-xs font-semibold text-foreground">
          {m.period_form_name()}
        </label>
        <Input
          id="per_name_ar"
          type="text"
          bind:value={nameAr}
          placeholder={m.period_form_name_placeholder()}
          disabled={isSubmitting}
        />
      </div>

      <div class="flex flex-col gap-2">
        <label for="per_name_en" class="text-xs font-semibold text-foreground">
          {m.period_form_name_en()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="per_name_en"
          type="text"
          bind:value={name}
          placeholder={m.period_form_name_en_placeholder()}
          required
          disabled={isSubmitting}
        />
      </div>
    </div>

    <!-- Type & Parent -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-2">
        <label for="per_type" class="text-xs font-semibold text-foreground">
          {m.period_form_type()}
        </label>
        <Select type="single" value={periodType} onValueChange={(v) => (periodType = v ?? "year")} disabled={isEdit}>
          <SelectTrigger class="h-9 text-sm w-full">
            {periodTypeLabels[periodType as keyof typeof periodTypeLabels] || periodType}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="year">{m.period_form_type_year()}</SelectItem>
            <SelectItem value="term">{m.period_form_type_term()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {#if periodType === "term"}
        <div class="flex flex-col gap-2">
          <label for="per_parent" class="text-xs font-semibold text-foreground">
            {m.period_form_parent()}
          </label>
          <Select type="single" value={parentId} onValueChange={(v) => (parentId = v ?? "")} disabled={isEdit}>
            <SelectTrigger class="h-9 text-sm w-full">
              {years.find(y => String(y.id) === parentId) ? (years.find(y => String(y.id) === parentId).name_ar || years.find(y => String(y.id) === parentId).name) : m.period_form_parent_none()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="">{m.period_form_parent_none()}</SelectItem>
              {#each years as y}
                <SelectItem value={String(y.id)}>{y.name_ar || y.name}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      {:else}
        <div class="flex flex-col gap-2">
          <label for="per_code" class="text-xs font-semibold text-foreground">
            Code
          </label>
          <Input
            id="per_code"
            type="text"
            bind:value={code}
            placeholder="e.g. AY-2026"
            disabled={isSubmitting || isEdit}
          />
        </div>
      {/if}
    </div>

    <!-- Dates -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-2">
        <label for="per_start" class="text-xs font-semibold text-foreground">
          {m.period_form_start_date()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="per_start"
          type="date"
          bind:value={startDate}
          required
          disabled={isSubmitting}
        />
      </div>

      <div class="flex flex-col gap-2">
        <label for="per_end" class="text-xs font-semibold text-foreground">
          {m.period_form_end_date()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="per_end"
          type="date"
          bind:value={endDate}
          required
          disabled={isSubmitting}
        />
      </div>
    </div>

    <!-- Grading configs -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="flex flex-col gap-2">
        <label for="per_grading" class="text-xs font-semibold text-foreground">
          {m.period_form_grading()}
        </label>
        <Select type="single" value={gradingSystem} onValueChange={(v) => (gradingSystem = v ?? "percentage")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {gradingLabels[gradingSystem as keyof typeof gradingLabels] || gradingSystem}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="percentage">{m.period_form_grading_percentage()}</SelectItem>
            <SelectItem value="letter">{m.period_form_grading_letter()}</SelectItem>
            <SelectItem value="gpa">{m.period_form_grading_gpa()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex flex-col gap-2">
        <label for="per_max" class="text-xs font-semibold text-foreground">
          Max Grade
        </label>
        <Input
          id="per_max"
          type="number"
          bind:value={maxGrade}
          min={1}
          disabled={isSubmitting}
        />
      </div>

      <div class="flex flex-col gap-2">
        <label for="per_passing" class="text-xs font-semibold text-foreground">
          {m.period_form_passing_grade()}
        </label>
        <Input
          id="per_passing"
          type="number"
          bind:value={passingGrade}
          min={0}
          max={maxGrade}
          disabled={isSubmitting}
        />
      </div>
    </div>
  </div>

  <!-- Actions -->
  <div class="px-6 py-4 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={isSubmitting}
    >
      {m.period_form_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold px-6"
      disabled={isSubmitting}
    >
      {isSubmitting ? m.period_form_saving() : m.period_form_save()}
    </Button>
  </div>
</form>
