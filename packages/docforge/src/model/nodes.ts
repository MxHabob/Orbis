// ═══════════════════════════════════════════════════════════════
// @docforge/core — Node Type Definitions
// All 18 node types that can appear in the document tree.
// Each node is a self-contained, serializable unit.
// ═══════════════════════════════════════════════════════════════

import type { Position, Spacing } from "./document";

// ── Discriminated Union of All Node Types ────────────────────

export type TemplateNode =
  | PageNode
  | FrameNode
  | GridNode
  | TextNode
  | ImageNode
  | ShapeNode
  | TableNode
  | BarcodeNode
  | QRCodeNode
  | SeparatorNode
  | SpacerNode
  | RepeaterNode
  | ConditionalNode
  | ComponentRefNode
  | PageBreakNode
  | HeaderNode
  | FooterNode
  | WatermarkNode;

/** All possible node type discriminators */
export type NodeType = TemplateNode["type"];

// ── Base Node ────────────────────────────────────────────────

/** Common properties shared by every node in the tree */
export interface BaseNode {
  /** Node type discriminator */
  type: string;
  /** Unique identifier within the document */
  id: string;
  /** Optional human-readable name for the layer panel */
  name?: string;
  /** If true, node cannot be moved or resized in the editor */
  locked?: boolean;
  /** If false, node is hidden in both editor and output */
  visible?: boolean;
  /** Opacity (0.0 – 1.0) */
  opacity?: number;
}

// ── Container Nodes ──────────────────────────────────────────

/** Root page container — always the top-level node */
export interface PageNode extends BaseNode {
  type: "Page";
  children: TemplateNode[];
}

/** Flexbox-like layout container */
export interface FrameNode extends BaseNode {
  type: "Frame";
  props: {
    position?: Position;
    layout: "horizontal" | "vertical";
    alignment?: "start" | "center" | "end" | "space-between" | "space-around";
    crossAlignment?: "start" | "center" | "end" | "stretch";
    gap?: number;
    padding?: Spacing;
    backgroundColor?: string;
    borderColor?: string;
    borderWidth?: number;
    borderRadius?: number;
    /** Whether children wrap to the next line */
    wrap?: boolean;
  };
  children: TemplateNode[];
}

/** CSS Grid-like 2D layout container */
export interface GridNode extends BaseNode {
  type: "Grid";
  props: {
    position?: Position;
    /** Column definitions (e.g., ["1fr", "2fr", "100mm"]) */
    columns: string[];
    /** Row definitions (e.g., ["auto", "50mm"]) */
    rows?: string[];
    gap?: number;
    columnGap?: number;
    rowGap?: number;
    padding?: Spacing;
    backgroundColor?: string;
    borderColor?: string;
    borderWidth?: number;
  };
  children: TemplateNode[];
}

// ── Content Nodes ────────────────────────────────────────────

/** Rich text block with optional expression interpolation */
export interface TextNode extends BaseNode {
  type: "Text";
  props: {
    /** Text content, may contain {{expressions}} */
    content: string;
    position?: Position;
    fontSize: number;
    fontFamily: string;
    fontWeight?: "normal" | "bold" | "light";
    fontStyle?: "normal" | "italic";
    color?: string;
    backgroundColor?: string;
    textAlign?: "start" | "center" | "end" | "justify";
    direction?: "rtl" | "ltr" | "auto";
    lineHeight?: number;
    letterSpacing?: number;
    textDecoration?: "none" | "underline" | "line-through";
    textTransform?: "none" | "uppercase" | "lowercase";
    padding?: Spacing;
    maxLines?: number;
    overflow?: "visible" | "hidden" | "ellipsis";
  };
}

/** Static or data-bound image */
export interface ImageNode extends BaseNode {
  type: "Image";
  props: {
    /** Reference to an asset in the document's assets map */
    assetRef?: string;
    /** Direct URL (fallback when no assetRef) */
    src?: string;
    /** Expression that resolves to an image URL from data context */
    expression?: string;
    position?: Position;
    width?: number;
    height?: number;
    fit?: "contain" | "cover" | "fill" | "none";
    borderRadius?: number;
    borderColor?: string;
    borderWidth?: number;
    /** Render order: "background" renders behind all siblings */
    layer?: "foreground" | "background";
  };
}

/** Geometric shape (rectangle, circle, line, polygon) */
export interface ShapeNode extends BaseNode {
  type: "Shape";
  props: {
    shapeType: "rectangle" | "circle" | "ellipse" | "line" | "rounded-rect";
    position?: Position;
    fill?: string;
    stroke?: string;
    strokeWidth?: number;
    borderRadius?: number;
    /** Rotation in degrees */
    rotation?: number;
  };
}

/** Data-driven table with headers and rows */
export interface TableNode extends BaseNode {
  type: "Table";
  props: {
    position?: Position;
    /** Column definitions */
    columns: TableColumn[];
    /** Data source path for rows (resolves to an array) */
    dataSource?: string;
    /** Static rows (alternative to dataSource for fixed tables) */
    staticRows?: TableRow[];
    /** Show header row */
    showHeader?: boolean;
    /** Show row numbers column */
    showRowNumbers?: boolean;
    /** Show outer border */
    showBorder?: boolean;
    /** Show grid lines */
    showGrid?: boolean;
    /** Header style */
    headerBackgroundColor?: string;
    headerTextColor?: string;
    headerFontSize?: number;
    headerFontWeight?: "normal" | "bold";
    /** Cell style */
    cellFontSize?: number;
    cellPadding?: number;
    cellTextAlign?: "start" | "center" | "end";
    /** Row alternation */
    alternateRowColor?: string;
    /** Border */
    borderColor?: string;
    borderWidth?: number;
    /** Text direction */
    direction?: "rtl" | "ltr" | "auto";
  };
}

export interface TableColumn {
  /** Column header label */
  header: string;
  /** Data source field path for this column */
  field?: string;
  /** Static content or expression */
  content?: string;
  /** Column width in mm or percentage */
  width?: number | string;
  /** Text alignment */
  align?: "start" | "center" | "end";
  /** Formatter to apply to values */
  formatter?: string;
}

export interface TableRow {
  cells: string[];
}

/** Native barcode element supporting 7 formats */
export interface BarcodeNode extends BaseNode {
  type: "Barcode";
  props: {
    /** Value to encode — may contain {{expressions}} */
    data: string;
    /** Barcode format */
    format: BarcodeFormat;
    position?: Position;
    /** Barcode height in mm */
    height: number;
    /** Bar width (module width) in mm */
    moduleWidth?: number;
    /** Show human-readable text below the barcode */
    showText?: boolean;
    /** Font size for the human-readable text */
    textFontSize?: number;
    /** Barcode color */
    color?: string;
    /** Background color */
    backgroundColor?: string;
    /** Rotation in degrees */
    rotation?: number;
  };
}

export type BarcodeFormat =
  | "code128"
  | "code39"
  | "ean13"
  | "ean8"
  | "upc_a"
  | "itf"
  | "codabar";

/** Native QR code element with configurable error correction */
export interface QRCodeNode extends BaseNode {
  type: "QRCode";
  props: {
    /** Data to encode — may contain {{expressions}} */
    data: string;
    position?: Position;
    /** QR code size in mm (square) */
    size: number;
    /** Error correction level */
    errorCorrection: "L" | "M" | "Q" | "H";
    /** Foreground (module) color */
    color?: string;
    /** Background color */
    backgroundColor?: string;
    /** Quiet zone width in modules */
    quietZone?: number;
    /** Optional embedded logo */
    logoAssetRef?: string;
    /** Logo size as percentage of QR size */
    logoSize?: number;
  };
}

/** Horizontal or vertical divider line */
export interface SeparatorNode extends BaseNode {
  type: "Separator";
  props: {
    direction: "horizontal" | "vertical";
    position?: Position;
    color?: string;
    thickness?: number;
    style?: "solid" | "dashed" | "dotted";
    /** Length in mm (if omitted, stretches to parent width) */
    length?: number;
  };
}

/** Empty space for layout purposes */
export interface SpacerNode extends BaseNode {
  type: "Spacer";
  props: {
    /** Height in mm (for vertical spacers) */
    height?: number;
    /** Width in mm (for horizontal spacers) */
    width?: number;
  };
}

// ── Logic Nodes ──────────────────────────────────────────────

/** Loop over a data collection, repeating children for each item */
export interface RepeaterNode extends BaseNode {
  type: "Repeater";
  props: {
    /** Path to the array in the data context */
    dataSource: string;
    /** Variable name for the current item in each iteration */
    itemVariable: string;
    /** Variable name for the current 0-based index */
    indexVariable?: string;
    /** Layout direction for repeated items */
    layout: "vertical" | "horizontal" | "grid";
    /** Gap between items in mm */
    gap?: number;
    /** Number of columns (for grid layout) */
    columns?: number;
    /** Sort field path */
    sortBy?: string;
    /** Sort direction */
    sortDirection?: "asc" | "desc";
    /** Filter expression (items where this evaluates to false are skipped) */
    filter?: string;
    /** Maximum number of items to render */
    limit?: number;
    /** Content to display when the array is empty */
    emptyMessage?: string;
  };
  children: TemplateNode[];
}

/** Conditional rendering based on an expression */
export interface ConditionalNode extends BaseNode {
  type: "Conditional";
  props: {
    /** Boolean expression — children render when true */
    condition: string;
  };
  /** Rendered when condition is true */
  children: TemplateNode[];
  /** Rendered when condition is false (optional) */
  elseChildren?: TemplateNode[];
}

/** Reference to a reusable component */
export interface ComponentRefNode extends BaseNode {
  type: "ComponentRef";
  props: {
    /** ID of the component to inline */
    componentId: string;
    /** Per-instance property overrides (path → value) */
    overrides?: Record<string, unknown>;
  };
}

// ── Control Nodes ────────────────────────────────────────────

/** Force a page break at this position */
export interface PageBreakNode extends BaseNode {
  type: "PageBreak";
}

/** Content repeated at the top of every page */
export interface HeaderNode extends BaseNode {
  type: "Header";
  props: {
    /** Height in mm */
    height: number;
  };
  children: TemplateNode[];
}

/** Content repeated at the bottom of every page */
export interface FooterNode extends BaseNode {
  type: "Footer";
  props: {
    /** Height in mm */
    height: number;
    /** Show page numbers */
    showPageNumber?: boolean;
    /** Page number format */
    pageNumberFormat?: "{{page}}/{{total}}" | "{{page}}" | string;
  };
  children: TemplateNode[];
}

/** Semi-transparent overlay text or image */
export interface WatermarkNode extends BaseNode {
  type: "Watermark";
  props: {
    /** Watermark type */
    watermarkType: "text" | "image";
    /** Text content (for text watermarks) */
    text?: string;
    /** Asset reference (for image watermarks) */
    assetRef?: string;
    /** Opacity (0.0 – 1.0) */
    opacity?: number;
    /** Rotation in degrees */
    rotation?: number;
    /** Font size for text watermarks */
    fontSize?: number;
    /** Color for text watermarks */
    color?: string;
  };
}

// ── Type Guards ──────────────────────────────────────────────

export function isContainerNode(
  node: TemplateNode,
): node is PageNode | FrameNode | GridNode | RepeaterNode | ConditionalNode | HeaderNode | FooterNode {
  return "children" in node;
}

export function isContentNode(
  node: TemplateNode,
): node is TextNode | ImageNode | ShapeNode | TableNode | BarcodeNode | QRCodeNode | SeparatorNode | SpacerNode {
  return !isContainerNode(node) && node.type !== "ComponentRef" && node.type !== "PageBreak" && node.type !== "Watermark";
}

export function hasPosition(node: TemplateNode): boolean {
  return "props" in node && node.props !== undefined && "position" in (node.props as Record<string, unknown>);
}
