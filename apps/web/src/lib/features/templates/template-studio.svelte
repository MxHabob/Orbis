<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import type { TemplateDto, UpdateTemplateRequest } from "$lib/api/types";
import { l } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import ErrorState from "$lib/shared/components/error-state.svelte";
import { TemplateEditorSkeleton } from "$lib/shared/components/skeletons";
import {
  createDocument,
  deserializeDocument,
  serializeDocument,
  type TemplateDocument,
} from "@docforge/core";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { onMount } from "svelte";
import CanvasContextMenu from "./canvas/canvas-context-menu.svelte";
import CanvasRoot from "./canvas/canvas-root.svelte";
import ContextToolbar from "./layout/context-toolbar.svelte";
import FlyoutPanel from "./layout/flyout-panel.svelte";
import SideRail from "./layout/side-rail.svelte";
import StatusBar from "./layout/status-bar.svelte";
import PropertiesInspector from "./panels/properties-inspector.svelte";
import { copyNodes, cutNodes, pasteNodes } from "./stores/clipboard-manager";
import { createEditorStore } from "./stores/editor-store.svelte";
import { registerTemplateAiEditor } from "./template-ai-bridge";
import { templateBindingSchemas } from "./binding-schemas";
import { templatePreviewData } from "./preview-data";

let { orgId, templateId }: { orgId: string; templateId: string } = $props();

// State
let templateRecord = $state<TemplateDto | null>(null);
let isSaving = $state(false);
let isPreviewLoading = $state(false);
let previewPdfUrl = $state<string | null>(null);
let isPropertiesOpen = $state(false);
let contextMenuVisible = $state(false);
let contextMenuX = $state(0);
let contextMenuY = $state(0);
let canvasWidth = $state(0);
let canvasHeight = $state(0);
let cursorMm = $state({ x: 0, y: 0 });

function updateCanvasSize(width: number, height: number) {
  if (width <= 0 || height <= 0) return;
  if (!canvasWidth) store.zoomToFit(width, height);
  canvasWidth = width;
  canvasHeight = height;
}

// Editor Store
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

let propertiesWidth = $state(300);
let isLoading = $state(true);
let error = $state<string | null>(null);
let unregisterTemplateAiEditor: (() => void) | null = null;
let mounted = false;

$effect(() => {
  if (store.selectedIds.length > 0) {
    isPropertiesOpen = true;
    if (window.matchMedia("(max-width: 900px)").matches)
      store.activeRailPanel = null;
  }
});

function toggleProperties() {
  const shouldOpen = !isPropertiesOpen;
  isPropertiesOpen = shouldOpen;
  if (shouldOpen && window.matchMedia("(max-width: 900px)").matches) {
    store.activeRailPanel = null;
  }
}

// Initialize
async function initEditor() {
  isLoading = true;
  error = null;
  try {
    const data = await api.getTemplate(templateId || "");
    if (!mounted) return;
    templateRecord = data;

    const rawJson = data.document_json ?? (data as any).documentJson;
    let doc: TemplateDocument;

    if (
      rawJson &&
      typeof rawJson === "string" &&
      rawJson.trim().startsWith("{")
    ) {
      try {
        doc = deserializeDocument(rawJson);
      } catch (deserErr) {
        console.warn(
          "Failed to deserialize document JSON, generating default:",
          deserErr,
        );
        doc = createDocument(
          data.document_type ?? (data as any).documentType ?? "REPORT",
          data.entity_type ?? (data as any).entityType ?? "student",
          (data.locale as "ar" | "en") || "ar",
        );
      }
    } else {
      doc = createDocument(
        data.document_type ?? (data as any).documentType ?? "REPORT",
        data.entity_type ?? (data as any).entityType ?? "student",
        (data.locale as "ar" | "en") || "ar",
      );
    }

    doc.meta.id = templateId || data.id;
    if (data.name) doc.meta.name = data.name;
    if (data.name_ar ?? (data as any).nameAr) {
      doc.meta.nameAr = data.name_ar ?? (data as any).nameAr;
    }
    if (data.page_size ?? (data as any).pageSize) {
      doc.meta.pageSetup.size = (data.page_size ??
        (data as any).pageSize) as any;
    }
    if (data.orientation) {
      doc.meta.pageSetup.orientation = data.orientation as any;
    }

    store.setSystemSchemas(templateBindingSchemas());
    store.loadDocument(doc);
    unregisterTemplateAiEditor = registerTemplateAiEditor(store);
    isLoading = false;
  } catch (e: unknown) {
    const msg =
      (e as any)?.message || m.shared_ui_template_editor_initialize_failed();
    toast.error(msg);
    error = msg;
    isLoading = false;
  }
}

async function saveTemplate(newName?: string) {
  if (!templateRecord || isSaving) return;
  const name = newName?.trim();
  const arabic = getLocale() === "ar";
  const record = templateRecord;
  const renamed = !!name && name !== l(record.name_ar, record.name);
  if (!renamed && !store.isDirty) return;

  isSaving = true;
  const snapshot = JSON.stringify(store.document);
  try {
    const document = JSON.parse(snapshot) as TemplateDocument;
    const update: UpdateTemplateRequest = {
      name: renamed && !arabic ? name : record.name,
      name_ar: renamed && arabic ? name : record.name_ar,
      description: record.description,
      description_ar: record.description_ar,
      page_size: document.meta.pageSetup.size,
      orientation: document.meta.pageSetup.orientation,
      margins_json: JSON.stringify(document.meta.pageSetup.margins),
      document_json: serializeDocument(document),
    };
    await api.updateTemplate(record.id, update);
    if (!mounted) return;
    templateRecord = {
      ...record,
      name: update.name ?? record.name,
      name_ar: update.name_ar,
    };
    store.markSaved(snapshot);
    toast.success(m.ui_copy_template_saved_successfully_299be978());
  } catch (e: unknown) {
    toast.error(
      e instanceof Error ? e.message : m.shared_ui_template_save_failed(),
    );
  } finally {
    isSaving = false;
  }
}

function closePreview() {
  store.editorMode = "design";
  if (previewPdfUrl) URL.revokeObjectURL(previewPdfUrl);
  previewPdfUrl = null;
}

async function generatePreview() {
  if (isPreviewLoading) return;
  isPreviewLoading = true;
  try {
    const pdfBytes = await api.previewTemplate(
      serializeDocument(store.document),
      templatePreviewData(store.document),
      store.document.meta.locale ?? templateRecord?.locale ?? "ar",
    );
    if (!mounted) return;
    const isPdf =
      pdfBytes[0] === 0x25 &&
      pdfBytes[1] === 0x50 &&
      pdfBytes[2] === 0x44 &&
      pdfBytes[3] === 0x46;
    if (!isPdf) throw new Error(m.shared_ui_template_preview_failed());
    closePreview();
    previewPdfUrl = URL.createObjectURL(
      new Blob([Uint8Array.from(pdfBytes)], { type: "application/pdf" }),
    );
    store.editorMode = "preview";
  } catch (e: unknown) {
    toast.error(
      e instanceof Error ? e.message : m.shared_ui_template_preview_failed(),
    );
  } finally {
    isPreviewLoading = false;
  }
}

// Keyboard Shortcuts
function handleKeydown(e: KeyboardEvent) {
  if (e.defaultPrevented) return;
  const isCtrl = e.ctrlKey || e.metaKey;
  const isInputFocused =
    e.target instanceof HTMLElement &&
    (e.target.isContentEditable ||
      !!e.target.closest("input, textarea, select, [role=combobox]"));

  if (store.editorMode === "preview") {
    if (e.key === "Escape") closePreview();
    return;
  }

  if (contextMenuVisible) {
    contextMenuVisible = false;
  }

  // Close flyout on Escape
  if (e.code === "Escape" && store.activeRailPanel) {
    e.preventDefault();
    store.activeRailPanel = null;
    return;
  }

  if (isCtrl && e.code === "KeyZ" && !isInputFocused) {
    e.preventDefault();
    if (e.shiftKey) {
      store.redo();
    } else {
      store.undo();
    }
  } else if (isCtrl && e.code === "KeyY" && !isInputFocused) {
    e.preventDefault();
    store.redo();
  } else if (isCtrl && e.code === "KeyS") {
    e.preventDefault();
    saveTemplate();
  } else if (isCtrl && e.code === "KeyD" && !isInputFocused) {
    e.preventDefault();
    store.duplicateSelected();
  } else if (isCtrl && e.code === "KeyA" && !isInputFocused) {
    e.preventDefault();
    store.selectAll();
  } else if (isCtrl && e.code === "KeyC" && !isInputFocused) {
    e.preventDefault();
    copyNodes(store.document, store.selectedIds);
  } else if (isCtrl && e.code === "KeyX" && !isInputFocused) {
    e.preventDefault();
    cutNodes(store.document, store.selectedIds).then((cmd) => {
      if (cmd) {
        store.executeCommand(cmd);
        store.clearSelection();
      }
    });
  } else if (isCtrl && e.code === "KeyV" && !isInputFocused) {
    e.preventDefault();
    const parentId = store.activePage?.id;
    if (parentId) {
      pasteNodes(parentId).then((cmd) => {
        if (cmd) store.executeCommand(cmd);
      });
    }
  } else if (e.code === "Delete" || e.code === "Backspace") {
    if (store.selectedIds.length > 0 && !isInputFocused) {
      e.preventDefault();
      store.deleteSelected();
    }
  } else if (e.code === "Escape") {
    store.clearSelection();
  } else if (
    ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.code) &&
    !isInputFocused
  ) {
    if (store.selectedIds.length > 0 && store.selectedNode) {
      e.preventDefault();
      const nudge = e.shiftKey ? 5 : 0.5;
      const pos =
        (store.selectedNode as any).props?.position ??
        (store.selectedNode as any).position;
      if (pos) {
        const dx =
          e.code === "ArrowLeft" ? -nudge : e.code === "ArrowRight" ? nudge : 0;
        const dy =
          e.code === "ArrowUp" ? -nudge : e.code === "ArrowDown" ? nudge : 0;
        store.moveNodes([
          {
            id: store.selectedNode.id,
            x: Math.round((pos.x + dx) * 10) / 10,
            y: Math.round((pos.y + dy) * 10) / 10,
          },
        ]);
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

async function goBack() {
  if (isSaving) return;
  if (store.isDirty) await saveTemplate();
  if (!store.isDirty) await goto(`/${orgId}/templates`);
}

onMount(() => {
  mounted = true;
  store.setAutoSaveCallback(() => {
    void saveTemplate();
  });
  void initEditor();
  window.addEventListener("keydown", handleKeydown);
  return () => {
    window.removeEventListener("keydown", handleKeydown);
    mounted = false;
    closePreview();
    unregisterTemplateAiEditor?.();
    store.destroy();
  };
});
</script>

<div class="editor-root">
  {#if isLoading}
    <TemplateEditorSkeleton />
  {:else if error}
    <div class="h-full w-full flex items-center justify-center bg-card">
      <ErrorState 
        title={m.common_action_edit()} 
        message={error} 
        onRetry={initEditor} 
      />
    </div>
  {:else}
    <ContextToolbar
          {store}
          isPropertiesVisible={isPropertiesOpen}
          ontoggleproperties={toggleProperties}
          templateName={templateRecord ? (l(templateRecord.name_ar, templateRecord.name) ?? undefined) : "..."}
          onback={goBack}
          onsave={saveTemplate}
          onpreview={generatePreview}
          onrename={(name) => {
            const value = name.trim();
            if (value && templateRecord && value !== l(templateRecord.name_ar, templateRecord.name)) void saveTemplate(value);
          }}
          {isSaving}
          {isPreviewLoading}
        />
    <div class="editor-main">
      {#if store.editorMode === "design"}
        <SideRail {store} onopen={() => {
          if (window.matchMedia("(max-width: 900px)").matches) isPropertiesOpen = false;
        }} />
        <FlyoutPanel {store} {orgId} />
      {/if}

      <div class="canvas-area">


        {#if store.editorMode === "preview" && previewPdfUrl}
          <div class="flex grow flex-col min-h-0">
            <div class="flex items-center justify-between gap-4 border-b bg-card px-4 py-2">
              <span class="text-sm font-medium">{m.template_preview_sample_data()}</span>
              <Button variant="outline" size="sm" onclick={closePreview}>
                {m.common_td_back_to_editor()}
              </Button>
            </div>
            <iframe src={previewPdfUrl} title={m.shared_ui_pdf_preview()} class="w-full grow border-none"></iframe>
          </div>
        {:else}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="canvas-workspace" oncontextmenu={handleContextMenu}>
            <CanvasRoot {store} onresize={updateCanvasSize} oncursor={(position) => { cursorMm = position; }} />
          </div>
        {/if}

      </div>
      {#if store.editorMode === "design" && isPropertiesOpen}
        <PropertiesInspector
          {store}
          bind:width={propertiesWidth}
          onclose={() => isPropertiesOpen = false}
        />
      {/if}
    </div>
    {#if store.editorMode === "design"}
      <StatusBar {store} containerWidth={canvasWidth} containerHeight={canvasHeight} {cursorMm} />
    {/if}
  {/if}

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
    background: var(--color-muted);
    overflow: hidden;
  }

  .editor-main {
    flex-grow: 1;
    display: flex;
    overflow: hidden;
    position: relative;
    min-width: 0;
    min-height: 0;
    background: var(--color-muted);
  }

  .canvas-workspace {
    flex: 1;
    min-height: 0;
    display: flex;
  }

  .canvas-area {
    flex: 1 1 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    min-width: 0;
    background: var(--color-muted);
  }
</style>
