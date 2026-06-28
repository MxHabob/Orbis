// ═══════════════════════════════════════════════════════════════
// @docforge/core — Document Model Types
// The canonical type definitions for the template document AST.
// This is the single source of truth for all template structures.
// ═══════════════════════════════════════════════════════════════

import type { PageNode } from "./nodes";

// ── Document Root ────────────────────────────────────────────

/** Schema version for migration compatibility */
export const DOCUMENT_SCHEMA_VERSION = "1.0";

/**
 * Root type representing a complete template document.
 * This is the single source of truth stored in the database
 * and consumed by both the editor and the rendering engine.
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
  /** The root page node containing all visual content */
  root: PageNode;
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
