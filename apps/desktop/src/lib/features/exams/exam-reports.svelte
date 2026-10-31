<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Card } from "@orbis/ui/card";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { academicContext } from "$lib/stores";
import {
  createPeriodsQuery,
  createRoomsQuery,
  createSectionsQuery,
} from "$lib/features/academics";
import { createExamSessionsQuery } from "$lib/features/exams/queries";
import { createReportDefinitionsQuery } from "$lib/features/reporting/queries";
import {
  extractErrorMessage,
  type ExamSessionDto,
  type ReportDefinitionDto,
} from "$lib/api";
import { localizeHref } from "$lib/paraglide/runtime";
import { openPrintDialog } from "$lib/stores/print.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";

let { orgId }: { orgId: string } = $props();

// ── Search & Filter State ─────────────────────────────────────────────
let searchQuery = $state("");
let selectedCategory = $state<string>("all");
let selectedCode = $state<string>("");

// ── Parameters State ──────────────────────────────────────────────────
let selectedSessionId = $state<string>("");
let selectedRoomId = $state<string>("");
let selectedSectionId = $state<string>("");
let selectedDate = $state<string>("");
let selectedFormat = $state<"PDF" | "HTML" | "XLSX" | "CSV">("PDF");

// ── Report Viewer Dialog ──────────────────────────────────────────────
let isViewerOpen = $state(false);
let viewerParameters = $state<Record<string, any>>({});

// ── Queries ───────────────────────────────────────────────────────────
const periodsQuery = createPeriodsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);

// Active academic year & period
const activeYearName = $derived(academicContext.yearLabelAr);
const activePeriod = $derived(
  periods.find(
    (p) =>
      p.status === "active" ||
      p.status === "active_teaching" ||
      p.status === "grade_submission",
  ) || periods[0],
);
const activePeriodId = $derived(activePeriod ? String(activePeriod.id) : "");
const activePeriodName = $derived(activePeriod ? activePeriod.name : null);

// Grade Entry Lock Status
// Dynamic Real Exam Reports from Catalog / Database
const examReportsQuery = createReportDefinitionsQuery(
  () => orgId,
  () => ({ domain: "exam" }),
);
const allExamReports = $derived<ReportDefinitionDto[]>(
  examReportsQuery.data ?? [],
);

// Exam Sessions Query
const examSessionsQuery = createExamSessionsQuery(() => activePeriodId);
const examSessions = $derived<ExamSessionDto[]>(examSessionsQuery.data ?? []);

// Rooms & Sections Queries
const roomsQuery = createRoomsQuery(() => orgId);
const rooms = $derived(roomsQuery.data ?? []);

const sectionsQuery = createSectionsQuery(() => orgId);
const sections = $derived(sectionsQuery.data ?? []);

const isLoading = $derived(
  periodsQuery.isLoading ||
    examReportsQuery.isLoading ||
    examSessionsQuery.isLoading,
);

const error = $derived(
  periodsQuery.error || examReportsQuery.error || examSessionsQuery.error,
);

// Categories
const categoryTabs = [
  { id: "all", label: () => m.exam_reports_category_all() },
  { id: "seating", label: () => m.exam_reports_category_seating() },
  { id: "halls", label: () => m.exam_reports_category_halls() },
  { id: "proctors", label: () => m.exam_reports_category_proctors() },
  { id: "control", label: () => m.exam_reports_category_control() },
];

// Filtered Reports
const filteredReports = $derived.by(() => {
  return allExamReports.filter((report) => {
    const q = searchQuery.toLowerCase().trim();
    const matchesSearch =
      !q ||
      report.name.toLowerCase().includes(q) ||
      (report.name_ar && report.name_ar.toLowerCase().includes(q)) ||
      (report.description && report.description.toLowerCase().includes(q)) ||
      (report.description_ar &&
        report.description_ar.toLowerCase().includes(q)) ||
      report.code.toLowerCase().includes(q);

    const matchesCat =
      selectedCategory === "all" ||
      report.category.toLowerCase() === selectedCategory.toLowerCase();

    return matchesSearch && matchesCat;
  });
});

// Reactively select the first report if none or invalid
$effect(() => {
  if (filteredReports.length > 0) {
    if (
      !selectedCode ||
      !filteredReports.some((r) => r.code === selectedCode)
    ) {
      selectedCode = filteredReports[0].code;
    }
  }
});

const selectedReport = $derived<ReportDefinitionDto | null>(
  allExamReports.find((r) => r.code === selectedCode) ??
    (filteredReports[0] || null),
);

function handleSelectReport(code: string) {
  selectedCode = code;
}

function buildParameters(): Record<string, any> {
  const params: Record<string, any> = {
    org_id: orgId,
    period_id: activePeriodId || undefined,
  };
  if (selectedSessionId) params.session_id = selectedSessionId;
  if (selectedRoomId) params.room_id = selectedRoomId;
  if (selectedSectionId) params.section_id = selectedSectionId;
  if (selectedDate) params.date = selectedDate;
  return params;
}

function handlePreviewAndPrint() {
  if (!selectedReport) return;
  viewerParameters = buildParameters();
  isViewerOpen = true;
}

function handlePrintTemplate() {
  if (!selectedReport) return;
  const session = examSessions.find((s) => String(s.id) === selectedSessionId);
  const room = rooms.find((r) => String(r.id) === selectedRoomId);
  const section = sections.find((sec) => String(sec.id) === selectedSectionId);

  const examContext = {
    report: {
      code: selectedReport.code,
      title: selectedReport.name_ar || selectedReport.name,
      description: selectedReport.description_ar || selectedReport.description,
      category: selectedReport.category,
      domain: selectedReport.domain,
    },
    exam: {
      sessionId: selectedSessionId || null,
      sessionName:
        session?.title_ar || session?.title || activePeriodName || "",
      hallId: selectedRoomId || null,
      hallName: room?.name_ar || room?.name || "",
      classId: selectedSectionId || null,
      className: section?.name_ar || section?.name || "",
      examDate: selectedDate || "",
    },
    AcademicCalendar: {
      name: activeYearName || "",
      periodName: activePeriodName || "",
    },
  };

  openPrintDialog(
    "exam",
    [selectedReport.code],
    [selectedReport.name_ar || selectedReport.name],
    {
      documentType: selectedReport.code,
      contexts: [examContext],
    },
  );
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden bg-background p-3 font-sans text-start select-none sm:p-4 lg:p-6">
  {#if isLoading}
    <div class="grid min-h-0 flex-1 grid-cols-1 gap-3 lg:grid-cols-2">
      <Skeleton class="h-full min-h-36 w-full rounded-xl" />
      <Skeleton class="h-full min-h-36 w-full rounded-xl" />
    </div>
  {:else if error}
    <div class="flex min-h-0 flex-1 flex-col items-center justify-center gap-3 rounded-xl border border-border/60 bg-card p-8 text-center">
      <p class="text-sm text-destructive">{extractErrorMessage(error)}</p>
      <Button variant="outline" class="h-9 px-4 text-xs" onclick={() => examReportsQuery.refetch()}>
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <div class="grid min-h-0 flex-1 w-full grid-cols-1 grid-rows-[minmax(0,0.8fr)_minmax(0,1.2fr)] items-stretch gap-3 overflow-hidden lg:grid-cols-[minmax(0,1.15fr)_minmax(18rem,0.85fr)] lg:grid-rows-1 lg:gap-4">
      
      <Card class="flex min-h-0 flex-col gap-3 overflow-hidden p-3 shadow-xs sm:p-4">
        
        <div class="shrink-0">
          <Input
            type="text"
            bind:value={searchQuery}
            placeholder={m.exam_reports_search_placeholder()}
            class="h-9 border-border/60 bg-background text-xs"
          />
        </div>

        <!-- Category filter pills -->
        <div class="flex shrink-0 items-center gap-1.5 overflow-x-auto pb-1 scrollbar-hide">
          {#each categoryTabs as tab}
            <button
              type="button"
              onclick={() => (selectedCategory = tab.id)}
              class="px-2.5 py-1 rounded-md text-[10px] font-semibold transition-colors duration-150 whitespace-nowrap cursor-pointer border {selectedCategory === tab.id ? 'bg-primary text-primary-foreground border-primary' : 'bg-muted/40 text-muted-foreground border-border/40 hover:bg-muted/70 hover:text-foreground'}"
            >
              {tab.label()}
            </button>
          {/each}
        </div>

        <!-- Reports list -->
        <div class="flex min-h-0 flex-1 flex-col gap-1 overflow-y-auto pe-1 scrollbar-hide">
          {#if filteredReports.length === 0}
            <div class="rounded-lg border border-dashed border-border/60 px-4 py-10 text-center text-sm text-muted-foreground">
              {m.exam_reports_no_matches()}
            </div>
          {:else}
            {#each filteredReports as report (report.code)}
              <button
                type="button"
                onclick={() => handleSelectReport(report.code)}
                class="w-full min-w-0 rounded-lg border px-3 py-3 text-start transition-colors
                  {selectedCode === report.code ? 'border-primary/40 bg-primary/5' : 'border-transparent hover:border-border/60 hover:bg-muted/40'}"
              >
                <span class="block truncate text-sm font-medium text-foreground">
                  {report.name_ar || report.name}
                </span>
              </button>
            {/each}
          {/if}
        </div>
      </Card>

      <Card class="flex min-h-0 flex-col gap-4 overflow-y-auto p-4 scrollbar-hide shadow-xs sm:p-5">
        
        {#if selectedReport}
          <div class="flex flex-col gap-4">
            <span class="text-sm font-semibold text-foreground">{selectedReport.name_ar || selectedReport.name}</span>

            <div class="flex flex-col gap-3">
              <!-- Session Selector -->
              {#if examSessions.length > 0}
                <div class="space-y-1">
                  <Label class="text-xs font-medium text-muted-foreground">{m.exam_reports_session_label()}</Label>
                  <Select type="single" value={selectedSessionId} onValueChange={(v) => (selectedSessionId = v ?? "")}>
                    <SelectTrigger class="h-8 text-xs bg-background/50">
                      <span>{examSessions.find((s) => String(s.id) === selectedSessionId)?.title_ar || examSessions.find((s) => String(s.id) === selectedSessionId)?.title || m.exam_reports_all_sessions()}</span>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="" class="text-xs">{m.exam_reports_all_sessions()}</SelectItem>
                      {#each examSessions as sess (sess.id)}
                        <SelectItem value={String(sess.id)} class="text-xs">{sess.title_ar || sess.title}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                </div>
              {/if}

              <!-- Room / Hall Selector -->
              {#if rooms.length > 0}
                <div class="space-y-1">
                  <Label class="text-xs font-medium text-muted-foreground">{m.exam_reports_room_committee()}</Label>
                  <Select type="single" value={selectedRoomId} onValueChange={(v) => (selectedRoomId = v ?? "")}>
                    <SelectTrigger class="h-8 text-xs bg-background/50">
                      <span>{rooms.find((r) => String(r.id) === selectedRoomId)?.name_ar || rooms.find((r) => String(r.id) === selectedRoomId)?.name || m.exam_reports_all_rooms_committees()}</span>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="" class="text-xs">{m.exam_reports_all_rooms_committees()}</SelectItem>
                      {#each rooms as rm (rm.id)}
                        <SelectItem value={String(rm.id)} class="text-xs">{rm.name_ar || rm.name}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                </div>
              {/if}

              <!-- Section / Class Selector -->
              {#if sections.length > 0}
                <div class="space-y-1">
                  <Label class="text-xs font-medium text-muted-foreground">{m.exam_reports_class_section()}</Label>
                  <Select type="single" value={selectedSectionId} onValueChange={(v) => (selectedSectionId = v ?? "")}>
                    <SelectTrigger class="h-8 text-xs bg-background/50">
                      <span>{sections.find((s) => String(s.id) === selectedSectionId)?.name_ar || sections.find((s) => String(s.id) === selectedSectionId)?.name || m.exam_reports_all_classes_sections()}</span>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="" class="text-xs">{m.exam_reports_all_classes_sections()}</SelectItem>
                      {#each sections as sec (sec.id)}
                        <SelectItem value={String(sec.id)} class="text-xs">{sec.name_ar || sec.name}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                </div>
              {/if}

              <!-- Date Filter -->
              <div class="space-y-1">
                <Label class="text-xs font-medium text-muted-foreground">{m.exam_reports_session_date()}</Label>
                <Input
                  type="date"
                  bind:value={selectedDate}
                  class="h-8 text-xs bg-background/50 font-mono"
                />
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="pt-3 border-t border-border/40 flex flex-col gap-2">
              <Button
                variant="lime"
                size="sm" 
                class="h-9 w-full justify-center text-xs font-semibold"
                onclick={handlePreviewAndPrint}
              >
                {m.exam_reports_preview_print_pdf()}
              </Button>

              <Button 
                variant="outline" 
                size="sm" 
                class="h-9 w-full justify-center border-border/60 text-xs font-semibold"
                onclick={handlePrintTemplate}
              >
                {m.exam_reports_smart_template_print()}
              </Button>
            </div>
          </div>
        {/if}

      </Card>

    </div>
  {/if}
</div>

<!-- Dynamic PDF Report Viewer Modal -->
{#if selectedReport}
  <PdfReportViewer
    bind:open={isViewerOpen}
    templateId={selectedReport.code}
    parameters={viewerParameters}
    reportTitle={selectedReport.name_ar || selectedReport.name}
  />
{/if}
