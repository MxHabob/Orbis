<!--
  @component StudentBehavior
  Behavior incidents log and AI-assisted evaluation summary.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, type BehaviorRecordDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Card } from "@orbis/ui/card";
import { Plus, Sparkles } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";
import BehaviorRecordDialog from "$lib/features/students/components/behavior-record-dialog.svelte";
import StudentSectionState from "$lib/features/students/components/student-section-state.svelte";
import {
  createRecordBehaviorMutation,
  createStudentBehaviorQuery,
} from "$lib/features/students/queries";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { behaviorFilterFields } from "$lib/core/filter-configs";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

const behaviorQuery = createStudentBehaviorQuery(() => studentId);
const recordBehaviorMutation = createRecordBehaviorMutation();

const records = $derived<BehaviorRecordDto[]>(behaviorQuery.data ?? []);
const isLoading = $derived(behaviorQuery.isLoading);

let isGeneratingAI = $state(false);
let aiSummary = $state<string | null>(null);
let aiError = $state<string | null>(null);

let showAddModal = $state(false);
let newRecordType = $state("positive");
let newDescription = $state("");
let newPoints = $state(1);

async function handleAddRecord() {
  if (!newDescription.trim()) {
    toast.error(m.academics_behavior_description_required());
    return;
  }
  try {
    await recordBehaviorMutation.mutateAsync({
      student_id: studentId,
      record_type: newRecordType.toLowerCase(),
      description: newDescription.trim(),
      points: Number(newPoints) || 0,
    });
    toast.success(m.academics_behavior_add_success());
    showAddModal = false;
    newDescription = "";
    newPoints = 1;
  } catch (e: unknown) {
    toast.error(
      `${m.academics_behavior_add_failed()}: ${extractErrorMessage(e)}`,
    );
  }
}

async function handleAISummarize() {
  if (isGeneratingAI) return;
  isGeneratingAI = true;
  aiSummary = null;
  aiError = null;
  try {
    const contextLines = records
      .slice(0, 20)
      .map(
        (rec) =>
          `- [${rec.record_type}] (${rec.points} pts, ${rec.created_at}): ${rec.description}`,
      )
      .join("\n");
    const prompt =
      records.length > 0
        ? `Summarize this student's behavioral standing concisely based on these records:\n${contextLines}`
        : `This student currently has no recorded behavior incidents. Provide a brief, neutral behavioral summary note.`;
    const response = await api.aiChat({
      messages: [{ role: "user", content: prompt }],
      ui_context: {
        entity_type: "student",
        entity_id: studentId,
        screen: "student_behavior",
      },
    });
    aiSummary = response.response;
    toast.success(m.academics_behavior_ai_success());
  } catch (e: unknown) {
    aiError = extractErrorMessage(e);
    toast.error(aiError);
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
    created_at: rec.created_at || null,
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
    type: "date",
    sortable: true,
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
    aggregate: "sum",
    sortable: true,
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

{#if behaviorQuery.isError}
  <StudentSectionState
    icon={Sparkles}
    title={m.common_search_load_failed()}
    description={extractErrorMessage(behaviorQuery.error)}
  >
    {#snippet action()}
      <Button
        variant="outline"
        size="sm"
        onclick={() => void behaviorQuery.refetch()}
      >
        {m.common_retry()}
      </Button>
    {/snippet}
  </StudentSectionState>
{:else}
  <div class="flex flex-col gap-6 text-xs text-start font-sans">
    <Card class="p-4">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <h3 class="text-xs font-semibold text-foreground">
          {m.academics_behavior_ai_title()}
        </h3>
        <Button
          variant="default"
          class="h-7 px-2.5 text-[10px] font-semibold"
          onclick={handleAISummarize}
          disabled={isGeneratingAI || isLoading}
        >
          <Sparkles size={11} class="me-1.5" />
          {isGeneratingAI
            ? m.common_promo_running()
            : m.academics_behavior_ai_generate()}
        </Button>
      </div>
      {#if aiError}
        <p class="mt-2 text-[11px] text-destructive">{aiError}</p>
      {:else if aiSummary}
        <p class="mt-2 text-[11px] leading-relaxed text-foreground">
          {aiSummary}
        </p>
      {:else}
        <p class="mt-2 text-[11px] text-muted-foreground">
          {m.academics_behavior_ai_hint()}
        </p>
      {/if}
    </Card>

    <section class="flex flex-col gap-2">
      <h3 class="text-xs font-semibold text-foreground">
        {m.academics_behavior_records_title()}
      </h3>
      <div class="h-[380px] w-full">
        <OrbisGrid
          {columns}
          source={gridSource}
          loading={isLoading}
          clientQuery
          showFilterButton={false}
          filter={activeFilter}
          emptyTitle={m.academics_behavior_empty_title()}
          emptyDescription={m.academics_behavior_empty_desc()}
          {rtl}
          readonly
        >
          {#snippet toolbarFilters()}
            <FilterTrigger
              bind:filter={activeFilter}
              fields={behaviorFilterFields}
            />
          {/snippet}
          {#snippet toolbarActions()}
            <Button
              variant="outline"
              size="sm"
              class="text-xs h-8 px-2.5 font-semibold gap-1"
              onclick={() => (showAddModal = true)}
            >
              <Plus size={13} />
              {m.academics_behavior_add_record()}
            </Button>
          {/snippet}
        </OrbisGrid>
      </div>
    </section>
  </div>
{/if}

<BehaviorRecordDialog
  bind:open={showAddModal}
  bind:recordType={newRecordType}
  bind:points={newPoints}
  bind:description={newDescription}
  isSaving={recordBehaviorMutation.isPending}
  onsubmit={handleAddRecord}
/>
