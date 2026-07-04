<script lang="ts">
/**
 * Template Designer — Main Editor Page (v2.0)
 *
 * Full WYSIWYG visual editor with:
 *   - SVG Canvas with zoom/pan
 *   - Layers panel (left)
 *   - Inspector panel (right)
 *   - Toolbar (top)
 *   - Status bar (bottom of canvas)
 *
 * Layout:
 * ┌────────────────────────────────────────────────┐
 * │ Header (back + title + status)                 │
 * ├────────────────────────────────────────────────┤
 * │ ToolbarPanel                                   │
 * ├──────────┬─────────────────────┬───────────────┤
 * │ Layers   │     CanvasRoot      │  Inspector    │
 * │ Panel    │   (SVG viewport)    │  Panel        │
 * │ (240px)  │                     │  (260px)      │
 * └──────────┴─────────────────────┴───────────────┘
 */
import { page } from "$app/state";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/i18n-utils";
import {
  ArrowLeft,
  RefreshCw,
  Sparkles,
} from "lucide-svelte";

import { deserializeDocument, serializeDocument } from "@docforge/core";
import { createEditorStore } from "$lib/components/template-designer/stores/editor-store.svelte";
import { copyNodes, cutNodes, pasteNodes } from "$lib/components/template-designer/stores/clipboard-manager";
import CanvasRoot from "$lib/components/template-designer/canvas/CanvasRoot.svelte";
import CanvasContextMenu from "$lib/components/template-designer/canvas/CanvasContextMenu.svelte";
import ToolbarPanel from "$lib/components/template-designer/panels/ToolbarPanel.svelte";
import LayersPanel from "$lib/components/template-designer/panels/LayersPanel.svelte";
import PagesPanel from "$lib/components/template-designer/panels/PagesPanel.svelte";
import VariablesPanel from "$lib/components/template-designer/panels/VariablesPanel.svelte";
import InspectorPanel from "$lib/components/template-designer/panels/InspectorPanel.svelte";
import AssetPanel from "$lib/components/template-designer/panels/AssetPanel.svelte";
import HistoryPanel from "$lib/components/template-designer/panels/HistoryPanel.svelte";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import {
  Layers,
  FileText,
  Variable,
  Sliders,
  ImageIcon,
  History,
} from "lucide-svelte";

// ── Router ───────────────────────────────────────────────────

const orgId = $derived(page.params.orgId);
const templateId = $derived(page.params.templateId);

// ── State ────────────────────────────────────────────────────

let templateRecord = $state<any>(null);
let isLoading = $state(true);
let isSaving = $state(false);
let isPreviewLoading = $state(false);
let previewPdfUrl = $state<string | null>(null);
let leftPanelTab = $state("layers");
let rightPanelTab = $state("inspector");
let contextMenuVisible = $state(false);
let contextMenuX = $state(0);
let contextMenuY = $state(0);

// ── Editor Store ─────────────────────────────────────────────

// Create with a minimal placeholder document — replaced on load
let store = createEditorStore({
  version: "2.0",
  meta: {
    name: "",
    documentType: "",
    entityType: "",
    locale: "ar",
    pageSetup: {
      size: "A4",
      orientation: "portrait",
      margins: { top: 10, bottom: 10, left: 10, right: 10 },
      unit: "mm",
    },
  },
  variables: {},
  assets: {},
  pages: [{ type: "Page", id: "page-root", children: [] }],
});

// ── Initialize ───────────────────────────────────────────────

async function initEditor() {
  isLoading = true;
  try {
    // 1. Fetch template record from DB
    templateRecord = await commands.documentTemplates.getTemplate(
      orgId || "",
      templateId || "",
    );

    // 2. Parse and migrate document model
    const doc = deserializeDocument(templateRecord.document_json);

    // 3. Load into editor store
    store.loadDocument(doc);

    isLoading = false;
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to initialize editor";
    toast.error(msg);
    isLoading = false;
  }
}

// ── Save ─────────────────────────────────────────────────────

async function saveTemplate() {
  if (!templateRecord || !store.isDirty) return;
  isSaving = true;
  try {
    const docJson = serializeDocument(store.document);
    const setup = store.document.meta.pageSetup;

    const updateRequest = {
      name: templateRecord.name,
      name_ar: templateRecord.name_ar,
      description: templateRecord.description,
      category_id: templateRecord.category_id,
      code: templateRecord.code,
      page_size: setup.size,
      orientation: setup.orientation,
      margins_json: JSON.stringify(setup.margins),
      document_json: docJson,
      locale: templateRecord.locale || "ar",
      tags: templateRecord.tags,
    };

    await commands.documentTemplates.updateTemplate(
      orgId || "",
      templateId || "",
      "current-user",
      updateRequest,
    );

    store.markSaved();
    toast.success(m.settings_templates_saved?.() || "Template saved successfully");
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to save template";
    toast.error(msg);
  } finally {
    isSaving = false;
  }
}

// ── Preview ──────────────────────────────────────────────────

function uint8ArrayToBase64(bytes: number[]): string {
  let binary = "";
  const len = bytes.length;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]!);
  }
  return window.btoa(binary);
}

async function generatePreview() {
  isPreviewLoading = true;
  try {
    const docJson = serializeDocument(store.document);
    const pdfBytes = await commands.docforge.previewDocumentTemplate(
      docJson,
      '{"name": "Preview", "id": "preview-001"}',
    );

    if (pdfBytes && pdfBytes.length > 0) {
      const base64 = uint8ArrayToBase64(pdfBytes);
      previewPdfUrl = `data:application/pdf;base64,${base64}`;
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to generate preview";
    toast.error(msg);
  } finally {
    isPreviewLoading = false;
  }
}

// ── Keyboard Shortcuts ───────────────────────────────────────

function handleKeydown(e: KeyboardEvent) {
  const isCtrl = e.ctrlKey || e.metaKey;
  const isInputFocused = e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement;

  // Close context menu on any key
  if (contextMenuVisible) { contextMenuVisible = false; }

  if (isCtrl && e.key === "z") { e.preventDefault(); store.undo(); }
  else if (isCtrl && e.key === "y") { e.preventDefault(); store.redo(); }
  else if (isCtrl && e.key === "s") { e.preventDefault(); saveTemplate(); }
  else if (isCtrl && e.key === "d") { e.preventDefault(); store.duplicateSelected(); }
  else if (isCtrl && e.key === "a" && !isInputFocused) { e.preventDefault(); store.selectAll(); }
  else if (isCtrl && e.key === "c" && !isInputFocused) {
    e.preventDefault();
    copyNodes(store.document, store.selectedIds);
  }
  else if (isCtrl && e.key === "x" && !isInputFocused) {
    e.preventDefault();
    cutNodes(store.document, store.selectedIds).then((cmd) => {
      if (cmd) { store.executeCommand(cmd); store.clearSelection(); }
    });
  }
  else if (isCtrl && e.key === "v" && !isInputFocused) {
    e.preventDefault();
    const parentId = store.activePage?.id;
    if (parentId) {
      pasteNodes(parentId).then((cmd) => {
        if (cmd) store.executeCommand(cmd);
      });
    }
  }
  else if (e.key === "Delete" || e.key === "Backspace") {
    if (store.selectedIds.length > 0 && !isInputFocused) {
      e.preventDefault();
      store.deleteSelected();
    }
  }
  else if (e.key === "Escape") { store.clearSelection(); }
  // Arrow key nudge
  else if (["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.key) && !isInputFocused) {
    if (store.selectedIds.length > 0 && store.selectedNode) {
      e.preventDefault();
      const nudge = e.shiftKey ? 10 : 1; // mm
      const pos = (store.selectedNode as any).position;
      if (pos) {
        const dx = e.key === "ArrowLeft" ? -nudge : e.key === "ArrowRight" ? nudge : 0;
        const dy = e.key === "ArrowUp" ? -nudge : e.key === "ArrowDown" ? nudge : 0;
        store.moveSelected(
          Math.round((pos.x + dx) * 10) / 10,
          Math.round((pos.y + dy) * 10) / 10,
        );
      }
    }
  }
}

function handleContextMenu(e: MouseEvent) {
  e.preventDefault();
  contextMenuX = e.clientX;
  contextMenuY = e.clientY;
  contextMenuVisible = true;
}

// ── Lifecycle ────────────────────────────────────────────────

onMount(() => {
  initEditor();
  window.addEventListener("keydown", handleKeydown);
  return () => {
    window.removeEventListener("keydown", handleKeydown);
    store.destroy();
  };
});
</script>

<div class="h-full w-full flex flex-col bg-background/40" oncontextmenu={handleContextMenu}>

  <!-- ═══ Header ═══ -->
  <header class="h-12 border-b border-border bg-card/60 px-4 flex items-center justify-between gap-4 flex-nowrap select-none shrink-0">
    <div class="flex items-center gap-3">
      <Button variant="ghost" size="icon-sm" href={`/${orgId}/templates`} class="cursor-pointer">
        <ArrowLeft size={16} class="rtl:rotate-180" />
      </Button>
      <div class="flex flex-col text-start">
        <div class="flex items-center gap-2">
          <h1 class="text-sm font-bold text-foreground truncate max-w-[250px]">
            {templateRecord ? l(templateRecord.name_ar, templateRecord.name) : "..."}
          </h1>
          {#if templateRecord}
            <Badge variant="outline" class="text-[9px] capitalize">{templateRecord.entity_type}</Badge>
          {/if}
          {#if store.isDirty}
            <Badge variant="secondary" class="text-[9px]">●</Badge>
          {/if}
        </div>
        <div class="flex items-center gap-1.5">
          <Sparkles size={10} class="text-primary" />
          <p class="text-[9px] text-muted-foreground">Template Designer v2.0</p>
        </div>
      </div>
    </div>
  </header>

  {#if isLoading}
    <!-- Loading State -->
    <div class="flex-grow flex flex-col items-center justify-center gap-3">
      <RefreshCw size={24} class="animate-spin text-primary" />
      <p class="text-xs text-muted-foreground">جاري تحميل المحرر...</p>
    </div>
  {:else}
    <!-- ═══ Toolbar ═══ -->
    <ToolbarPanel {store} onsave={saveTemplate} onpreview={generatePreview} />

    <!-- ═══ Main Editor Area ═══ -->
    <div class="flex-grow flex overflow-hidden">

      <!-- Left Panel: Tabbed (Layers / Pages / Variables) -->
      <aside class="w-[240px] border-e border-border bg-card/10 shrink-0 overflow-hidden flex flex-col">
        <Tabs bind:value={leftPanelTab} class="flex flex-col h-full">
          <TabsList class="flex h-8 p-0 bg-muted/20 border-b border-border/30 rounded-none shrink-0">
            <TabsTrigger value="layers" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <Layers size={11} />
              الطبقات
            </TabsTrigger>
            <TabsTrigger value="pages" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <FileText size={11} />
              الصفحات
            </TabsTrigger>
            <TabsTrigger value="variables" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <Variable size={11} />
              المتغيرات
            </TabsTrigger>
          </TabsList>
          <TabsContent value="layers" class="flex-grow overflow-hidden mt-0">
            <LayersPanel {store} />
          </TabsContent>
          <TabsContent value="pages" class="flex-grow overflow-hidden mt-0">
            <PagesPanel {store} />
          </TabsContent>
          <TabsContent value="variables" class="flex-grow overflow-hidden mt-0">
            <VariablesPanel {store} />
          </TabsContent>
        </Tabs>
      </aside>

      <!-- Center: Canvas -->
      <div class="flex-grow flex flex-col overflow-hidden relative">
        {#if store.editorMode === "preview" && previewPdfUrl}
          <!-- PDF Preview Mode -->
          <div class="flex-grow relative">
            <iframe src={previewPdfUrl} title="PDF Preview" class="w-full h-full border-none"></iframe>
            <div class="absolute top-2 end-2">
              <Button variant="outline" size="xs" onclick={() => { store.editorMode = "design"; }} class="cursor-pointer">
                Back to Editor
              </Button>
            </div>
          </div>
        {:else}
          <!-- Design Canvas -->
          <CanvasRoot {store} />
        {/if}
      </div>

      <!-- Right Panel: Tabbed (Inspector / Assets / History) -->
      <aside class="w-[260px] border-s border-border bg-card/10 shrink-0 overflow-hidden flex flex-col">
        <Tabs bind:value={rightPanelTab} class="flex flex-col h-full">
          <TabsList class="flex h-8 p-0 bg-muted/20 border-b border-border/30 rounded-none shrink-0">
            <TabsTrigger value="inspector" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <Sliders size={11} />
              الخصائص
            </TabsTrigger>
            <TabsTrigger value="assets" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <ImageIcon size={11} />
              الأصول
            </TabsTrigger>
            <TabsTrigger value="history" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <History size={11} />
              السجل
            </TabsTrigger>
          </TabsList>
          <TabsContent value="inspector" class="flex-grow overflow-hidden mt-0">
            <InspectorPanel {store} />
          </TabsContent>
          <TabsContent value="assets" class="flex-grow overflow-hidden mt-0">
            <AssetPanel {store} />
          </TabsContent>
          <TabsContent value="history" class="flex-grow overflow-hidden mt-0">
            <HistoryPanel {store} />
          </TabsContent>
        </Tabs>
      </aside>

    </div>
  {/if}

  <!-- Context Menu -->
  <CanvasContextMenu
    {store}
    x={contextMenuX}
    y={contextMenuY}
    visible={contextMenuVisible}
    onclose={() => contextMenuVisible = false}
  />
</div>
