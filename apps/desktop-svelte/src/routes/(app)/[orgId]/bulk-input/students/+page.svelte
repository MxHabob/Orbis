<!--
@component BulkStudentImport
Full bulk student import page with class selection, dynamic column toggle,
paste from Excel, real-time validation, and submit via create_user_in_org + create_enrollment.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@orbis/ui-svelte/popover";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Label } from "@orbis/ui-svelte/label";
import {
  ArrowLeft,
  Upload,
  Columns3,
  Trash2,
  Info,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { commands } from "$lib/services/ipc";
import { studentsStore, authStore } from "$lib/stores";

import BulkGrid from "$lib/components/bulk-input/BulkGrid.svelte";
import BulkSummary from "$lib/components/bulk-input/BulkSummary.svelte";
import { STUDENT_HEADER_PATTERNS } from "$lib/components/bulk-input/paste-handler";
import {
  buildStudentColumns,
  createEmptyStudentRow,
  STUDENT_DATA_PROPS,
  EXTENDED_PROPS,
} from "$lib/components/bulk-input/students/student-columns";
import {
  validateStudentRows,
  getStudentValidationSummary,
} from "$lib/components/bulk-input/students/student-validator";

let { data } = $props();
const locale = getLocale();
const isRtl = locale === "ar";

// ── State ──────────────────────────────────────────────────────────
let rows = $state<Record<string, unknown>[]>([]);
let selectedClassId = $state("");
let visibleExtended = $state<Set<string>>(new Set());
let isSubmitting = $state(false);
let submitProgress = $state(0);

// Validation state
let validationResults = $state<ReturnType<typeof validateStudentRows>>([]);
let summary = $derived(getStudentValidationSummary(validationResults, rows));

// Debounced validation
let validationTimeout: ReturnType<typeof setTimeout> | null = null;
function runValidation() {
  if (validationTimeout) clearTimeout(validationTimeout);
  validationTimeout = setTimeout(() => {
    const classIds = new Set(studentsStore.classes.map((c: any) => c.id));
    validationResults = validateStudentRows(rows, classIds);
    rows = [...rows]; // trigger reactivity for cell styling
  }, 300);
}

// Column labels
const labels: Record<string, string> = {
  full_name: m.bulk_col_full_name(),
  name_ar: m.bulk_col_name_ar(),
  email: m.bulk_col_email(),
  phone: m.bulk_col_phone(),
  national_id: m.bulk_col_national_id(),
  sex: m.bulk_col_sex(),
  dob: m.bulk_col_dob(),
  class: m.bulk_col_class(),
  enrollment_status: m.bulk_col_enrollment_status(),
  nationality: m.bulk_col_nationality(),
  address: m.bulk_col_address(),
  city: m.bulk_col_city(),
  country: m.bulk_col_country(),
  guardian_name: m.bulk_col_guardian_name(),
  guardian_phone: m.bulk_col_guardian_phone(),
  guardian_rel: m.bulk_col_guardian_rel(),
  blood_type: m.bulk_col_blood_type(),
  emergency: m.bulk_col_emergency(),
  allergies: m.bulk_col_allergies(),
  medical: m.bulk_col_medical(),
};

// Columns (reactive to extended column toggle)
let columns = $derived(
  buildStudentColumns(labels, visibleExtended, studentsStore.classes),
);

function toggleExtended(prop: string) {
  const newSet = new Set(visibleExtended);
  if (newSet.has(prop)) {
    newSet.delete(prop);
  } else {
    newSet.add(prop);
  }
  visibleExtended = newSet;
}

// ── Data Handlers ──────────────────────────────────────────────────
function handleRowsChange(_: Record<string, unknown>[]) {
  runValidation();
}

function handleClearAll() {
  if (summary.totalFilled > 0) {
    if (!confirm(m.bulk_input_confirm_clear())) return;
  }
  rows = [];
}

async function handleSubmit() {
  if (summary.totalFilled === 0) {
    toast.warning(m.bulk_input_no_data());
    return;
  }
  if (summary.errorCount > 0) {
    toast.error(m.bulk_input_fix_errors());
    return;
  }

  isSubmitting = true;
  submitProgress = 0;

  try {
    // Filter only non-empty rows and add class_id if globally selected
    const dataRows = rows
      .filter((r) => {
        for (const [k, v] of Object.entries(r)) {
          if (k.startsWith("__")) continue;
          if (v !== "" && v !== null && v !== undefined) return true;
        }
        return false;
      })
      .map((r) => {
        const row: Record<string, unknown> = {};
        for (const prop of STUDENT_DATA_PROPS) {
          row[prop] = r[prop] ?? "";
        }
        // Apply global class selection if row doesn't have one
        if (!String(row.class_id ?? "").trim() && selectedClassId) {
          row.class_id = selectedClassId;
        }
        return row;
      });

    submitProgress = 10;

    const result = (await commands.bulkImport.createStudents(
      data.orgId,
      authStore.user?.id ?? "system",
      dataRows,
    )) as any;

    submitProgress = 100;

    if (result.created > 0) {
      toast.success(
        m.bulk_input_success_students({ count: result.created }),
      );
    }
    if (result.failed > 0) {
      toast.error(
        m.bulk_input_failed_rows({ count: result.failed }),
      );
    }

    // Refresh students store
    await studentsStore.load(data.orgId);
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Unknown error";
    toast.error(msg);
  } finally {
    isSubmitting = false;
  }
}

// ── Init ───────────────────────────────────────────────────────────
$effect(() => {
  if (studentsStore.classes.length === 0 && data.orgId) {
    studentsStore.load(data.orgId);
  }
});
</script>

<div
  class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-foreground"
  dir={isRtl ? "rtl" : "ltr"}
>
  <div class="max-w-[1600px] mx-auto p-6 flex flex-col gap-4">
    <!-- Header -->
    <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
      <div class="flex items-center gap-3">
        <Button
          variant="ghost"
          size="icon-sm"
          class="cursor-pointer"
          onclick={() =>
            goto(localizeHref(`/${data.orgId}/students/bulk-input`))}
        >
          <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
        </Button>
        <div class="flex flex-col gap-0.5">
          <h1 class="text-lg font-bold tracking-tight text-foreground">
            {m.bulk_students_title()}
          </h1>
          <p class="text-xs text-muted-foreground">
            {m.bulk_students_desc()}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2 flex-nowrap shrink-0">
        <!-- Column Toggle -->
        <Popover>
          <PopoverTrigger>
            <Button variant="outline" size="sm" class="text-xs cursor-pointer">
              <Columns3 size={13} class="me-1.5" />
              {m.bulk_extra_columns()}
            </Button>
          </PopoverTrigger>
          <PopoverContent class="w-56 p-3" align="end">
            <div class="flex flex-col gap-2.5">
              <span class="text-xs font-semibold text-foreground mb-1">
                {m.bulk_toggle_columns()}
              </span>
              {#each EXTENDED_PROPS as prop}
                <div class="flex items-center gap-2">
                  <Checkbox
                    checked={visibleExtended.has(prop)}
                    onCheckedChange={() => toggleExtended(prop)}
                  />
                  <Label class="text-xs text-muted-foreground cursor-pointer">
                    {labels[prop] ?? prop}
                  </Label>
                </div>
              {/each}
            </div>
          </PopoverContent>
        </Popover>

        <Button
          variant="outline"
          size="sm"
          class="text-xs cursor-pointer text-destructive hover:text-destructive"
          onclick={handleClearAll}
        >
          <Trash2 size={13} class="me-1.5" />
          {m.bulk_input_clear()}
        </Button>

        <Button
          variant="lime"
          size="sm"
          class="text-xs cursor-pointer"
          disabled={isSubmitting || summary.totalFilled === 0 || summary.errorCount > 0}
          onclick={handleSubmit}
        >
          <Upload size={13} class="me-1.5" />
          {m.bulk_input_submit()} ({summary.valid})
        </Button>
      </div>
    </div>

    <!-- Context Bar -->
    <div class="flex items-center gap-4 p-3 rounded-lg bg-card border border-border">
      <div class="flex items-center gap-2">
        <Label class="text-xs font-medium text-muted-foreground shrink-0">
          {m.bulk_col_class()}:
        </Label>
        <Select type="single" value={selectedClassId} onValueChange={(v) => (selectedClassId = v)}>
          <SelectTrigger class="w-52 h-8 text-xs">
            {#if selectedClassId}
              {studentsStore.classes.find((c: any) => c.id === selectedClassId)?.name ?? selectedClassId}
            {:else}
              {m.bulk_class_all()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">{m.bulk_class_per_row()}</SelectItem>
            {#each studentsStore.classes as cls}
              <SelectItem value={cls.id}>{cls.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
        <Info size={13} class="shrink-0" />
        {m.bulk_paste_hint()}
      </div>
    </div>

    <!-- Grid -->
    <BulkGrid
      {columns}
      dataProps={[...STUDENT_DATA_PROPS]}
      bind:rows
      headerPatterns={STUDENT_HEADER_PATTERNS}
      createEmptyRow={createEmptyStudentRow}
      onrowschange={handleRowsChange}
    />

    <!-- Summary -->
    <BulkSummary
      totalFilled={summary.totalFilled}
      valid={summary.valid}
      errorCount={summary.errorCount}
      errorDetails={summary.errorDetails}
      {isSubmitting}
      {submitProgress}
    />
  </div>
</div>
