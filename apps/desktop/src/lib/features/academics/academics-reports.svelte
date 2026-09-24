<!--
  @component AcademicsReports
  Feature component listing available reports for the Academics domain.
-->
<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { ChartBar, DocumentPage, InfoCircle, Refresh } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { extractErrorMessage } from "$lib/api";
import { createReportDefinitionsQuery } from "$lib/features/academics/queries";

let { orgId }: { orgId: string } = $props();

const reportsQuery = createReportDefinitionsQuery(() => "academics");
const reports = $derived(reportsQuery.data ?? []);
const isLoading = $derived(reportsQuery.isLoading);
const error = $derived(reportsQuery.error);
</script>

<div class="p-6 flex flex-col gap-6 font-sans text-start select-none">
  <div>
    <h1 class="text-xl font-bold">
      {m.common_nav_reports ? m.common_nav_reports() : "Academic Reports"}
    </h1>
    <p class="text-sm text-muted-foreground mt-1">
      {m.template_placeholder_description ? m.template_placeholder_description() : "Select a report template to preview, export, or print."}
    </p>
  </div>

  {#if isLoading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each Array(3) as _}
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
        <span>{m.common_retry ? m.common_retry() : "Retry"}</span>
      </Button>
    </div>
  {:else if reports.length === 0}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground border border-border/40 rounded-xl p-8 bg-card/40">
      <DocumentPage size={36} class="opacity-40" />
      <p class="text-sm">
        {m.finance_report_no_accounts ? m.finance_report_no_accounts() : "No report definitions found."}
      </p>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each reports as report (report.id)}
        <a
          href={localizeHref(`/${orgId}/academics/reports/${report.id}`)}
          class="flex flex-col gap-2 p-6 rounded-xl border border-border/70 bg-card hover:border-primary hover:shadow-md transition-all group"
        >
          <ChartBar class="h-8 w-8 text-primary mb-2 transition-transform group-hover:scale-105" />
          <h3 class="font-medium text-foreground group-hover:text-primary transition-colors">{report.name}</h3>
          {#if report.description}
            <p class="text-xs text-muted-foreground line-clamp-2">{report.description}</p>
          {/if}
        </a>
      {/each}
    </div>
  {/if}
</div>
