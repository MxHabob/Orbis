<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
/**
 * PrintDialog — Enhanced bulk printing dialog with Multi-Up support.
 * Supports multiple documents per sheet with WYSIWYG preview.
 */
import { page } from "$app/state";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import { Progress } from "@orbis/ui/progress";
import { Separator } from "@orbis/ui/separator";
import { toast } from "@orbis/ui";

import { getLocale } from "$lib/paraglide/runtime";
import { CheckCircle, DocumentPage, Download, Grid, InfoCircle, Label, Refresh, Printer } from "@orbis/icons";

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
  documentType,
  entityIds = [],
  entityLabels = [],
  contexts: suppliedContexts,
}: {
  open: boolean;
  entityType: string;
  documentType?: string;
  entityIds: string[];
  entityLabels?: string[];
  contexts?: Record<string, unknown>[];
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
    const list = await api.listTemplates({
      entityType: entityType,
      documentType,
      status: "published",
    });
    templates = Array.isArray(list) ? list : [];
    if (templates.length > 0) {
      const defaultTemplate = templates.find((template) => template.is_default);
      const initialTemplate = defaultTemplate ?? templates[0];
      selectedTemplateId = initialTemplate.id;
      updateConfigFromTemplate(initialTemplate);
    } else {
      selectedTemplateId = "";
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || m.shared_ui_print_templates_load_failed();
    console.error(msg);
  } finally {
    loadingTemplates = false;
  }
}

async function setSelectedTemplateDefault() {
  const selected = templates.find((template) => template.id === selectedTemplateId);
  if (!selected || selected.is_default) return;

  try {
    const updated = await api.updateTemplate(selected.id, { is_default: true });
    templates = templates.map((template) => ({
      ...template,
      is_default: template.id === updated.id,
    }));
    toast.success(m.settings_templates_set_default_success());
  } catch (error: unknown) {
    const message =
      error instanceof Error ? error.message : String(error);
    console.error("Failed to set print template as default:", error);
    toast.error(message);
  }
}

function updateConfigFromTemplate(template: any) {
  // Extract template dimensions from document JSON
  try {
    const rawJson = template.documentJson ?? template.document_json;
    const doc = typeof rawJson === "string" ? JSON.parse(rawJson) : rawJson;
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
      const [presetWidth, presetHeight] = dimensions[pageSetup.size] ?? [
        90, 50,
      ];
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
  if (
    selectedTemplateId &&
    templates.length > 0 &&
    selectedTemplateId !== lastProcessedTemplateId
  ) {
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

async function resolveContexts(): Promise<Record<string, unknown>[]> {
  if (suppliedContexts) return suppliedContexts;
  if (entityType === "student") {
    const students = await Promise.all(entityIds.map((id) => api.getStudent(id)));
    const classIds = [
      ...new Set(
        students
          .map(({ enrollment }) => enrollment?.class_id)
          .filter((id): id is string => Boolean(id)),
      ),
    ];
    const classes = await Promise.all(classIds.map((id) => api.getSection(id)));
    const classesById = new Map(classes.map((item) => [item.id, item]));

    return students.map(({ student, person, enrollment }, index) => {
      const classInfo = enrollment?.class_id
        ? classesById.get(enrollment.class_id)
        : undefined;
      return {
        student: {
          ...student,
          name: person.full_name,
          nameAr: person.full_name_ar ?? student.name_ar ?? student.full_name,
          studentNumber: student.student_number,
          nationalId: person.national_id ?? student.national_id,
          avatarUrl: student.avatar_url,
          classId: enrollment?.class_id,
          className: classInfo?.name,
          classNameAr: classInfo?.name_ar,
          level: classInfo?.level,
        },
        person,
        enrollment,
        class: classInfo,
        label: entityLabels[index],
      };
    });
  }
  return entityIds.map((id, index) => ({
    entity: { id, label: entityLabels[index] },
  }));
}

async function generatePdf(): Promise<boolean> {
  if (!selectedTemplateId || entityIds.length === 0) return false;
  generatingPdf = true;
  pdfUrl = null;
  try {
    const template = templates.find((t) => t.id === selectedTemplateId);
    if (!template) throw new Error(m.shared_ui_print_preview_failed());

    const docJson = template.documentJson ?? (template as any).document_json;
    const pdfBytes = await api.batchPrint({
      templateId: selectedTemplateId,
      documentJson: docJson,
      contexts: await resolveContexts(),
      title: template.name,
      batchConfig: JSON.parse(serializeMultiUpConfig(multiUpConfig)),
      locale: getLocale(),
    });
    if (!pdfBytes.length) throw new Error(m.shared_ui_print_preview_failed());

    const base64 = uint8ArrayToBase64(Array.from(pdfBytes));
    pdfUrl = `data:application/pdf;base64,${base64}`;
    activeTab = "pdf";
    return true;
  } catch (e: unknown) {
    const msg = (e as any)?.message || m.shared_ui_print_preview_failed();
    toast.error(msg);
    return false;
  } finally {
    generatingPdf = false;
  }
}

const generatePreview = () => generatePdf();

// ── Print Action ─────────────────────────────────────────────

async function handlePrintAction() {
  if (!selectedTemplateId) {
    toast.error(m.common_print_multi_up_no_templates());
    return;
  }

  if (entityIds.length <= multiUpLayout.itemsPerSheet) {
    if (await generatePdf()) {
      toast.success(m.shared_ui_template_preview_generated());
    }
  } else {
    const printWin = window.open("", "_blank");
    if (!printWin) {
      toast.error(m.shared_ui_print_frame_unavailable());
      return;
    }

    isBatchProcessing = true;
    batchProgress = 30;
    batchStatusText = m.common_print_multi_up_processing();

    try {
      const template = templates.find((item) => item.id === selectedTemplateId);
      if (!template) throw new Error(m.shared_ui_print_generation_failed());
      const docJson = template.documentJson ?? (template as any).document_json;
      const pdfBytes = await api.batchPrint({
        templateId: selectedTemplateId,
        documentJson: docJson,
        contexts: await resolveContexts(),
        title: template.name,
        batchConfig: JSON.parse(serializeMultiUpConfig(multiUpConfig)),
        locale: getLocale(),
      });
      if (!pdfBytes.length) throw new Error(m.shared_ui_print_generation_failed());
      batchProgress = 80;

      const blobUrl = URL.createObjectURL(
        new Blob([pdfBytes], { type: "application/pdf" }),
      );
      printWin.addEventListener(
        "load",
        () => {
          printWin.focus();
          printWin.print();
          URL.revokeObjectURL(blobUrl);
        },
        { once: true },
      );
      printWin.location.href = blobUrl;

      batchProgress = 100;
      toast.success(
        m.common_print_multi_up_complete_detail({
          count: String(entityIds.length),
        }),
      );
    } catch (e: unknown) {
      printWin.close();
      const msg = (e as any)?.message || m.shared_ui_print_generation_failed();
      toast.error(msg);
    } finally {
      isBatchProcessing = false;
    }
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
        <Grid class="text-primary h-5 w-5" />
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
            <CheckCircle size={16} />
            <span>{m.common_print_multi_up_complete()}</span>
          </div>
        {:else}
          <LoadingIndicator class="text-primary mt-2"  size={24} />
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
                <InfoCircle size={12} />
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
                      {(isRtl && t.name_ar ? t.name_ar : t.name)}{t.is_default ? ` — ${m.settings_templates_default_badge()}` : ""}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
              {#if templates.some((template) => template.id === selectedTemplateId && !template.is_default)}
                <Button
                  variant="outline"
                  size="xs"
                  class="mt-1 w-full"
                  onclick={setSelectedTemplateDefault}
                >
                  {m.settings_templates_action_set_default()}
                </Button>
              {/if}
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

            <!-- Grid Preview Tab -->
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
                  <LoadingIndicator class="text-primary"  size={20} />
                  <p class="text-[10px] font-semibold">{m.common_print_multi_up_preview_loading()}</p>
                </div>
              {/if}

              {#if pdfUrl}
                <iframe
                  src={pdfUrl}
                  title={m.shared_ui_multi_up_print_preview()}
                  class="w-full h-full border-none"
                ></iframe>
              {:else}
                <div class="text-muted-foreground flex flex-col items-center gap-2 text-[10px]">
                  <DocumentPage size={28} />
                  <p>{m.common_print_multi_up_preview_empty()}</p>
                  <Button
                    variant="outline"
                    size="xs"
                    onclick={generatePreview}
                    disabled={generatingPdf || !selectedTemplateId}
                    class="mt-2 cursor-pointer"
                  >
                    <Refresh size={12} class="me-1" />
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
