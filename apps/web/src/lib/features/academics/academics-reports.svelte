<!--
  @component AcademicsReports
  Unified reporting center for all Academic, Grading, and Examination reports.
  Consolidates grade sheets, official report cards, certificates, and exam control reports.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import {
  AcademicCap,
  ChartBar,
  DocumentCheck,
  DocumentPage,
  InfoCircle,
  Refresh,
  Search,
} from "@orbis/icons";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, type ReportDefinitionDto } from "$lib/api";
import { createQuery } from "@tanstack/svelte-query";

let { orgId }: { orgId: string } = $props();

const isRtl = $derived(getLocale() === "ar");

// ── Search & Filter State ─────────────────────────────────────────────
let searchQuery = $state("");
let selectedCategory = $state<string>("all");

// ── Query both academic and exam reports ──────────────────────────────
const reportsQuery = createQuery<ReportDefinitionDto[]>(() => ({
  queryKey: ["academics", "reports", "all-definitions", orgId],
  queryFn: async () => {
    const [academicReports, examReports] = await Promise.all([
      api.listReportDefinitions("academic").catch(() => []),
      api.listReportDefinitions("exam").catch(() => []),
    ]);

    // Deduplicate by code
    const seen = new Set<string>();
    const combined: ReportDefinitionDto[] = [];
    for (const r of [...academicReports, ...examReports]) {
      if (!seen.has(r.code)) {
        seen.add(r.code);
        combined.push(r);
      }
    }
    return combined;
  },
  staleTime: 10 * 60 * 1000,
}));

const allReports = $derived<ReportDefinitionDto[]>(reportsQuery.data ?? []);
const isLoading = $derived(reportsQuery.isLoading);
const error = $derived(reportsQuery.error);

// ── Category Tabs ─────────────────────────────────────────────────────
const categories = $derived([
  { id: "all", label: m.academics_reports_cat_all() },
  { id: "grades", label: m.academics_reports_cat_grades() },
  { id: "certificates", label: m.academics_reports_cat_certificates() },
  { id: "transcripts", label: m.academics_reports_cat_transcripts() },
  { id: "analytics", label: m.academics_reports_cat_analytics() },
  { id: "exam", label: m.academics_reports_cat_exams() },
]);

// ── Filtered reports ──────────────────────────────────────────────────
const filteredReports = $derived.by(() => {
  const q = searchQuery.toLowerCase().trim();
  return allReports.filter((r) => {
    // Search match
    const nameMatch =
      !q ||
      r.name.toLowerCase().includes(q) ||
      (r.name_ar && r.name_ar.toLowerCase().includes(q)) ||
      (r.description && r.description.toLowerCase().includes(q)) ||
      (r.description_ar && r.description_ar.toLowerCase().includes(q)) ||
      r.code.toLowerCase().includes(q);

    if (!nameMatch) return false;

    // Category match
    if (selectedCategory === "all") return true;
    if (selectedCategory === "exam") {
      return (
        r.domain === "exam" ||
        r.category === "seating" ||
        r.category === "halls" ||
        r.category === "proctors" ||
        r.category === "control"
      );
    }
    return r.category?.toLowerCase() === selectedCategory.toLowerCase();
  });
});

function getCategoryLabel(category: string, domain?: string): string {
  if (domain === "exam" || category === "seating" || category === "control") {
    return m.academics_reports_cat_exams();
  }
  switch (category) {
    case "grades":
      return m.academics_reports_cat_grades();
    case "certificates":
      return m.academics_reports_cat_certificates();
    case "transcripts":
      return m.academics_reports_cat_transcripts();
    case "analytics":
      return m.academics_reports_cat_analytics();
    default:
      return category;
  }
}

function getReportIcon(category: string, domain?: string) {
  if (domain === "exam") return DocumentCheck;
  switch (category) {
    case "grades":
      return DocumentCheck;
    case "certificates":
      return AcademicCap;
    case "transcripts":
      return DocumentPage;
    case "analytics":
      return ChartBar;
    default:
      return DocumentPage;
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 font-sans text-start select-none">
  <div class="max-w-7xl mx-auto flex flex-col gap-6">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border/40 pb-4">
      <div>
        <h1 class="text-base font-bold text-foreground">
          {m.common_nav_reports()}
        </h1>
        <p class="text-xs text-muted-foreground mt-0.5">
          {m.academics_reports_center_subtitle()}
        </p>
      </div>

      <Button
        variant="outline"
        size="sm"
        class="h-8 gap-1.5 text-xs font-semibold"
        onclick={() => reportsQuery.refetch()}
      >
        {#if isLoading}
          <LoadingIndicator size={13} />
        {:else}
          <Refresh size={13} />
        {/if}
        <span>{m.common_retry()}</span>
      </Button>
    </div>

    <!-- Search & Category Filters -->
    <div class="flex flex-col gap-3">
      <div class="relative w-full sm:w-96">
        <Search
          size={14}
          class="absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground opacity-60"
        />
        <Input
          bind:value={searchQuery}
          placeholder={m.academics_reports_search_placeholder()}
          class="h-9 ps-8 pe-3 text-xs w-full rounded-lg"
        />
      </div>

      <!-- Category Filter Chips -->
      <div class="flex flex-wrap items-center gap-1.5">
        {#each categories as cat (cat.id)}
          <button
            type="button"
            class="px-3 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer border"
            class:bg-primary={selectedCategory === cat.id}
            class:text-primary-foreground={selectedCategory === cat.id}
            class:border-primary={selectedCategory === cat.id}
            class:bg-card={selectedCategory !== cat.id}
            class:text-muted-foreground={selectedCategory !== cat.id}
            class:border-border={selectedCategory !== cat.id}
            class:hover:text-foreground={selectedCategory !== cat.id}
            onclick={() => { selectedCategory = cat.id; }}
          >
            {cat.label}
          </button>
        {/each}
      </div>
    </div>

    <!-- Content -->
    {#if isLoading}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each Array(6) as _}
          <Skeleton class="h-36 w-full rounded-xl" />
        {/each}
      </div>
    {:else if error}
      <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground border border-destructive/20 bg-destructive/5 rounded-xl p-6">
        <InfoCircle size={32} class="text-destructive opacity-80" />
        <p class="text-sm text-destructive font-medium">{extractErrorMessage(error)}</p>
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs font-semibold"
          onclick={() => reportsQuery.refetch()}
        >
          <Refresh size={13} />
          <span>{m.common_retry()}</span>
        </Button>
      </div>
    {:else if filteredReports.length === 0}
      <div class="flex flex-col items-center justify-center py-16 gap-3 text-muted-foreground border border-border/40 rounded-xl p-8 bg-card/40">
        <DocumentPage size={40} class="opacity-30" />
        <p class="text-sm font-medium">
          {searchQuery || selectedCategory !== "all"
            ? m.academics_reports_no_matches()
            : m.academics_reports_empty()}
        </p>
        {#if searchQuery || selectedCategory !== "all"}
          <Button
            variant="ghost"
            size="sm"
            class="text-xs"
            onclick={() => { searchQuery = ""; selectedCategory = "all"; }}
          >
            {m.academics_reports_reset_filters()}
          </Button>
        {/if}
      </div>
    {:else}
      <!-- Reports Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each filteredReports as report (report.id || report.code)}
          {@const ReportIcon = getReportIcon(report.category, report.domain)}
          {@const displayName = isRtl && report.name_ar ? report.name_ar : report.name}
          {@const displayDesc = isRtl && report.description_ar ? report.description_ar : report.description}
          <a
            href={localizeHref(`/${orgId}/academics/reports/${report.id || report.code}`)}
            class="no-underline group block"
          >
            <Card class="hover:border-primary/60 transition-all group-hover:shadow-sm cursor-pointer h-full flex flex-col justify-between rounded-xl border-border/60">
              <CardHeader class="p-4 pb-2">
                <div class="flex items-start justify-between gap-2 mb-2">
                  <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary group-hover:scale-105 transition-transform shrink-0">
                    <ReportIcon size={16} />
                  </div>
                  <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4.5 rounded font-mono">
                    {getCategoryLabel(report.category, report.domain)}
                  </Badge>
                </div>
                <CardTitle class="text-xs font-bold text-foreground group-hover:text-primary transition-colors line-clamp-1">
                  {displayName}
                </CardTitle>
              </CardHeader>
              {#if displayDesc}
                <CardContent class="p-4 pt-0">
                  <CardDescription class="text-[11px] text-muted-foreground line-clamp-2 leading-relaxed">
                    {displayDesc}
                  </CardDescription>
                </CardContent>
              {/if}
            </Card>
          </a>
        {/each}
      </div>
    {/if}
  </div>
</div>
