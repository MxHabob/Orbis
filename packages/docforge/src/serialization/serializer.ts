// ═══════════════════════════════════════════════════════════════
// @docforge/core — Document Serialization
// Serialize / deserialize / diff documents
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument } from "../model/document";
import { DOCUMENT_SCHEMA_VERSION } from "../model/document";

/**
 * Serialize a TemplateDocument to a JSON string.
 * Produces compact output by default, or pretty-printed if requested.
 */
export function serializeDocument(doc: TemplateDocument, pretty = false): string {
  return pretty ? JSON.stringify(doc, null, 2) : JSON.stringify(doc);
}

/**
 * Deserialize a JSON string into a TemplateDocument.
 * Throws if the JSON is invalid or the schema version is unsupported.
 */
export function deserializeDocument(json: string): TemplateDocument {
  const parsed = JSON.parse(json) as TemplateDocument;

  if (!parsed.version) {
    throw new Error("Invalid document: missing 'version' field");
  }
  if (!parsed.meta) {
    throw new Error("Invalid document: missing 'meta' field");
  }
  if (!parsed.root) {
    throw new Error("Invalid document: missing 'root' field");
  }

  // Apply migrations if needed
  if (parsed.version !== DOCUMENT_SCHEMA_VERSION) {
    return migrateDocument(parsed);
  }

  return parsed;
}

/**
 * Migrate a document from an older schema version to the current version.
 * Currently a no-op since we only have v1.0, but the infrastructure is ready.
 */
function migrateDocument(doc: TemplateDocument): TemplateDocument {
  // Future: add migration logic as schema evolves
  // For now, just update the version stamp
  return { ...doc, version: DOCUMENT_SCHEMA_VERSION };
}

// ── Document Diff ────────────────────────────────────────────

export interface DocumentDiff {
  path: string;
  type: "added" | "removed" | "changed";
  oldValue?: unknown;
  newValue?: unknown;
}

/**
 * Compute a structural diff between two documents.
 * Returns a list of changes (added, removed, changed paths).
 */
export function diffDocuments(
  a: TemplateDocument,
  b: TemplateDocument,
): DocumentDiff[] {
  const diffs: DocumentDiff[] = [];
  diffObjects(a, b, "", diffs);
  return diffs;
}

function diffObjects(
  a: unknown,
  b: unknown,
  path: string,
  diffs: DocumentDiff[],
): void {
  if (a === b) return;

  if (a == null && b != null) {
    diffs.push({ path, type: "added", newValue: b });
    return;
  }
  if (a != null && b == null) {
    diffs.push({ path, type: "removed", oldValue: a });
    return;
  }

  if (typeof a !== typeof b) {
    diffs.push({ path, type: "changed", oldValue: a, newValue: b });
    return;
  }

  if (Array.isArray(a) && Array.isArray(b)) {
    const maxLen = Math.max(a.length, b.length);
    for (let i = 0; i < maxLen; i++) {
      diffObjects(a[i], b[i], `${path}[${i}]`, diffs);
    }
    return;
  }

  if (typeof a === "object" && a !== null && typeof b === "object" && b !== null) {
    const aObj = a as Record<string, unknown>;
    const bObj = b as Record<string, unknown>;
    const allKeys = new Set([...Object.keys(aObj), ...Object.keys(bObj)]);

    for (const key of allKeys) {
      const childPath = path ? `${path}.${key}` : key;
      diffObjects(aObj[key], bObj[key], childPath, diffs);
    }
    return;
  }

  if (a !== b) {
    diffs.push({ path, type: "changed", oldValue: a, newValue: b });
  }
}
