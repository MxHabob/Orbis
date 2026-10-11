<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import type { TemplateDto, UpdateTemplateRequest } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import {
  createDocument,
  deserializeDocument,
  serializeDocument,
  type TemplateDocument,
} from "@docforge/core";
import { createEditorStore } from "./stores/editor-store.svelte";
import {
  copyNodes,
  cutNodes,
  pasteNodes,
} from "./stores/clipboard-manager";
import CanvasRoot from "./canvas/canvas-root.svelte";
import CanvasContextMenu from "./canvas/canvas-context-menu.svelte";
import * as m from "$lib/paraglide/messages";
import SideRail from "./layout/side-rail.svelte";
import FlyoutPanel from "./layout/flyout-panel.svelte";
import ContextToolbar from "./layout/context-toolbar.svelte";
import StatusBar from "./layout/status-bar.svelte";
import { TemplateEditorSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import PropertiesInspector from "./panels/properties-inspector.svelte";

let { orgId, templateId }: { orgId: string; templateId: string } = $props();

// State
let templateRecord = $state<TemplateDto | null>(null);
let isSaving = $state(false);
let isPreviewLoading = $state(false);
let previewPdfUrl = $state<string | null>(null);
let contextMenuVisible = $state(false);
let contextMenuX = $state(0);
let contextMenuY = $state(0);

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

let isLoading = $state(true);
let error = $state<string | null>(null);

// Initialize
async function initEditor() {
  isLoading = true;
  error = null;
  try {
    const data = await api.getTemplate(templateId || "");
    templateRecord = data;

    const rawJson = data.document_json ?? (data as any).documentJson;
    let doc: TemplateDocument;

    if (rawJson && typeof rawJson === "string" && rawJson.trim().startsWith("{")) {
      try {
        doc = deserializeDocument(rawJson);
      } catch (deserErr) {
        console.warn("Failed to deserialize document JSON, generating default:", deserErr);
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
      doc.meta.pageSetup.size = (data.page_size ?? (data as any).pageSize) as any;
    }
    if (data.orientation) {
      doc.meta.pageSetup.orientation = data.orientation as any;
    }

    // System schemas for data variables and auto-completion
    store.setSystemSchemas({
      student: {
        type: "object",
        label: "بيانات الطالب",
        properties: {
          name: { type: "string", label: "اسم الطالب الرباعي" },
          student_number: { type: "string", label: "الرقم الأكاديمي" },
          national_id: { type: "string", label: "رقم الهوية / الإقامة" },
          seat_number: { type: "string", label: "رقم الجلوس" },
          class_name: { type: "string", label: "الصف والشعبة" },
          grade_level: { type: "string", label: "المرحلة الدراسية" },
          academic_year: { type: "string", label: "العام الدراسي" },
          birth_date: { type: "string", label: "تاريخ الميلاد" },
          birth_place: { type: "string", label: "مكان الميلاد" },
          nationality: { type: "string", label: "الجنسية" },
          guardian_name: { type: "string", label: "اسم ولي الأمر" },
        },
      },
      academic: {
        type: "object",
        label: "البيانات الأكاديمية والدرجات",
        properties: {
          subjects: {
            type: "array",
            label: "جدول المواد والدرجات",
            items: { type: "object" },
          },
          annual_total: { type: "number", label: "المجموع السنوي العام" },
          max_score: { type: "number", label: "النهاية الكبرى العامة" },
          min_score: { type: "number", label: "النهاية الصغرى العامة" },
          percentage: { type: "string", label: "المعدل التراكمي / النسبة المئوية" },
          appreciation: { type: "string", label: "التقدير العام" },
          status: { type: "string", label: "النتيجة النهائية" },
          rank_in_class: { type: "string", label: "الترتيب على الصف" },
        },
      },
      summary: {
        type: "object",
        label: "ملخص النتيجة والإحصائيات",
        properties: {
          total_score: { type: "number", label: "مجموع الدرجات المحصلة" },
          percentage: { type: "string", label: "النسبة المئوية العامة" },
          status: { type: "string", label: "الحالة والقرار الأكاديمي" },
          appreciation: { type: "string", label: "التقدير التراكمي" },
          rank_in_class: { type: "string", label: "الترتيب الأكاديمي" },
        },
      },
      report: {
        type: "object",
        label: "بيانات التقرير والاعتماد",
        properties: {
          period_name: { type: "string", label: "فترة التقرير / الفصل الدراسي" },
          academic_year: { type: "string", label: "العام الدراسي" },
          issued_date: { type: "string", label: "تاريخ الإصدار والطباعة" },
          principal_name: { type: "string", label: "مدير المدرسة" },
          class_teacher: { type: "string", label: "مربي الصف" },
          verification_url: { type: "string", label: "رابط التحقق الرقمي QR" },
        },
      },
      $org: {
        type: "object",
        label: "بيانات المنشأة والوزارة",
        properties: {
          country_name: { type: "string", label: "اسم الدولة" },
          ministry_name: { type: "string", label: "اسم الوزارة" },
          region_name: { type: "string", label: "المكتب / المحافظة / المديرية" },
          name_ar: { type: "string", label: "اسم المنشأة التعليمية (عربي)" },
          name: { type: "string", label: "اسم المنشأة التعليمية (English)" },
          logo_url: { type: "string", label: "شعار المنشأة" },
          emblem_url: { type: "string", label: "شعار الوزارة أو الجمهورية" },
        },
      },
    });

    store.loadDocument(doc);
    isLoading = false;
  } catch (e: unknown) {
    const msg = (e as any)?.message || m.shared_ui_template_editor_initialize_failed();
    toast.error(msg);
    error = msg;
    isLoading = false;
  }
}

// Rename
async function renameTemplate(newName: string) {
  if (!templateRecord || !newName || newName.trim() === "") return;
  const currentName = templateRecord.name;
  const currentNameAr = templateRecord.name ?? templateRecord.name_ar;

  // Optimistic UI update
  templateRecord.name = newName;
  templateRecord.name = newName;

  try {
    const docJson = serializeDocument(store.document);
    const setup = store.document.meta.pageSetup;

    const updateRequest: UpdateTemplateRequest = {
      name: newName,
      name_ar: newName,
      page_size: setup.size,
      orientation: setup.orientation,
      margins_json: JSON.stringify(setup.margins),
      document_json: docJson,
    };

    await api.updateTemplate(templateId || "", updateRequest);

    if (store.isDirty) {
      store.markSaved();
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || m.shared_ui_template_rename_failed();
    toast.error(msg);
    // revert
    templateRecord.name = currentName;
    templateRecord.name_ar = currentNameAr;
  }
}

// Save
async function saveTemplate() {
  if (!templateRecord || !store.isDirty || isSaving) return;
  isSaving = true;
  try {
    const docJson = serializeDocument(store.document);
    const setup = store.document.meta.pageSetup;

    const updateRequest: UpdateTemplateRequest = {
      name: templateRecord.name,
      name_ar: templateRecord.name_ar,
      description: templateRecord.description,
      description_ar: templateRecord.description_ar,
      page_size: setup.size,
      orientation: setup.orientation,
      margins_json: JSON.stringify(setup.margins),
      document_json: docJson,
    };

    await api.updateTemplate(templateId || "", updateRequest);

    store.markSaved();
    toast.success(
      m.ui_copy_template_saved_successfully_299be978(),
    );
  } catch (e: unknown) {
    const msg = (e as any)?.message || m.shared_ui_template_save_failed();
    toast.error(msg);
  } finally {
    isSaving = false;
  }
}

// Preview
async function generatePreview() {
  isPreviewLoading = true;
  try {
    const docJson = serializeDocument(store.document);
    const pdfBytes = await api.previewTemplate(
      docJson,
      {},
      templateRecord?.locale ?? "ar",
    );
    const blob = new Blob([pdfBytes as unknown as BlobPart], {
      type: "application/pdf",
    });
    if (previewPdfUrl) URL.revokeObjectURL(previewPdfUrl);
    previewPdfUrl = URL.createObjectURL(blob);
    toast.success(m.shared_ui_template_preview_generated());
  } catch (e: unknown) {
    const msg = (e as any)?.message || m.shared_ui_template_preview_failed();
    toast.error(msg);
  } finally {
    isPreviewLoading = false;
  }
}

// Keyboard Shortcuts
function handleKeydown(e: KeyboardEvent) {
  const isCtrl = e.ctrlKey || e.metaKey;
  const isInputFocused =
    e.target instanceof HTMLInputElement ||
    e.target instanceof HTMLTextAreaElement;

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
  } else if (isCtrl && e.code === "KeyD") {
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

function goBack() {
  goto(`/${orgId}/templates`);
}

onMount(() => {
  store.setAutoSaveCallback(() => {
    void saveTemplate();
  });
  void initEditor();
  window.addEventListener("keydown", handleKeydown);
  return () => {
    window.removeEventListener("keydown", handleKeydown);
    store.destroy();
  };
});
</script>

<div class="editor-root" role="application" oncontextmenu={handleContextMenu}>
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
    <div class="editor-main">
      <SideRail {store} />

      <div class="canvas-area">
        <ContextToolbar
          {store}
          templateName={templateRecord ? l(templateRecord.name_ar, templateRecord.name) : "..."}
          onback={goBack}
          onsave={saveTemplate}
          onpreview={generatePreview}
          onrename={renameTemplate}
        />

        {#if store.editorMode === "preview" && previewPdfUrl}
          <div class="grow relative">
            <iframe src={previewPdfUrl} title={m.shared_ui_pdf_preview()} class="w-full h-full border-none"></iframe>
            <div class="absolute top-2 inset-e-2">
              <Button variant="outline" size="xs" onclick={() => { store.editorMode = "design"; }} class="cursor-pointer">
                {m.common_td_back_to_editor()}
              </Button>
            </div>
          </div>
        {:else}
          <CanvasRoot {store} />
        {/if}

        <FlyoutPanel {store} />
      </div>

      {#if store.editorMode === "design"}
        <PropertiesInspector {store} />
      {/if}
    </div>
    <StatusBar {store} />
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
