// ═══════════════════════════════════════════════════════════════
// @docforge/core — Editor State
// Framework-agnostic state container with undo/redo.
// UI frameworks (Svelte, React) wrap this with reactive bindings.
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument } from "../model/document";
import { getPages } from "../model/document";
import type { PageNode, TemplateNode } from "../model/nodes";
import {
  addNode as addOp,
  findNode,
  moveNode as moveOp,
  removeNode as removeOp,
  updateNodeProps as updateOp,
} from "../operations/node-ops";
import { generateNodeId } from "../utils/id-generator";

// ── History Manager ──────────────────────────────────────────

interface HistoryEntry {
  document: TemplateDocument;
  description: string;
  timestamp: number;
}

const MAX_HISTORY = 100;

// ── Change Listener Type ─────────────────────────────────────

type ChangeListener = (doc: TemplateDocument) => void;
type SelectionListener = (ids: string[]) => void;

// ── Editor State Class ───────────────────────────────────────

/**
 * Framework-agnostic editor state container.
 * Manages the document, selection, and undo/redo history.
 *
 * UI frameworks should wrap this with reactive bindings:
 * - Svelte 5: wrap with $state() runes
 * - React: wrap with useState/useReducer
 * - Vue: wrap with reactive()
 */
export class EditorState {
  private _document: TemplateDocument;
  private _selectedIds: string[] = [];
  private _activePageIndex = 0;
  private _undoStack: HistoryEntry[] = [];
  private _redoStack: HistoryEntry[] = [];
  private _isDirty = false;
  private _savedDocument: string; // serialized snapshot of last save

  private _changeListeners: ChangeListener[] = [];
  private _selectionListeners: SelectionListener[] = [];

  constructor(doc: TemplateDocument) {
    this._document = doc;
    this._savedDocument = JSON.stringify(doc);
  }

  // ── Getters ──────────────────────────────────────────────

  /** Current document state */
  getDocument(): TemplateDocument {
    return this._document;
  }

  /** Currently selected node IDs */
  getSelectedIds(): string[] {
    return this._selectedIds;
  }

  /** The first selected node, or null */
  getSelectedNode(): TemplateNode | null {
    if (this._selectedIds.length === 0) return null;
    return findNode(this._document, this._selectedIds[0]!);
  }

  /** Currently active page index */
  getActivePageIndex(): number {
    return this._activePageIndex;
  }

  /** Currently active page node */
  getActivePage(): PageNode | null {
    const pages = getPages(this._document);
    return pages[this._activePageIndex] ?? pages[0] ?? null;
  }

  /** Total number of pages */
  getPageCount(): number {
    return getPages(this._document).length;
  }

  /** Whether the document has been modified since last save */
  getIsDirty(): boolean {
    return this._isDirty;
  }

  /** Whether undo is available */
  getCanUndo(): boolean {
    return this._undoStack.length > 0;
  }

  /** Whether redo is available */
  getCanRedo(): boolean {
    return this._redoStack.length > 0;
  }

  /** History information */
  getHistory(): {
    canUndo: boolean;
    canRedo: boolean;
    undoCount: number;
    redoCount: number;
  } {
    return {
      canUndo: this._undoStack.length > 0,
      canRedo: this._redoStack.length > 0,
      undoCount: this._undoStack.length,
      redoCount: this._redoStack.length,
    };
  }

  // ── Selection ────────────────────────────────────────────

  /** Select a node. If multi is true, adds to selection; otherwise replaces. */
  selectNode(id: string, multi = false): void {
    if (multi) {
      const idx = this._selectedIds.indexOf(id);
      if (idx >= 0) {
        this._selectedIds = this._selectedIds.filter((_, i) => i !== idx);
      } else {
        this._selectedIds = [...this._selectedIds, id];
      }
    } else {
      this._selectedIds = [id];
    }
    this._notifySelectionChange();
  }

  /** Clear all selections */
  clearSelection(): void {
    this._selectedIds = [];
    this._notifySelectionChange();
  }

  // ── Document Mutations (with history) ────────────────────

  /** Update properties of a specific node */
  updateNodeProps(nodeId: string, props: Record<string, unknown>): void {
    this._pushHistory("Update node properties");
    this._document = updateOp(this._document, nodeId, props);
    this._markDirty();
  }

  /** Add a new child node to a container */
  addNode(parentId: string, node: TemplateNode, index?: number): void {
    this._pushHistory("Add node");
    this._document = addOp(this._document, parentId, node, index);
    this._selectedIds = [node.id];
    this._markDirty();
    this._notifySelectionChange();
  }

  /** Remove a node from the document */
  removeNode(nodeId: string): void {
    this._pushHistory("Remove node");
    this._document = removeOp(this._document, nodeId);
    this._selectedIds = this._selectedIds.filter((id) => id !== nodeId);
    this._markDirty();
    this._notifySelectionChange();
  }

  /** Delete all currently selected nodes */
  deleteSelected(): void {
    if (this._selectedIds.length === 0) return;
    this._pushHistory("Delete selected");
    for (const id of this._selectedIds) {
      try {
        this._document = removeOp(this._document, id);
      } catch {
        // Skip if node was already removed (e.g., child of a deleted parent)
      }
    }
    this._selectedIds = [];
    this._markDirty();
    this._notifySelectionChange();
  }

  /** Move a node to a new parent */
  moveNode(nodeId: string, newParentId: string, index: number): void {
    this._pushHistory("Move node");
    this._document = moveOp(this._document, nodeId, newParentId, index);
    this._markDirty();
  }

  /** Update document metadata */
  updateMeta(updates: Partial<TemplateDocument["meta"]>): void {
    this._pushHistory("Update metadata");
    this._document = {
      ...this._document,
      meta: { ...this._document.meta, ...updates },
    };
    this._markDirty();
  }

  /** Update a variable definition */
  updateVariable(
    name: string,
    definition: TemplateDocument["variables"][string],
  ): void {
    this._pushHistory("Update variable");
    this._document = {
      ...this._document,
      variables: { ...this._document.variables, [name]: definition },
    };
    this._markDirty();
  }

  /** Remove a variable definition */
  removeVariable(name: string): void {
    this._pushHistory("Remove variable");
    const { [name]: _, ...rest } = this._document.variables;
    this._document = { ...this._document, variables: rest };
    this._markDirty();
  }

  // ── Page Management ──────────────────────────────────────

  /** Set the active page by index */
  setActivePage(index: number): void {
    const pages = getPages(this._document);
    if (index >= 0 && index < pages.length) {
      this._activePageIndex = index;
      this._selectedIds = [];
      this._notifySelectionChange();
      this._notifyChange();
    }
  }

  /** Add a new empty page at the given index (or at the end) */
  addPage(index?: number): void {
    this._pushHistory("Add page");
    const newPage: PageNode = {
      type: "Page",
      id: generateNodeId("page"),
      children: [],
    };
    const pages = [...getPages(this._document)];
    const insertAt = index ?? pages.length;
    pages.splice(insertAt, 0, newPage);
    this._document = { ...this._document, pages };
    this._activePageIndex = insertAt;
    this._markDirty();
  }

  /** Remove a page by index. Cannot remove the last page. */
  removePage(index: number): void {
    const pages = getPages(this._document);
    if (pages.length <= 1) {
      throw new Error("Cannot remove the last page");
    }
    if (index < 0 || index >= pages.length) return;

    this._pushHistory("Remove page");
    const newPages = pages.filter((_, i) => i !== index);
    this._document = { ...this._document, pages: newPages };

    // Adjust active page index if needed
    if (this._activePageIndex >= newPages.length) {
      this._activePageIndex = newPages.length - 1;
    }
    this._selectedIds = [];
    this._markDirty();
    this._notifySelectionChange();
  }

  /** Reorder pages by moving page at `fromIndex` to `toIndex` */
  reorderPage(fromIndex: number, toIndex: number): void {
    const pages = [...getPages(this._document)];
    if (
      fromIndex < 0 ||
      fromIndex >= pages.length ||
      toIndex < 0 ||
      toIndex >= pages.length ||
      fromIndex === toIndex
    )
      return;

    this._pushHistory("Reorder pages");
    const [moved] = pages.splice(fromIndex, 1);
    pages.splice(toIndex, 0, moved!);
    this._document = { ...this._document, pages };
    this._activePageIndex = toIndex;
    this._markDirty();
  }

  // ── Undo / Redo ──────────────────────────────────────────

  /** Undo the last operation */
  undo(): void {
    const entry = this._undoStack.pop();
    if (!entry) return;

    this._redoStack.push({
      document: this._document,
      description: "Redo",
      timestamp: Date.now(),
    });

    this._document = entry.document;
    this._isDirty = JSON.stringify(this._document) !== this._savedDocument;
    this._notifyChange();
  }

  /** Redo the last undone operation */
  redo(): void {
    const entry = this._redoStack.pop();
    if (!entry) return;

    this._undoStack.push({
      document: this._document,
      description: "Undo redo",
      timestamp: Date.now(),
    });

    this._document = entry.document;
    this._isDirty = JSON.stringify(this._document) !== this._savedDocument;
    this._notifyChange();
  }

  // ── Save ─────────────────────────────────────────────────

  /** Mark the current state as saved (resets dirty flag) */
  markSaved(): TemplateDocument {
    this._savedDocument = JSON.stringify(this._document);
    this._isDirty = false;
    this._notifyChange();
    return this._document;
  }

  /** Replace the entire document (e.g., after loading from server) */
  loadDocument(doc: TemplateDocument): void {
    this._document = doc;
    this._savedDocument = JSON.stringify(doc);
    this._selectedIds = [];
    this._activePageIndex = 0;
    this._undoStack = [];
    this._redoStack = [];
    this._isDirty = false;
    this._notifyChange();
    this._notifySelectionChange();
  }

  // ── Observers ────────────────────────────────────────────

  /** Register a listener for document changes */
  onChange(listener: ChangeListener): () => void {
    this._changeListeners.push(listener);
    return () => {
      this._changeListeners = this._changeListeners.filter(
        (l) => l !== listener,
      );
    };
  }

  /** Register a listener for selection changes */
  onSelectionChange(listener: SelectionListener): () => void {
    this._selectionListeners.push(listener);
    return () => {
      this._selectionListeners = this._selectionListeners.filter(
        (l) => l !== listener,
      );
    };
  }

  // ── Internal Helpers ─────────────────────────────────────

  private _pushHistory(description: string): void {
    this._undoStack.push({
      document: this._document,
      description,
      timestamp: Date.now(),
    });

    // Limit history size
    if (this._undoStack.length > MAX_HISTORY) {
      this._undoStack.shift();
    }

    // Clear redo stack on new action
    this._redoStack = [];
  }

  private _markDirty(): void {
    this._isDirty = true;
    this._notifyChange();
  }

  private _notifyChange(): void {
    for (const listener of this._changeListeners) {
      listener(this._document);
    }
  }

  private _notifySelectionChange(): void {
    for (const listener of this._selectionListeners) {
      listener(this._selectedIds);
    }
  }
}
