<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import { LoadingIndicator } from "@orbis/ui";
  import { Button } from "@orbis/ui/button";
  import { Badge } from "@orbis/ui/badge";
  import {
    Card,
    CardAction,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "@orbis/ui/card";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "@orbis/ui/select";
  import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
  import {
    AcademicCap,
    BookOpen,
    ChartBar,
    CheckCircle,
    DocumentChartBar,
    DocumentCheck,
    InfoCircle,
    Printer,
    Refresh,
  } from "@orbis/icons";
  import {
    api,
    extractErrorMessage,
    type GradeDto,
    type StudentTermReportCard,
    type StudentAnnualReportCard,
    type StudentCertificateReportCard,
  } from "$lib/api";
  import { academicContext } from "$lib/stores";
  import { getLocale } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";
  import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";

  let {
    orgId,
    studentId,
    classId,
  }: {
    orgId: string;
    studentId: string;
    classId?: string;
  } = $props();

  const isRtl = $derived(getLocale() === "ar");

  // View mode: "records" | "term" | "annual" | "certificate"
  let activeTab = $state<"records" | "term" | "annual" | "certificate">("records");

  // Selection states
  let selectedPeriodId = $state("all");
  let selectedTermPeriodId = $state(
    academicContext.effectivePeriodId || academicContext.allPeriods[0]?.id || "",
  );
  let selectedCalendarId = $state(
    academicContext.effectiveYearId || academicContext.allYears[0]?.id || "",
  );
  let selectedCertType = $state<"secondary_thanawiya" | "basic_grade9">("secondary_thanawiya");

  // Dialog states for printing
  let showPrintCertificateDialog = $state(false);
  let printTemplateId = $state("academic.student-certificate-yemeni");
  let printTitle = $state("");

  // ── 1. Raw Assessment Records Query ─────────────────────────
  const gradesQuery = createQuery(() => ({
    queryKey: ["assessment-grades", "student", orgId, studentId],
    queryFn: () => api.listGrades({ studentId }),
    enabled: !!orgId && !!studentId && activeTab === "records",
  }));

  // ── 2. Term Report Card Query ────────────────────────────────
  const termReportQuery = createQuery(() => ({
    queryKey: [
      "assessment-term-results",
      studentId,
      selectedTermPeriodId,
      classId ?? "",
    ],
    queryFn: () =>
      api.getStudentTermResults(
        studentId,
        selectedTermPeriodId,
        classId || undefined,
      ),
    enabled:
      !!orgId &&
      !!studentId &&
      !!selectedTermPeriodId &&
      activeTab === "term",
  }));

  // ── 3. Annual Report Query ───────────────────────────────────
  const annualReportQuery = createQuery(() => ({
    queryKey: [
      "assessment-annual-results",
      studentId,
      selectedCalendarId,
      classId ?? "",
    ],
    queryFn: () =>
      api.getStudentAnnualResults(
        studentId,
        selectedCalendarId,
        classId || undefined,
      ),
    enabled:
      !!orgId &&
      !!studentId &&
      !!selectedCalendarId &&
      activeTab === "annual",
  }));

  // ── 4. Certificate Results Query ─────────────────────────────
  const certificateReportQuery = createQuery(() => ({
    queryKey: [
      "assessment-certificate-results",
      studentId,
      selectedCalendarId,
      selectedCertType,
      classId ?? "",
    ],
    queryFn: () =>
      api.getStudentCertificateResults(
        studentId,
        selectedCalendarId,
        selectedCertType,
        classId || undefined,
      ),
    enabled:
      !!orgId &&
      !!studentId &&
      !!selectedCalendarId &&
      activeTab === "certificate",
  }));

  const periodNames = $derived(
    new Map(
      academicContext.allPeriods.map((period) => [
        period.id,
        period.name_ar || period.name,
      ]),
    ),
  );

  const yearNames = $derived(
    new Map(
      academicContext.allYears.map((year) => [
        year.id,
        year.name_ar || year.name,
      ]),
    ),
  );

  // ── Helper: Format Status ────────────────────────────────────
  function getStatusBadge(status?: string | null) {
    const s = String(status || "").toLowerCase();
    if (s.includes("honor") || s === "passedwithhonors") {
      return {
        label: isRtl ? "ناجح بتفوق" : "Passed with Honors",
        variant: "default" as const,
        color: "bg-emerald-600/15 text-emerald-700 dark:text-emerald-400 border-emerald-600/30",
      };
    }
    if (s.includes("pass") || s === "passed") {
      return {
        label: isRtl ? "ناجح" : "Passed",
        variant: "default" as const,
        color: "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border-emerald-500/30",
      };
    }
    if (s.includes("conditional") || s.includes("makeup") || s.includes("second")) {
      return {
        label: isRtl ? "دور ثان / مكمل" : "Conditional / Retake",
        variant: "secondary" as const,
        color: "bg-amber-500/15 text-amber-700 dark:text-amber-400 border-amber-500/30",
      };
    }
    if (s.includes("fail") || s === "failed") {
      return {
        label: isRtl ? "راسب" : "Failed",
        variant: "destructive" as const,
        color: "bg-rose-500/15 text-rose-600 dark:text-rose-400 border-rose-500/30",
      };
    }
    return {
      label: isRtl ? "قيد المعالجة" : "Incomplete / Pending",
      variant: "secondary" as const,
      color: "bg-muted text-muted-foreground border-border",
    };
  }

  function getAppreciation(percentage?: number | null) {
    if (percentage == null) return "—";
    if (percentage >= 90) return isRtl ? "ممتاز" : "Excellent";
    if (percentage >= 80) return isRtl ? "جيد جداً" : "Very Good";
    if (percentage >= 65) return isRtl ? "جيد" : "Good";
    if (percentage >= 50) return isRtl ? "مقبول" : "Pass";
    return isRtl ? "ضعيف" : "Fail";
  }

  // ── Print Triggers ───────────────────────────────────────────
  function triggerPrintCertificate() {
    printTemplateId = "academic.student-certificate-yemeni";
    printTitle = isRtl
      ? "الشهادة المدرسية الرسمية المعتمدة"
      : "Official Academic Certificate";
    showPrintCertificateDialog = true;
  }

  function triggerPrintTermReport() {
    printTemplateId = "academic.student-term-report-card";
    printTitle = isRtl
      ? "كشف درجات الطالب للفصل الدراسي"
      : "Student Term Report Card";
    showPrintCertificateDialog = true;
  }

  // ── Grid Configuration for Raw Records ──────────────────────
  const filteredGrades = $derived(
    (gradesQuery.data ?? []).filter(
      (grade) =>
        selectedPeriodId === "all" || grade.period_id === selectedPeriodId,
    ),
  );

  const rows = $derived(
    filteredGrades.map((grade: GradeDto) => ({
      period: periodNames.get(grade.period_id) ?? grade.period_id,
      offering_id: grade.section_subject_id,
      component_id: grade.component_id ?? "—",
      score: Number(grade.score),
      maximum: Number(grade.max_score),
      weight: grade.weight == null ? "—" : Number(grade.weight),
      letter_grade: grade.letter_grade ?? "—",
      record_state: grade.is_exempt
        ? (isRtl ? "معفى" : "Exempt")
        : grade.is_excused
          ? (isRtl ? "معذور" : "Excused")
          : grade.is_final
            ? (isRtl ? "نهائي" : "Final")
            : (isRtl ? "مرصود" : "Recorded"),
      updated_at: grade.updated_at,
    })),
  );

  const columns: OrbisColumnDef[] = [
    {
      id: "period",
      prop: "period",
      header: isRtl ? "الفصل الدراسي" : "Period",
      name: isRtl ? "الفصل الدراسي" : "Period",
      width: 170,
      size: 170,
    },
    {
      id: "offering_id",
      prop: "offering_id",
      header: isRtl ? "المقرر / المادة" : "Course or offering",
      name: isRtl ? "المقرر / المادة" : "Course or offering",
      width: 230,
      size: 230,
      pin: "left",
    },
    {
      id: "component_id",
      prop: "component_id",
      header: isRtl ? "بند التقييم" : "Assessment component",
      name: isRtl ? "بند التقييم" : "Assessment component",
      width: 190,
      size: 190,
    },
    {
      id: "score",
      prop: "score",
      header: isRtl ? "الدرجة" : "Score",
      name: isRtl ? "الدرجة" : "Score",
      width: 95,
      size: 95,
      align: "end",
      type: "number",
    },
    {
      id: "maximum",
      prop: "maximum",
      header: isRtl ? "النهاية الكبرى" : "Maximum",
      name: isRtl ? "النهاية الكبرى" : "Maximum",
      width: 105,
      size: 105,
      align: "end",
      type: "number",
    },
    {
      id: "weight",
      prop: "weight",
      header: isRtl ? "الوزن" : "Weight",
      name: isRtl ? "الوزن" : "Weight",
      width: 90,
      size: 90,
      align: "end",
      type: "number",
    },
    {
      id: "letter_grade",
      prop: "letter_grade",
      header: isRtl ? "التقدير" : "Grade",
      name: isRtl ? "التقدير" : "Grade",
      width: 90,
      size: 90,
    },
    {
      id: "record_state",
      prop: "record_state",
      header: isRtl ? "حالة القيد" : "Record state",
      name: isRtl ? "حالة القيد" : "Record state",
      width: 115,
      size: 115,
    },
    {
      id: "updated_at",
      prop: "updated_at",
      header: isRtl ? "آخر تحديث" : "Last updated",
      name: isRtl ? "آخر تحديث" : "Last updated",
      width: 180,
      size: 180,
    },
  ];
</script>

<div class="flex flex-col gap-4 text-xs text-start font-sans print:p-0">
  <!-- Mode Selection Tabs & Action Bar -->
  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/80 pb-3">
    <div class="inline-flex rounded-lg border border-border bg-muted/30 p-1">
      <button
        type="button"
        class={`inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-semibold transition-all ${
          activeTab === "records"
            ? "bg-background text-foreground shadow-xs"
            : "text-muted-foreground hover:text-foreground"
        }`}
        onclick={() => (activeTab = "records")}
      >
        <BookOpen size={13} class={activeTab === "records" ? "text-primary" : ""} />
        <span>{isRtl ? "سجل التقييمات" : "Assessment Records"}</span>
      </button>

      <button
        type="button"
        class={`inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-semibold transition-all ${
          activeTab === "term"
            ? "bg-background text-foreground shadow-xs"
            : "text-muted-foreground hover:text-foreground"
        }`}
        onclick={() => (activeTab = "term")}
      >
        <DocumentChartBar size={13} class={activeTab === "term" ? "text-primary" : ""} />
        <span>{isRtl ? "كشف درجات الفصل" : "Term Report Card"}</span>
      </button>

      <button
        type="button"
        class={`inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-semibold transition-all ${
          activeTab === "annual"
            ? "bg-background text-foreground shadow-xs"
            : "text-muted-foreground hover:text-foreground"
        }`}
        onclick={() => (activeTab = "annual")}
      >
        <ChartBar size={13} class={activeTab === "annual" ? "text-primary" : ""} />
        <span>{isRtl ? "النتيجة السنوية" : "Annual Results"}</span>
      </button>

      <button
        type="button"
        class={`inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-semibold transition-all ${
          activeTab === "certificate"
            ? "bg-background text-foreground shadow-xs"
            : "text-muted-foreground hover:text-foreground"
        }`}
        onclick={() => (activeTab = "certificate")}
      >
        <AcademicCap size={13} class={activeTab === "certificate" ? "text-primary" : ""} />
        <span>{isRtl ? "شهادة إتمام المرحلة" : "Certificate Results"}</span>
      </button>
    </div>

    <!-- Contextual Actions -->
    <div class="flex items-center gap-2">
      {#if activeTab === "certificate"}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs font-semibold text-primary border-primary/30 hover:bg-primary/5"
          onclick={triggerPrintCertificate}
        >
          <Printer size={13} />
          <span>{isRtl ? "طباعة الشهادة الرسمية" : "Print Official Certificate"}</span>
        </Button>
      {:else if activeTab === "term"}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs font-semibold"
          onclick={triggerPrintTermReport}
        >
          <Printer size={13} />
          <span>{isRtl ? "طباعة كشف الفصل" : "Print Term Report"}</span>
        </Button>
      {/if}
    </div>
  </div>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 1: RAW ASSESSMENT RECORDS                               -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {#if activeTab === "records"}
    <Card class="overflow-hidden">
      <CardHeader class="flex flex-row flex-wrap items-center justify-between gap-3 border-b border-border px-4 py-3">
        <div class="flex items-center gap-2">
          <BookOpen size={16} class="text-primary" />
          <div class="flex flex-col">
            <CardTitle class="text-xs font-bold uppercase tracking-wider">
              {isRtl ? "سجل القيود والتقييمات" : "Assessment records"}
            </CardTitle>
            <CardDescription class="text-[10px]">
              {filteredGrades.length} {isRtl ? "سجل مرصود" : "records"}
            </CardDescription>
          </div>
        </div>

        <CardAction>
          <div class="flex items-center gap-2 print:hidden">
            <Select
              type="single"
              value={selectedPeriodId}
              onValueChange={(value) => {
                if (value) selectedPeriodId = value;
              }}
            >
              <SelectTrigger class="h-8 min-w-[160px] text-xs">
                {selectedPeriodId === "all"
                  ? (isRtl ? "جميع الفترات" : "All periods")
                  : periodNames.get(selectedPeriodId) ?? (isRtl ? "اختر الفترة" : "Select period")}
              </SelectTrigger>
              <SelectContent class="text-xs">
                <SelectItem value="all">{isRtl ? "جميع الفترات" : "All periods"}</SelectItem>
                {#each academicContext.allPeriods as period (period.id)}
                  <SelectItem value={period.id}>
                    {period.name_ar || period.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => gradesQuery.refetch()}
              disabled={gradesQuery.isFetching}
            >
              {#if gradesQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardAction>
      </CardHeader>

      <CardContent class="p-0">
        {#if gradesQuery.isLoading}
          <div class="flex min-h-64 flex-col items-center justify-center gap-2">
            <LoadingIndicator size={24} />
            <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
          </div>
        {:else if gradesQuery.error}
          <div class="flex min-h-64 flex-col items-center justify-center p-6 text-center text-destructive">
            <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
            <p class="text-xs font-semibold">{m.common_load_error()}</p>
            <p class="mt-1 text-[10px] text-muted-foreground">
              {extractErrorMessage(gradesQuery.error)}
            </p>
            <Button
              variant="outline"
              size="sm"
              class="mt-4"
              onclick={() => gradesQuery.refetch()}
            >
              {m.common_retry()}
            </Button>
          </div>
        {:else}
          <div class="h-[460px] w-full">
            <OrbisGrid
              {columns}
              source={rows}
              loading={gradesQuery.isFetching}
              clientQuery
              emptyTitle={m.academics_grades_empty_title()}
              emptyDescription={m.academics_grades_empty_subtitle()}
              rtl={isRtl}
              readonly
            />
          </div>
        {/if}
      </CardContent>
    </Card>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 2: TERM REPORT CARD                                     -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "term"}
    <div class="flex flex-col gap-4">
      <!-- Period Selector Header -->
      <Card>
        <CardContent class="flex flex-wrap items-center justify-between gap-3 p-3">
          <div class="flex items-center gap-3">
            <div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <DocumentChartBar size={18} />
            </div>
            <div>
              <h2 class="text-xs font-bold text-foreground">
                {isRtl ? "كشف درجات الفصل الدراسي" : "Official Term Report Card"}
              </h2>
              <p class="text-[11px] text-muted-foreground">
                {isRtl
                  ? "محصلة أعمال الفصل واختبار نهاية الفصل لجميع المواد"
                  : "Coursework and final term exam breakdown per subject"}
              </p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <Select
              type="single"
              value={selectedTermPeriodId}
              onValueChange={(val) => {
                if (val) selectedTermPeriodId = val;
              }}
            >
              <SelectTrigger class="h-8 min-w-[180px] text-xs">
                {periodNames.get(selectedTermPeriodId) ?? (isRtl ? "اختر الفصل" : "Select Term")}
              </SelectTrigger>
              <SelectContent class="text-xs">
                {#each academicContext.allPeriods as period (period.id)}
                  <SelectItem value={period.id}>
                    {period.name_ar || period.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => termReportQuery.refetch()}
              disabled={termReportQuery.isFetching}
            >
              {#if termReportQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardContent>
      </Card>

      {#if termReportQuery.isLoading}
        <div class="flex min-h-64 flex-col items-center justify-center gap-2 rounded-xl border border-border bg-card p-8">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if termReportQuery.error}
        <div class="flex min-h-64 flex-col items-center justify-center rounded-xl border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
          <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="mt-1 text-[10px] text-muted-foreground">
            {extractErrorMessage(termReportQuery.error)}
          </p>
        </div>
      {:else if termReportQuery.data}
        {@const termReport = termReportQuery.data}
        {@const badge = getStatusBadge(termReport.status)}

        <!-- Performance Summary KPIs -->
        <div class="grid grid-cols-2 gap-3 sm:grid-cols-5">
          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "المجموع الكلي" : "Total Score"}
            </span>
            <div class="mt-1 flex items-baseline gap-1">
              <span class="text-lg font-bold text-foreground">
                {Number(termReport.total_score).toFixed(1)}
              </span>
              <span class="text-xs text-muted-foreground">
                / {Number(termReport.max_possible_score).toFixed(0)}
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "النسبة المئوية" : "Percentage"}
            </span>
            <div class="mt-1 flex items-baseline gap-1">
              <span class="text-lg font-bold text-primary">
                {Number(termReport.overall_percentage).toFixed(1)}%
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "التقدير العام" : "Appreciation"}
            </span>
            <div class="mt-1 text-sm font-bold text-foreground">
              {getAppreciation(Number(termReport.overall_percentage))}
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "الحالة والقرار" : "Academic Decision"}
            </span>
            <div class="mt-1">
              <span class={`inline-flex items-center px-2 py-0.5 rounded text-[11px] font-semibold border ${badge.color}`}>
                {badge.label}
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "الترتيب على الصف" : "Rank in Class"}
            </span>
            <div class="mt-1 text-base font-bold text-foreground">
              {termReport.rank_in_class ? `#${termReport.rank_in_class}` : "—"}
              {#if termReport.total_students_in_class}
                <span class="text-xs font-normal text-muted-foreground">
                  ({isRtl ? "من" : "of"} {termReport.total_students_in_class})
                </span>
              {/if}
            </div>
          </div>
        </div>

        <!-- Subject Results Table -->
        <Card class="overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full text-xs text-start">
              <thead class="bg-muted/40 border-b border-border text-[11px] font-semibold text-muted-foreground">
                <tr>
                  <th class="px-4 py-2.5 text-start">{isRtl ? "المادة الدراسية" : "Subject"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "محصلة أعمال (20)" : "Coursework (20)"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "اختبار الفصل (30)" : "Exam (30)"}</th>
                  <th class="px-3 py-2.5 text-center font-bold text-foreground">{isRtl ? "المجموع (50)" : "Total (50)"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "النسبة" : "Percentage"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "التقدير" : "Grade"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "النتيجة" : "Status"}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/60">
                {#each termReport.subjects as subj (subj.subject_id)}
                  {@const subjPassed = String(subj.status).toLowerCase().includes("pass")}
                  <tr class="hover:bg-muted/20 transition-colors">
                    <td class="px-4 py-2.5 font-medium text-foreground">
                      {isRtl ? (subj.subject_name_ar || subj.subject_name) : subj.subject_name}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {Number(subj.monthly_contribution).toFixed(1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {subj.term_final_exam != null ? Number(subj.term_final_exam).toFixed(1) : "—"}
                    </td>
                    <td class="px-3 py-2.5 text-center font-bold text-foreground">
                      {Number(subj.term_total).toFixed(1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {Number(subj.percentage).toFixed(0)}%
                    </td>
                    <td class="px-3 py-2.5 text-center font-medium">
                      {subj.letter_grade ?? getAppreciation(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center">
                      <span class={`inline-block px-2 py-0.5 rounded text-[10px] font-semibold ${
                        subjPassed
                          ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
                          : "bg-rose-500/10 text-rose-600 dark:text-rose-400"
                      }`}>
                        {subjPassed ? (isRtl ? "ناجح" : "Passed") : (isRtl ? "راسب" : "Failed")}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card>
      {/if}
    </div>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 3: ANNUAL REPORT CARD                                   -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "annual"}
    <div class="flex flex-col gap-4">
      <!-- Calendar Selector Header -->
      <Card>
        <CardContent class="flex flex-wrap items-center justify-between gap-3 p-3">
          <div class="flex items-center gap-3">
            <div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <ChartBar size={18} />
            </div>
            <div>
              <h2 class="text-xs font-bold text-foreground">
                {isRtl ? "النتيجة السنوية العامة (شهادة النقل)" : "Annual Academic Results (Promotion Certificate)"}
              </h2>
              <p class="text-[11px] text-muted-foreground">
                {isRtl
                  ? "تجميع نتائج الفصلين الدراسيين الأول والثاني واحتساب القرار النهائي"
                  : "Aggregated results of Semester 1 and Semester 2 with promotion decision"}
              </p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <Select
              type="single"
              value={selectedCalendarId}
              onValueChange={(val) => {
                if (val) selectedCalendarId = val;
              }}
            >
              <SelectTrigger class="h-8 min-w-[180px] text-xs">
                {yearNames.get(selectedCalendarId) ?? (isRtl ? "اختر العام الدراسي" : "Select Academic Year")}
              </SelectTrigger>
              <SelectContent class="text-xs">
                {#each academicContext.allYears as year (year.id)}
                  <SelectItem value={year.id}>
                    {year.name_ar || year.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => annualReportQuery.refetch()}
              disabled={annualReportQuery.isFetching}
            >
              {#if annualReportQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardContent>
      </Card>

      {#if annualReportQuery.isLoading}
        <div class="flex min-h-64 flex-col items-center justify-center gap-2 rounded-xl border border-border bg-card p-8">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if annualReportQuery.error}
        <div class="flex min-h-64 flex-col items-center justify-center rounded-xl border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
          <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="mt-1 text-[10px] text-muted-foreground">
            {extractErrorMessage(annualReportQuery.error)}
          </p>
        </div>
      {:else if annualReportQuery.data}
        {@const annualReport = annualReportQuery.data}
        {@const badge = getStatusBadge(annualReport.status)}

        <!-- Performance Summary KPIs -->
        <div class="grid grid-cols-2 gap-3 sm:grid-cols-5">
          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "المجموع السنوي" : "Annual Total"}
            </span>
            <div class="mt-1 flex items-baseline gap-1">
              <span class="text-lg font-bold text-foreground">
                {Number(annualReport.total_score).toFixed(1)}
              </span>
              <span class="text-xs text-muted-foreground">
                / {Number(annualReport.max_possible_score).toFixed(0)}
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "النسبة العامة" : "Annual Percentage"}
            </span>
            <div class="mt-1 flex items-baseline gap-1">
              <span class="text-lg font-bold text-primary">
                {Number(annualReport.overall_percentage).toFixed(1)}%
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "التقدير السنوي" : "Appreciation"}
            </span>
            <div class="mt-1 text-sm font-bold text-foreground">
              {getAppreciation(Number(annualReport.overall_percentage))}
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "قرار النقل والنتيجة" : "Promotion Status"}
            </span>
            <div class="mt-1">
              <span class={`inline-flex items-center px-2 py-0.5 rounded text-[11px] font-semibold border ${badge.color}`}>
                {badge.label}
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "الترتيب" : "Rank"}
            </span>
            <div class="mt-1 text-base font-bold text-foreground">
              {annualReport.rank_in_class ? `#${annualReport.rank_in_class}` : "—"}
              {#if annualReport.total_students_in_class}
                <span class="text-xs font-normal text-muted-foreground">
                  ({isRtl ? "على الصف" : "in class"})
                </span>
              {/if}
            </div>
          </div>
        </div>

        <!-- Subject Results Table -->
        <Card class="overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full text-xs text-start">
              <thead class="bg-muted/40 border-b border-border text-[11px] font-semibold text-muted-foreground">
                <tr>
                  <th class="px-4 py-2.5 text-start">{isRtl ? "المادة الدراسية" : "Subject"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "الفصل الأول (50)" : "Term 1 (50)"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "الفصل الثاني (50)" : "Term 2 (50)"}</th>
                  <th class="px-3 py-2.5 text-center font-bold text-foreground">{isRtl ? "المجموع السنوي (100)" : "Annual Total (100)"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "النسبة" : "Percentage"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "التقدير" : "Grade"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "النتيجة" : "Status"}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/60">
                {#each annualReport.subjects as subj (subj.subject_id)}
                  {@const subjPassed = String(subj.status).toLowerCase().includes("pass")}
                  <tr class="hover:bg-muted/20 transition-colors">
                    <td class="px-4 py-2.5 font-medium text-foreground">
                      {isRtl ? (subj.subject_name_ar || subj.subject_name) : subj.subject_name}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {subj.term_1_result ? Number(subj.term_1_result.term_total).toFixed(1) : "—"}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {subj.term_2_result ? Number(subj.term_2_result.term_total).toFixed(1) : "—"}
                    </td>
                    <td class="px-3 py-2.5 text-center font-bold text-foreground">
                      {Number(subj.annual_total).toFixed(1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {Number(subj.percentage).toFixed(0)}%
                    </td>
                    <td class="px-3 py-2.5 text-center font-medium">
                      {subj.letter_grade ?? getAppreciation(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center">
                      <span class={`inline-block px-2 py-0.5 rounded text-[10px] font-semibold ${
                        subjPassed
                          ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
                          : "bg-rose-500/10 text-rose-600 dark:text-rose-400"
                      }`}>
                        {subjPassed ? (isRtl ? "ناجح" : "Passed") : (isRtl ? "راسب" : "Failed")}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card>
      {/if}
    </div>

  <!-- ═══════════════════════════════════════════════════════════ -->
  <!-- TAB 4: CERTIFICATE RESULTS (GRADE 9 & 12 PUBLIC EXAMS)      -->
  <!-- ═══════════════════════════════════════════════════════════ -->
  {:else if activeTab === "certificate"}
    <div class="flex flex-col gap-4">
      <!-- Certificate Configuration & Filters Header -->
      <Card>
        <CardContent class="flex flex-wrap items-center justify-between gap-3 p-3">
          <div class="flex items-center gap-3">
            <div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <AcademicCap size={18} />
            </div>
            <div>
              <h2 class="text-xs font-bold text-foreground">
                {isRtl ? "نتائج الشهادات العامة المعتمدة" : "Official Public Certificate Results"}
              </h2>
              <p class="text-[11px] text-muted-foreground">
                {isRtl
                  ? "شهادة إتمام المرحلة الأساسية (تاسع) والشهادة الثانوية العامة (80+20)"
                  : "Basic Education Certificate (Gr9) and Secondary Thanawiya (80+20 formula)"}
              </p>
            </div>
          </div>

          <div class="flex flex-wrap items-center gap-2">
            <!-- Certificate Type Selector -->
            <Select
              type="single"
              value={selectedCertType}
              onValueChange={(val) => {
                if (val === "secondary_thanawiya" || val === "basic_grade9") {
                  selectedCertType = val;
                }
              }}
            >
              <SelectTrigger class="h-8 min-w-[210px] text-xs">
                {selectedCertType === "secondary_thanawiya"
                  ? (isRtl ? "الشهادة الثانوية العامة (80+20)" : "Secondary Thanawiya (80+20)")
                  : (isRtl ? "شهادة التعليم الأساسي (الصف التاسع)" : "Basic Certificate (Grade 9)")}
              </SelectTrigger>
              <SelectContent class="text-xs">
                <SelectItem value="secondary_thanawiya">
                  {isRtl ? "الشهادة الثانوية العامة (80+20)" : "Secondary Thanawiya (80+20)"}
                </SelectItem>
                <SelectItem value="basic_grade9">
                  {isRtl ? "شهادة التعليم الأساسي (الصف التاسع)" : "Basic Certificate (Grade 9)"}
                </SelectItem>
              </SelectContent>
            </Select>

            <!-- Calendar Selector -->
            <Select
              type="single"
              value={selectedCalendarId}
              onValueChange={(val) => {
                if (val) selectedCalendarId = val;
              }}
            >
              <SelectTrigger class="h-8 min-w-[170px] text-xs">
                {yearNames.get(selectedCalendarId) ?? (isRtl ? "اختر العام الدراسي" : "Select Academic Year")}
              </SelectTrigger>
              <SelectContent class="text-xs">
                {#each academicContext.allYears as year (year.id)}
                  <SelectItem value={year.id}>
                    {year.name_ar || year.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="sm"
              class="h-8 gap-1.5 text-xs"
              onclick={() => certificateReportQuery.refetch()}
              disabled={certificateReportQuery.isFetching}
            >
              {#if certificateReportQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
              {m.common_retry()}
            </Button>
          </div>
        </CardContent>
      </Card>

      <!-- Yemen Sana'a MoE Policy Formula Explanation Banner -->
      {#if selectedCertType === "secondary_thanawiya"}
        <div class="flex items-center gap-2.5 rounded-lg border border-primary/25 bg-primary/5 px-3.5 py-2.5 text-xs text-primary">
          <InfoCircle size={16} class="shrink-0" />
          <div class="flex-1">
            <span class="font-bold">
              {isRtl ? "معادلة احتساب معدل الثانوية العامة المعتمدة:" : "Approved Thanawiya Formula:"}
            </span>
            <span class="ms-1 text-foreground">
              {isRtl
                ? "درجة كل مادة (100) = الاختبار الوزاري الموحد (80 درجة) + المعدل التراكمي لسنوات النقل السابقة (20 درجة)."
                : "Subject Total (100) = Unified Ministerial Examination (80 marks) + Cumulative Prior Years Average (20 marks)."}
            </span>
          </div>
        </div>
      {/if}

      {#if certificateReportQuery.isLoading}
        <div class="flex min-h-64 flex-col items-center justify-center gap-2 rounded-xl border border-border bg-card p-8">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if certificateReportQuery.error}
        <div class="flex min-h-64 flex-col items-center justify-center rounded-xl border border-destructive/20 bg-destructive/5 p-8 text-center text-destructive">
          <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="mt-1 text-[10px] text-muted-foreground">
            {extractErrorMessage(certificateReportQuery.error)}
          </p>
        </div>
      {:else if certificateReportQuery.data}
        {@const certReport = certificateReportQuery.data}
        {@const badge = getStatusBadge(certReport.status)}

        <!-- Performance Summary KPIs -->
        <div class="grid grid-cols-2 gap-3 sm:grid-cols-5">
          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "المجموع الكلي" : "Total Score"}
            </span>
            <div class="mt-1 flex items-baseline gap-1">
              <span class="text-lg font-bold text-foreground">
                {Number(certReport.total_score).toFixed(1)}
              </span>
              <span class="text-xs text-muted-foreground">
                / {Number(certReport.max_possible_score).toFixed(0)}
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "النسبة العامة" : "Overall Percentage"}
            </span>
            <div class="mt-1 flex items-baseline gap-1">
              <span class="text-lg font-bold text-primary">
                {Number(certReport.overall_percentage).toFixed(2)}%
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "التقدير العام" : "Appreciation"}
            </span>
            <div class="mt-1 text-sm font-bold text-foreground">
              {getAppreciation(Number(certReport.overall_percentage))}
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "النتيجة والقرار" : "Final Status"}
            </span>
            <div class="mt-1">
              <span class={`inline-flex items-center px-2 py-0.5 rounded text-[11px] font-semibold border ${badge.color}`}>
                {badge.label}
              </span>
            </div>
          </div>

          <div class="rounded-xl border border-border bg-card p-3">
            <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
              {isRtl ? "الترتيب" : "Rank"}
            </span>
            <div class="mt-1 text-base font-bold text-foreground">
              {certReport.rank_in_class ? `#${certReport.rank_in_class}` : "—"}
              {#if certReport.total_students_in_class}
                <span class="text-xs font-normal text-muted-foreground">
                  ({isRtl ? "على الدفعة" : "in cohort"})
                </span>
              {/if}
            </div>
          </div>
        </div>

        <!-- Certificate Subjects Table -->
        <Card class="overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full text-xs text-start">
              <thead class="bg-muted/40 border-b border-border text-[11px] font-semibold text-muted-foreground">
                <tr>
                  <th class="px-4 py-2.5 text-start">{isRtl ? "المادة الدراسية" : "Subject"}</th>
                  {#if selectedCertType === "secondary_thanawiya"}
                    <th class="px-3 py-2.5 text-center">{isRtl ? "الاختبار الوزاري (80)" : "Exam (80)"}</th>
                    <th class="px-3 py-2.5 text-center">{isRtl ? "المعدل التراكمي (20)" : "Cumulative (20)"}</th>
                  {/if}
                  <th class="px-3 py-2.5 text-center font-bold text-foreground">
                    {isRtl ? "الدرجة النهائية (100)" : "Final Score (100)"}
                  </th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "النهاية الصغرى (50)" : "Min Pass (50)"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "النسبة" : "Percentage"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "التقدير" : "Grade"}</th>
                  <th class="px-3 py-2.5 text-center">{isRtl ? "النتيجة" : "Status"}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/60">
                {#each certReport.subjects as subj (subj.subject_id)}
                  {@const subjPassed = String(subj.status).toLowerCase().includes("pass")}
                  <tr class="hover:bg-muted/20 transition-colors">
                    <td class="px-4 py-2.5 font-medium text-foreground">
                      {isRtl ? (subj.subject_name_ar || subj.subject_name) : subj.subject_name}
                    </td>

                    {#if selectedCertType === "secondary_thanawiya"}
                      <td class="px-3 py-2.5 text-center text-muted-foreground">
                        {subj.term_final_exam != null ? Number(subj.term_final_exam).toFixed(1) : "—"}
                      </td>
                      <td class="px-3 py-2.5 text-center text-muted-foreground">
                        {Number(subj.monthly_contribution).toFixed(1)}
                      </td>
                    {/if}

                    <td class="px-3 py-2.5 text-center font-bold text-foreground">
                      {Number(subj.term_total).toFixed(1)}
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      50
                    </td>
                    <td class="px-3 py-2.5 text-center text-muted-foreground">
                      {Number(subj.percentage).toFixed(0)}%
                    </td>
                    <td class="px-3 py-2.5 text-center font-medium">
                      {subj.letter_grade ?? getAppreciation(Number(subj.percentage))}
                    </td>
                    <td class="px-3 py-2.5 text-center">
                      <span class={`inline-block px-2 py-0.5 rounded text-[10px] font-semibold ${
                        subjPassed
                          ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
                          : "bg-rose-500/10 text-rose-600 dark:text-rose-400"
                      }`}>
                        {subjPassed ? (isRtl ? "ناجح" : "Passed") : (isRtl ? "راسب" : "Failed")}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card>
      {/if}
    </div>
  {/if}
</div>

<!-- Print PDF Report Viewer Dialog -->
<PdfReportViewer
  bind:open={showPrintCertificateDialog}
  templateId={printTemplateId}
  parameters={{
    student_id: studentId,
    calendar_id: selectedCalendarId,
    period_id: selectedTermPeriodId,
    certificate_type: selectedCertType,
    class_id: classId,
  }}
  reportTitle={printTitle}
/>
