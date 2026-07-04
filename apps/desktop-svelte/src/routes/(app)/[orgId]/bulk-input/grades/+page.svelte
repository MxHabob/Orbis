<!--
@component BulkGradeImport
Bulk grade entry page with smart student matching and active cycle display.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  ArrowLeft,
  Upload,
  Info,
  Trash2,
  CalendarDays,
} from "lucide-svelte";
import { Badge } from "@orbis/ui-svelte/badge";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { commands } from "$lib/services/ipc";
import { authStore } from "$lib/stores";

import BulkGrid from "$lib/components/bulk-input/BulkGrid.svelte";
import BulkSummary from "$lib/components/bulk-input/BulkSummary.svelte";
import { GRADE_HEADER_PATTERNS } from "$lib/components/bulk-input/paste-handler";
import {
  buildGradeColumns,
  createEmptyGradeRow,
  GRADE_DATA_PROPS,
} from "$lib/components/bulk-input/grades/grade-columns";
import {
  validateGradeRows,
  getGradeValidationSummary,
} from "$lib/components/bulk-input/grades/grade-validator";

let { data } = $props();
const locale = getLocale();
const isRtl = locale === "ar";

// ── State ──────────────────────────────────────────────────────────
let rows = $state<Record<string, unknown>[]>([]);
let isSubmitting = $state(false);
let submitProgress = $state(0);
let identifierType = $state("auto");
let activeCycleName = $state("");

// Validation
let validationResults = $state<ReturnType<typeof validateGradeRows>>([]);
let summary = $derived(getGradeValidationSummary(validationResults, rows));

let validationTimeout: ReturnType<typeof setTimeout> | null = null;
function runValidation() {
  if (validationTimeout) clearTimeout(validationTimeout);
  validationTimeout = setTimeout(() => {
    validationResults = validateGradeRows(rows);
    rows = [...rows];
  }, 300);
}

// Column labels
const labels: Record<string, string> = {
  student_id: m.bulk_col_student_identifier(),
  subject: m.bulk_col_subject(),
  score: m.bulk_col_score(),
  max_score: m.bulk_col_max_score(),
  exam_type: m.bulk_col_exam_type(),
  semester: m.bulk_col_semester(),
  notes: m.bulk_col_notes(),
};

let columns = $derived(buildGradeColumns(labels));

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
        for (const prop of GRADE_DATA_PROPS) {
          row[prop] = r[prop] ?? "";
        }
        // Apply identifier type
        row.identifier_type = identifierType;
        // Convert score/max_score to numbers
        row.score = parseFloat(String(row.score)) || 0;
        row.max_score = parseFloat(String(row.max_score)) || 100;
        return row;
      });

    submitProgress = 10;

    const result = (await commands.bulkImport.createGrades(
      data.orgId,
      authStore.user?.id ?? "system",
      null, // auto-detect active cycle
      dataRows,
    )) as any;

    submitProgress = 100;

    if (result.active_cycle_name) {
      activeCycleName = result.active_cycle_name;
    }

    if (result.created > 0) {
      toast.success(
        m.bulk_input_success_grades({ count: result.created }),
      );
    }
    if (result.failed > 0) {
      toast.error(
        m.bulk_input_failed_rows({ count: result.failed }),
      );
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Unknown error";
    toast.error(msg);
  } finally {
    isSubmitting = false;
  }
}
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
            {m.bulk_grades_title()}
          </h1>
          <p class="text-xs text-muted-foreground">
            {m.bulk_grades_desc()}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2 flex-nowrap shrink-0">
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
    <div class="flex items-center gap-6 p-3 rounded-lg bg-card border border-border flex-wrap">
      <!-- Active Cycle -->
      <div class="flex items-center gap-2">
        <CalendarDays size={13} class="text-muted-foreground shrink-0" />
        <Label class="text-xs font-medium text-muted-foreground shrink-0">
          {m.bulk_cycle()}:
        </Label>
        {#if activeCycleName}
          <Badge variant="secondary" class="text-xs">
            {activeCycleName}
          </Badge>
        {:else}
          <span class="text-xs text-muted-foreground">{m.bulk_cycle_auto()}</span>
        {/if}
      </div>

      <!-- Identifier Type -->
      <div class="flex items-center gap-2">
        <Label class="text-xs font-medium text-muted-foreground shrink-0">
          {m.bulk_identifier_type()}:
        </Label>
        <Select type="single" value={identifierType} onValueChange={(v) => (identifierType = v)}>
          <SelectTrigger class="w-44 h-8 text-xs">
            {identifierType === "auto"
              ? m.bulk_id_auto()
              : identifierType === "email"
                ? m.bulk_col_email()
                : identifierType === "student_number"
                  ? m.bulk_id_student_number()
                  : identifierType === "national_id"
                    ? m.bulk_col_national_id()
                    : m.bulk_id_name()}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="auto">{m.bulk_id_auto()}</SelectItem>
            <SelectItem value="email">{m.bulk_col_email()}</SelectItem>
            <SelectItem value="student_number">{m.bulk_id_student_number()}</SelectItem>
            <SelectItem value="national_id">{m.bulk_col_national_id()}</SelectItem>
            <SelectItem value="name">{m.bulk_id_name()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
        <Info size={13} class="shrink-0" />
        {m.bulk_grades_hint()}
      </div>
    </div>

    <!-- Grid -->
    <BulkGrid
      {columns}
      dataProps={[...GRADE_DATA_PROPS]}
      bind:rows
      headerPatterns={GRADE_HEADER_PATTERNS}
      createEmptyRow={createEmptyGradeRow}
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
