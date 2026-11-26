<script lang="ts">
import { onDestroy } from "svelte";
import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
/**
 * PrintDialog — Enhanced bulk printing dialog with Multi-Up support.
 * Supports multiple documents per sheet with WYSIWYG preview.
 */
import { page } from "$app/state";
import {
  api,
  extractErrorMessage,
  isDesktopApp,
  queryKeys,
  type BatchPrintParams,
} from "$lib/api";
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

import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import {
  CheckCircle,
  DocumentPage,
  Download,
  Grid,
  InfoCircle,
  Label,
  Refresh,
  Printer,
} from "@orbis/icons";

import {
  computeMultiUpLayout,
  computeTotalSheets,
  suggestLayouts,
  createDefaultMultiUpConfig,
  getPageDimensions,
  layoutLabel,
  layoutLabelAr,
} from "@docforge/core";
import type { MultiUpConfig, MultiUpLayout } from "@docforge/core";

import MultiUpPreview from "./multi-up-preview.svelte";
import PrintConfigPanel from "./print-config-panel.svelte";
import { resolveDocumentImageDataUri } from "$lib/features/documents/queries";
import {
  createShowInFolderToastAction,
  downloadBlob,
  revealDownloadedPath,
} from "$lib/shared/utils/download";
import { printState, savePrintPreferences } from "$lib/stores/print.svelte";
import SplitActionMenu, {
  type SplitActionItem,
} from "$lib/shared/components/split-action-menu.svelte";

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

let selectedTemplateId = $state<string>("");
let activeTab = $state("layout");
let pdfPreviewLoading = $state(false);
let pdfUrl = $state<string | null>(null);
let pdfBytes: Uint8Array | null = null;
let previewError = $state<string | null>(null);
let pdfPreviewFrame: HTMLIFrameElement | undefined = $state();
let printAfterPreviewLoad = false;
let pdfLoadTimeout: ReturnType<typeof setTimeout> | null = null;
let dialogOpenRequest = 0;

// Batch progress
let batchProgress = $state(0);
let isBatchProcessing = $state(false);
let batchStatusText = $state("");
let batchJobId = $state<string | null>(null);
let generatingPdf = $state(false);

// Multi-Up config (initialized when template is selected)
let multiUpConfig = $state<MultiUpConfig>(createDefaultMultiUpConfig(90, 50));
let previewPage = $state(1);

// ── Derived ──────────────────────────────────────────────────

const orgId = $derived(page.params.orgId);
const isRtl = $derived(getLocale() === "ar");
const templatesQuery = createQuery(() => ({
  queryKey: queryKeys.report.templates(orgId, {
    entityType,
    documentType,
    status: "published",
  }),
  queryFn: async () => {
    let list = await api.listTemplates({
      entityType,
      documentType,
      status: "published",
    });
    if (entityType.startsWith("finance") && documentType) return Array.isArray(list) ? list : [];
    if ((!list || list.length === 0) && documentType) {
      list = await api.listTemplates({ documentType, status: "published" });
    }
    if (!list || list.length === 0) {
      list = await api.listTemplates({ entityType, status: "published" });
    }
    if (!list || list.length === 0) {
      list = await api.listTemplates({ status: "published" });
    }
    return Array.isArray(list) ? list : [];
  },
  enabled: Boolean(open && entityType && orgId),
  staleTime: 60_000,
}));
const batchPrintMutation = createMutation(() => ({
  mutationFn: (request: BatchPrintParams) => api.batchPrint(request),
}));
const queryClient = useQueryClient();
const templates = $derived(templatesQuery.data ?? []);
const loadingTemplates = $derived(templatesQuery.isFetching);
const selectedTemplate = $derived(
  templates.find((template) => template.id === selectedTemplateId) ??
    templates.find((template) => template.is_default) ??
    templates[0] ??
    null,
);
let configuredTemplateId = "";
const printActions = $derived<SplitActionItem[]>([
  {
    id: "save-pdf",
    label: m.common_print_multi_up_download(),
    icon: Download,
    disabled: !pdfBytes || generatingPdf || isBatchProcessing,
    onSelect: savePdf,
  },
]);
const selectedTemplateKey = $derived(selectedTemplate?.id ?? "");
const templateLoadError = $derived(
  templatesQuery.error ? extractErrorMessage(templatesQuery.error) : null,
);

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

async function setSelectedTemplateDefault() {
  const selected = selectedTemplate;
  if (!selected || selected.is_default) return;

  try {
    const updated = await api.updateTemplate(selected.id, { is_default: true });
    queryClient.setQueryData(
      queryKeys.report.templates(orgId, {
        entityType,
        documentType,
        status: "published",
      }),
      (current: typeof templates | undefined) =>
        (current ?? []).map((template) => ({
          ...template,
          is_default: template.id === updated.id,
        })),
    );
    toast.success(m.settings_templates_set_default_success());
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    console.error("Failed to set print template as default:", error);
    toast.error(message);
  }
}

function updateConfigFromTemplate(template: Record<string, any>) {
  // Extract template dimensions from document JSON
  try {
    const rawJson = template.documentJson ?? template.document_json;
    const doc = typeof rawJson === "string" ? JSON.parse(rawJson) : rawJson;
    const pageSetup = doc?.meta?.pageSetup ?? doc?.meta?.page_setup;
    if (pageSetup) {
      const orientation = pageSetup.orientation ?? "portrait";
      const { width, height } = getPageDimensions(
        pageSetup.size ?? "A4",
        orientation,
        pageSetup.width ?? pageSetup.width_mm,
        pageSetup.height ?? pageSetup.height_mm,
      );
      const defaults = createDefaultMultiUpConfig(width, height);
      multiUpConfig = {
        ...defaults,
        ...printState.preferences,
        sheetMargins: {
          ...defaults.sheetMargins,
          ...printState.preferences.sheetMargins,
        },
        sheetOrientation:
          printState.preferences.sheetOrientation ?? orientation,
        fillOrder:
          printState.preferences.fillOrder ?? (isRtl ? "rtl-ttb" : "ltr-ttb"),
      };
    }
  } catch {
    // Use defaults
  }
}

function ensureTemplateConfig(template: Record<string, any>) {
  if (configuredTemplateId === template.id) return;
  updateConfigFromTemplate(template);
  configuredTemplateId = template.id;
}

// ── Helpers ──────────────────────────────────────────────────

function clearPdfPreview() {
  clearPdfLoadTimeout();
  pdfPreviewLoading = false;
  if (pdfUrl?.startsWith("blob:")) URL.revokeObjectURL(pdfUrl);
  pdfUrl = null;
  pdfBytes = null;
}

function clearPdfLoadTimeout() {
  if (pdfLoadTimeout !== null) {
    clearTimeout(pdfLoadTimeout);
    pdfLoadTimeout = null;
  }
}

onDestroy(() => {
  clearPdfLoadTimeout();
  clearPdfPreview();
});

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
    const students = await Promise.all(
      entityIds.map((id) => api.getStudent(id)),
    );
    const enrollments = await Promise.all(
      students.map(({ student }) =>
        api.listEnrollments({ student_id: student.id, page_size: 100 }),
      ),
    );
    const activeEnrollments = enrollments.map((studentEnrollments) => {
      const latestWithStatus = (status: string) =>
        studentEnrollments
          .filter((item) => item.status === status)
          .sort((left, right) =>
            right.enrollment_date.localeCompare(left.enrollment_date),
          )[0] ?? null;

      return latestWithStatus("active") ?? latestWithStatus("pending_approval");
    });
    const classIds = [
      ...new Set(
        activeEnrollments
          .map((enrollment) => enrollment?.section_id)
          .filter((id): id is string => Boolean(id)),
      ),
    ];
    const classes = await Promise.all(classIds.map((id) => api.getSection(id)));
    const classesById = new Map(classes.map((item) => [item.id, item]));

    return Promise.all(
      students.map(async ({ student, person }, index) => {
        const enrollment = activeEnrollments[index];
        const classInfo = enrollment?.section_id
          ? classesById.get(enrollment.section_id)
          : undefined;
        const avatarSource = person.avatar_url ?? student.avatar_url;
        const avatarUrl = avatarSource
          ? await resolveDocumentImageDataUri(
              avatarSource,
              "student",
              student.id,
            )
          : undefined;
        return {
          student: {
            ...student,
            name: person.full_name,
            nameAr: person.full_name_ar ?? student.name_ar ?? student.full_name,
            studentNumber: student.student_number,
            student_number: student.student_number,
            nationalId:
              person.national_id ??
              student.national_id ??
              person.passport_number ??
              student.passport_number,
            national_id:
              person.national_id ??
              student.national_id ??
              person.passport_number ??
              student.passport_number,
            avatar_url: avatarUrl,
            avatarUrl,
            classId: enrollment?.section_id,
            className: isRtl
              ? (classInfo?.name_ar ?? classInfo?.name)
              : classInfo?.name,
            classNameAr: classInfo?.name_ar,
            class_name: isRtl
              ? (classInfo?.name_ar ?? classInfo?.name)
              : classInfo?.name,
            class_name_ar: classInfo?.name_ar,
            class_code: classInfo?.code,
            level: classInfo?.level,
          },
          person: { ...person, avatar_url: avatarUrl },
          enrollment,
          class: classInfo,
          label: entityLabels[index],
        };
      }),
    );
  }
  return entityIds.map((id, index) => ({
    entity: { id, label: entityLabels[index] },
  }));
}

async function generatePdf(): Promise<boolean> {
  if (!selectedTemplateKey) {
    previewError = m.common_print_multi_up_no_templates();
    return false;
  }
  if (entityIds.length === 0) {
    previewError = m.shared_ui_print_preview_failed();
    return false;
  }
  generatingPdf = true;
  clearPdfPreview();
  previewError = null;
  try {
    const template = selectedTemplate;
    if (!template) throw new Error(m.common_print_multi_up_no_templates());

    const docJson = template.document_json;
    const generatedBytes = await batchPrintMutation.mutateAsync({
      templateId: selectedTemplateKey,
      documentJson: docJson,
      contexts: await resolveContexts(),
      title: template.name,
      batchConfig: JSON.parse(serializeMultiUpConfig(multiUpConfig)),
      locale: getLocale(),
    });
    const isPdf =
      generatedBytes.length >= 4 &&
      generatedBytes[0] === 0x25 &&
      generatedBytes[1] === 0x50 &&
      generatedBytes[2] === 0x44 &&
      generatedBytes[3] === 0x46;
    if (!isPdf) throw new Error(m.shared_ui_print_preview_failed());

    pdfBytes = generatedBytes;
    pdfUrl = URL.createObjectURL(
      new Blob([new Uint8Array(generatedBytes)], { type: "application/pdf" }),
    );
    pdfPreviewLoading = true;
    activeTab = "preview";
    pdfLoadTimeout = setTimeout(() => {
      pdfLoadTimeout = null;
      if (!pdfPreviewLoading) return;
      pdfPreviewLoading = false;
      previewError = m.shared_ui_print_frame_unavailable();
      if (printAfterPreviewLoad) {
        printAfterPreviewLoad = false;
        isBatchProcessing = false;
      }
      toast.error(previewError);
    }, 30_000);
    return true;
  } catch (e: unknown) {
    previewError = extractErrorMessage(e) || m.shared_ui_print_preview_failed();
    console.error("Failed to generate print preview:", e);
    toast.error(previewError);
    return false;
  } finally {
    generatingPdf = false;
  }
}

function handleTemplateChange(templateId: string | undefined) {
  if (!templateId) return;
  selectedTemplateId = templateId;
  clearPdfPreview();
  previewError = null;
  const template = templates.find((item) => item.id === templateId);
  if (template) ensureTemplateConfig(template);
  void generatePdf();
}

function handlePreviewRequest() {
  if (loadingTemplates) return;
  if (templatesQuery.isError || templates.length === 0) {
    void reloadTemplates(true);
    return;
  }
  if (selectedTemplate) ensureTemplateConfig(selectedTemplate);
  void generatePdf();
}

async function reloadTemplates(force = false) {
  const requestId = dialogOpenRequest;
  const data =
    force || !templatesQuery.data
      ? (await templatesQuery.refetch()).data
      : templatesQuery.data;
  if (!open || requestId !== dialogOpenRequest || !data?.length) return;

  const initialTemplate =
    data.find((template) => template.is_default) ?? data[0];
  if (!initialTemplate) return;

  selectedTemplateId = initialTemplate.id;
  ensureTemplateConfig(initialTemplate);
  previewError = null;
}

function handleConfigChange() {
  savePrintPreferences(multiUpConfig);
  clearPdfPreview();
  previewError = null;
}

function handleTabChange(value: string) {
  activeTab = value;
  if (value === "preview" && !pdfUrl && !generatingPdf) {
    handlePreviewRequest();
  }
}

function handleDialogOpenChange(nextOpen: boolean) {
  dialogOpenRequest += 1;
  open = nextOpen;
  if (nextOpen) {
    previewError = null;
    batchProgress = 0;
    isBatchProcessing = false;
    batchJobId = null;
    previewPage = 1;
    activeTab = "layout";
    void reloadTemplates();
    return;
  }
  printAfterPreviewLoad = false;
  clearPdfPreview();
  isBatchProcessing = false;
}

// ── Print Action ─────────────────────────────────────────────

function handlePdfPreviewLoad() {
  pdfPreviewLoading = false;
  clearPdfLoadTimeout();
  if (!printAfterPreviewLoad) return;
  printAfterPreviewLoad = false;
  const printWindow = pdfPreviewFrame?.contentWindow;
  if (!printWindow) {
    isBatchProcessing = false;
    toast.error(m.shared_ui_print_frame_unavailable());
    return;
  }
  try {
    printWindow.addEventListener(
      "afterprint",
      () => {
        batchProgress = 100;
        toast.success(
          m.common_print_multi_up_complete_detail({
            count: String(entityIds.length),
          }),
        );
      },
      { once: true },
    );
    printWindow.focus();
    printWindow.print();
  } catch (error) {
    isBatchProcessing = false;
    toast.error(
      extractErrorMessage(error) || m.shared_ui_print_frame_unavailable(),
    );
  }
  isBatchProcessing = false;
}

function handlePdfPreviewError() {
  pdfPreviewLoading = false;
  clearPdfLoadTimeout();
  previewError = m.shared_ui_print_frame_unavailable();
  if (printAfterPreviewLoad) {
    printAfterPreviewLoad = false;
    isBatchProcessing = false;
  }
  toast.error(previewError);
}

async function handlePrintAction() {
  if (isBatchProcessing || generatingPdf) return;
  if (!selectedTemplateKey) {
    toast.error(m.common_print_multi_up_no_templates());
    return;
  }
  if (entityIds.length === 0) {
    previewError = m.shared_ui_print_preview_failed();
    toast.error(previewError);
    return;
  }

  isBatchProcessing = true;
  batchProgress = 30;
  batchStatusText = m.common_print_multi_up_processing();
  if (selectedTemplate) ensureTemplateConfig(selectedTemplate);
  printAfterPreviewLoad = true;
  if (!(await generatePdf())) {
    printAfterPreviewLoad = false;
    isBatchProcessing = false;
    return;
  }
  if (!printAfterPreviewLoad) return;

  batchProgress = 85;
}

async function savePdf() {
  if (!pdfBytes) {
    toast.error(m.shared_ui_print_preview_failed());
    return;
  }

  const filename = `multi_up_${entityType}_${Date.now()}.pdf`;
  try {
    if (!isDesktopApp) {
      downloadBlob(
        new Blob([new Uint8Array(pdfBytes)], { type: "application/pdf" }),
        filename,
      );
      toast.success(m.common_print_multi_up_pdf_saved());
      return;
    }

    const [{ save }, { writeFile }] = await Promise.all([
      import("@tauri-apps/plugin-dialog"),
      import("@tauri-apps/plugin-fs"),
    ]);
    const path = await save({
      defaultPath: filename,
      filters: [{ name: "PDF", extensions: ["pdf"] }],
    });
    if (!path) return;

    await writeFile(path, new Uint8Array(pdfBytes));
    toast.success(m.common_print_multi_up_pdf_saved(), {
      action: createShowInFolderToastAction(revealDownloadedPath(path)),
    });
  } catch (error) {
    console.error("Failed to save print PDF:", error);
    toast.error(extractErrorMessage(error));
  }
}

function applySuggestion(suggestion: MultiUpLayout) {
  // Update config to match the suggestion's orientation
  multiUpConfig.sheetOrientation =
    suggestion.sheetWidthMm < suggestion.sheetHeightMm
      ? "portrait"
      : "landscape";
  handleConfigChange();
}
</script>

<Dialog bind:open={open} onOpenChange={handleDialogOpenChange}>
  <DialogContent class="w-[calc(100dvw-1rem)] h-[92dvh] max-h-[92dvh] text-start flex flex-col overflow-hidden sm:w-[calc(100dvw-2rem)] sm:max-w-195">
    <DialogHeader class="shrink-0">
      <DialogTitle class="flex items-center gap-2">
        <Grid class="text-primary h-5 w-5" />
        <span>{m.common_print_multi_up_title()}</span>
      </DialogTitle>
    </DialogHeader>

    <!-- ═══ Main Content: Config + Preview ═══ -->
    <div class="relative flex-1 min-h-0 overflow-hidden">
      <div class="absolute inset-0 flex flex-col md:flex-row gap-3 md:gap-4 overflow-hidden py-2 md:py-3">

        <!-- Left Panel: Config -->
        <div class="w-full max-h-[36%] min-h-0 md:max-h-none md:w-60 flex flex-col gap-3 shrink-0 overflow-y-auto scrollbar-hide">
          <!-- Template Selection -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-muted-foreground">
              {m.common_print_multi_up_template()}
            </Label>
            {#if loadingTemplates}
              <div class="h-8 bg-muted rounded animate-pulse"></div>
            {:else if templatesQuery.isError && templates.length === 0}
              <div class="space-y-2">
                <p class="text-xs text-destructive" role="alert">{m.common_loading_error()}</p>
                <Button type="button" size="sm" variant="outline" onclick={() => templatesQuery.refetch()}>{m.common_retry()}</Button>
              </div>
            {:else if templates.length === 0}
              <div class="text-xs text-destructive flex items-center gap-1">
                <InfoCircle size={12} />
                <span>{m.common_print_multi_up_no_templates()}</span>
              </div>
              {#if orgId && entityType.startsWith("finance")}
                <a href={localizeHref("/" + orgId + "/templates")} class="text-xs text-primary underline underline-offset-2">{m.finance_ui_manage_print_templates()}</a>
              {/if}
            {:else}
              <Select type="single" value={selectedTemplateKey} onValueChange={handleTemplateChange}>
                <SelectTrigger class="h-8 text-xs" disabled={generatingPdf || loadingTemplates}>
                  {(() => {
                    return selectedTemplate ? (isRtl && selectedTemplate.name_ar ? selectedTemplate.name_ar : selectedTemplate.name) : m.common_print_multi_up_template_placeholder();
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
              {#if selectedTemplate && !selectedTemplate.is_default}
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
          <PrintConfigPanel
            bind:config={multiUpConfig}
            onConfigChange={handleConfigChange}
            disabled={generatingPdf || isBatchProcessing}
          />
        </div>

        <!-- Right Panel: Preview -->
        <div class="min-h-0 min-w-0 flex-1 flex flex-col gap-2 md:gap-3 overflow-hidden">
          <Tabs value={activeTab} onValueChange={handleTabChange} class="min-h-0 min-w-0 flex-1 flex flex-col overflow-hidden">
            <TabsList class="w-full shrink-0">
              <TabsTrigger value="preview" class="flex-1 text-xs">
                {m.common_action_preview()}
              </TabsTrigger>
              <TabsTrigger value="layout" class="flex-1 text-xs">
                {m.common_print_multi_up_layout_preview()}
              </TabsTrigger>
            </TabsList>

            <!-- Rendered document preview -->
            <TabsContent value="preview" class="min-h-0 min-w-0 flex-1 border border-border rounded-xl bg-muted/20 mt-2 relative overflow-hidden flex items-center justify-center">
              {#if generatingPdf || pdfPreviewLoading}
                <div class="absolute inset-0 bg-background/60 backdrop-blur-xs flex flex-col items-center justify-center gap-2 z-10">
                  <LoadingIndicator class="text-primary"  size={20} />
                  <p class="text-[10px] font-semibold">{m.common_print_multi_up_preview_loading()}</p>
                </div>
              {/if}

              {#if pdfUrl}
                <iframe
                  bind:this={pdfPreviewFrame}
                  src={pdfUrl}
                  title={m.shared_ui_multi_up_print_preview()}
                  class="w-full h-full border-none"
                  onload={handlePdfPreviewLoad}
                  onerror={handlePdfPreviewError}
                ></iframe>
              {:else}
                <div class="text-muted-foreground flex flex-col items-center gap-2 text-[10px]">
                  <DocumentPage size={28} />
                  <p class={previewError || templateLoadError ? "text-destructive" : ""}>
                    {previewError || templateLoadError ||
                      (generatingPdf
                        ? m.common_print_multi_up_preview_loading()
                        : m.common_print_multi_up_preview_empty())}
                  </p>
                  <Button
                    variant="outline"
                    size="xs"
                    onclick={handlePreviewRequest}
                    disabled={generatingPdf || loadingTemplates || entityIds.length === 0}
                    class="mt-2 cursor-pointer"
                  >
                    <Refresh size={12} class="me-1" />
                    {templatesQuery.isError || templates.length === 0
                      ? m.common_retry()
                      : m.common_action_preview()}
                  </Button>
                </div>
              {/if}
            </TabsContent>

            <!-- Layout-only preview -->
            <TabsContent value="layout" class="min-h-0 min-w-0 flex-1 flex items-center justify-center p-2 md:p-3 border border-border rounded-xl bg-muted/10 mt-2 overflow-hidden">
              <MultiUpPreview
                layout={multiUpLayout}
                totalRecords={entityIds.length}
                cutLineWidthMm={multiUpConfig.cutLineWidthMm}
                cutLineColor={multiUpConfig.cutLineColor}
                sheetMargins={multiUpConfig.sheetMargins}
                bind:currentPage={previewPage}
              />
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
                    disabled={generatingPdf || isBatchProcessing}
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

      {#if isBatchProcessing}
        <div
          class="absolute inset-0 z-20 flex flex-col items-center justify-center gap-6 bg-background/85 px-4 backdrop-blur-xs"
          role="status"
          aria-live="polite"
        >
          <div class="w-full max-w-md flex flex-col gap-3">
            <div class="flex items-center justify-between text-xs text-muted-foreground">
              <span>{batchStatusText}</span>
              <span class="font-bold text-foreground">{batchProgress}%</span>
            </div>
            <Progress value={batchProgress} class="h-2 w-full" />
          </div>
          {#if batchProgress === 100}
            <div class="flex items-center gap-2 text-emerald-500 text-xs font-semibold mt-2">
              <CheckCircle size={16} />
              <span>{m.common_print_multi_up_complete()}</span>
            </div>
          {:else}
            <LoadingIndicator class="text-primary mt-2" size={24} />
          {/if}
        </div>
      {/if}
    </div>

    <!-- ═══ Footer ═══ -->
    <DialogFooter class="shrink-0 flex-row items-center justify-between border-t border-border/40 pt-4 gap-3">
      <Button
        variant="outline"
        size="lg"
        onclick={() => (open = false)}
      >
        {m.common_action_cancel()}
      </Button>

      <SplitActionMenu
        label={entityIds.length > multiUpLayout.itemsPerSheet
          ? m.common_print_multi_up_start_batch_count({ count: String(entityIds.length) })
          : m.common_print_multi_up_start_batch()}
        menuLabel={m.operations_ui_actions()}
        icon={Printer}
        actions={printActions}
        variant="lime"
        size="lg"
        disabled={!selectedTemplateKey || generatingPdf || isBatchProcessing || loadingTemplates || entityIds.length === 0}
        onprimary={handlePrintAction}
      />
    </DialogFooter>
  </DialogContent>
</Dialog>
