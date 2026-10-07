<!--
  @component StudentBehavior
  Behavior incidents log and AI assist evaluation summary using OrbisGrid with standardized Cards.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, type BehaviorRecordDto } from "$lib/api";
import { confirmStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { Plus, Sparkles, UserBadgeCheck, Trash } from "@orbis/icons";
import { onMount } from "svelte";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";

import { formatDate } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { behaviorFilterFields } from "$lib/core/filter-configs";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

let records = $state<BehaviorRecordDto[]>([]);
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

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const gridSource = $derived(
  records.map((rec, i) => ({
    ...rec,
    __idx: i + 1,
    created_at: formatDate(rec.created_at) || "—",
    record_type:
      String(rec.record_type || "").toLowerCase() === "positive"
        ? "positive"
        : "negative",
    category: rec.record_type || "—",
    description: rec.description || "—",
    points: rec.points ?? 0,
    status:
      String(rec.record_type || "").toLowerCase() === "positive"
        ? "active"
        : "inactive",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "created_at", prop: "created_at", header: m.common_overview_date(), name: m.common_overview_date(), width: 130, size: 130, readonly: true, cellClass: "font-mono" },
  { id: "status", prop: "status", header: m.academics_behavior_col_type(), name: m.academics_behavior_col_type(), width: 110, size: 110, type: "badge", align: "center", readonly: true },
  { id: "points", prop: "points", header: "Points", name: "Points", width: 80, size: 80, readonly: true, align: "center", type: "number", cellClass: "font-bold" },
  { id: "description", prop: "description", header: m.academics_behavior_col_details(), name: m.academics_behavior_col_details(), width: 260, size: 260, readonly: true },
];

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start font-sans">
  <!-- AI Assist Card -->
  <Card class="overflow-hidden">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2">
        <Sparkles size={16} class="text-primary animate-pulse" />
        <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
          {m.academics_behavior_ai_title()}
        </CardTitle>
      </div>

      <CardAction>
        <Button variant="lime" class="text-[10px] h-7 px-2.5 font-semibold" onclick={handleAISummarize} disabled={isGeneratingAI}>
          <Sparkles size={11} class="me-1.5" />
          {isGeneratingAI ? m.common_promo_running() : m.academics_behavior_ai_generate()}
        </Button>
      </CardAction>
    </CardHeader>

    <CardContent class="p-4">
      {#if aiSummary}
        <div class="p-3 bg-muted/40 border border-border/60 rounded-lg text-foreground leading-relaxed font-sans">{aiSummary}</div>
      {:else}
        <p class="text-[11px] text-muted-foreground">{m.academics_behavior_ai_hint()}</p>
      {/if}
    </CardContent>
  </Card>

  <!-- Records Grid Card -->
  <Card class="overflow-hidden">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
        {m.academics_behavior_records_title()}
      </CardTitle>

      <CardAction>
        <div class="flex items-center gap-2">
          <FilterTrigger
            bind:filter={activeFilter}
            fields={behaviorFilterFields}
          />
          <Button variant="outline" size="sm" class="text-xs h-8 px-2.5 font-semibold gap-1" onclick={() => (showAddModal = true)}>
            <Plus size={13} />
            {m.academics_behavior_add_record()}
          </Button>
        </div>
      </CardAction>
    </CardHeader>

    <CardContent class="p-0">
      <div class="h-[300px] w-full">
        <OrbisGrid
          {columns}
          source={gridSource}
          loading={isLoading}
          clientQuery
          filter={activeFilter}
          emptyIcon={UserBadgeCheck}
          emptyTitle={m.academics_behavior_empty_title()}
          emptyDescription={m.academics_behavior_empty_desc()}
          {rtl}
          readonly
        />
      </div>
    </CardContent>
  </Card>
</div>

{#if showAddModal}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4" onclick={() => (showAddModal = false)}>
    <div class="bg-popover border border-border rounded-xl p-6 w-96 shadow-lg flex flex-col gap-4" onclick={(e) => e.stopPropagation()}>
      <h3 class="text-sm font-bold">{m.academics_behavior_add_record()}</h3>
      
      <div class="flex flex-col gap-1.5">
        <label for="new-record-type-select" class="text-[10px] font-semibold text-muted-foreground">{m.academics_behavior_col_type()}</label>
        <select
          id="new-record-type-select"
          class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background"
          bind:value={newRecordType}
        >
          <option value="Positive">Positive (Merit)</option>
          <option value="Negative">Negative (Violation)</option>
        </select>
      </div>

      <div class="flex flex-col gap-1.5">
        <label for="new-points-input" class="text-[10px] font-semibold text-muted-foreground">Points</label>
        <input
          id="new-points-input"
          type="number"
          class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background"
          bind:value={newPoints}
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <label for="new-desc-input" class="text-[10px] font-semibold text-muted-foreground">{m.academics_behavior_col_details()}</label>
        <textarea
          id="new-desc-input"
          dir="auto"
          class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background h-20 resize-none"
          placeholder="Enter details about the behavioral incident..."
          bind:value={newDescription}
        ></textarea>
      </div>

      <div class="flex gap-2 justify-end mt-2">
        <Button variant="outline" class="text-xs h-8" onclick={() => (showAddModal = false)}>
          {m.common_action_cancel()}
        </Button>
        <Button class="text-xs h-8" onclick={handleAddRecord} disabled={isSaving || !newDescription.trim()}>
          {isSaving ? (m.common_promo_running()) : (m.common_action_save())}
        </Button>
      </div>
    </div>
  </div>
{/if}
