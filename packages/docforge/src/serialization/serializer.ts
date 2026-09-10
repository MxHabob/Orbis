// ═══════════════════════════════════════════════════════════════
// @docforge/core — Document Serialization (v2.0)
// Serialize / deserialize / diff documents.
// Handles v1.0 → v2.0 migration transparently.
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument } from "../model/document";
import { DOCUMENT_SCHEMA_VERSION } from "../model/document";
import type { PageNode } from "../model/nodes";

/**
 * Serialize a TemplateDocument to a JSON string.
 * Produces compact output by default, or pretty-printed if requested.
 * Strips the deprecated `root` field from output.
 */
export function serializeDocument(
  doc: TemplateDocument,
  pretty = false,
): string {
  // Create a clean copy without the deprecated `root` field
  const { root: _deprecated, ...cleanDoc } = doc;
  return pretty ? JSON.stringify(cleanDoc, null, 2) : JSON.stringify(cleanDoc);
}

/**
 * Deserialize a JSON string into a TemplateDocument.
 * Handles v1.0 → v2.0 migration automatically:
 *   - v1.0 has `root: PageNode` → migrated to `pages: [root]`
 *   - v2.0 has `pages: PageNode[]` → used directly
 */
export function deserializeDocument(json: string): TemplateDocument {
  const parsed = JSON.parse(json) as Record<string, unknown>;

  if (!parsed.meta) {
    throw new Error("Invalid document: missing 'meta' field");
  }

  // Detect v1.0 and migrate
  if (parsed.version !== DOCUMENT_SCHEMA_VERSION) {
    return migrateDocument(parsed);
  }

  // v2.0: validate pages exist
  const doc = parsed as unknown as TemplateDocument;
  if (!doc.pages || !Array.isArray(doc.pages) || doc.pages.length === 0) {
    throw new Error("Invalid v2.0 document: missing or empty 'pages' array");
  }

  return doc;
}

/**
 * Migrate a document from an older schema version to v2.0.
 *
 * v1.0 → v2.0:
 *   - Moves `root` PageNode into `pages[0]`
 *   - Removes `root` field
 *   - Sets version to "2.0"
 *   - Initializes empty `styles`, `components` if missing
 */
function migrateDocument(raw: Record<string, unknown>): TemplateDocument {
  const version = (raw.version as string) || "1.0";

  if (version === "1.0" || !raw.pages) {
    // v1.0 → v2.0: single root → pages array
    const root = raw.root as PageNode | undefined;
    if (!root) {
      throw new Error("Invalid v1.0 document: missing 'root' field");
    }

    const migrated: TemplateDocument = {
      version: DOCUMENT_SCHEMA_VERSION,
      meta: raw.meta as TemplateDocument["meta"],
      variables: (raw.variables as TemplateDocument["variables"]) || {},
      assets: (raw.assets as TemplateDocument["assets"]) || {},
      pages: [root],
      components: {},
      styles: {},
    };

    return migrated;
  }

  // Unknown future version — pass through with version bump
  return {
    ...(raw as unknown as TemplateDocument),
    version: DOCUMENT_SCHEMA_VERSION,
  };
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

  if (
    typeof a === "object" &&
    a !== null &&
    typeof b === "object" &&
    b !== null
  ) {
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
