// ═══════════════════════════════════════════════════════════════
// Template Designer — Editor Store (Svelte 5 Runes)
// Central reactive state for the template designer.
// Wraps @docforge/core EditorState with Svelte 5 reactivity.
// ═══════════════════════════════════════════════════════════════

import type {
  TemplateDocument,
  TemplateNode,
  PageNode,
  PageSetup,
} from "@docforge/core";
import {
  getPages,
  getPrimaryPage,
} from "@docforge/core";
import {
  findNode,
  addNode,
  removeNode,
  cloneNode,
} from "@docforge/core";
import type { ViewportTransform, BoundingBox, NodeLayoutInfo } from "@docforge/core";
import { createDefaultViewport, clampZoom, calcZoomToFit } from "@docforge/core";
import type { EditorCommand } from "./command-system";
import {
  UpdatePropsCommand,
  MoveNodeCommand,
  ResizeNodeCommand,
  AddNodeCommand,
  RemoveNodeCommand,
  DuplicateNodeCommand,
  AddPageCommand,
  RemovePageCommand,
  BatchCommand,
} from "./command-system";
import { generateNodeId } from "@docforge/core";

// ── Types ────────────────────────────────────────────────────

export type EditorTool =
  | "select"
  | "text"
  | "image"
  | "shape"
  | "frame"
  | "table"
  | "barcode"
  | "qrcode"
  | "separator"
  | "pan";

export type EditorMode = "design" | "preview" | "data";

// ── Store Factory ────────────────────────────────────────────

export function createEditorStore(initialDoc: TemplateDocument) {
  // ── Core Document State ──────────────────────────────────
  let document = $state<TemplateDocument>(initialDoc);
  let savedSnapshot = $state<string>(JSON.stringify(initialDoc));

  // ── Selection ────────────────────────────────────────────
  let selectedIds = $state<string[]>([]);
  let activePageIndex = $state(0);

  // ── Viewport ─────────────────────────────────────────────
  let viewport = $state<ViewportTransform>(createDefaultViewport());

  // ── UI State ─────────────────────────────────────────────
  let activeTool = $state<EditorTool>("select");
  let editorMode = $state<EditorMode>("design");
  let gridVisible = $state(true);
  let gridSize = $state(5); // mm
  let snapEnabled = $state(true);
  let rulersVisible = $state(true);
  let guidesVisible = $state(true);

  // ── Layout UI (Canva-style) ──────────────────────────────
  type RailPanel = "templates" | "elements" | "text" | "uploads" | "tools" | "page-setup" | null;
  let activeRailPanel = $state<RailPanel>(null);
  let flyoutWidth = $state(320);

  // ── Layout Cache ─────────────────────────────────────────
  let nodeLayouts = $state<Map<string, NodeLayoutInfo>>(new Map());

  // ── Undo/Redo ────────────────────────────────────────────
  let undoStack = $state<{ cmd: EditorCommand; docBefore: string }[]>([]);
  let redoStack = $state<{ cmd: EditorCommand; docAfter: string }[]>([]);
  const MAX_UNDO = 100;

  // Flattened history view for the HistoryPanel
  // All executed commands in order (including those ahead of current position if we've undone)
  const commandHistory = $derived([
    ...undoStack.map((e) => ({
      description: e.cmd.description,
      descriptionAr: e.cmd.descriptionAr,
      timestamp: (e.cmd as any).timestamp,
    })),
    ...redoStack.slice().reverse().map((e) => ({
      description: e.cmd.description,
      descriptionAr: e.cmd.descriptionAr,
      timestamp: (e.cmd as any).timestamp,
    })),
  ]);
  const historyIndex = $derived(undoStack.length - 1);

  // ── Auto-save timer ──────────────────────────────────────
  let autoSaveTimer: ReturnType<typeof setTimeout> | null = null;
  let onAutoSave: ((doc: TemplateDocument) => void) | null = null;

  // ── Derived ──────────────────────────────────────────────
  const pages = $derived(getPages(document));
  const activePage = $derived<PageNode | null>(pages[activePageIndex] ?? pages[0] ?? null);
  const isDirty = $derived(JSON.stringify(document) !== savedSnapshot);
  const canUndo = $derived(undoStack.length > 0);
  const canRedo = $derived(redoStack.length > 0);
  const selectedNodes = $derived(
    selectedIds.map((id) => findNode(document, id)).filter(Boolean) as TemplateNode[],
  );
  const selectedNode = $derived<TemplateNode | null>(selectedNodes[0] ?? null);
  const pageCount = $derived(pages.length);
  const zoomPercent = $derived(Math.round(viewport.zoom * 100));

  // ── Command Execution ────────────────────────────────────

  function executeCommand(cmd: EditorCommand): void {
    const docBefore = JSON.stringify(document);

    // Try to merge with the last command
    if (undoStack.length > 0) {
      const last = undoStack[undoStack.length - 1]!;
      if (cmd.canMerge && cmd.canMerge(last.cmd) && cmd.merge) {
        const merged = cmd.merge(last.cmd);
        undoStack[undoStack.length - 1] = {
          cmd: merged,
          docBefore: last.docBefore,
        };
        document = merged.execute(JSON.parse(last.docBefore) as TemplateDocument);
        redoStack = [];
        scheduleAutoSave();
        return;
      }
    }

    undoStack = [...undoStack.slice(-(MAX_UNDO - 1)), { cmd, docBefore }];
    document = cmd.execute(document);
    redoStack = [];
    scheduleAutoSave();
  }

  function undo(): void {
    if (undoStack.length === 0) return;
    const entry = undoStack[undoStack.length - 1]!;
    const docAfter = JSON.stringify(document);
    document = JSON.parse(entry.docBefore) as TemplateDocument;
    undoStack = undoStack.slice(0, -1);
    redoStack = [...redoStack, { cmd: entry.cmd, docAfter }];
  }

  function redo(): void {
    if (redoStack.length === 0) return;
    const entry = redoStack[redoStack.length - 1]!;
    document = JSON.parse(entry.docAfter) as TemplateDocument;
    redoStack = redoStack.slice(0, -1);
    undoStack = [...undoStack, { cmd: entry.cmd, docBefore: JSON.stringify(document) }];
  }

  // ── Selection ────────────────────────────────────────────

  function select(id: string, addToSelection = false): void {
    if (addToSelection) {
      if (selectedIds.includes(id)) {
        selectedIds = selectedIds.filter((sid) => sid !== id);
      } else {
        selectedIds = [...selectedIds, id];
      }
    } else {
      selectedIds = [id];
    }
  }

  function selectAll(): void {
    if (!activePage) return;
    const ids: string[] = [];
    function collect(node: TemplateNode) {
      if (node.type !== "Page") ids.push(node.id);
      if ("children" in node && Array.isArray((node as any).children)) {
        for (const child of (node as any).children) {
          collect(child);
        }
      }
    }
    collect(activePage);
    selectedIds = ids;
  }

  function clearSelection(): void {
    selectedIds = [];
  }

  // ── Page Management ──────────────────────────────────────

  function setActivePage(index: number): void {
    if (index >= 0 && index < pages.length) {
      activePageIndex = index;
      selectedIds = [];
    }
  }

  function addPage(index?: number): void {
    const newPage: PageNode = {
      type: "Page",
      id: generateNodeId("page"),
      children: [],
    };
    executeCommand(new AddPageCommand(newPage, index));
    activePageIndex = index ?? pages.length; // will be updated after execute
  }

  function removePage(index: number): void {
    if (pages.length <= 1) return;
    executeCommand(new RemovePageCommand(index));
    if (activePageIndex >= pages.length - 1) {
      activePageIndex = Math.max(0, pages.length - 2);
    }
    selectedIds = [];
  }

  // ── Node Operations ──────────────────────────────────────

  function addElement(type: string, props: Record<string, unknown> = {}, x?: number, y?: number): void {
    const parentId = activePage?.id;
    if (!parentId) return;

    const newNode = createDefaultNode(type, props);
    // Apply drop position if provided
    if (x !== undefined && y !== undefined && "position" in newNode) {
      (newNode as any).position = {
        ...(newNode as any).position,
        x: Math.round(x * 10) / 10,
        y: Math.round(y * 10) / 10,
      };
    }
    executeCommand(new AddNodeCommand(parentId, newNode));
    selectedIds = [newNode.id];
  }

  function updateSelectedProps(props: Record<string, unknown>): void {
    if (selectedIds.length === 0) return;
    if (selectedIds.length === 1) {
      executeCommand(
        new UpdatePropsCommand(selectedIds[0]!, props, selectedNode?.name),
      );
    } else {
      const cmds = selectedIds.map(
        (id) => new UpdatePropsCommand(id, props),
      );
      executeCommand(new BatchCommand(cmds, "Update multiple", "تحديث متعدد"));
    }
  }

  function moveSelected(x: number, y: number): void {
    if (selectedIds.length !== 1) return;
    executeCommand(new MoveNodeCommand(selectedIds[0]!, x, y));
  }

  function resizeSelected(w: number | string, h?: number | string): void {
    if (selectedIds.length !== 1) return;
    executeCommand(new ResizeNodeCommand(selectedIds[0]!, w, h));
  }

  // No-op for backwards compat (called by SelectionOverlay on drag end)
  function commitLiveMove(): void {
    // Move is already committed via executeCommand
  }

  function deleteSelected(): void {
    if (selectedIds.length === 0) return;
    if (selectedIds.length === 1) {
      executeCommand(new RemoveNodeCommand(selectedIds[0]!));
    } else {
      const cmds = selectedIds.map((id) => new RemoveNodeCommand(id));
      executeCommand(new BatchCommand(cmds, "Delete multiple", "حذف متعدد"));
    }
    selectedIds = [];
  }

  function duplicateSelected(): void {
    if (selectedIds.length !== 1) return;
    executeCommand(new DuplicateNodeCommand(selectedIds[0]!));
  }

  // ── Viewport ─────────────────────────────────────────────

  function setZoom(zoom: number): void {
    viewport = { ...viewport, zoom: clampZoom(zoom) };
  }

  function zoomIn(): void {
    setZoom(viewport.zoom + 0.1);
  }

  function zoomOut(): void {
    setZoom(viewport.zoom - 0.1);
  }

  function zoomToFit(containerWidth: number, containerHeight: number): void {
    if (!activePage) return;
    const setup = document.meta.pageSetup;
    const pageW = getEffectivePageWidth(setup);
    const pageH = getEffectivePageHeight(setup);
    const zoom = calcZoomToFit(pageW, pageH, containerWidth, containerHeight);
    viewport = { ...viewport, zoom, panX: 0, panY: 0 };
  }

  function setPan(x: number, y: number): void {
    viewport = { ...viewport, panX: x, panY: y };
  }

  // ── Save ─────────────────────────────────────────────────

  function markSaved(): void {
    savedSnapshot = JSON.stringify(document);
  }

  function setAutoSaveCallback(cb: (doc: TemplateDocument) => void): void {
    onAutoSave = cb;
  }

  function scheduleAutoSave(): void {
    if (autoSaveTimer) clearTimeout(autoSaveTimer);
    autoSaveTimer = setTimeout(() => {
      if (isDirty && onAutoSave) {
        onAutoSave(document);
      }
    }, 30_000); // 30 seconds
  }

  // ── Layout Cache ─────────────────────────────────────────

  function updateNodeLayout(nodeId: string, layout: NodeLayoutInfo): void {
    const newMap = new Map(nodeLayouts);
    newMap.set(nodeId, layout);
    nodeLayouts = newMap;
  }

  function getNodeBounds(nodeId: string): BoundingBox | undefined {
    return nodeLayouts.get(nodeId)?.bounds;
  }

  // ── Cleanup ──────────────────────────────────────────────

  function destroy(): void {
    if (autoSaveTimer) clearTimeout(autoSaveTimer);
  }

  // ── Public API ───────────────────────────────────────────

  return {
    // State (readonly reactive)
    get document() { return document; },
    get pages() { return pages; },
    get activePage() { return activePage; },
    get activePageIndex() { return activePageIndex; },
    get selectedIds() { return selectedIds; },
    get selectedNode() { return selectedNode; },
    get selectedNodes() { return selectedNodes; },
    get isDirty() { return isDirty; },
    get canUndo() { return canUndo; },
    get canRedo() { return canRedo; },
    get viewport() { return viewport; },
    get zoomPercent() { return zoomPercent; },
    get pageCount() { return pageCount; },
    get activeTool() { return activeTool; },
    get editorMode() { return editorMode; },
    get gridVisible() { return gridVisible; },
    get gridSize() { return gridSize; },
    get snapEnabled() { return snapEnabled; },
    get rulersVisible() { return rulersVisible; },
    get guidesVisible() { return guidesVisible; },
    get nodeLayouts() { return nodeLayouts; },
    get undoHistory() { return undoStack.map((e) => e.cmd.description); },
    get commandHistory() { return commandHistory; },
    get historyIndex() { return historyIndex; },
    get activeRailPanel() { return activeRailPanel; },
    get flyoutWidth() { return flyoutWidth; },

    // Setters
    set activeTool(t: EditorTool) { activeTool = t; },
    set editorMode(m: EditorMode) { editorMode = m; },
    set gridVisible(v: boolean) { gridVisible = v; },
    set gridSize(s: number) { gridSize = s; },
    set snapEnabled(v: boolean) { snapEnabled = v; },
    set rulersVisible(v: boolean) { rulersVisible = v; },
    set guidesVisible(v: boolean) { guidesVisible = v; },
    set activeRailPanel(p: RailPanel) { activeRailPanel = p; },
    set flyoutWidth(w: number) { flyoutWidth = Math.max(280, Math.min(480, w)); },
    toggleRailPanel(panel: string) {
      if (activeRailPanel === panel) {
        activeRailPanel = null;
      } else {
        activeRailPanel = panel as RailPanel;
      }
    },

    // Commands
    executeCommand,
    undo,
    redo,
    jumpToHistoryIndex(targetIndex: number) {
      // Jump to a specific point in the unified history
      // targetIndex = -1 means initial state, 0 = after first command, etc.
      const currentIdx = undoStack.length - 1;
      if (targetIndex === currentIdx) return;

      if (targetIndex < currentIdx) {
        // Undo multiple times
        const steps = currentIdx - targetIndex;
        for (let i = 0; i < steps; i++) undo();
      } else {
        // Redo multiple times
        const steps = targetIndex - currentIdx;
        for (let i = 0; i < steps; i++) redo();
      }
    },

    // Selection
    select,
    selectAll,
    clearSelection,

    // Pages
    setActivePage,
    addPage,
    removePage,

    // Nodes
    addElement,
    updateSelectedProps,
    moveSelected,
    resizeSelected,
    deleteSelected,
    duplicateSelected,
    commitLiveMove,

    // Layer ordering
    bringToFront() {
      if (selectedIds.length !== 1 || !activePage) return;
      const page = activePage;
      const idx = page.children.findIndex((c) => c.id === selectedIds[0]);
      if (idx < 0 || idx === page.children.length - 1) return;
      const node = page.children[idx]!;
      const newChildren = [...page.children];
      newChildren.splice(idx, 1);
      newChildren.push(node);
      executeCommand({
        type: "reorder",
        description: `Bring ${node.name || node.id} to front`,
        descriptionAr: `إحضار ${node.name || node.id} للأمام`,
        execute: (doc) => {
          const p = getPages(doc).find((pg) => pg.id === page.id);
          if (p) (p as any).children = newChildren;
          return { ...doc };
        },
        undo: (doc) => {
          const p = getPages(doc).find((pg) => pg.id === page.id);
          if (p) (p as any).children = [...page.children];
          return { ...doc };
        },
      } as EditorCommand);
    },
    sendToBack() {
      if (selectedIds.length !== 1 || !activePage) return;
      const page = activePage;
      const idx = page.children.findIndex((c) => c.id === selectedIds[0]);
      if (idx <= 0) return;
      const node = page.children[idx]!;
      const newChildren = [...page.children];
      newChildren.splice(idx, 1);
      newChildren.unshift(node);
      executeCommand({
        type: "reorder",
        description: `Send ${node.name || node.id} to back`,
        descriptionAr: `إرسال ${node.name || node.id} للخلف`,
        execute: (doc) => {
          const p = getPages(doc).find((pg) => pg.id === page.id);
          if (p) (p as any).children = newChildren;
          return { ...doc };
        },
        undo: (doc) => {
          const p = getPages(doc).find((pg) => pg.id === page.id);
          if (p) (p as any).children = [...page.children];
          return { ...doc };
        },
      } as EditorCommand);
    },

    // Viewport
    setZoom,
    zoomIn,
    zoomOut,
    zoomToFit,
    setPan,

    // Save
    markSaved,
    setAutoSaveCallback,

    // Layout
    updateNodeLayout,
    getNodeBounds,

    // Load
    loadDocument(doc: TemplateDocument) {
      document = doc;
      savedSnapshot = JSON.stringify(doc);
      selectedIds = [];
      activePageIndex = 0;
      undoStack = [];
      redoStack = [];
    },

    // Cleanup
    destroy,
  };
}

// ── Export type ───────────────────────────────────────────────

export type EditorStore = ReturnType<typeof createEditorStore>;

// ── Helpers ──────────────────────────────────────────────────

function getEffectivePageWidth(setup: PageSetup): number {
  if (setup.size === "Custom" && setup.width) return setup.width;
  const sizes: Record<string, number> = {
    A4: 210, A3: 297, A5: 148, Letter: 216, Legal: 216,
    ID_CARD: 85.6, BADGE: 86,
  };
  const w = sizes[setup.size] ?? 210;
  const h = ({ A4: 297, A3: 420, A5: 210, Letter: 279, Legal: 356, ID_CARD: 53.98, BADGE: 54 } as Record<string, number>)[setup.size] ?? 297;
  return setup.orientation === "landscape" ? h : w;
}

function getEffectivePageHeight(setup: PageSetup): number {
  if (setup.size === "Custom" && setup.height) return setup.height;
  const w = getEffectivePageWidth({ ...setup, orientation: "portrait" } as PageSetup);
  const sizes: Record<string, number> = {
    A4: 297, A3: 420, A5: 210, Letter: 279, Legal: 356,
    ID_CARD: 53.98, BADGE: 54,
  };
  const h = sizes[setup.size] ?? 297;
  return setup.orientation === "landscape" ? w : h;
}

function createDefaultNode(type: string, overrides: Record<string, unknown> = {}): TemplateNode {
  const id = generateNodeId(type.toLowerCase());
  const base = { id, name: type, visible: true, locked: false };

  switch (type) {
    case "Text":
      return {
        ...base,
        type: "Text",
        props: {
          content: "نص جديد",
          fontSize: 14,
          fontFamily: "Rubik",
          fontWeight: "400",
          color: "#000000",
          textAlign: "start",
          direction: "rtl",
          ...overrides,
        },
        position: { x: 10, y: 10, width: 80, height: 20 },
        positionMode: "absolute",
      } as any;

    case "Image":
      return {
        ...base,
        type: "Image",
        props: {
          src: "",
          fit: "contain",
          ...overrides,
        },
        position: { x: 10, y: 10, width: 50, height: 50 },
        positionMode: "absolute",
      } as any;

    case "Shape":
      return {
        ...base,
        type: "Shape",
        props: {
          shape: "rectangle",
          fillColor: "#e2e8f0",
          strokeColor: "#94a3b8",
          strokeWidth: 1,
          borderRadius: 2,
          ...overrides,
        },
        position: { x: 10, y: 10, width: 60, height: 40 },
        positionMode: "absolute",
      } as any;

    case "Frame":
      return {
        ...base,
        type: "Frame",
        children: [],
        position: { x: 10, y: 10, width: "100%", height: 50 },
        positionMode: "absolute",
        props: { layout: "flex", direction: "row", gap: 2, ...overrides },
      } as any;

    case "Barcode":
      return {
        ...base,
        type: "Barcode",
        props: {
          data: "{{id}}",
          format: "Code128",
          height: 12,
          showText: true,
          ...overrides,
        },
        position: { x: 10, y: 10, width: 60, height: 20 },
        positionMode: "absolute",
      } as any;

    case "QRCode":
      return {
        ...base,
        type: "QRCode",
        props: {
          data: "{{id}}",
          size: 25,
          errorCorrection: "Medium",
          ...overrides,
        },
        position: { x: 10, y: 10, width: 25, height: 25 },
        positionMode: "absolute",
      } as any;

    case "Separator":
      return {
        ...base,
        type: "Separator",
        props: {
          direction: "horizontal",
          thickness: 0.5,
          color: "#d1d5db",
          ...overrides,
        },
        position: { x: 10, y: 10, width: "100%" },
        positionMode: "absolute",
      } as any;

    case "Table":
      return {
        ...base,
        type: "Table",
        props: {
          columns: [
            { key: "col1", header: "عمود 1", headerAr: "عمود 1", width: 50 },
            { key: "col2", header: "عمود 2", headerAr: "عمود 2", width: 50 },
          ],
          rows: [],
          headerStyle: {},
          cellStyle: {},
          ...overrides,
        },
        position: { x: 10, y: 10, width: "100%", height: 40 },
        positionMode: "absolute",
      } as any;

    default:
      return {
        ...base,
        type: "Frame",
        children: [],
        position: { x: 10, y: 10, width: 60, height: 40 },
        positionMode: "absolute",
        ...overrides,
      } as any;
  }
}
