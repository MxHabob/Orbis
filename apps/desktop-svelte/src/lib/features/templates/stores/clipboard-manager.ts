// ═══════════════════════════════════════════════════════════════
// Template Designer — Clipboard Manager
// Copy, cut, paste for template nodes.
// Uses JSON serialization to the system clipboard.
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument, TemplateNode } from "@docforge/core";
import { findNode, addNode, removeNode, cloneNode } from "@docforge/core";
import { getPages } from "@docforge/core";
import type { EditorCommand } from "./command-system";
import {
  AddNodeCommand,
  RemoveNodeCommand,
  BatchCommand,
} from "./command-system";

const CLIPBOARD_MIME = "application/x-docforge-nodes";

export interface ClipboardEntry {
  nodes: TemplateNode[];
  sourceDocId?: string;
  timestamp: number;
}

/**
 * Copy selected nodes to the system clipboard.
 */
export async function copyNodes(
  doc: TemplateDocument,
  nodeIds: string[],
): Promise<void> {
  const nodes: TemplateNode[] = [];
  for (const id of nodeIds) {
    const node = findNode(doc, id);
    if (node) nodes.push(node);
  }

  if (nodes.length === 0) return;

  const entry: ClipboardEntry = {
    nodes,
    sourceDocId: doc.meta.id,
    timestamp: Date.now(),
  };

  try {
    await navigator.clipboard.writeText(JSON.stringify(entry));
  } catch {
    // Fallback: store in session
    sessionStorage.setItem(CLIPBOARD_MIME, JSON.stringify(entry));
  }
}

/**
 * Cut selected nodes (copy + prepare remove command).
 * Returns the command to execute for removing the cut nodes.
 */
export async function cutNodes(
  doc: TemplateDocument,
  nodeIds: string[],
): Promise<EditorCommand | null> {
  await copyNodes(doc, nodeIds);

  if (nodeIds.length === 0) return null;

  if (nodeIds.length === 1) {
    return new RemoveNodeCommand(nodeIds[0]!);
  }

  return new BatchCommand(
    nodeIds.map((id) => new RemoveNodeCommand(id)),
    "Cut elements",
    "قص العناصر",
  );
}

/**
 * Paste nodes from the clipboard into the specified parent.
 * Returns the command to execute, or null if clipboard is empty.
 */
export async function pasteNodes(
  parentId: string,
): Promise<EditorCommand | null> {
  let entry: ClipboardEntry | null = null;

  try {
    const text = await navigator.clipboard.readText();
    entry = JSON.parse(text) as ClipboardEntry;
  } catch {
    // Fallback: read from session storage
    const stored = sessionStorage.getItem(CLIPBOARD_MIME);
    if (stored) entry = JSON.parse(stored) as ClipboardEntry;
  }

  if (!entry || !entry.nodes || entry.nodes.length === 0) return null;

  // Clone all nodes to get fresh IDs
  const clonedNodes = entry.nodes.map((node) => {
    const cloned = cloneNode(node);
    // Offset position slightly to indicate paste
    if ("position" in cloned && (cloned as any).position) {
      (cloned as any).position.x += 3;
      (cloned as any).position.y += 3;
    }
    return cloned;
  });

  if (clonedNodes.length === 1) {
    return new AddNodeCommand(parentId, clonedNodes[0]!);
  }

  return new BatchCommand(
    clonedNodes.map((node) => new AddNodeCommand(parentId, node)),
    "Paste elements",
    "لصق العناصر",
  );
}

/**
 * Check if clipboard has docforge nodes.
 */
export async function hasClipboardNodes(): Promise<boolean> {
  try {
    const text = await navigator.clipboard.readText();
    const parsed = JSON.parse(text);
    return parsed && Array.isArray(parsed.nodes);
  } catch {
    const stored = sessionStorage.getItem(CLIPBOARD_MIME);
    if (stored) {
      try {
        const parsed = JSON.parse(stored);
        return parsed && Array.isArray(parsed.nodes);
      } catch {
        return false;
      }
    }
    return false;
  }
}
