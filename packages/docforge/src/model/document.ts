// ═══════════════════════════════════════════════════════════════
// @docforge/core — Document Model Types  (v2.0)
//
// Hierarchical model:  Document → Pages[] → Elements
//
// v2.0 changes (backward-compatible):
//   - `root: PageNode`  →  `pages: PageNode[]`  (unlimited pages)
//   - Added `styles`, `components`, `theme`
//   - Added `StyleDefinition`, `ThemeDefinition`
// ═══════════════════════════════════════════════════════════════

import type { PageNode, NodeType, TemplateNode } from "./nodes";

// ── Document Root ────────────────────────────────────────────

/** Schema version for migration compatibility */
export const DOCUMENT_SCHEMA_VERSION = "2.0";

/**
 * Root type representing a complete template document.
 * This is the single source of truth stored in the database
 * and consumed by both the editor and the rendering engine.
 *
 * v2.0: Supports unlimited pages via `pages[]`.
 * Migration from v1.0 (`root`) is handled in serialization/serializer.ts.
 */
export interface TemplateDocument {
  /** Schema version for forward-compatible migration */
  version: string;
  /** Document metadata (name, type, page setup) */
  meta: DocumentMeta;
  /** Variable definitions mapping variable names to their data sources */
  variables: Record<string, VariableDefinition>;
  /** Asset references mapping logical names to asset IDs */
  assets: Record<string, AssetReference>;

  // ── v2.0: Multi-page hierarchy ───────────────────────────
  /** Ordered list of pages. At least one page is required. */
  pages: PageNode[];

  /** Reusable component library — keyed by component ID */
  components?: Record<string, TemplateNode>;
  /** Named style presets */
  styles?: Record<string, StyleDefinition>;
  /** Document-level theme */
  theme?: ThemeDefinition;

  // ── v1.0 backward compat (deprecated) ────────────────────
  /**
   * @deprecated Use `pages[0]` instead. Kept only for v1.0 migration.
   * The serializer transparently migrates `root` → `pages[root]`.
   */
  root?: PageNode;
}

// ── Document Metadata ────────────────────────────────────────

export interface DocumentMeta {
  /** Unique identifier (set by storage layer, not by the model) */
  id?: string;
  /** Template display name */
  name: string;
  /** Template name in Arabic */
  nameAr?: string;
  /** Type of document this template produces */
  documentType: string;
  /** Domain entity type this template binds to */
  entityType: string;
  /** Primary locale for text direction */
  locale: "ar" | "en";
  /** Page layout configuration */
  pageSetup: PageSetup;
}

// ── Page Setup ───────────────────────────────────────────────

export interface PageSetup {
  /** Paper size preset */
  size: PageSize;
  /** Custom width in mm (used when size is 'Custom') */
  width?: number;
  /** Custom height in mm (used when size is 'Custom') */
  height?: number;
  /** Page orientation */
  orientation: Orientation;
  /** Page margins in mm */
  margins: Spacing;
  /** Measurement unit (always mm internally) */
  unit: "mm";
}

export type PageSize =
  | "A4"
  | "A3"
  | "A5"
  | "Letter"
  | "Legal"
  | "ID_CARD"
  | "BADGE"
  | "Custom";

export type Orientation = "portrait" | "landscape";

export interface Spacing {
  top: number;
  bottom: number;
  left: number;
  right: number;
}

// ── Variable Definitions ─────────────────────────────────────

export interface VariableDefinition {
  /** Data source path (e.g., "enrollment.student.name_ar") */
  source: string;
  /** Value type for validation and formatting */
  type: VariableType;
  /** Human-readable label */
  label?: string;
  /** Arabic label */
  labelAr?: string;
  /** Default format string (for date/number types) */
  format?: string;
  /** Expression for computed variables (when source is "$computed") */
  expression?: string;
  /** Default value when source resolves to null/undefined */
  defaultValue?: string;
}

export type VariableType =
  | "string"
  | "number"
  | "date"
  | "boolean"
  | "image"
  | "array";

// ── Asset References ─────────────────────────────────────────

export interface AssetReference {
  /** Storage ID of the asset (resolved by AssetStore) */
  assetId: string;
  /** Asset category */
  type: AssetType;
}

export type AssetType =
  | "image"
  | "font"
  | "logo"
  | "signature"
  | "stamp"
  | "background"
  | "watermark";

// ── Position & Sizing ────────────────────────────────────────

export interface Position {
  /** X coordinate in mm from left edge */
  x: number;
  /** Y coordinate in mm from top edge */
  y: number;
  /** Width in mm, or percentage string like "100%" */
  width: number | string;
  /** Height in mm, percentage string, or undefined for auto-height */
  height?: number | string;
}

export type PositionMode = "absolute" | "flow";

// ── Styles ───────────────────────────────────────────────────

/** A named, reusable style preset that can be applied to nodes */
export interface StyleDefinition {
  /** Human-readable name */
  name: string;
  /** Arabic name */
  nameAr?: string;
  /** Node types this style can be applied to */
  appliesTo: NodeType[];
  /** Style properties (font, color, spacing, etc.) */
  props: Record<string, unknown>;
}

// ── Theme ────────────────────────────────────────────────────

/** Document-level theme controlling global visual defaults */
export interface ThemeDefinition {
  /** Primary accent color (hex) */
  primaryColor: string;
  /** Secondary color (hex) */
  secondaryColor: string;
  /** Default Latin/English font family */
  fontFamily: string;
  /** Default Arabic font family */
  fontFamilyAr: string;
  /** Reference to a style ID for headers */
  headerStyle?: string;
  /** Reference to a style ID for body text */
  bodyStyle?: string;
  /** Reference to a style ID for captions */
  captionStyle?: string;
}

// ── Helper: Get pages ────────────────────────────────────────

/**
 * Safely get the pages array from a document, handling v1.0 compat.
 * Always returns at least one page.
 */
export function getPages(doc: TemplateDocument): PageNode[] {
  if (doc.pages && doc.pages.length > 0) {
    return doc.pages;
  }
  // v1.0 fallback
  if (doc.root) {
    return [doc.root];
  }
  return [];
}

/**
 * Get the first (primary) page. Throws if no pages exist.
 */
export function getPrimaryPage(doc: TemplateDocument): PageNode {
  const pages = getPages(doc);
  if (pages.length === 0) {
    throw new Error("Document has no pages");
  }
  return pages[0]!;
}
