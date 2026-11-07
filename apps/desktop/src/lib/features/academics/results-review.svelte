<!--
  @component ResultsReview
  Overview of grade entry completion and result status per section.
  Allows academic managers to review, approve, and publish results.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage } from "$lib/api";
import type { StudentTermReportCard } from "$lib/api";
import { academicContext } from "$lib/stores";
import {
  createPeriodsQuery,
  createSectionsQuery,
} from "$lib/features/academics";
import { Button } from "@orbis/ui/button";
import { Card } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { LoadingIndicator, toast } from "@orbis/ui";
import {
  ChartBar,
  CheckCircle,
  DocumentCheck,
  InfoCircle,
  Refresh,
  Users,
} from "@orbis/icons";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { getLocale } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

let selectedPeriodId = $state(
  academicContext.effectivePeriodId || "",
);
let selectedClassId = $state("");
let isLoadingResults = $state(false);
let resultError = $state<string | null>(null);
let termResults = $state<StudentTermReportCard[]>([]);

const periodsQuery = createPeriodsQuery(() => orgId);
const sectionsQuery = createSectionsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);
const sections = $derived(sectionsQuery.data ?? []);
const isLoading = $derived(periodsQuery.isLoading || sectionsQuery.isLoading);
const isRtl = $derived(getLocale() === "ar");

// ── Load results when class and period are selected ─────────
async function loadResults() {
  if (!selectedClassId || !selectedPeriodId) return;
  isLoadingResults = true;
  resultError = null;
  try {
    termResults = await api.getClassTermResults(
      selectedClassId,
      selectedPeriodId,
    );
  } catch (err) {
    resultError = extractErrorMessage(err);
    termResults = [];
  } finally {
    isLoadingResults = false;
  }
}

$effect(() => {
  if (selectedClassId && selectedPeriodId) {
    void loadResults();
  } else {
    termResults = [];
  }
});

// ── Summary stats ───────────────────────────────────────────
const totalStudents = $derived(termResults.length);
const passedStudents = $derived(
  termResults.filter((r) => r.status === "passed" || r.status === "passed_with_honors").length,
);
const failedStudents = $derived(
  termResults.filter((r) => r.status === "failed").length,
);
const avgPercentage = $derived(
  totalStudents > 0
    ? Math.round(
        termResults.reduce((sum, r) => sum + r.overall_percentage, 0) /
          totalStudents,
      )
    : 0,
);
const passRate = $derived(
  totalStudents > 0 ? Math.round((passedStudents / totalStudents) * 100) : 0,
);

// ── Grid columns ────────────────────────────────────────────
const columns: OrbisColumnDef[] = [
  {
    id: "__rowIndex",
    prop: "__rowIndex",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    pin: "left",
    align: "center",
    sort: false,
  },
  {
    id: "student_name",
    prop: "student_name",
    header: m.common_word_student(),
    name: m.common_word_student(),
    width: 200,
    size: 200,
    readonly: true,
    pin: "left",
    sort: true,
  },
  {
    id: "total_score",
    prop: "total_score",
    header: m.common_grades_col_total(),
    name: m.common_grades_col_total(),
    width: 90,
    size: 90,
    type: "number",
    align: "end",
    readonly: true,
    sort: true,
  },
  {
    id: "overall_percentage",
    prop: "overall_percentage",
    header: "%",
    name: "%",
    width: 75,
    size: 75,
    type: "percentage",
    align: "end",
    readonly: true,
    sort: true,
  },
  {
    id: "rank_in_class",
    prop: "rank_in_class",
    header: m.student_grades_rank_in_class(),
    name: m.student_grades_rank_in_class(),
    width: 85,
    size: 85,
    type: "number",
    align: "center",
    readonly: true,
    sort: true,
  },
  {
    id: "status",
    prop: "status",
    header: m.student_grades_status(),
    name: m.student_grades_status(),
    width: 100,
    size: 100,
    type: "badge",
    align: "center",
    readonly: true,
    sort: true,
  },
];

const gridSource = $derived(
  termResults.map((r, i) => ({
    __rowIndex: i + 1,
    student_id: r.student_id,
    student_name: r.student_name,
    student_number: r.student_number,
    total_score: r.total_score,
    max_possible_score: r.max_possible_score,
    overall_percentage: r.overall_percentage,
    rank_in_class: r.rank_in_class ?? "—",
    status: r.status,
  })),
);

// ── Section overview (when no class selected) ───────────────
let sectionOverview = $state<
  { id: string; name: string; studentCount: number; avgPct: number | null; status: string }[]
>([]);
let isLoadingOverview = $state(false);

async function loadSectionOverview() {
  if (!selectedPeriodId || sections.length === 0) {
    sectionOverview = [];
    return;
  }
  isLoadingOverview = true;
  const results: typeof sectionOverview = [];
  for (const section of sections) {
    try {
      const cards = await api.getClassTermResults(
        String(section.id),
        selectedPeriodId,
      );
      const avg =
        cards.length > 0
          ? Math.round(
              cards.reduce((s, c) => s + c.overall_percentage, 0) / cards.length,
            )
          : null;
      results.push({
        id: String(section.id),
        name: section.name_ar || section.name,
        studentCount: cards.length,
        avgPct: avg,
        status: cards.length > 0 ? "ready" : "pending",
      });
    } catch {
      results.push({
        id: String(section.id),
        name: section.name_ar || section.name,
        studentCount: 0,
        avgPct: null,
        status: "error",
      });
    }
  }
  sectionOverview = results;
  isLoadingOverview = false;
}

// Load overview when period changes and no class is selected
$effect(() => {
  if (selectedPeriodId && !selectedClassId && sections.length > 0) {
    void loadSectionOverview();
  }
});
</script>

<div class="h-full w-full overflow-y-auto p-6 font-sans text-start select-none">
  <div class="max-w-7xl mx-auto flex flex-col gap-6">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border/40 pb-4">
      <div>
        <h1 class="text-base font-bold text-foreground">
          {m.academics_nav_results_review()}
        </h1>
        <p class="text-xs text-muted-foreground mt-0.5">
          {m.academics_results_review_subtitle()}
        </p>
      </div>
      <Button
        variant="outline"
        size="sm"
        class="h-8 gap-1.5 text-xs font-semibold"
        onclick={() => {
          if (selectedClassId) void loadResults();
          else void loadSectionOverview();
        }}
      >
        {#if isLoadingResults || isLoadingOverview}
          <LoadingIndicator size={13} />
        {:else}
          <Refresh size={13} />
        {/if}
        <span>{m.common_retry()}</span>
      </Button>
    </div>

    <!-- Filters -->
    <div class="flex flex-col gap-2 sm:flex-row sm:items-center">
      <PeriodSelect
        bind:value={selectedPeriodId}
        placeholder={m.student_grades_select_period()}
        class="h-9 w-full text-xs sm:w-64"
      />
      <ClassSelect
        bind:value={selectedClassId}
        disabled={!selectedPeriodId}
        placeholder={m.common_control_select_class()}
        class="h-9 w-full text-xs sm:w-64"
      />
    </div>

    {#if !selectedPeriodId}
      <!-- No period selected -->
      <Card class="flex flex-col items-center justify-center py-16 gap-3 border-border/40 bg-card/40 rounded-xl">
        <InfoCircle size={36} class="text-muted-foreground opacity-40" />
        <p class="text-sm text-muted-foreground">
          {m.student_grades_select_period()}
        </p>
      </Card>
    {:else if !selectedClassId}
      <!-- Overview mode: show all sections -->
      {#if isLoadingOverview || isLoading}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each Array(6) as _}
            <Skeleton class="h-28 w-full rounded-xl" />
          {/each}
        </div>
      {:else if sectionOverview.length === 0}
        <Card class="flex flex-col items-center justify-center py-16 gap-3 border-border/40 bg-card/40 rounded-xl">
          <Users size={36} class="text-muted-foreground opacity-40" />
          <p class="text-sm text-muted-foreground">
            {m.academics_results_no_sections()}
          </p>
        </Card>
      {:else}
        <!-- Section cards overview -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each sectionOverview as section (section.id)}
            <button
              class="text-start"
              onclick={() => { selectedClassId = section.id; }}
            >
              <Card class="p-4 hover:border-primary/30 transition-colors cursor-pointer rounded-xl border-border/60">
                <div class="flex items-center justify-between mb-3">
                  <span class="text-sm font-semibold text-foreground truncate">
                    {section.name}
                  </span>
                  {#if section.status === "ready"}
                    <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4.5 rounded font-sans bg-emerald-500/10 text-emerald-600 border-emerald-500/20">
                      <CheckCircle size={10} class="me-0.5" />
                      {m.academics_results_status_ready()}
                    </Badge>
                  {:else if section.status === "pending"}
                    <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4.5 rounded font-sans">
                      {m.academics_results_status_pending()}
                    </Badge>
                  {:else}
                    <Badge variant="destructive" class="text-[9px] px-1.5 py-0 h-4.5 rounded font-sans">
                      {m.academics_results_status_error()}
                    </Badge>
                  {/if}
                </div>
                <div class="flex items-center gap-4 text-xs text-muted-foreground">
                  <span class="flex items-center gap-1">
                    <Users size={12} />
                    {section.studentCount}
                  </span>
                  {#if section.avgPct !== null}
                    <span class="flex items-center gap-1">
                      <ChartBar size={12} />
                      {section.avgPct}%
                    </span>
                  {/if}
                </div>
              </Card>
            </button>
          {/each}
        </div>
      {/if}
    {:else}
      <!-- Detail mode: show results for selected class -->
      {#if isLoadingResults}
        <div class="flex items-center justify-center py-16">
          <LoadingIndicator size="lg" label={m.common_control_loading()} />
        </div>
      {:else if resultError}
        <Card class="flex flex-col items-center justify-center py-12 gap-3 border-destructive/20 bg-destructive/5 rounded-xl p-6">
          <InfoCircle size={32} class="text-destructive opacity-80" />
          <p class="text-sm text-destructive font-medium">{resultError}</p>
          <Button
            variant="outline"
            size="sm"
            class="h-8 gap-1.5 text-xs font-semibold"
            onclick={() => void loadResults()}
          >
            <Refresh size={13} />
            <span>{m.common_retry()}</span>
          </Button>
        </Card>
      {:else if termResults.length === 0}
        <Card class="flex flex-col items-center justify-center py-16 gap-3 border-border/40 bg-card/40 rounded-xl">
          <DocumentCheck size={36} class="text-muted-foreground opacity-40" />
          <p class="text-sm text-muted-foreground">
            {m.academics_results_empty_section()}
          </p>
        </Card>
      {:else}
        <!-- Summary stats -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
          <Card class="p-3 rounded-xl border-border/60">
            <p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {m.academics_results_stat_total_students()}
            </p>
            <p class="text-xl font-bold text-foreground mt-1">{totalStudents}</p>
          </Card>
          <Card class="p-3 rounded-xl border-border/60">
            <p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {m.academics_results_stat_pass_rate()}
            </p>
            <p class="text-xl font-bold text-foreground mt-1">{passRate}%</p>
          </Card>
          <Card class="p-3 rounded-xl border-border/60">
            <p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {m.academics_results_stat_average()}
            </p>
            <p class="text-xl font-bold text-foreground mt-1">{avgPercentage}%</p>
          </Card>
          <Card class="p-3 rounded-xl border-border/60">
            <p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {m.academics_results_stat_failed()}
            </p>
            <p class="text-xl font-bold mt-1"
              class:text-destructive={failedStudents > 0}
              class:text-foreground={failedStudents === 0}
            >{failedStudents}</p>
          </Card>
        </div>

        <!-- Results grid -->
        <Card class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border-border/60 bg-card p-3 shadow-xs sm:p-4">
          <div class="flex-1 min-h-[300px]">
            <OrbisGrid
              source={gridSource}
              columns={columns}
              readonly
            />
          </div>
        </Card>
      {/if}
    {/if}
  </div>
</div>

