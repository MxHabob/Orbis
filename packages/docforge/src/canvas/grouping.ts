// ═══════════════════════════════════════════════════════════════
// @docforge/core — Grouping Operations
// Group and ungroup elements in the document tree.
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument } from "../model/document";
import { getPages } from "../model/document";
import type { TemplateNode, FrameNode, PageNode } from "../model/nodes";
import { isContainerNode } from "../model/nodes";
import { generateNodeId } from "../utils/id-generator";
import type { BoundingBox } from "./hit-testing";
import { computeCombinedBounds } from "./hit-testing";

// ── Group Nodes ──────────────────────────────────────────────

/**
 * Group multiple nodes into a new Frame container.
 *
 * @param doc - Current document
 * @param nodeIds - IDs of nodes to group (must share the same parent)
 * @param layouts - Layout info for computing the group bounds
 * @returns New document with nodes wrapped in a Frame
 */
export function groupNodes(
  doc: TemplateDocument,
  nodeIds: string[],
  layouts: Map<string, BoundingBox>,
): TemplateDocument {
  if (nodeIds.length < 2) return doc;

  // Compute the combined bounding box for the group
  const boxes = nodeIds
    .map((id) => layouts.get(id))
    .filter((b): b is BoundingBox => !!b);
  const combinedBounds = computeCombinedBounds(boxes);

  const groupId = generateNodeId("group");

  // Create the group Frame
  const groupFrame: FrameNode = {
    type: "Frame",
    id: groupId,
    name: "Group",
    children: [],
    positionMode: "absolute",
    props: {
      position: {
        x: combinedBounds.x,
        y: combinedBounds.y,
        width: combinedBounds.width,
        height: combinedBounds.height,
      },
      layout: "vertical",
    },
  };

  // Process pages: find the parent containing all nodeIds and replace
  const newPages = getPages(doc).map((page) => {
    const result = groupNodesInParent(
      page,
      nodeIds,
      groupFrame,
      combinedBounds,
      layouts,
    );
    return result as PageNode;
  });

  return { ...doc, pages: newPages };
}

function groupNodesInParent(
  node: TemplateNode,
  nodeIds: string[],
  groupFrame: FrameNode,
  groupBounds: BoundingBox,
  layouts: Map<string, BoundingBox>,
): TemplateNode {
  if (!isContainerNode(node)) return node;

  // Check if this node contains the target children
  const childIds = node.children.map((c) => c.id);
  const matchingIds = nodeIds.filter((id) => childIds.includes(id));

  if (matchingIds.length === nodeIds.length) {
    // This is the parent — extract matching children, wrap in group
    const groupChildren: TemplateNode[] = [];
    const remainingChildren: TemplateNode[] = [];

    for (const child of node.children) {
      if (nodeIds.includes(child.id)) {
        // Offset child position relative to the group
        const childBounds = layouts.get(child.id);
        if (childBounds && hasAbsolutePosition(child)) {
          const offsetChild = {
            ...child,
            position: {
              ...(child as any).position,
              x: childBounds.x - groupBounds.x,
              y: childBounds.y - groupBounds.y,
            },
          };
          groupChildren.push(offsetChild);
        } else {
          groupChildren.push(child);
        }
      } else {
        remainingChildren.push(child);
      }
    }

    const populatedGroup: FrameNode = {
      ...groupFrame,
      children: groupChildren,
    };

    // Insert group at the position of the first matched child
    const firstIdx = node.children.findIndex((c) => nodeIds.includes(c.id));
    remainingChildren.splice(firstIdx, 0, populatedGroup);

    return { ...node, children: remainingChildren } as TemplateNode;
  }

  // Recurse into children
  return {
    ...node,
    children: node.children.map((child) =>
      groupNodesInParent(child, nodeIds, groupFrame, groupBounds, layouts),
    ),
  } as TemplateNode;
}

// ── Ungroup ──────────────────────────────────────────────────

/**
 * Ungroup a Frame node, promoting its children to the parent.
 *
 * @param doc - Current document
 * @param groupId - ID of the Frame to ungroup
 * @param layouts - Layout info for re-positioning children
 * @returns New document with the group dissolved
 */
export function ungroupNode(
  doc: TemplateDocument,
  groupId: string,
  layouts: Map<string, BoundingBox>,
): TemplateDocument {
  const newPages = getPages(doc).map((page) => {
    const result = ungroupInParent(page, groupId, layouts);
    return result as PageNode;
  });

  return { ...doc, pages: newPages };
}

function ungroupInParent(
  node: TemplateNode,
  groupId: string,
  layouts: Map<string, BoundingBox>,
): TemplateNode {
  if (!isContainerNode(node)) return node;

  const groupIndex = node.children.findIndex((c) => c.id === groupId);

  if (groupIndex !== -1) {
    const group = node.children[groupIndex]!;
    if (!isContainerNode(group)) return node;

    const groupBounds = layouts.get(groupId);
    const promotedChildren: TemplateNode[] = group.children.map((child) => {
      // Convert child's relative position to absolute in parent
      if (groupBounds && hasAbsolutePosition(child)) {
        const childPos = (child as any).position;
        return {
          ...child,
          position: {
            ...childPos,
            x: childPos.x + groupBounds.x,
            y: childPos.y + groupBounds.y,
          },
        };
      }
      return child;
    });

    // Replace the group with its children in-place
    const newChildren = [...node.children];
    newChildren.splice(groupIndex, 1, ...promotedChildren);

    return { ...node, children: newChildren } as TemplateNode;
  }

  // Recurse into children
  return {
    ...node,
    children: node.children.map((child) =>
      ungroupInParent(child, groupId, layouts),
    ),
  } as TemplateNode;
}

// ── Helpers ──────────────────────────────────────────────────

function hasAbsolutePosition(node: TemplateNode): boolean {
  return "position" in node && (node as any).position != null;
}
