<!--
@component BulkGradeImport
Bulk grade entry page with smart student matching and active period display.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { ArrowLeft, Upload, Trash2 } from "lucide-svelte";
import { toast } from "svelte-sonner";

import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { commands } from "$lib/core/ipc";
import { authStore } from "$lib/stores";
import { academicContext } from "$lib/stores/academic-context.svelte";
import { usePeriodGate } from "$lib/shared/utils/use-period-gate";
import PeriodStatusBar from "$lib/shared/components/period-status-bar.svelte";
import OverrideRequestDialog from "$lib/shared/components/override-request-dialog.svelte";

import BulkGrid from "$lib/features/bulk-input/bulk-grid.svelte";
import BulkSummary from "$lib/features/bulk-input/bulk-summary.svelte";
import { GRADE_HEADER_PATTERNS } from "$lib/features/bulk-input/paste-handler";
import {
  buildGradeColumns,
  createEmptyGradeRow,
  GRADE_DATA_PROPS,
} from "$lib/features/bulk-input/grades/grade-columns";
import {
  validateGradeRows,
  getGradeValidationSummary,
} from "$lib/features/bulk-input/grades/grade-validator";

let { data } = $props();
const locale = getLocale();
const isRtl = locale === "ar";

// ── State ──────────────────────────────────────────────────────────
let rows = $state<Record<string, unknown>[]>([]);
let isSubmitting = $state(false);
let submitProgress = $state(0);
let identifierType = $state("auto");
let activePeriodName = $state("");

const gate = usePeriodGate("grade_entry", () => String(academicContext.activePeriod?.id || ""));
let showOverrideDialog = $state(false);

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
  student_id: m.common_bulk_col_student_identifier(),
  subject: m.common_word_subject(),
  score: m.common_promo_score(),
  max_score: m.common_exams_subjects_score(),
  exam_type: m.common_bulk_col_exam_type(),
  semester: m.common_word_semester(),
  notes: m.common_year_notes(),
};

let columns = $derived(buildGradeColumns(labels));

// ── Data Handlers ──────────────────────────────────────────────────
function handleRowsChange(_: Record<string, unknown>[]) {
  runValidation();
}

function handleClearAll() {
  if (summary.totalFilled > 0) {
    if (!confirm(m.common_bulk_input_confirm_clear())) return;
  }
  rows = [];
}

async function handleSubmit() {
  if (summary.totalFilled === 0) {
    toast.warning(m.common_bulk_input_no_data());
    return;
  }
  if (summary.errorCount > 0) {
    toast.error(m.common_bulk_input_fix_errors());
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

    const result = (await commands.platform.createBulkGrades(
      authStore.user?.id ?? "system",
      academicContext.activePeriod?.id ?? "",
      dataRows,
    )) as any;

    submitProgress = 100;

    if (result.active_period_name) {
      activePeriodName = result.active_period_name;
    }

    if (result.created > 0) {
      toast.success(m.common_bulk_input_success_grades({ count: result.created }));
    }
    if (result.failed > 0) {
      toast.error(m.common_bulk_input_failed_rows({ count: result.failed }));
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
          variant="secondary"
          size="icon-sm"
          class="cursor-pointer"
          onclick={() =>
            goto(localizeHref(`/${data.orgId}/settings/bulk-input`))}
        >
          <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
        </Button>
      {#if academicContext.activePeriod?.id}
        <PeriodStatusBar periodId={String(academicContext.activePeriod.id)} showGates={["grade_entry"]} />
      {/if}
      <div class="flex items-center gap-2">
        <Select type="single" value={identifierType} onValueChange={(v) => (identifierType = v)}>
          <SelectTrigger class="w-44 h-8 text-xs">
            {identifierType === "auto"
              ? m.common_bulk_id_auto()
              : identifierType === "email"
                ? m.common_word_email()
                : identifierType === "student_number"
                  ? m.common_table_student_number()
                  : identifierType === "national_id"
                    ? m.common_user_form_national_id()
                    : m.common_word_fullname()}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="auto">{m.common_bulk_id_auto()}</SelectItem>
            <SelectItem value="email">{m.common_word_email()}</SelectItem>
            <SelectItem value="student_number">{m.common_table_student_number()}</SelectItem>
            <SelectItem value="national_id">{m.common_user_form_national_id()}</SelectItem>
            <SelectItem value="name">{m.common_word_fullname()}</SelectItem>
          </SelectContent>
        </Select>
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
          {m.common_bulk_input_clear()}
        </Button>

        {#if gate.requiresOverride}
          <Button
            variant="secondary"
            size="sm"
            class="text-xs cursor-pointer"
            onclick={() => showOverrideDialog = true}
          >
            🔑 {m.common_override_request_title()}
          </Button>
        {/if}
        <Button
          variant="lime"
          size="sm"
          class="text-xs cursor-pointer"
          disabled={isSubmitting || summary.totalFilled === 0 || summary.errorCount > 0 || !gate.isAllowed}
          onclick={handleSubmit}
        >
          <Upload size={13} class="me-1.5" />
          {m.common_bulk_input_submit()} ({summary.valid})
        </Button>
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
    
    <OverrideRequestDialog
      bind:open={showOverrideDialog}
      orgId={data.orgId}
      overrideType="grade_modification"
      entityType="period"
      entityId={academicContext.activePeriod?.id || ""}
      riskLevel="high"
      onSuccess={() => { toast.success(m.common_override_submitted_success()); gate.refetch(); }}
    />
  </div>
</div>
