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
import * as m from "$lib/paraglide/messages";

let {
  orgId,
  cycle = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  cycle?: any;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!cycle);

// Form state
let isSubmitting = $state(false);
let years = $state<any[]>([]);

let name = $state(cycle?.name || "");
let nameAr = $state(cycle?.name_ar || "");
let code = $state(cycle?.code || "");
let cycleType = $state(cycle?.cycle_type || "year");
let parentId = $state(cycle?.parent_id || "");
let startDate = $state(cycle?.start_date || "");
let endDate = $state(cycle?.end_date || "");
let gradingSystem = $state(cycle?.grading_system || "percentage");
let passingGrade = $state<number>(cycle?.passing_grade || 50);
let maxGrade = $state<number>(cycle?.max_grade || 100);

async function loadParentYears() {
  if (!orgId) return;
  try {
    const list = await commands.academics.getYears(orgId);
    // Exclude current cycle if editing to prevent self-referencing
    years = list.filter(
      (y: any) => !isEdit || String(y.id) !== String(cycle.id),
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
    if (isEdit && cycle) {
      const request = {
        name: name || nameAr,
        name_ar: nameAr || name,
        start_date: startDate,
        end_date: endDate,
        grading_system: gradingSystem,
        passing_grade: passingGrade ? Number(passingGrade) : 50,
        max_grade: maxGrade ? Number(maxGrade) : 100,
      };
      await commands.academics.updateCycle(cycle.id, request);
      toast.success(m.settings_users_update_success());
    } else {
      const request = {
        name: name || nameAr,
        name_ar: nameAr || name,
        code: code || undefined,
        cycle_type: cycleType,
        parent_id: cycleType === "term" && parentId ? parentId : undefined,
        start_date: startDate,
        end_date: endDate,
        grading_system: gradingSystem,
        passing_grade: passingGrade ? Number(passingGrade) : 50,
        max_grade: maxGrade ? Number(maxGrade) : 100,
      };
      await commands.academics.createCycle(orgId, request);
      toast.success(m.cycle_form_success());
    }

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.cycle_form_error({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-xl font-bold text-foreground">
      {isEdit ? "Edit Academic Cycle" : m.cycle_form_title_create()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide text-start">
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Arabic Name -->
      <div class="flex flex-col gap-2">
        <label for="cyc_name_ar" class="text-xs font-semibold text-foreground">
          {m.cycle_form_name()}
        </label>
        <Input
          id="cyc_name_ar"
          type="text"
          bind:value={nameAr}
          placeholder={m.cycle_form_name_placeholder()}
          disabled={isSubmitting}
        />
      </div>

      <!-- English Name -->
      <div class="flex flex-col gap-2">
        <label for="cyc_name_en" class="text-xs font-semibold text-foreground">
          {m.cycle_form_name_en()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="cyc_name_en"
          type="text"
          bind:value={name}
          placeholder={m.cycle_form_name_en_placeholder()}
          required
          disabled={isSubmitting}
        />
      </div>
    </div>

    <!-- Type & Parent (only if Type is term) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Cycle Type -->
      <div class="flex flex-col gap-2">
        <label for="cyc_type" class="text-xs font-semibold text-foreground">
          {m.cycle_form_type()}
        </label>
        {@const cycleTypeLabels = { year: m.cycle_form_type_year(), term: m.cycle_form_type_term() }}
        <Select type="single" value={cycleType} onValueChange={(v) => (cycleType = v ?? "year")} disabled={isEdit}>
          <SelectTrigger class="h-9 text-sm w-full">
            {cycleTypeLabels[cycleType as keyof typeof cycleTypeLabels] || cycleType}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="year">{m.cycle_form_type_year()}</SelectItem>
            <SelectItem value="term">{m.cycle_form_type_term()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Parent Year (Only shown when Type is term) -->
      {#if cycleType === "term"}
        <div class="flex flex-col gap-2">
          <label for="cyc_parent" class="text-xs font-semibold text-foreground">
            {m.cycle_form_parent()}
          </label>
          <Select type="single" value={parentId} onValueChange={(v) => (parentId = v ?? "")} disabled={isEdit}>
            <SelectTrigger class="h-9 text-sm w-full">
              {years.find(y => String(y.id) === parentId) ? (years.find(y => String(y.id) === parentId).name_ar || years.find(y => String(y.id) === parentId).name) : m.cycle_form_parent_none()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="">{m.cycle_form_parent_none()}</SelectItem>
              {#each years as y}
                <SelectItem value={String(y.id)}>{y.name_ar || y.name}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      {:else}
        <!-- Code input for Academic Year -->
        <div class="flex flex-col gap-2">
          <label for="cyc_code" class="text-xs font-semibold text-foreground">
            Code
          </label>
          <Input
            id="cyc_code"
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
      <!-- Start Date -->
      <div class="flex flex-col gap-2">
        <label for="cyc_start" class="text-xs font-semibold text-foreground">
          {m.cycle_form_start_date()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="cyc_start"
          type="date"
          bind:value={startDate}
          required
          disabled={isSubmitting}
        />
      </div>

      <!-- End Date -->
      <div class="flex flex-col gap-2">
        <label for="cyc_end" class="text-xs font-semibold text-foreground">
          {m.cycle_form_end_date()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="cyc_end"
          type="date"
          bind:value={endDate}
          required
          disabled={isSubmitting}
        />
      </div>
    </div>

    <!-- Grading configs -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <!-- Grading System -->
      <div class="flex flex-col gap-2">
        <label for="cyc_grading" class="text-xs font-semibold text-foreground">
          {m.cycle_form_grading()}
        </label>
        {@const gradingLabels = { percentage: m.cycle_form_grading_percentage(), letter: m.cycle_form_grading_letter(), gpa: m.cycle_form_grading_gpa() }}
        <Select type="single" value={gradingSystem} onValueChange={(v) => (gradingSystem = v ?? "percentage")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {gradingLabels[gradingSystem as keyof typeof gradingLabels] || gradingSystem}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="percentage">{m.cycle_form_grading_percentage()}</SelectItem>
            <SelectItem value="letter">{m.cycle_form_grading_letter()}</SelectItem>
            <SelectItem value="gpa">{m.cycle_form_grading_gpa()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Max Grade -->
      <div class="flex flex-col gap-2">
        <label for="cyc_max" class="text-xs font-semibold text-foreground">
          Max Grade
        </label>
        <Input
          id="cyc_max"
          type="number"
          bind:value={maxGrade}
          min={1}
          disabled={isSubmitting}
        />
      </div>

      <!-- Passing Grade -->
      <div class="flex flex-col gap-2">
        <label for="cyc_passing" class="text-xs font-semibold text-foreground">
          {m.cycle_form_passing_grade()}
        </label>
        <Input
          id="cyc_passing"
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
      {m.cycle_form_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold px-6"
      disabled={isSubmitting}
    >
      {isSubmitting ? m.cycle_form_saving() : m.cycle_form_save()}
    </Button>
  </div>
</form>
