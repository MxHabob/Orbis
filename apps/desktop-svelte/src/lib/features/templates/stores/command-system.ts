import * as m from "$lib/paraglide/messages";
// ═══════════════════════════════════════════════════════════════
// Template Designer — Command System
// Undoable command pattern for all editor operations.
// Every mutation = a command object with execute() and undo().
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument } from "@docforge/core";
import type { TemplateNode, PageNode, NodeType } from "@docforge/core";
import { getPages, isContainerNode } from "@docforge/core";
import {
  findNode,
  addNode,
  removeNode,
  updateNodeProps,
  moveNode,
  cloneNode,
} from "@docforge/core";

// ── Command Interface ────────────────────────────────────────

export interface EditorCommand {
  /** Command type identifier for debugging */
  readonly type: string;
  /** Human-readable description (for undo history panel) */
  readonly description: string;
  /** Arabic description */
  readonly descriptionAr: string;
  /** Apply the command to the document, returning a new document */
  execute(doc: TemplateDocument): TemplateDocument;
  /** Reverse the command, returning the document before execution */
  undo(doc: TemplateDocument): TemplateDocument;
  /** Whether this command can merge with the previous command of the same type */
  canMerge?(prev: EditorCommand): boolean;
  /** Merge this command with the previous one */
  merge?(prev: EditorCommand): EditorCommand;
}

// ── Concrete Commands ────────────────────────────────────────

/** Update properties on a single node */
export class UpdatePropsCommand implements EditorCommand {
  readonly type = "update-props";
  readonly description: string;
  readonly descriptionAr: string;
  private prevProps: Record<string, unknown> | null = null;

  constructor(
    private nodeId: string,
    private props: Record<string, unknown>,
    nodeName?: string,
  ) {
    this.description = `Update ${nodeName || nodeId}`;
    this.descriptionAr = `تحديث ${nodeName || nodeId}`;
  }

  execute(doc: TemplateDocument): TemplateDocument {
    // Save previous props for undo
    const node = findNode(doc, this.nodeId);
    if (node && "props" in node && node.props) {
      this.prevProps = {};
      for (const key of Object.keys(this.props)) {
        this.prevProps[key] = (node.props as Record<string, unknown>)[key];
      }
    }
    return updateNodeProps(doc, this.nodeId, this.props);
  }

  undo(doc: TemplateDocument): TemplateDocument {
    if (!this.prevProps) return doc;
    return updateNodeProps(doc, this.nodeId, this.prevProps);
  }

  canMerge(prev: EditorCommand): boolean {
    return prev instanceof UpdatePropsCommand && prev.nodeId === this.nodeId;
  }

  merge(prev: EditorCommand): EditorCommand {
    if (prev instanceof UpdatePropsCommand) {
      return new UpdatePropsCommand(
        this.nodeId,
        { ...prev.props, ...this.props },
        undefined,
      );
    }
    return this;
  }
}

/** Move a node to a new position (x, y in mm) */
export class MoveNodeCommand implements EditorCommand {
  readonly type = "move-node";
  readonly description: string;
  readonly descriptionAr: string;
  private prevX = 0;
  private prevY = 0;

  constructor(
    private nodeId: string,
    private newX: number,
    private newY: number,
  ) {
    this.description = `Move element`;
    this.descriptionAr = m.template_auto_c2f2cd44();
  }

  execute(doc: TemplateDocument): TemplateDocument {
    const node = findNode(doc, this.nodeId);
    if (node && "position" in node && (node as any).position) {
      this.prevX = (node as any).position.x;
      this.prevY = (node as any).position.y;
    }
    return updateNodeProps(doc, this.nodeId, {
      position: {
        ...((findNode(doc, this.nodeId) as any)?.position || {}),
        x: this.newX,
        y: this.newY,
      },
    } as any);
  }

  undo(doc: TemplateDocument): TemplateDocument {
    return updateNodeProps(doc, this.nodeId, {
      position: {
        ...((findNode(doc, this.nodeId) as any)?.position || {}),
        x: this.prevX,
        y: this.prevY,
      },
    } as any);
  }

  canMerge(prev: EditorCommand): boolean {
    return prev instanceof MoveNodeCommand && prev.nodeId === this.nodeId;
  }

  merge(_prev: EditorCommand): EditorCommand {
    // Keep the latest position but use the original prev position for undo
    return this;
  }
}

/** Resize a node (width, height in mm) */
export class ResizeNodeCommand implements EditorCommand {
  readonly type = "resize-node";
  readonly description = "Resize element";
  readonly descriptionAr = m.template_cmd_resize_element();
  private prevWidth: number | string = 0;
  private prevHeight: number | string | undefined = 0;

  constructor(
    private nodeId: string,
    private newWidth: number | string,
    private newHeight?: number | string,
  ) {}

  execute(doc: TemplateDocument): TemplateDocument {
    const node = findNode(doc, this.nodeId);
    if (node && "position" in node && (node as any).position) {
      this.prevWidth = (node as any).position.width;
      this.prevHeight = (node as any).position.height;
    }
    const pos = { ...((findNode(doc, this.nodeId) as any)?.position || {}) };
    pos.width = this.newWidth;
    if (this.newHeight !== undefined) pos.height = this.newHeight;
    return updateNodeProps(doc, this.nodeId, { position: pos } as any);
  }

  undo(doc: TemplateDocument): TemplateDocument {
    const pos = { ...((findNode(doc, this.nodeId) as any)?.position || {}) };
    pos.width = this.prevWidth;
    pos.height = this.prevHeight;
    return updateNodeProps(doc, this.nodeId, { position: pos } as any);
  }
}

/** Add a new node as a child of a parent */
export class AddNodeCommand implements EditorCommand {
  readonly type = "add-node";
  readonly description: string;
  readonly descriptionAr: string;

  constructor(
    private parentId: string,
    private node: TemplateNode,
    private index?: number,
  ) {
    this.description = `Add ${node.type}`;
    this.descriptionAr = `إضافة ${node.type}`;
  }

  execute(doc: TemplateDocument): TemplateDocument {
    return addNode(doc, this.parentId, this.node, this.index);
  }

  undo(doc: TemplateDocument): TemplateDocument {
    return removeNode(doc, this.node.id);
  }
}

/** Remove a node from the tree */
export class RemoveNodeCommand implements EditorCommand {
  readonly type = "remove-node";
  readonly description: string;
  readonly descriptionAr: string;
  private removedNode: TemplateNode | null = null;
  private parentId: string | null = null;
  private index = 0;

  constructor(private nodeId: string) {
    this.description = `Remove element`;
    this.descriptionAr = m.template_auto_7e2b43ff();
  }

  execute(doc: TemplateDocument): TemplateDocument {
    // Save the node and its parent for undo
    this.removedNode = findNode(doc, this.nodeId);
    // Find parent
    const parentInfo = findParentOfNode(doc, this.nodeId);
    if (parentInfo) {
      this.parentId = parentInfo.parentId;
      this.index = parentInfo.index;
    }
    return removeNode(doc, this.nodeId);
  }

  undo(doc: TemplateDocument): TemplateDocument {
    if (!this.removedNode || !this.parentId) return doc;
    return addNode(doc, this.parentId, this.removedNode, this.index);
  }
}

/** Duplicate a node */
export class DuplicateNodeCommand implements EditorCommand {
  readonly type = "duplicate-node";
  readonly description = "Duplicate element";
  readonly descriptionAr = m.template_cmd_duplicate_element();
  private clonedNode: TemplateNode | null = null;

  constructor(private nodeId: string) {}

  execute(doc: TemplateDocument): TemplateDocument {
    const node = findNode(doc, this.nodeId);
    if (!node) return doc;
    this.clonedNode = cloneNode(node);
    // Offset the clone slightly
    if ("position" in this.clonedNode && (this.clonedNode as any).position) {
      (this.clonedNode as any).position.x += 5;
      (this.clonedNode as any).position.y += 5;
    }
    const parentInfo = findParentOfNode(doc, this.nodeId);
    if (!parentInfo) return doc;
    return addNode(
      doc,
      parentInfo.parentId,
      this.clonedNode,
      parentInfo.index + 1,
    );
  }

  undo(doc: TemplateDocument): TemplateDocument {
    if (!this.clonedNode) return doc;
    return removeNode(doc, this.clonedNode.id);
  }
}

/** Add a new page */
export class AddPageCommand implements EditorCommand {
  readonly type = "add-page";
  readonly description = "Add page";
  readonly descriptionAr = m.template_cmd_add_page();
  private pageId: string;

  constructor(
    private page: PageNode,
    private index?: number,
  ) {
    this.pageId = page.id;
  }

  execute(doc: TemplateDocument): TemplateDocument {
    const pages = [...getPages(doc)];
    const idx = this.index ?? pages.length;
    pages.splice(idx, 0, this.page);
    return { ...doc, pages };
  }

  undo(doc: TemplateDocument): TemplateDocument {
    return {
      ...doc,
      pages: getPages(doc).filter((p) => p.id !== this.pageId),
    };
  }
}

/** Remove a page */
export class RemovePageCommand implements EditorCommand {
  readonly type = "remove-page";
  readonly description = "Remove page";
  readonly descriptionAr = m.template_cmd_remove_page();
  private removedPage: PageNode | null = null;
  private removedIndex = 0;

  constructor(private pageIndex: number) {}

  execute(doc: TemplateDocument): TemplateDocument {
    const pages = getPages(doc);
    if (pages.length <= 1) return doc;
    this.removedPage = pages[this.pageIndex] ?? null;
    this.removedIndex = this.pageIndex;
    return {
      ...doc,
      pages: pages.filter((_, i) => i !== this.pageIndex),
    };
  }

  undo(doc: TemplateDocument): TemplateDocument {
    if (!this.removedPage) return doc;
    const pages = [...getPages(doc)];
    pages.splice(this.removedIndex, 0, this.removedPage);
    return { ...doc, pages };
  }
}

/** Batch command: execute multiple commands as one undo step */
export class BatchCommand implements EditorCommand {
  readonly type = "batch";
  readonly description: string;
  readonly descriptionAr: string;

  constructor(
    private commands: EditorCommand[],
    description = "Multiple changes",
    descriptionAr = m.template_cmd_multiple_changes(),
  ) {
    this.description = description;
    this.descriptionAr = descriptionAr;
  }

  execute(doc: TemplateDocument): TemplateDocument {
    let current = doc;
    for (const cmd of this.commands) {
      current = cmd.execute(current);
    }
    return current;
  }

  undo(doc: TemplateDocument): TemplateDocument {
    let current = doc;
    for (let i = this.commands.length - 1; i >= 0; i--) {
      current = this.commands[i]!.undo(current);
    }
    return current;
  }
}

// ── Helpers ──────────────────────────────────────────────────

function findParentOfNode(
  doc: TemplateDocument,
  nodeId: string,
): { parentId: string; index: number } | null {
  for (const page of getPages(doc)) {
    const result = findParentInTree(page, nodeId);
    if (result) return result;
  }
  return null;
}

function findParentInTree(
  node: TemplateNode,
  targetId: string,
): { parentId: string; index: number } | null {
  if (!isContainerNode(node)) return null;
  for (let i = 0; i < node.children.length; i++) {
    if (node.children[i]!.id === targetId) {
      return { parentId: node.id, index: i };
    }
    const found = findParentInTree(node.children[i]!, targetId);
    if (found) return found;
  }
  return null;
}
