<!--
@component BulkInputPage
Bulk data entry page supporting students, teachers, and grades.
Uses OrbisGrid with dynamic columns, Excel paste, and real-time validation.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Tabs, TabsList, TabsTrigger } from "@orbis/ui-svelte/tabs";
import {
  AlertDialog,
  AlertDialogTrigger,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogFooter,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogAction,
  AlertDialogCancel,
} from "@orbis/ui-svelte/alert-dialog";
import { Upload, ArrowLeft, Trash2, Save, ClipboardPaste, Loader2 } from "lucide-svelte";
import { toast } from "svelte-sonner";
import { goto } from "$app/navigation";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";

import BulkInputGrid from "$lib/components/bulk-input/BulkInputGrid.svelte";
import BulkInputSummary from "$lib/components/bulk-input/BulkInputSummary.svelte";
import type { BulkInputType } from "$lib/components/bulk-input/bulk-input-config";
import { getColumnsForType } from "$lib/components/bulk-input/bulk-input-config";
import {
  validateRows,
  getValidationSummary,
  type RowValidationResult,
} from "$lib/components/bulk-input/bulk-input-validator";

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────
let activeType = $state<BulkInputType>("students");
let rows = $state<Record<string, unknown>[]>([]);
let validationResults = $state<RowValidationResult[]>([]);
let isSubmitting = $state(false);
let pendingType = $state<BulkInputType | null>(null);
let showClearDialog = $state(false);
let showSwitchDialog = $state(false);

// ── Column Labels (i18n) ───────────────────────────────────────────
let columnLabels = $derived<Record<string, string>>({
  full_name: m.bulk_col_full_name(),
  email: m.bulk_col_email(),
  phone: m.bulk_col_phone(),
  national_id: m.bulk_col_national_id(),
  sex: m.bulk_col_sex(),
  dob: m.bulk_col_dob(),
  specialization: m.bulk_col_specialization(),
  student_email: m.bulk_col_student_email(),
  subject: m.bulk_col_subject(),
  score: m.bulk_col_score(),
  max_score: m.bulk_col_max_score(),
  exam_type: m.bulk_col_exam_type(),
  semester: m.bulk_col_semester(),
});

// ── Validation Summary ─────────────────────────────────────────────
let summary = $derived(getValidationSummary(validationResults, rows));
let canSubmit = $derived(
  summary.totalFilledRows > 0 && summary.errorCount === 0 && !isSubmitting,
);

// ── Type Switch ────────────────────────────────────────────────────
function handleTypeSwitch(newType: string) {
  const typed = newType as BulkInputType;
  if (typed === activeType) return;

  // Check if there's data in the grid
  const hasData = rows.some((r) => {
    for (const [key, val] of Object.entries(r)) {
      if (key.startsWith("__")) continue;
      if (val !== "" && val !== null && val !== undefined) return true;
    }
    return false;
  });

  if (hasData) {
    pendingType = typed;
    showSwitchDialog = true;
  } else {
    switchType(typed);
  }
}

function switchType(newType: BulkInputType) {
  activeType = newType;
  rows = [];
  validationResults = [];
  showSwitchDialog = false;
  pendingType = null;
}

function confirmSwitch() {
  if (pendingType) {
    switchType(pendingType);
  }
}

// ── Clear All ──────────────────────────────────────────────────────
function clearAll() {
  rows = [];
  validationResults = [];
  showClearDialog = false;
}

// ── Submit ─────────────────────────────────────────────────────────
async function handleSubmit() {
  // Run final validation
  const finalResults = validateRows(activeType, rows);
  const finalSummary = getValidationSummary(finalResults, rows);

  if (finalSummary.errorCount > 0) {
    toast.error(m.bulk_input_has_errors());
    validationResults = finalResults;
    return;
  }

  if (finalSummary.totalFilledRows === 0) {
    toast.error(m.bulk_input_no_data());
    return;
  }

  isSubmitting = true;

  try {
    // Filter out empty rows and strip internal fields
    const dataRows = rows
      .filter((r) => {
        for (const [key, val] of Object.entries(r)) {
          if (key.startsWith("__")) continue;
          if (val !== "" && val !== null && val !== undefined) return true;
        }
        return false;
      })
      .map((r) => {
        const clean: Record<string, unknown> = {};
        for (const [key, val] of Object.entries(r)) {
          if (!key.startsWith("__")) {
            clean[key] = val;
          }
        }
        return clean;
      });

    let result: any;
    const userId = authStore.userId;

    switch (activeType) {
      case "students":
        result = await commands.bulkImport.createStudents(data.orgId, userId, dataRows);
        break;
      case "teachers":
        result = await commands.bulkImport.createTeachers(data.orgId, userId, dataRows);
        break;
      case "grades":
        result = await commands.bulkImport.createGrades(data.orgId, userId, dataRows);
        break;
    }

    if (result.failed === 0) {
      toast.success(m.bulk_input_success({ count: String(result.created) }));
      // Clear and navigate back
      rows = [];
      validationResults = [];
      goto(localizeHref(`/${data.orgId}/students`));
    } else if (result.created > 0) {
      toast.warning(
        m.bulk_input_partial_success({
          created: String(result.created),
          failed: String(result.failed),
        }),
      );
      // Show backend errors
      if (result.errors?.length) {
        for (const err of result.errors) {
          toast.error(`${m.bulk_input_row()} ${err.row_index}: ${err.message}`);
        }
      }
    } else {
      toast.error(m.bulk_input_all_failed());
      if (result.errors?.length) {
        for (const err of result.errors.slice(0, 5)) {
          toast.error(`${m.bulk_input_row()} ${err.row_index}: ${err.message}`);
        }
      }
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Unknown error";
    toast.error(msg);
  } finally {
    isSubmitting = false;
  }
}
</script>

<svelte:head>
  <title>{m.bulk_input_title()} — Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-7xl mx-auto p-6 flex flex-col gap-5">

    <!-- Header -->
    <div class="flex items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <Button
          variant="ghost"
          size="icon-sm"
          onclick={() => goto(localizeHref(`/${data.orgId}/students`))}
        >
          <ArrowLeft size={16} class="rtl:rotate-180" />
        </Button>
        <div class="flex flex-col gap-0.5 text-start">
          <h1 class="text-lg font-bold tracking-tight text-foreground">{m.bulk_input_title()}</h1>
          <p class="text-xs text-muted-foreground">{m.bulk_input_description()}</p>
        </div>
      </div>

      <div class="flex items-center gap-2 flex-nowrap shrink-0">
        <Button
          variant="ghost"
          size="sm"
          class="text-xs text-muted-foreground"
          onclick={() => {
            if (summary.totalFilledRows > 0) {
              showClearDialog = true;
            }
          }}
        >
          <Trash2 size={13} class="me-1.5" />
          {m.bulk_input_clear()}
        </Button>

        <Button
          variant="lime"
          size="sm"
          class="text-xs"
          disabled={!canSubmit}
          onclick={handleSubmit}
        >
          {#if isSubmitting}
            <Loader2 size={13} class="me-1.5 animate-spin" />
            {m.bulk_input_submitting()}
          {:else}
            <Save size={13} class="me-1.5" />
            {m.bulk_input_submit()} ({summary.validCount})
          {/if}
        </Button>
      </div>
    </div>

    <!-- Type Tabs -->
    <div class="bg-card border border-border rounded-xl p-1">
      <Tabs value={activeType} onValueChange={handleTypeSwitch}>
        <TabsList class="w-full grid grid-cols-3">
          <TabsTrigger value="students" class="text-xs">
            {m.bulk_input_tab_students()}
          </TabsTrigger>
          <TabsTrigger value="teachers" class="text-xs">
            {m.bulk_input_tab_teachers()}
          </TabsTrigger>
          <TabsTrigger value="grades" class="text-xs">
            {m.bulk_input_tab_grades()}
          </TabsTrigger>
        </TabsList>
      </Tabs>
    </div>

    <!-- Paste Hint -->
    <div class="flex items-center gap-2 px-3 py-2 rounded-lg bg-muted/30 border border-border/60">
      <ClipboardPaste size={14} class="text-muted-foreground shrink-0" />
      <p class="text-xs text-muted-foreground">{m.bulk_input_paste_hint()}</p>
    </div>

    <!-- Grid -->
    <BulkInputGrid
      type={activeType}
      labels={columnLabels}
      bind:rows
      bind:validationResults
    />

    <!-- Summary -->
    <BulkInputSummary
      totalFilledRows={summary.totalFilledRows}
      validCount={summary.validCount}
      errorCount={summary.errorCount}
      errorDetails={summary.errorDetails}
    />

  </div>
</div>

<!-- Switch Type Confirmation Dialog -->
<AlertDialog bind:open={showSwitchDialog}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.bulk_input_confirm_switch_title()}</AlertDialogTitle>
      <AlertDialogDescription>
        {m.bulk_input_confirm_switch()}
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel onclick={() => (showSwitchDialog = false)}>
        {m.bulk_input_cancel()}
      </AlertDialogCancel>
      <AlertDialogAction onclick={confirmSwitch}>
        {m.bulk_input_confirm_yes()}
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>

<!-- Clear All Confirmation Dialog -->
<AlertDialog bind:open={showClearDialog}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.bulk_input_confirm_clear_title()}</AlertDialogTitle>
      <AlertDialogDescription>
        {m.bulk_input_confirm_clear()}
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel onclick={() => (showClearDialog = false)}>
        {m.bulk_input_cancel()}
      </AlertDialogCancel>
      <AlertDialogAction onclick={clearAll}>
        {m.bulk_input_confirm_yes()}
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
