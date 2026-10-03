// ═══════════════════════════════════════════════════════════════
// @docforge/core — Data Binding Types
// Abstractions for data sources and data contexts.
// Consumers implement DataResolver for their specific domain.
// ═══════════════════════════════════════════════════════════════

import type { DataContext } from "../expression/evaluator";

export type { DataContext } from "../expression/evaluator";

// ── Data Schema ──────────────────────────────────────────────

/**
 * Describes the shape of data available for a given entity type.
 * Used by the editor to show variable pickers and validate bindings.
 */
export interface DataSchema {
  /** Entity type identifier (e.g., "enrollment") */
  entityType: string;
  /** Human-readable label */
  label: string;
  /** Arabic label */
  labelAr?: string;
  /** Available fields */
  fields: DataFieldSchema[];
}

export interface DataFieldSchema {
  /** Dot-path (e.g., "student.name_ar") */
  path: string;
  /** Value type */
  type: "string" | "number" | "date" | "boolean" | "image" | "array" | "object";
  /** Human-readable label */
  label: string;
  /** Arabic label */
  labelAr?: string;
  /** For array/object types, child fields */
  children?: DataFieldSchema[];
}

// ── Data Resolver Interface ──────────────────────────────────

/**
 * Interface implemented by consumers to provide data for template rendering.
 * This is the primary extension point for integrating docforge with any data source.
 */
export interface DataResolver {
  /** The entity type this resolver handles */
  entityType: string;

  /** Describe available fields */
  schema(): DataSchema;

  /** Resolve a single record by ID */
  resolveOne(id: string): Promise<DataContext>;

  /** Resolve multiple records with filtering */
  resolveMany(filter: DataFilter): Promise<DataContext[]>;

  /** Get the count of records matching a filter */
  count(filter: DataFilter): Promise<number>;

  /** Return sample data for editor preview */
  sampleData(): DataContext;
}

export interface DataFilter {
  /** Filter conditions */
  conditions: FilterCondition[];
  /** Sort field path */
  sortBy?: string;
  /** Sort direction */
  sortDirection?: "asc" | "desc";
  /** Pagination offset */
  offset?: number;
  /** Pagination limit */
  limit?: number;
}

export interface FilterCondition {
  field: string;
  operator: "eq" | "neq" | "gt" | "gte" | "lt" | "lte" | "in" | "contains";
  value: unknown;
}

// ── System Variables ─────────────────────────────────────────

/** System variables automatically available in every template */
export interface SystemVars {
  date: string;
  date_hijri: string;
  time: string;
  timestamp: string;
  page_number: number;
  total_pages: number;
  generation_id: string;
  record_sequence: number;
  total_records: number;
  batch_index: number;
}

/** Organization variables injected by the host application */
export interface OrgVars {
  name: string;
  name_ar: string;
  code: string;
  logo_url?: string;
  timezone?: string;
  locale?: string;
  currency?: string;
}

/**
 * Create a full data context by merging record data with system/org variables.
 */
export function createFullContext(
  recordData: DataContext,
  systemVars?: Partial<SystemVars>,
  orgVars?: Partial<OrgVars>,
): DataContext {
  return {
    ...recordData,
    $system: {
      date: new Date().toISOString().split("T")[0],
      time: new Date().toISOString().split("T")[1]?.split(".")[0] ?? "",
      timestamp: new Date().toISOString(),
      page_number: 1,
      total_pages: 1,
      generation_id: "",
      record_sequence: 0,
      total_records: 1,
      batch_index: 0,
      ...systemVars,
    },
    $org: {
      name: "",
      name_ar: "",
      code: "",
      ...orgVars,
    },
  };
}
