<!--
  @component ResultsReview
  Flexible ERP results review and batch certificate printing for a class or selected student group
  across any period (monthly, midterm, term) or full academic year.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  api,
  extractErrorMessage,
  type StudentAnnualReportCard,
  type StudentTermReportCard,
} from "$lib/api";
import { formatDecimal, formatNumber, formatPercent } from "$lib/core/i18n";
import { academicContext, orgContext } from "$lib/stores";
import {
  createPeriodsQuery,
  createSectionsQuery,
} from "$lib/features/academics";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Card } from "@orbis/ui/card";
import { Skeleton } from "@orbis/ui/skeleton";
import { LoadingIndicator } from "@orbis/ui";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  ChartBar,
  CheckCircle,
  DocumentCheck,
  InfoCircle,
  Printer,
  Refresh,
  Users,
} from "@orbis/icons";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import {
  OrbisGrid,
  selectionColumn,
  type GridActionEvent,
  type OrbisColumnDef,
} from "@orbis/svelte-grid";
import { getLocale } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

const isRtl = $derived(getLocale() === "ar");

let viewMode = $state<"period" | "annual">("period");
let selectedPeriodId = $state(academicContext.effectivePeriodId || "");
let selectedYearId = $state(
  academicContext.currentYear?.id || academicContext.allYears[0]?.id || "",
);
let selectedClassId = $state("");

let isLoadingResults = $state(false);
let resultError = $state<string | null>(null);
let termResults = $state<StudentTermReportCard[]>([]);
let annualResults = $state<StudentAnnualReportCard[]>([]);
let selectedStudentIds = $state<Set<string>>(new Set());

// Printing dialogs
let printDialogOpen = $state(false);
let pdfViewerOpen = $state(false);
let pdfReportCode = $state("rep.academic.student-term-report-card");
let pdfReportTitle = $state("");

const periodsQuery = createPeriodsQuery(() => orgId);
const sectionsQuery = createSectionsQuery(() => orgId);
const sections = $derived(sectionsQuery.data ?? []);
const isLoading = $derived(periodsQuery.isLoading || sectionsQuery.isLoading);

const activeResults = $derived(
  viewMode === "period" ? termResults : annualResults,
);

const yearNames = $derived(
  new Map(
    academicContext.allYears.map((y) => [
      y.id,
      isRtl ? y.name_ar || y.name : y.name,
    ]),
  ),
);

// ── Load class results (period or annual) ───────────────────
let activeRequestId = 0;

async function loadResults() {
  if (!selectedClassId) return;
  if (viewMode === "period" && !selectedPeriodId) return;
  if (viewMode === "annual" && !selectedYearId) return;

  const requestId = ++activeRequestId;
  isLoadingResults = true;
  resultError = null;
  selectedStudentIds = new Set();
  try {
    if (viewMode === "period") {
      const data = await api.getClassTermResults(
        selectedClassId,
        selectedPeriodId,
      );
      if (requestId !== activeRequestId) return;
      termResults = data;
      annualResults = [];
    } else {
      const data = await api.getClassAnnualResults(
        selectedClassId,
        selectedYearId,
      );
      if (requestId !== activeRequestId) return;
      annualResults = data;
      termResults = [];
    }
  } catch (err) {
    if (requestId !== activeRequestId) return;
    resultError = extractErrorMessage(err);
    termResults = [];
    annualResults = [];
  } finally {
    if (requestId === activeRequestId) {
      isLoadingResults = false;
    }
  }
}

$effect(() => {
  if (
    selectedClassId &&
    ((viewMode === "period" && selectedPeriodId) ||
      (viewMode === "annual" && selectedYearId))
  ) {
    void loadResults();
  } else {
    termResults = [];
    annualResults = [];
  }
});

// ── Summary stats ───────────────────────────────────────────
const totalStudents = $derived(activeResults.length);
const passedStudents = $derived(
  activeResults.filter(
    (r) => r.status === "passed" || r.status === "passed_with_honors",
  ).length,
);
const failedStudents = $derived(
  activeResults.filter((r) => r.status === "failed").length,
);
const avgPercentage = $derived(
  totalStudents > 0
    ? Math.round(
        activeResults.reduce((sum, r) => sum + Number(r.overall_percentage), 0) /
          totalStudents,
      )
    : 0,
);
const passRate = $derived(
  totalStudents > 0 ? Math.round((passedStudents / totalStudents) * 100) : 0,
);

// ── Grid columns & source ───────────────────────────────────
const columns = $derived.by<OrbisColumnDef[]>(() => {
  const cols: OrbisColumnDef[] = [
    selectionColumn(),
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
      width: 95,
      size: 95,
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
      width: 80,
      size: 80,
      type: "percentage",
      align: "end",
      readonly: true,
      sort: true,
    },
  ];

  if (orgContext.isUniversity) {
    cols.push({
      id: "gpa",
      prop: "gpa",
      header: viewMode === "annual" ? "CGPA" : "GPA",
      name: viewMode === "annual" ? "CGPA" : "GPA",
      width: 80,
      size: 80,
      type: "number",
      align: "end",
      readonly: true,
      sort: true,
    });
  }

  cols.push(
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
      width: 110,
      size: 110,
      type: "badge",
      align: "center",
      readonly: true,
      sort: true,
    },
  );
  return cols;
});

const gridSource = $derived(
  activeResults.map((r, i) => ({
    id: r.student_id,
    __rowIndex: i + 1,
    student_id: r.student_id,
    student_name: r.student_name,
    student_number: r.student_number,
    total_score: Number(r.total_score),
    max_possible_score: Number(r.max_possible_score),
    overall_percentage: Number(r.overall_percentage),
    gpa:
      "gpa" in r && r.gpa != null
        ? Number(r.gpa)
        : "cumulative_gpa" in r && r.cumulative_gpa != null
          ? Number(r.cumulative_gpa)
          : "—",
    rank_in_class: r.rank_in_class ?? "—",
    status: r.status,
  })),
);

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "toggle-select" || event.actionId === "select") {
    const id = String(event.row?.id || "");
    if (!id) return;
    const next = new Set(selectedStudentIds);
    if (
      event.selected === false ||
      (event.selected === undefined && next.has(id))
    ) {
      next.delete(id);
    } else {
      next.add(id);
    }
    selectedStudentIds = next;
  } else if (event.actionId === "select-all") {
    const next = new Set(selectedStudentIds);
    for (const id of event.selectedIds ?? []) {
      if (event.selected) next.add(id);
      else next.delete(id);
    }
    selectedStudentIds = next;
  }
}

const targetStudentIds = $derived(
  selectedStudentIds.size > 0
    ? [...selectedStudentIds]
    : activeResults.map((r) => r.student_id),
);

const targetStudentLabels = $derived(
  targetStudentIds.map(
    (id) => activeResults.find((r) => r.student_id === id)?.student_name ?? id,
  ),
);

function openClassPdfReport() {
  pdfReportCode =
    viewMode === "annual"
      ? "rep.academic.student-certificate"
      : "rep.academic.student-term-report-card";
  pdfReportTitle =
    viewMode === "annual"
      ? isRtl
        ? "شهادات النتيجة السنوية للصف"
        : "Class Annual Certificates"
      : isRtl
        ? "كشف وشهادات الفترة للصف"
        : "Class Period Report Cards";
  pdfViewerOpen = true;
}
</script>

<div class="h-full min-h-0 w-full overflow-hidden p-6 font-sans text-start select-none">
  <div class="mx-auto flex h-full min-h-0 max-w-7xl flex-col gap-5">
    <!-- Header -->
    <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/60 pb-4">
      <div>
        <h1 class="text-base font-bold text-foreground">
          {m.academics_nav_results_review()}
        </h1>
        <p class="mt-0.5 text-xs text-muted-foreground">
          {m.academics_results_review_subtitle()}
        </p>
      </div>

      <div class="flex flex-wrap items-center gap-2">
        {#if selectedClassId && activeResults.length > 0}
          <Button
            variant="outline"
            size="sm"
            class="h-8 gap-1.5 text-xs font-medium"
            onclick={openClassPdfReport}
          >
            <Printer size={13} />
            <span>
              {viewMode === "annual"
                ? isRtl
                  ? "كشف الشهادات السنوي (PDF)"
                  : "Annual Report (PDF)"
                : isRtl
                  ? "كشف الفترة (PDF)"
                  : "Period Report (PDF)"}
            </span>
          </Button>

          <Button
            variant="default"
            size="sm"
            class="h-8 gap-1.5 text-xs font-semibold"
            onclick={() => (printDialogOpen = true)}
          >
            <Printer size={13} />
            <span>
              {selectedStudentIds.size > 0
                ? isRtl
                  ? `طباعة شهادات المحددين (${formatNumber(selectedStudentIds.size)})`
                  : `Print Selected (${formatNumber(selectedStudentIds.size)})`
                : isRtl
                  ? `طباعة شهادات الصف (${formatNumber(activeResults.length)})`
                  : `Print Class Certificates (${formatNumber(activeResults.length)})`}
            </span>
          </Button>
        {/if}

        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs font-semibold"
          onclick={() => {
            if (selectedClassId) void loadResults();
            else void sectionsQuery.refetch();
          }}
        >
          {#if isLoadingResults || isLoading}
            <LoadingIndicator size={13} />
          {:else}
            <Refresh size={13} />
          {/if}
          <span>{m.common_retry()}</span>
        </Button>
      </div>
    </div>

    <!-- Filters: Mode (Period vs Annual) + Period/Year + Class -->
    <div class="flex flex-wrap items-center gap-2">
      <div class="inline-flex rounded-md border border-border bg-muted/30 p-0.5">
        <button
          type="button"
          onclick={() => (viewMode = "period")}
          class={`rounded px-3 py-1.5 text-xs font-medium transition-colors ${
            viewMode === "period"
              ? "bg-background text-foreground shadow-xs font-semibold"
              : "text-muted-foreground hover:text-foreground"
          }`}
        >
          {isRtl ? "نتائج فترة (شهري / فصلي)" : "Period / Term"}
        </button>
        <button
          type="button"
          onclick={() => (viewMode = "annual")}
          class={`rounded px-3 py-1.5 text-xs font-medium transition-colors ${
            viewMode === "annual"
              ? "bg-background text-foreground shadow-xs font-semibold"
              : "text-muted-foreground hover:text-foreground"
          }`}
        >
          {isRtl ? "النتيجة السنوية / التراكمية" : "Annual / Cumulative"}
        </button>
      </div>

      {#if viewMode === "period"}
        <PeriodSelect
          bind:value={selectedPeriodId}
          placeholder={m.student_grades_select_period()}
          class="h-9 w-full text-xs sm:w-60"
        />
      {:else}
        <Select
          type="single"
          value={selectedYearId}
          onValueChange={(val) => {
            if (val) selectedYearId = val;
          }}
        >
          <SelectTrigger class="h-9 w-full text-xs sm:w-60">
            {yearNames.get(selectedYearId) ??
              m.student_grades_select_academic_year()}
          </SelectTrigger>
          <SelectContent class="text-xs">
            {#each academicContext.allYears as year (year.id)}
              <SelectItem value={year.id}>
                {isRtl ? year.name_ar || year.name : year.name}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {/if}

      <ClassSelect
        bind:value={selectedClassId}
        disabled={viewMode === "period" ? !selectedPeriodId : !selectedYearId}
        placeholder={m.common_control_select_class()}
        class="h-9 w-full text-xs sm:w-60"
      />
    </div>

    {#if (viewMode === "period" && !selectedPeriodId) || (viewMode === "annual" && !selectedYearId)}
      <Card class="flex flex-col items-center justify-center gap-3 rounded-md border-border/60 bg-card py-16">
        <InfoCircle size={32} class="text-muted-foreground opacity-40" />
        <p class="text-xs text-muted-foreground">
          {viewMode === "period"
            ? m.student_grades_select_period()
            : m.student_grades_select_academic_year()}
        </p>
      </Card>
    {:else if !selectedClassId}
      <!-- Overview mode: show all sections immediately without N+1 calculation bottleneck -->
      {#if isLoading}
        <div class="grid min-h-0 flex-1 grid-cols-1 content-start gap-3 overflow-y-auto md:grid-cols-2 lg:grid-cols-3">
          {#each Array(6) as _, i (i)}
            <Skeleton class="h-24 w-full rounded-md" />
          {/each}
        </div>
      {:else if sections.length === 0}
        <Card class="flex flex-col items-center justify-center gap-3 rounded-md border-border/60 bg-card py-16">
          <Users size={32} class="text-muted-foreground opacity-40" />
          <p class="text-xs text-muted-foreground">
            {m.academics_results_no_sections()}
          </p>
        </Card>
      {:else}
        <div class="grid min-h-0 flex-1 grid-cols-1 content-start gap-3 overflow-y-auto md:grid-cols-2 lg:grid-cols-3">
          {#each sections as section (section.id)}
            <button
              type="button"
              class="text-start"
              onclick={() => {
                selectedClassId = String(section.id);
              }}
            >
              <Card class="cursor-pointer rounded-md border-border p-4 transition-colors hover:border-primary/40">
                <div class="mb-2 flex items-center justify-between gap-2">
                  <span class="truncate text-xs font-semibold text-foreground">
                    {isRtl ? section.name_ar || section.name : section.name}
                  </span>
                  <Badge variant="outline" class="h-5 rounded px-2 text-[10px] font-medium">
                    {isRtl ? "عرض النتائج" : "View Results"}
                  </Badge>
                </div>
                <p class="text-[11px] text-muted-foreground">
                  {isRtl
                    ? "اضغط لعرض نتائج الطلاب وطباعة الشهادات"
                    : "Click to load student results & print certificates"}
                </p>
              </Card>
            </button>
          {/each}
        </div>
      {/if}
    {:else}
      <!-- Detail mode: show results for selected class -->
      <div class="flex min-h-0 flex-1 flex-col gap-4">
        {#if isLoadingResults}
          <div class="flex flex-1 items-center justify-center py-16">
            <LoadingIndicator size="lg" label={m.common_control_loading()} />
          </div>
        {:else if resultError}
          <Card class="flex flex-col items-center justify-center gap-3 rounded-md border-destructive/20 bg-destructive/5 p-6 py-12">
            <InfoCircle size={28} class="text-destructive opacity-80" />
            <p class="text-xs font-medium text-destructive">{resultError}</p>
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
        {:else if activeResults.length === 0}
          <Card class="flex flex-col items-center justify-center gap-3 rounded-md border-border/60 bg-card py-16">
            <DocumentCheck size={32} class="text-muted-foreground opacity-40" />
            <p class="text-xs text-muted-foreground">
              {m.academics_results_empty_section()}
            </p>
          </Card>
        {:else}
          <!-- Unified Summary Strip -->
          <div class="grid shrink-0 grid-cols-2 divide-y divide-border rounded-md border border-border bg-card sm:grid-cols-4 sm:divide-x sm:divide-y-0 rtl:sm:divide-x-reverse">
            <div class="px-4 py-3">
              <p class="text-[11px] text-muted-foreground">
                {m.academics_results_stat_total_students()}
              </p>
              <p class="mt-1 text-base font-bold tabular-nums text-foreground">
                {formatNumber(totalStudents)}
              </p>
            </div>
            <div class="px-4 py-3">
              <p class="text-[11px] text-muted-foreground">
                {m.academics_results_stat_pass_rate()}
              </p>
              <p class="mt-1 text-base font-bold tabular-nums text-foreground">
                {formatPercent(passRate)}
              </p>
            </div>
            <div class="px-4 py-3">
              <p class="text-[11px] text-muted-foreground">
                {m.academics_results_stat_average()}
              </p>
              <p class="mt-1 text-base font-bold tabular-nums text-foreground">
                {formatPercent(avgPercentage)}
              </p>
            </div>
            <div class="px-4 py-3">
              <p class="text-[11px] text-muted-foreground">
                {m.academics_results_stat_failed()}
              </p>
              <p
                class="mt-1 text-base font-bold tabular-nums"
                class:text-destructive={failedStudents > 0}
                class:text-foreground={failedStudents === 0}
              >
                {formatNumber(failedStudents)}
              </p>
            </div>
          </div>

          <!-- Results grid with multi-selection for group certificate printing -->
          <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
            <div class="min-h-0 flex-1">
              <OrbisGrid
                source={gridSource}
                {columns}
                selectedRowIds={selectedStudentIds}
                onaction={handleGridAction}
                rtl={isRtl}
                readonly
              />
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<PrintDialog
  bind:open={printDialogOpen}
  entityType="student"
  entityIds={targetStudentIds}
  entityLabels={targetStudentLabels}
/>

<PdfReportViewer
  bind:open={pdfViewerOpen}
  templateId={pdfReportCode}
  parameters={{
    class_id: selectedClassId,
    period_id: viewMode === "period" ? selectedPeriodId : undefined,
    year_id: selectedYearId,
    calendar_id: selectedYearId,
  }}
  reportTitle={pdfReportTitle}
/>
