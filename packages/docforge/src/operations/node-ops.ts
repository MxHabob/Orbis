// ═══════════════════════════════════════════════════════════════
// @docforge/core — Node Operations (v2.0)
// Immutable operations on the document tree.
// Every operation returns a new document — never mutates in place.
//
// v2.0: All operations work across `pages[]` instead of `root`.
// ═══════════════════════════════════════════════════════════════

import type { PageSetup, Spacing, TemplateDocument } from "../model/document";
import { getPages } from "../model/document";
import type { NodeType, PageNode, TemplateNode } from "../model/nodes";
import { isContainerNode } from "../model/nodes";
import { generateNodeId } from "../utils/id-generator";

// ── Node Factory ─────────────────────────────────────────────

/** Default props for each node type */
const NODE_DEFAULTS: Record<string, () => TemplateNode> = {
  Text: () => ({
    type: "Text",
    id: generateNodeId("text"),
    props: {
      content: "",
      fontSize: 14,
      fontFamily: "Rubik",
      fontWeight: "normal",
      color: "#000000",
      textAlign: "start",
      direction: "rtl",
      lineHeight: 1.5,
    },
  }),
  Image: () => ({
    type: "Image",
    id: generateNodeId("image"),
    props: {},
  }),
  Frame: () => ({
    type: "Frame",
    id: generateNodeId("frame"),
    props: { layout: "vertical", gap: 4 },
    children: [],
  }),
  Grid: () => ({
    type: "Grid",
    id: generateNodeId("grid"),
    props: { columns: ["1fr", "1fr"], gap: 4 },
    children: [],
  }),
  Shape: () => ({
    type: "Shape",
    id: generateNodeId("shape"),
    props: {
      shapeType: "rectangle",
      fill: "#e5e7eb",
      stroke: "#6b7280",
      strokeWidth: 1,
    },
  }),
  Table: () => ({
    type: "Table",
    id: generateNodeId("table"),
    props: {
      columns: [
        { header: "Column 1", field: "" },
        { header: "Column 2", field: "" },
      ],
      showHeader: true,
      showBorder: true,
      showGrid: true,
      headerFontWeight: "bold",
      cellFontSize: 12,
      direction: "rtl",
    },
  }),
  Barcode: () => ({
    type: "Barcode",
    id: generateNodeId("barcode"),
    props: {
      data: "",
      format: "code128",
      height: 15,
      showText: true,
      color: "#000000",
    },
  }),
  QRCode: () => ({
    type: "QRCode",
    id: generateNodeId("qrcode"),
    props: {
      data: "",
      size: 25,
      errorCorrection: "M",
      color: "#000000",
      backgroundColor: "#ffffff",
    },
  }),
  Separator: () => ({
    type: "Separator",
    id: generateNodeId("separator"),
    props: {
      direction: "horizontal",
      color: "#d1d5db",
      thickness: 0.5,
      style: "solid",
    },
  }),
  Spacer: () => ({
    type: "Spacer",
    id: generateNodeId("spacer"),
    props: { height: 10 },
  }),
  Repeater: () => ({
    type: "Repeater",
    id: generateNodeId("repeater"),
    props: {
      dataSource: "",
      itemVariable: "item",
      layout: "vertical",
      gap: 2,
    },
    children: [],
  }),
  Conditional: () => ({
    type: "Conditional",
    id: generateNodeId("conditional"),
    props: { condition: "" },
    children: [],
  }),
  ComponentRef: () => ({
    type: "ComponentRef",
    id: generateNodeId("componentref"),
    props: { componentId: "" },
  }),
  PageBreak: () => ({
    type: "PageBreak",
    id: generateNodeId("pagebreak"),
  }),
  Header: () => ({
    type: "Header",
    id: generateNodeId("header"),
    props: { height: 20 },
    children: [],
  }),
  Footer: () => ({
    type: "Footer",
    id: generateNodeId("footer"),
    props: {
      height: 15,
      showPageNumber: true,
      pageNumberFormat: "{{page}}/{{total}}",
    },
    children: [],
  }),
  Watermark: () => ({
    type: "Watermark",
    id: generateNodeId("watermark"),
    props: {
      watermarkType: "text",
      text: "",
      opacity: 0.1,
      rotation: -45,
      fontSize: 48,
      color: "#000000",
    },
  }),
};

/**
 * Create a new node of the given type with sensible defaults.
 * Optionally merge additional props.
 */
export function createNode<T extends TemplateNode>(
  type: NodeType,
  overrides?: { props?: Record<string, unknown>; name?: string },
): T {
  const factory = NODE_DEFAULTS[type];
  if (!factory) {
    throw new Error(`Unknown node type: ${type}`);
  }
  const node = factory() as T;

  if (overrides) {
    if ("props" in overrides && overrides.props && "props" in node) {
      (node as any).props = { ...(node as any).props, ...overrides.props };
    }
    if ("name" in overrides) node.name = overrides.name as string;
  }

  return node;
}

// ── Deep Clone ───────────────────────────────────────────────

/**
 * Deep clone a node, generating new IDs for the clone and all descendants.
 */
export function cloneNode(node: TemplateNode): TemplateNode {
  const cloned = JSON.parse(JSON.stringify(node)) as TemplateNode;
  reassignIds(cloned);
  return cloned;
}

function reassignIds(node: TemplateNode): void {
  node.id = generateNodeId(node.type);
  if (isContainerNode(node)) {
    for (const child of node.children) {
      reassignIds(child);
    }
    if (node.type === "Conditional" && node.elseChildren) {
      for (const child of node.elseChildren) {
        reassignIds(child);
      }
    }
  }
}

// ── Tree Search ──────────────────────────────────────────────

/**
 * Find a node by ID in the document tree. Searches all pages.
 * Returns null if not found.
 */
export function findNode(
  doc: TemplateDocument,
  nodeId: string,
): TemplateNode | null {
  for (const page of getPages(doc)) {
    const found = findNodeInTree(page, nodeId);
    if (found) return found;
  }
  return null;
}

function findNodeInTree(
  node: TemplateNode,
  nodeId: string,
): TemplateNode | null {
  if (node.id === nodeId) return node;
  if (isContainerNode(node)) {
    for (const child of node.children) {
      const found = findNodeInTree(child, nodeId);
      if (found) return found;
    }
    if (node.type === "Conditional" && node.elseChildren) {
      for (const child of node.elseChildren) {
        const found = findNodeInTree(child, nodeId);
        if (found) return found;
      }
    }
  }
  return null;
}

/**
 * Find the path of parent IDs from root to the given node.
 * Returns an array of node IDs, or empty array if not found.
 */
export function findNodePath(doc: TemplateDocument, nodeId: string): string[] {
  for (const page of getPages(doc)) {
    const path: string[] = [];
    if (findNodePathRecursive(page, nodeId, path)) {
      return path;
    }
  }
  return [];
}

function findNodePathRecursive(
  node: TemplateNode,
  nodeId: string,
  path: string[],
): boolean {
  if (node.id === nodeId) {
    path.push(node.id);
    return true;
  }
  if (isContainerNode(node)) {
    path.push(node.id);
    for (const child of node.children) {
      if (findNodePathRecursive(child, nodeId, path)) return true;
    }
    if (node.type === "Conditional" && node.elseChildren) {
      for (const child of node.elseChildren) {
        if (findNodePathRecursive(child, nodeId, path)) return true;
      }
    }
    path.pop();
  }
  return false;
}

// ── Tree Traversal ───────────────────────────────────────────

/**
 * Walk all nodes in the document tree, calling the visitor for each.
 */
export function walkNodes(
  doc: TemplateDocument,
  visitor: (node: TemplateNode, path: string[], depth: number) => void,
): void {
  for (const page of getPages(doc)) {
    walkNodeRecursive(page, visitor, [], 0);
  }
}

function walkNodeRecursive(
  node: TemplateNode,
  visitor: (node: TemplateNode, path: string[], depth: number) => void,
  path: string[],
  depth: number,
): void {
  const currentPath = [...path, node.id];
  visitor(node, currentPath, depth);

  if (isContainerNode(node)) {
    for (const child of node.children) {
      walkNodeRecursive(child, visitor, currentPath, depth + 1);
    }
    if (node.type === "Conditional" && node.elseChildren) {
      for (const child of node.elseChildren) {
        walkNodeRecursive(child, visitor, currentPath, depth + 1);
      }
    }
  }
}

// ── Immutable Document Updates ───────────────────────────────

/**
 * Update properties of a specific node. Returns a new document.
 */
export function updateNodeProps(
  doc: TemplateDocument,
  nodeId: string,
  props: Record<string, unknown>,
): TemplateDocument {
  return {
    ...doc,
    pages: getPages(doc).map(
      (page) =>
        updateNodeInTree(page, nodeId, (node) => {
          const rootKeys = [
            "id",
            "type",
            "name",
            "locked",
            "visible",
            "opacity",
            "rotation",
            "zIndex",
            "groupId",
            "styleRef",
            "position",
            "positionMode",
          ];

          const rootUpdates: Record<string, any> = {};
          const innerUpdates: Record<string, any> = {};

          for (const [key, val] of Object.entries(props)) {
            if (key === "props" && val && typeof val === "object") {
              // Unpack nested props if passed as { props: { ... } }
              Object.assign(innerUpdates, val);
            } else if (rootKeys.includes(key)) {
              rootUpdates[key] = val;
            } else {
              innerUpdates[key] = val;
            }
          }

          const updated = { ...node, ...rootUpdates } as any;
          if (
            Object.keys(innerUpdates).length > 0 &&
            "props" in updated &&
            updated.props
          ) {
            updated.props = { ...updated.props, ...innerUpdates };
          }
          return updated as TemplateNode;
        }) as PageNode,
    ),
  };
}

/**
 * Add a child node to a container. Returns a new document.
 * If index is omitted, the node is appended at the end.
 */
export function addNode(
  doc: TemplateDocument,
  parentId: string,
  node: TemplateNode,
  index?: number,
): TemplateDocument {
  return {
    ...doc,
    pages: getPages(doc).map(
      (page) =>
        updateNodeInTree(page, parentId, (parent) => {
          if (!isContainerNode(parent)) {
            throw new Error(`Node ${parentId} is not a container`);
          }
          const children = [...parent.children];
          if (index !== undefined && index >= 0 && index <= children.length) {
            children.splice(index, 0, node);
          } else {
            children.push(node);
          }
          return { ...parent, children } as TemplateNode;
        }) as PageNode,
    ),
  };
}

/**
 * Remove a node by ID from the document tree. Returns a new document.
 */
export function removeNode(
  doc: TemplateDocument,
  nodeId: string,
): TemplateDocument {
  const pages = getPages(doc);
  // Cannot remove the last page
  if (pages.length === 1 && pages[0]!.id === nodeId) {
    throw new Error("Cannot remove the last page");
  }
  // If the nodeId is a page, filter it out of pages
  const isPageNode = pages.some((p) => p.id === nodeId);
  if (isPageNode) {
    return {
      ...doc,
      pages: pages.filter((p) => p.id !== nodeId),
    };
  }
  // Otherwise, remove from within pages
  return {
    ...doc,
    pages: pages.map((page) => removeNodeFromTree(page, nodeId) as PageNode),
  };
}

/**
 * Move a node to a new parent (or reorder within the same parent).
 */
export function moveNode(
  doc: TemplateDocument,
  nodeId: string,
  newParentId: string,
  index: number,
): TemplateDocument {
  const node = findNode(doc, nodeId);
  if (!node) throw new Error(`Node not found: ${nodeId}`);

  // Remove from old position
  let updated = removeNode(doc, nodeId);
  // Add to new position
  updated = addNode(updated, newParentId, node, index);

  return updated;
}

// ── Internal Tree Mutation Helpers ───────────────────────────

function updateNodeInTree(
  node: TemplateNode,
  targetId: string,
  updater: (node: TemplateNode) => TemplateNode,
): TemplateNode {
  if (node.id === targetId) {
    return updater(node);
  }

  if (isContainerNode(node)) {
    const updatedChildren = node.children.map((child) =>
      updateNodeInTree(child, targetId, updater),
    );

    let updatedElse: TemplateNode[] | undefined;
    if (node.type === "Conditional" && node.elseChildren) {
      updatedElse = node.elseChildren.map((child) =>
        updateNodeInTree(child, targetId, updater),
      );
    }

    if (node.type === "Conditional") {
      return {
        ...node,
        children: updatedChildren,
        elseChildren: updatedElse,
      } as TemplateNode;
    }
    return { ...node, children: updatedChildren } as TemplateNode;
  }

  return node;
}

function removeNodeFromTree(
  node: TemplateNode,
  targetId: string,
): TemplateNode {
  if (isContainerNode(node)) {
    const filteredChildren = node.children
      .filter((child) => child.id !== targetId)
      .map((child) => removeNodeFromTree(child, targetId));

    let filteredElse: TemplateNode[] | undefined;
    if (node.type === "Conditional" && node.elseChildren) {
      filteredElse = node.elseChildren
        .filter((child) => child.id !== targetId)
        .map((child) => removeNodeFromTree(child, targetId));
    }

    if (node.type === "Conditional") {
      return {
        ...node,
        children: filteredChildren,
        elseChildren: filteredElse,
      } as TemplateNode;
    }
    return { ...node, children: filteredChildren } as TemplateNode;
  }

  return node;
}

// ── Document Factory ─────────────────────────────────────────

/**
 * Create a new, empty template document with sensible defaults.
 * v2.0: Creates a single-page document using `pages[]`.
 */
export function createDocument(
  documentType: string,
  entityType: string,
  locale: "ar" | "en" = "ar",
): TemplateDocument {
  const defaultMargins: Spacing = { top: 10, bottom: 10, left: 10, right: 10 };
  const pageSetup: PageSetup = {
    size: "A4",
    orientation: "portrait",
    margins: defaultMargins,
    unit: "mm",
  };

  return {
    version: "2.0",
    meta: {
      name: "",
      documentType,
      entityType,
      locale,
      pageSetup,
    },
    variables: {},
    assets: {},
    pages: [
      {
        type: "Page",
        id: generateNodeId("page"),
        children: [],
      },
    ],
    components: {},
    styles: {},
  };
}
