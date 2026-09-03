<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * PrintDialog — Enhanced bulk printing dialog with Multi-Up support.
 * Supports multiple documents per sheet with WYSIWYG preview.
 */
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Label } from "@orbis/ui-svelte/label";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import { Progress } from "@orbis/ui-svelte/progress";
import { Separator } from "@orbis/ui-svelte/separator";
import { toast } from "@orbis/ui-svelte";

import { getLocale } from "$lib/paraglide/runtime";
import {
  Printer,
  Download,
  RefreshCw,
  AlertCircle,
  CheckCircle2,
  FileText,
  LayoutGrid,
} from "@lucide/svelte";

import {
  computeMultiUpLayout,
  computeTotalSheets,
  suggestLayouts,
  createDefaultMultiUpConfig,
  layoutLabel,
  layoutLabelAr,
} from "@docforge/core";
import type { MultiUpConfig, MultiUpLayout } from "@docforge/core";

import MultiUpPreview from "./multi-up-preview.svelte";
import PrintConfigPanel from "./print-config-panel.svelte";

// ── Props ────────────────────────────────────────────────────

let {
  open = $bindable(false),
  entityType,
  entityIds = [],
  entityLabels = [],
}: {
  open: boolean;
  entityType: string;
  entityIds: string[];
  entityLabels?: string[];
} = $props();

// ── State ────────────────────────────────────────────────────

let templates = $state<any[]>([]);
let selectedTemplateId = $state<string>("");
let loadingTemplates = $state(false);
let generatingPdf = $state(false);
let pdfUrl = $state<string | null>(null);
let activeTab = $state("preview");

// Batch progress
let batchProgress = $state(0);
let isBatchProcessing = $state(false);
let batchStatusText = $state("");
let batchJobId = $state<string | null>(null);

// Multi-Up config (initialized when template is selected)
let multiUpConfig = $state<MultiUpConfig>(createDefaultMultiUpConfig(90, 50));
let previewPage = $state(1);

// ── Derived ──────────────────────────────────────────────────

const orgId = $derived(page.params.orgId);
const isRtl = $derived(getLocale() === "ar");

// Compute layout reactively whenever config changes
const multiUpLayout = $derived(computeMultiUpLayout(multiUpConfig));
const totalSheets = $derived(
  computeTotalSheets(entityIds.length, multiUpLayout.itemsPerSheet),
);

// Suggested layouts for the current template dimensions
const suggestions = $derived(
  multiUpConfig.templateWidthMm > 0 && multiUpConfig.templateHeightMm > 0
    ? suggestLayouts(
        multiUpConfig.templateWidthMm,
        multiUpConfig.templateHeightMm,
        multiUpConfig.sheetSize,
      )
    : [],
);

// ── Load Templates ───────────────────────────────────────────

async function loadTemplates() {
  if (!entityType || !orgId) return;
  loadingTemplates = true;
  try {
    const list = await commands.docforge.listTemplates({
      org_id: orgId,
      entity_type: entityType,
      status: "published",
    });
    templates = Array.isArray(list) ? list : [];
    if (templates.length > 0) {
      selectedTemplateId = templates[0].id;
      updateConfigFromTemplate(templates[0]);
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to load templates";
    console.error(msg);
  } finally {
    loadingTemplates = false;
  }
}

function updateConfigFromTemplate(template: any) {
  // Extract template dimensions from document JSON
  try {
    const doc = JSON.parse(template.document_json);
    const pageSetup = doc?.meta?.pageSetup ?? doc?.meta?.page_setup;
    if (pageSetup) {
      const dimensions: Record<string, [number, number]> = {
        A4: [210, 297],
        A3: [297, 420],
        A5: [148, 210],
        Letter: [216, 279],
        Legal: [216, 356],
        ID_CARD: [85.6, 53.98],
        BADGE: [86, 54],
      };
      const [presetWidth, presetHeight] = dimensions[pageSetup.size] ?? [90, 50];
      const w = pageSetup.width ?? pageSetup.width_mm ?? presetWidth;
      const h = pageSetup.height ?? pageSetup.height_mm ?? presetHeight;
      multiUpConfig = createDefaultMultiUpConfig(w, h);
      multiUpConfig.sheetOrientation = pageSetup.orientation ?? "portrait";
      // Set fill order based on locale
      multiUpConfig.fillOrder = isRtl ? "rtl-ttb" : "ltr-ttb";
    }
  } catch {
    // Use defaults
  }
}

// ── React to open/entityType ─────────────────────────────────

$effect(() => {
  if (open && entityType) {
    loadTemplates();
    pdfUrl = null;
    batchProgress = 0;
    isBatchProcessing = false;
    batchJobId = null;
    previewPage = 1;
    activeTab = "preview";
  }
});

// ── React to template selection ──────────────────────────────

let lastProcessedTemplateId = "";
$effect(() => {
  if (selectedTemplateId && templates.length > 0 && selectedTemplateId !== lastProcessedTemplateId) {
    lastProcessedTemplateId = selectedTemplateId;
    const tpl = templates.find((t: any) => t.id === selectedTemplateId);
    if (tpl) {
      updateConfigFromTemplate(tpl);
      pdfUrl = null;
    }
  }
});

// ── Helpers ──────────────────────────────────────────────────

function uint8ArrayToBase64(bytes: number[]): string {
  let binary = "";
  const len = bytes.length;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

function serializeMultiUpConfig(config: MultiUpConfig): string {
  const sheetSize =
    config.sheetSize === "Custom"
      ? {
          type: "Custom",
          value: {
            width_mm: config.customSheetWidthMm ?? 210,
            height_mm: config.customSheetHeightMm ?? 297,
          },
        }
      : { type: config.sheetSize };

  return JSON.stringify({
    ...config,
    sheetSize,
    fillOrder:
      config.fillOrder === "rtl-ttb"
        ? "right-to-left-top-to-bottom"
        : "left-to-right-top-to-bottom",
  });
}

// ── Preview Generation ───────────────────────────────────────

async function generatePreview() {
  if (!selectedTemplateId || entityIds.length === 0) return;
  generatingPdf = true;
  pdfUrl = null;
  try {
    const configJson = serializeMultiUpConfig(multiUpConfig);
    // Use first N entity IDs for preview (one sheet)
    const previewIds = entityIds.slice(0, multiUpLayout.itemsPerSheet);
    const pdfBytes = await commands.docforge.previewMultiUp(
      selectedTemplateId,
      previewIds,
      configJson,
    );
    if (pdfBytes && pdfBytes.length > 0) {
      const base64 = uint8ArrayToBase64(pdfBytes);
      pdfUrl = `data:application/pdf;base64,${base64}`;
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to generate preview";
    toast.error(msg);
  } finally {
    generatingPdf = false;
  }
}

// ── Print Action ─────────────────────────────────────────────

async function handlePrintAction() {
  if (!selectedTemplateId) {
    toast.error(m.common_print_multi_up_no_templates());
    return;
  }

  if (entityIds.length <= multiUpLayout.itemsPerSheet) {
    // Single sheet — just generate preview
    await generatePreview();
    toast.success(m.common_print_multi_up_complete());
  } else {
    // Batch multi-up processing
    isBatchProcessing = true;
    batchProgress = 0;
    batchStatusText = m.common_print_multi_up_processing();

    try {
      const configJson = serializeMultiUpConfig(multiUpConfig);
      const started = await commands.docforge.startMultiUpPrint(
        selectedTemplateId,
        entityIds,
        configJson,
      );
      batchJobId = started.jobId;
      const pollingDeadline = Date.now() + 10 * 60 * 1000;

      while (batchJobId) {
        if (Date.now() >= pollingDeadline) {
          throw new Error("Batch print timed out. Please try again.");
        }

        const status = await commands.docforge.getMultiUpPrintStatus(batchJobId);
        const total = Number(status.totalRecords ?? entityIds.length);
        const processed = Number(status.processedRecords ?? 0);
        batchProgress = total > 0 ? Math.round((processed / total) * 100) : 0;
        batchStatusText = status.status || m.common_print_multi_up_processing();

        if (["completed", "failed", "cancelled"].includes(status.status)) {
          if (status.status === "completed") {
            batchProgress = 100;
            toast.success(
              m.common_print_multi_up_complete_detail({
                count: String(entityIds.length),
              }),
            );
          } else if (status.errorMessage) {
            throw new Error(status.errorMessage);
          }
          break;
        }

        await new Promise((resolve) => setTimeout(resolve, 500));
      }
    } catch (e: unknown) {
      const msg = (e as any)?.message || "Batch print failed";
      toast.error(msg);
      isBatchProcessing = false;
      batchJobId = null;
    }
    isBatchProcessing = false;
  }
}

function triggerDownload() {
  if (!pdfUrl) return;
  const link = document.createElement("a");
  link.href = pdfUrl;
  link.download = `multi_up_${entityType}_${Date.now()}.pdf`;
  link.click();
}

function applySuggestion(suggestion: MultiUpLayout) {
  // Update config to match the suggestion's orientation
  multiUpConfig.sheetOrientation =
    suggestion.sheetWidthMm < suggestion.sheetHeightMm
      ? "portrait"
      : "landscape";
  // Recalculate will happen automatically via $derived
  pdfUrl = null;
}
</script>

<Dialog bind:open={open}>
  <DialogContent class="sm:max-w-195 text-start max-h-[90vh] flex flex-col overflow-hidden">
    <DialogHeader class="shrink-0">
      <DialogTitle class="flex items-center gap-2">
        <LayoutGrid class="text-primary h-5 w-5" />
        <span>{m.common_print_multi_up_title()}</span>
      </DialogTitle>
    </DialogHeader>

    {#if isBatchProcessing}
      <!-- ═══ Batch Progress View ═══ -->
      <div class="grow py-12 flex flex-col items-center justify-center gap-6">
        <div class="w-full max-w-md flex flex-col gap-3">
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>{batchStatusText}</span>
            <span class="font-bold text-foreground">{batchProgress}%</span>
          </div>
          <Progress value={batchProgress} class="h-2 w-full" />
        </div>
        {#if batchProgress === 100}
          <div class="flex items-center gap-2 text-emerald-500 text-xs font-semibold animate-bounce mt-2">
            <CheckCircle2 size={16} />
            <span>{m.common_print_multi_up_complete()}</span>
          </div>
        {:else}
          <RefreshCw size={24} class="animate-spin text-primary mt-2" />
        {/if}
      </div>

    {:else}
      <!-- ═══ Main Content: Config + Preview ═══ -->
      <div class="grow flex flex-col md:flex-row gap-4 overflow-hidden py-3">

        <!-- Left Panel: Config -->
        <div class="w-full md:w-60 flex flex-col gap-3 shrink-0 overflow-y-auto scrollbar-hide">
          <!-- Template Selection -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-muted-foreground">
              {m.common_print_multi_up_template()}
            </Label>
            {#if loadingTemplates}
              <div class="h-8 bg-muted rounded animate-pulse"></div>
            {:else if templates.length === 0}
              <div class="text-[10px] text-destructive flex items-center gap-1">
                <AlertCircle size={12} />
                <span>{m.common_print_multi_up_no_templates()}</span>
              </div>
            {:else}
              <Select type="single" bind:value={selectedTemplateId} onValueChange={(val) => { if (val) { selectedTemplateId = val; pdfUrl = null; } }}>
                <SelectTrigger class="h-8 text-xs">
                  {(() => {
                    const selected = templates.find((t: any) => t.id === selectedTemplateId);
                    return selected ? (isRtl && selected.name_ar ? selected.name_ar : selected.name) : m.common_print_multi_up_template_placeholder();
                  })()}
                </SelectTrigger>
                <SelectContent>
                  {#each templates as t (t.id)}
                    <SelectItem value={t.id}>
                      {isRtl && t.name_ar ? t.name_ar : t.name}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/if}
          </div>

          <!-- Stats Card -->
          <div class="bg-card border border-border rounded-xl p-3 flex flex-col gap-2">
            <div class="flex items-center justify-between text-[10px] text-muted-foreground border-b border-border/40 pb-2">
              <span>{m.common_print_multi_up_records_selected()}</span>
              <Badge variant="secondary" class="text-[10px]">{entityIds.length}</Badge>
            </div>
            <div class="flex items-center justify-between text-[10px] text-muted-foreground border-b border-border/40 pb-2">
              <span>{m.common_print_multi_up_items_per_sheet()}</span>
              <Badge variant="success" class="text-[10px]">{multiUpLayout.itemsPerSheet}</Badge>
            </div>
            <div class="flex items-center justify-between text-[10px] text-muted-foreground border-b border-border/40 pb-2">
              <span>{m.common_print_multi_up_sheets_needed()}</span>
              <Badge variant="outline" class="text-[10px]">{totalSheets}</Badge>
            </div>
            <div class="flex items-center justify-between text-[10px] text-muted-foreground">
              <span>{m.common_print_multi_up_entity_type()}</span>
              <Badge variant="ghost" class="text-[10px] capitalize">{entityType}</Badge>
            </div>
          </div>

          <Separator />

          <!-- Print Config Panel -->
          <PrintConfigPanel bind:config={multiUpConfig} />
        </div>

        <!-- Right Panel: Preview -->
        <div class="grow flex flex-col gap-3 overflow-hidden">
          <Tabs bind:value={activeTab} class="grow flex flex-col overflow-hidden">
            <TabsList class="w-full shrink-0">
              <TabsTrigger value="preview" class="flex-1 text-xs">
                {m.common_action_preview()}
              </TabsTrigger>
              <TabsTrigger value="pdf" class="flex-1 text-xs">
                PDF
              </TabsTrigger>
            </TabsList>

            <!-- Layout Preview Tab -->
            <TabsContent value="preview" class="grow flex items-center justify-center p-4 border border-border rounded-xl bg-muted/10 mt-2 overflow-hidden">
              <MultiUpPreview
                layout={multiUpLayout}
                totalRecords={entityIds.length}
                bind:currentPage={previewPage}
                entityLabels={entityLabels}
              />
            </TabsContent>

            <!-- PDF Preview Tab -->
            <TabsContent value="pdf" class="grow border border-border rounded-xl bg-muted/20 mt-2 relative min-h-70 overflow-hidden flex items-center justify-center">
              {#if generatingPdf}
                <div class="absolute inset-0 bg-background/60 backdrop-blur-xs flex flex-col items-center justify-center gap-2 z-10">
                  <RefreshCw size={20} class="text-primary animate-spin" />
                  <p class="text-[10px] font-semibold">{m.common_print_multi_up_preview_loading()}</p>
                </div>
              {/if}

              {#if pdfUrl}
                <iframe
                  src={pdfUrl}
                  title="Multi-Up Print Preview"
                  class="w-full h-full border-none"
                ></iframe>
              {:else}
                <div class="text-muted-foreground flex flex-col items-center gap-2 text-[10px]">
                  <FileText size={28} />
                  <p>{m.common_print_multi_up_preview_empty()}</p>
                  <Button
                    variant="outline"
                    size="xs"
                    onclick={generatePreview}
                    disabled={generatingPdf || !selectedTemplateId}
                    class="mt-2 cursor-pointer"
                  >
                    <RefreshCw size={12} class="me-1" />
                    {m.common_print_multi_up_refresh_preview()}
                  </Button>
                </div>
              {/if}
            </TabsContent>
          </Tabs>

          <!-- Suggested Layouts -->
          {#if suggestions.length > 1}
            <div class="flex flex-col gap-1.5">
              <Label class="text-[10px] text-muted-foreground font-semibold">
                {m.common_print_multi_up_suggested_layouts()}
              </Label>
              <div class="flex items-center gap-1.5 flex-wrap">
                {#each suggestions.slice(0, 4) as suggestion (suggestion.itemsPerSheet)}
                  <Button
                    variant={suggestion.itemsPerSheet === multiUpLayout.itemsPerSheet ? "default" : "outline"}
                    size="xs"
                    class="text-[10px] cursor-pointer"
                    onclick={() => applySuggestion(suggestion)}
                  >
                    {suggestion.columns}×{suggestion.rows}={suggestion.itemsPerSheet}
                  </Button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <!-- ═══ Footer ═══ -->
    <DialogFooter class="shrink-0 border-t border-border/40 pt-3 gap-2 sm:gap-0">
      <Button variant="outline" size="sm" onclick={() => (open = false)}>
        {m.common_action_cancel()}
      </Button>

      {#if pdfUrl && !isBatchProcessing}
        <Button variant="outline" size="sm" onclick={triggerDownload}>
          <Download size={14} class="me-1" />
          {m.common_print_multi_up_download()}
        </Button>
      {/if}

      <Button
        variant="lime"
        size="sm"
        disabled={!selectedTemplateId || generatingPdf || isBatchProcessing}
        onclick={handlePrintAction}
      >
        <Printer size={14} class="me-1.5" />
        {entityIds.length > multiUpLayout.itemsPerSheet
          ? m.common_print_multi_up_start_batch_count({ count: String(entityIds.length) })
          : m.common_print_multi_up_start_batch()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
