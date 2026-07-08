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
import { ArrowLeft, Upload, Columns3, Trash2, Info } from "lucide-svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { commands } from "$lib/core/ipc";
import { authStore } from "$lib/stores";
import { useQueryClient } from "@tanstack/svelte-query";
import { createClassesQuery } from "$lib/features/academics";

import BulkGrid from "$lib/features/bulk-input/bulk-grid.svelte";
import BulkSummary from "$lib/features/bulk-input/bulk-summary.svelte";
import { STUDENT_HEADER_PATTERNS } from "$lib/features/bulk-input/paste-handler";
import {
  buildStudentColumns,
  createEmptyStudentRow,
  STUDENT_DATA_PROPS,
  EXTENDED_PROPS,
} from "$lib/features/bulk-input/students/student-columns";
import {
  validateStudentRows,
  getStudentValidationSummary,
} from "$lib/features/bulk-input/students/student-validator";

let { data } = $props();
const locale = getLocale();
const isRtl = locale === "ar";

const queryClient = useQueryClient();
const classesQuery = createClassesQuery(data.orgId);
const classes = $derived($classesQuery.data ?? []);

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
    let updated = false;
    for (let i = 0; i < rows.length; i++) {
      const classInput = String(rows[i].class_id ?? "").trim();
      if (classInput) {
        const match = classes.find(
          (c: any) =>
            c.name.trim().toLowerCase() === classInput.toLowerCase() ||
            c.id === classInput,
        );
        if (match && match.name !== classInput) {
          rows[i].class_id = match.name;
          updated = true;
        }
      }
    }

    validationResults = validateStudentRows(rows, classes);
    if (updated) {
      rows = [...rows]; // trigger reactivity for cell styling
    }
  }, 300);
}

function handleTopClassChange(v: string) {
  selectedClassId = v;
  if (v) {
    const selectedClass = classes.find((c: any) => c.id === v);
    if (selectedClass) {
      const isRowEmpty = (row: Record<string, unknown>) => {
        for (const [k, val] of Object.entries(row)) {
          if (k.startsWith("__")) continue;
          if (val !== "" && val !== null && val !== undefined) return false;
        }
        return true;
      };

      rows = rows.map((r) => {
        if (!isRowEmpty(r)) {
          return { ...r, class_id: selectedClass.name };
        }
        return r;
      });
      runValidation();
    }
  }
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
let columns = $derived(buildStudentColumns(labels, visibleExtended, classes));

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
        // Resolve class name/ID to ID
        const classInput = String(row.class_id ?? "").trim();
        if (classInput) {
          const match = classes.find(
            (c: any) =>
              c.name.trim().toLowerCase() === classInput.toLowerCase() ||
              c.id === classInput,
          );
          if (match) {
            row.class_id = match.id;
          }
        } else if (selectedClassId) {
          // Apply global class selection if row doesn't have one
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
      toast.success(m.bulk_input_success_students({ count: result.created }));
    }
    if (result.failed > 0) {
      toast.error(m.bulk_input_failed_rows({ count: result.failed }));
      console.error("Bulk Import failures:", result.errors);
      if (result.errors && Array.isArray(result.errors)) {
        for (const err of result.errors) {
          toast.error(`الصف ${err.row_index} (${err.field}): ${err.message}`);
        }
      }
    }

    // Refresh enrollments query
    queryClient.invalidateQueries({ queryKey: ["enrollments", data.orgId] });
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Unknown error";
    toast.error(msg);
  } finally {
    isSubmitting = false;
  }
}
</script>

<div
  class="h-full w-full overflow-hidden bg-background/50 text-foreground"
  dir={isRtl ? "rtl" : "ltr"}
>
  <div class="max-w-[1600px] w-full h-full mx-auto p-6 flex flex-col gap-4 overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
      <div class="flex items-center gap-3">
        <Button
          variant="secondary"
          size="icon-sm"
          class="cursor-pointer"
          onclick={() =>
            goto(localizeHref(`/${data.orgId}/settings/bulk-input`))}
        >
          <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
        </Button>
      <div class="flex items-center gap-2">
        <Select type="single" value={selectedClassId} onValueChange={handleTopClassChange}>
          <SelectTrigger class="w-52 h-8 text-xs">
            {#if selectedClassId}
              {classes.find((c: any) => c.id === selectedClassId)?.name ?? selectedClassId}
            {:else}
              {m.bulk_class_all()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">{m.bulk_class_per_row()}</SelectItem>
            {#each classes as cls}
              <SelectItem value={cls.id}>{cls.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
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
                    onclick={() => toggleExtended(prop)}
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
