<script lang="ts">
import { onMount } from "svelte";
import { api } from "$lib/api";
import type { ReportDefinition, ReportDataset, ColumnDef } from "@orbis/sdk";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { AdjustmentsHorizontal, Download, InfoCircle, Refresh, Printer } from "@orbis/icons";
import { OrbisGrid } from "@orbis/svelte-grid";
import * as m from "$lib/paraglide/messages";
import ReportExportDialog from "./report-export-dialog.svelte";
import { getLocale } from "$lib/paraglide/runtime";

interface Props {
  definitionId: string;
}

let { definitionId }: Props = $props();

let isLoading = $state(true);
let error = $state<string | null>(null);
let definition = $state<ReportDefinition | null>(null);
let dataset = $state<ReportDataset | null>(null);
let filters = $state<Record<string, any>>({});

let isExportDialogOpen = $state(false);
let gridColumns = $state<any[]>([]);

// Map Rust ReportColumn to OrbisGrid column
function buildGridColumns(cols: ColumnDef[]) {
  return cols.map((c) => ({
    name: c.key,
    header: getLocale() === "ar" ? c.headerAr : c.header,
    width: c.width ? Number.parseInt(c.width, 10) || 120 : 120,
    type:
      c.dataType === "number" ||
      c.dataType === "currency" ||
      c.dataType === "decimal"
        ? "number"
        : "string",
    align:
      c.dataType === "number" ||
      c.dataType === "currency" ||
      c.dataType === "decimal"
        ? "right"
        : "left",
    editable: false,
  }));
}

async function loadDefinition() {
  try {
    isLoading = true;
    error = null;
    definition = await api.getReportDefinition(definitionId);
    // Initialize default filters
    if (definition) {
      filters = {};
      for (const p of definition.parameters) {
        filters[p.key] = null;
      }
    }
  } catch (e: any) {
    error = e.message || "Failed to load report definition";
  } finally {
    isLoading = false;
  }
}

async function queryData() {
  if (!definition) return;
  try {
    isLoading = true;
    const jobId = await api.generateReport(definitionId, "json", filters);
    dataset = {
      columns: [],
      rows: [],
    } as any;
  } catch (e: any) {
    error = e.message || "Failed to query report data";
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  loadDefinition();
});
</script>

<div class="h-full w-full flex flex-col bg-background relative overflow-hidden">
  <!-- Top Bar -->
  <div class="flex items-center justify-between border-b px-4 py-3 bg-card shrink-0">
    <div>
      <h2 class="text-lg font-semibold flex items-center gap-2">
        {#if definition}
          {definition.name}
        {:else if isLoading}
          <Loader2 class="h-4 w-4 animate-spin text-muted-foreground" />
        {:else}
          Unknown Report
        {/if}
      </h2>
      {#if definition?.description}
        <p class="text-xs text-muted-foreground mt-0.5">{definition.description}</p>
      {/if}
    </div>
    
    <div class="flex items-center gap-2">
      <Button variant="outline" size="sm" onclick={queryData} disabled={isLoading || !definition}>
        <Refresh class="mr-2 h-4 w-4 {isLoading ? 'animate-spin' : ''}" />
        {m.common_refresh()}
      </Button>
      <Button size="sm" onclick={() => isExportDialogOpen = true} disabled={!dataset}>
        <Download class="mr-2 h-4 w-4" />
        {m.common_export()}
      </Button>
    </div>
  </div>

  <div class="flex flex-1 overflow-hidden">
    <!-- AdjustmentsHorizontal Sidebar -->
    {#if definition}
      <div class="w-72 border-r bg-card/30 flex flex-col shrink-0 overflow-y-auto">
        <div class="p-4 border-b bg-muted/20 flex items-center gap-2">
          <Printer class="h-4 w-4 text-muted-foreground" />
          <h3 class="font-medium text-sm">المعاملات</h3>
        </div>
        
        <div class="p-4 flex flex-col gap-4">
          {#each definition.parameters as param}
            <div class="flex flex-col gap-1.5">
              <label for={`report-param-${param.key}`} class="text-xs font-medium flex items-center gap-1">
                {param.label}
                {#if param.required}
                  <span class="text-destructive">*</span>
                {/if}
              </label>
              
              <!-- Dynamic AdjustmentsHorizontal Inputs (Simplified for MVP) -->
              <Input 
                id={`report-param-${param.key}`}
                type={param.paramType === 'number' ? 'number' : param.paramType === 'date' ? 'date' : 'text'}
                bind:value={filters[param.key]}
                placeholder={param.label}
                class="h-8 text-xs"
              />
            </div>
          {/each}
          
          <Button class="w-full mt-2" size="sm" onclick={queryData}>
            تنفيذ
          </Button>
        </div>
      </div>
    {/if}

    <!-- Main Content Area -->
    <div class="flex-1 bg-muted/10 relative">
      {#if error}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-8 text-center bg-background/50 backdrop-blur-sm z-10">
          <InfoCircle class="h-10 w-10 text-destructive mb-3" />
          <h3 class="text-lg font-semibold">Error</h3>
          <p class="text-sm text-muted-foreground max-w-md">{error}</p>
          <Button variant="outline" class="mt-4" onclick={loadDefinition}>Retry</Button>
        </div>
      {/if}

      {#if dataset && gridColumns.length > 0}
        <div class="w-full h-full p-2">
          <OrbisGrid
            columns={gridColumns}
            source={dataset.rows as Array<Record<string, any>>}
            height="calc(100vh - 120px)"
            rtl={getLocale() === "ar"}
          />
        </div>
      {:else if !isLoading && !error && definition}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground">
          <DocumentChartBar class="h-12 w-12 opacity-20 mb-3" />
          <p class="text-sm">أدخل المعاملات ثم اضغط "تحديث" لعرض البيانات.</p>
        </div>
      {:else if isLoading && !dataset}
        <div class="absolute inset-0 flex items-center justify-center">
          <Loader2 class="h-8 w-8 animate-spin text-muted-foreground/30" />
        </div>
      {/if}
    </div>
  </div>
</div>

<ReportExportDialog 
  bind:open={isExportDialogOpen} 
  definitionId={definitionId} 
  {filters} 
  onOpenChange={(value) => (isExportDialogOpen = value)}
/>
