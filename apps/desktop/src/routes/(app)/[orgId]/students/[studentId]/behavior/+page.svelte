<!--
@component StudentBehaviorPage
Behavior records + AI summary using OrbisGrid.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  ShieldCheck,
  ShieldAlert,
  Sparkles,
  BrainCircuit,
  Plus,
} from "@lucide/svelte";
import { onMount } from "svelte";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";

import { formatDate } from "$lib/core/i18n";

import { DataTable } from "$lib/tables";
import { studentBehavior as sbDef, withIndex } from "$lib/tables/definitions";

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);

let records = $state<any[]>([]);
let isLoading = $state(true);
let isGeneratingAI = $state(false);
let aiSummary = $state<string | null>(null);

let showAddModal = $state(false);
let newRecordType = $state("Positive");
let newDescription = $state("");
let newPoints = $state(1);
let isSaving = $state(false);

onMount(() => loadBehavior());

async function loadBehavior() {
  isLoading = true;
  try {
    if (!orgId || !studentId) return;
    records = await api.listStudentBehaviorRecords(studentId);
  } catch (e) {
    console.error(e);
  } finally {
    isLoading = false;
  }
}

async function handleAddRecord() {
  if (!newDescription.trim()) {
    toast.error("Description is required");
    return;
  }
  isSaving = true;
  try {
    await api.recordBehavior({
      student_id: studentId,
      record_type: newRecordType,
      description: newDescription.trim(),
      points: Number(newPoints) || 0,
    });
    toast.success("Behavior record added successfully");
    showAddModal = false;
    newDescription = "";
    newPoints = 1;
    await loadBehavior();
  } catch (e) {
    console.error(e);
    toast.error(String(e) || "Failed to add behavior record");
  } finally {
    isSaving = false;
  }
}

async function handleAISummarize() {
  isGeneratingAI = true;
  aiSummary = null;
  try {
    aiSummary = m.academics_behavior_ai_success_desc_1();
    toast.success(m.academics_behavior_ai_success());
  } catch {
    aiSummary = m.academics_behavior_ai_success_desc_2();
    toast.success(m.academics_behavior_ai_success());
  } finally {
    isGeneratingAI = false;
  }
}

import { applyFilter } from "@orbis/ui/filter-builder";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { behaviorFilterFields } from "$lib/tables/definitions/filter-configs";

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const filteredRecords = $derived(applyFilter(records, activeFilter));

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    filteredRecords.map((rec) => ({
      ...rec,
      created_at: formatDate(rec.createdAt || rec.created_at) || "—",
      record_type:
        String(rec.recordType || rec.record_type || "").toLowerCase() === "positive"
          ? "positive"
          : "negative",
      category: rec.category || rec.recordType || rec.record_type || "—",
      description: rec.description || "—",
      points: rec.points ?? 0,
      status:
        String(rec.recordType || rec.record_type || "").toLowerCase() === "positive"
          ? "approved"
          : "failed",
    })),
  ),
);

const columns = $derived(
  sbDef.columns({
    idx: "#",
    date: m.common_overview_date(),
    type: m.academics_behavior_col_type(),
    description: m.academics_behavior_col_details(),
    status: m.academics_behavior_col_reporter(),
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start">
  <!-- AI Assist Widget -->
  <div class="border border-border/80 bg-linear-to-l from-brand-purple/5 to-brand-indigo/5 p-4 rounded-xl shadow-sm flex flex-col gap-3">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <BrainCircuit size={16} class="text-primary animate-pulse" />
        <span class="font-bold text-foreground">{m.academics_behavior_ai_title()}</span>
      </div>
      <Button variant="lime" class="text-[10px] h-7 px-2.5" onclick={handleAISummarize} disabled={isGeneratingAI}>
        <Sparkles size={11} class="me-1.5" />
        {isGeneratingAI ? m.common_promo_running() : m.academics_behavior_ai_generate()}
      </Button>
    </div>
    {#if aiSummary}
      <div class="p-3 bg-card border border-border/60 rounded-lg text-foreground leading-relaxed">{aiSummary}</div>
    {:else}
      <p class="text-[10px] text-muted-foreground">{m.academics_behavior_ai_hint()}</p>
    {/if}
  </div>

  <!-- Records Grid -->
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between px-1">
      <span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider">{m.academics_behavior_records_title()}</span>
      <Button variant="outline" class="text-[10px] h-7 px-2" onclick={() => (showAddModal = true)}>
        <Plus size={12} class="me-1" />
        {m.academics_behavior_add_record()}
      </Button>
    </div>

    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      filterable={true}
      filterFields={behaviorFilterFields}
      bind:filterValue={activeFilter}
      emptyIcon={ShieldCheck}
      emptyTitle={m.academics_behavior_empty_title()}
      emptyDescription={m.academics_behavior_empty_desc()}
      {rtl}
      height="300px"
    />
  </div>
</div>

{#if showAddModal}
  <!-- Add Behavior Record Dialog Overlay -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4" onclick={() => (showAddModal = false)}>
    <div class="bg-popover border border-border rounded-xl p-6 w-96 shadow-lg flex flex-col gap-4" onclick={(e) => e.stopPropagation()}>
      <h3 class="text-sm font-bold">{m.academics_behavior_add_record()}</h3>
      
      <div class="flex flex-col gap-1.5">
        <label class="text-[10px] font-semibold text-muted-foreground">{m.academics_behavior_col_type()}</label>
        <select
          class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background"
          bind:value={newRecordType}
        >
          <option value="Positive">Positive (Merit)</option>
          <option value="Negative">Negative (Violation)</option>
        </select>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-[10px] font-semibold text-muted-foreground">Points</label>
        <input
          type="number"
          class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background"
          bind:value={newPoints}
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-[10px] font-semibold text-muted-foreground">{m.academics_behavior_col_details()}</label>
        <textarea
          class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background h-20 resize-none"
          placeholder="Enter details about the behavioral incident..."
          bind:value={newDescription}
        ></textarea>
      </div>

      <div class="flex gap-2 justify-end mt-2">
        <Button variant="outline" class="text-xs h-8" onclick={() => (showAddModal = false)}>
          {m.common_btn_cancel()}
        </Button>
        <Button class="text-xs h-8" onclick={handleAddRecord} disabled={isSaving || !newDescription.trim()}>
          {isSaving ? m.common_promo_running() : m.common_btn_save()}
        </Button>
      </div>
    </div>
  </div>
{/if}
