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
import { Plus, Sparkles, UserBadgeCheck } from "@orbis/icons";
import { onMount } from "svelte";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";
import BehaviorRecordDialog from "$lib/features/students/components/behavior-record-dialog.svelte";
import StudentSectionState from "$lib/features/students/components/student-section-state.svelte";

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
let newRecordType = $state("positive");
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
    toast.error(m.academics_behavior_description_required());
    return;
  }
  isSaving = true;
  try {
    await api.recordBehavior({
      student_id: studentId,
      record_type: newRecordType.toLowerCase(),
      description: newDescription.trim(),
      points: Number(newPoints) || 0,
    });
    toast.success(m.academics_behavior_add_success());
    showAddModal = false;
    newDescription = "";
    newPoints = 1;
    await loadBehavior();
  } catch (e) {
    console.error(e);
    toast.error(`${m.academics_behavior_add_failed()}: ${String(e)}`);
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
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    pin: "left",
    align: "center",
  },
  {
    id: "created_at",
    prop: "created_at",
    header: m.common_overview_date(),
    name: m.common_overview_date(),
    width: 130,
    size: 130,
    readonly: true,
    cellClass: "font-mono",
  },
  {
    id: "status",
    prop: "status",
    header: m.academics_behavior_col_type(),
    name: m.academics_behavior_col_type(),
    width: 110,
    size: 110,
    type: "badge",
    align: "center",
    readonly: true,
  },
  {
    id: "points",
    prop: "points",
    header: m.academics_behavior_points(),
    name: m.academics_behavior_points(),
    width: 80,
    size: 80,
    readonly: true,
    align: "center",
    type: "number",
    cellClass: "font-bold",
  },
  {
    id: "description",
    prop: "description",
    header: m.academics_behavior_col_details(),
    name: m.academics_behavior_col_details(),
    width: 260,
    size: 260,
    readonly: true,
  },
];

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start font-sans">
  <!-- AI Assist Card -->
  <Card class="overflow-hidden">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2">
        <Sparkles size={16} class="text-primary" />
        <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
          {m.academics_behavior_ai_title()}
        </CardTitle>
      </div>

      <CardAction>
        <Button variant="default" class="text-[10px] h-7 px-2.5 font-semibold" onclick={handleAISummarize} disabled={isGeneratingAI}>
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
      {#if !isLoading && records.length === 0}
        <StudentSectionState
          icon={UserBadgeCheck}
          title={m.academics_behavior_empty_title()}
          description={m.academics_behavior_empty_desc()}
        />
      {:else}
      <div class="h-72 w-full">
        <OrbisGrid
          {columns}
          source={gridSource}
          loading={isLoading}
          clientQuery
          filter={activeFilter}
          {rtl}
          readonly
        />
      </div>
      {/if}
    </CardContent>
  </Card>
</div>

<BehaviorRecordDialog
  bind:open={showAddModal}
  bind:recordType={newRecordType}
  bind:points={newPoints}
  bind:description={newDescription}
  {isSaving}
  onsubmit={handleAddRecord}
/>
