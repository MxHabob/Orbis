<!--
  @component AcademicsReports
  Feature component listing available reports for the Academics domain.
-->
<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
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
  <div class="flex items-center justify-between border-b border-border/40 pb-4">
    <div>
      <h1 class="text-base font-bold text-foreground">
        {m.common_nav_reports()}
      </h1>
      <p class="text-xs text-muted-foreground mt-0.5">
        {m.academics_reports_desc()}
      </p>
    </div>

    <Button
      variant="outline"
      size="sm"
      class="h-8 gap-1.5 text-xs font-semibold"
      onclick={() => reportsQuery.refetch()}
    >
      <Refresh size={13} class={isLoading ? "animate-spin" : ""} />
      <span>{m.common_retry()}</span>
    </Button>
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
        <span>{m.common_retry()}</span>
      </Button>
    </div>
  {:else if reports.length === 0}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground border border-border/40 rounded-xl p-8 bg-card/40">
      <DocumentPage size={36} class="opacity-40" />
      <p class="text-sm">
        {m.academics_reports_empty()}
      </p>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each reports as report (report.id)}
        <a
          href={localizeHref(`/${orgId}/academics/reports/${report.id}`)}
          class="no-underline group block"
        >
          <Card class="hover:border-primary/60 transition-all group-hover:shadow-md cursor-pointer h-full flex flex-col justify-between">
            <CardHeader class="py-3 px-4">
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary group-hover:scale-105 transition-transform shrink-0">
                  <ChartBar class="h-4 w-4" />
                </div>
                <CardTitle class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
                  {report.name}
                </CardTitle>
              </div>
            </CardHeader>
            {#if report.description}
              <CardContent class="p-4 pt-1">
                <CardDescription class="text-xs text-muted-foreground line-clamp-2">
                  {report.description}
                </CardDescription>
              </CardContent>
            {/if}
          </Card>
        </a>
      {/each}
    </div>
  {/if}
</div>
