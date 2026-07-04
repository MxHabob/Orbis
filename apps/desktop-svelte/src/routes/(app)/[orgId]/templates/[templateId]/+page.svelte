<script lang="ts">
/**
 * Template Designer — Main Editor Page (v3.0 — Canva-Inspired)
 *
 * Layout:
 * ┌──────────────────────────────────────────────────┐
 * │ ContextToolbar (contextual by selection)          │
 * ├────────────────────────────────────────────┬──────┤
 * │                                            │ Side │
 * │   Canvas (WYSIWYG) + FloatingActionBar     │ Rail │
 * │   + FlyoutPanel (overlay)                  │ 56px │
 * │                                            │      │
 * ├────────────────────────────────────────────┴──────┤
 * │ StatusBar (zoom slider | page | cursor | snap)    │
 * └──────────────────────────────────────────────────┘
 */
import { page } from "$app/state";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/i18n-utils";
import { RefreshCw } from "lucide-svelte";

import { deserializeDocument, serializeDocument } from "@docforge/core";
import { createEditorStore } from "$lib/components/template-designer/stores/editor-store.svelte";
import { copyNodes, cutNodes, pasteNodes } from "$lib/components/template-designer/stores/clipboard-manager";
import CanvasRoot from "$lib/components/template-designer/canvas/CanvasRoot.svelte";
import CanvasContextMenu from "$lib/components/template-designer/canvas/CanvasContextMenu.svelte";

// ── New Layout Components ────────────────────────────────────
import SideRail from "$lib/components/template-designer/layout/SideRail.svelte";
import FlyoutPanel from "$lib/components/template-designer/layout/FlyoutPanel.svelte";
import ContextToolbar from "$lib/components/template-designer/layout/ContextToolbar.svelte";
import FloatingActionBar from "$lib/components/template-designer/layout/FloatingActionBar.svelte";
import StatusBar from "$lib/components/template-designer/layout/StatusBar.svelte";

// ── Router ───────────────────────────────────────────────────

const orgId = $derived(page.params.orgId);
const templateId = $derived(page.params.templateId);

// ── State ────────────────────────────────────────────────────

let templateRecord = $state<any>(null);
let isLoading = $state(true);
let isSaving = $state(false);
let isPreviewLoading = $state(false);
let previewPdfUrl = $state<string | null>(null);
let contextMenuVisible = $state(false);
let contextMenuX = $state(0);
let contextMenuY = $state(0);

// ── Editor Store ─────────────────────────────────────────────

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
    templateRecord = await commands.documentTemplates.getTemplate(
      orgId || "",
      templateId || "",
    );

    const doc = deserializeDocument(templateRecord.document_json);
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
      store.editorMode = "preview";
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

  if (contextMenuVisible) { contextMenuVisible = false; }

  // Close flyout on Escape
  if (e.key === "Escape" && store.activeRailPanel) {
    e.preventDefault();
    store.activeRailPanel = null;
    return;
  }

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
  else if (["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.key) && !isInputFocused) {
    if (store.selectedIds.length > 0 && store.selectedNode) {
      e.preventDefault();
      const nudge = e.shiftKey ? 10 : 1;
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

// ── Back Navigation ──────────────────────────────────────────

function goBack() {
  goto(`/${orgId}/templates`);
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

<div class="editor-root" oncontextmenu={handleContextMenu}>

  {#if isLoading}
    <!-- Loading State -->
    <div class="flex-grow flex flex-col items-center justify-center gap-3 bg-background/40">
      <RefreshCw size={24} class="animate-spin text-primary" />
      <p class="text-xs text-muted-foreground">جاري تحميل المحرر...</p>
    </div>
  {:else}
    <!-- ═══ Context Toolbar ═══ -->
    <ContextToolbar
      {store}
      templateName={templateRecord ? l(templateRecord.name_ar, templateRecord.name) : "..."}
      onback={goBack}
      onsave={saveTemplate}
      onpreview={generatePreview}
    />

    <!-- ═══ Main Area: Canvas + SideRail + FlyoutPanel ═══ -->
    <div class="editor-main">

      <!-- Canvas Area (takes full remaining space) -->
      <div class="canvas-area">
        {#if store.editorMode === "preview" && previewPdfUrl}
          <!-- PDF Preview Mode -->
          <div class="flex-grow relative">
            <iframe src={previewPdfUrl} title="PDF Preview" class="w-full h-full border-none"></iframe>
            <div class="absolute top-2 end-2">
              <Button variant="outline" size="xs" onclick={() => { store.editorMode = "design"; }} class="cursor-pointer">
                العودة للمحرر
              </Button>
            </div>
          </div>
        {:else}
          <!-- Design Canvas -->
          <CanvasRoot {store} />

          <!-- Floating Action Bar (overlay above selected element) -->
          <FloatingActionBar {store} />
        {/if}

        <!-- Flyout Panel (overlay from right) -->
        <FlyoutPanel {store} />
      </div>

      <!-- Side Rail (icon strip - always visible) -->
      <SideRail {store} />
    </div>

    <!-- ═══ Status Bar ═══ -->
    <StatusBar {store} />
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

<style>
  .editor-root {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-background);
    overflow: hidden;
  }

  .editor-main {
    flex-grow: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .canvas-area {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }
</style>
