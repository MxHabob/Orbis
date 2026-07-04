/**
 * @module paste-handler
 * Parses Excel clipboard data (tab-separated values) and maps to grid rows.
 */

import type { BulkInputType } from "./bulk-input-config";
import { getDataProps } from "./bulk-input-config";

/**
 * Parse tab-separated clipboard text from Excel into grid row objects.
 * Maps columns in order based on the active bulk input type.
 *
 * @param clipboardText - Raw clipboard text (TSV from Excel)
 * @param type - Active bulk input type (determines column mapping)
 * @returns Array of row objects with properties matching the grid columns
 */
export function parseExcelPaste(
  clipboardText: string,
  type: BulkInputType,
): Record<string, unknown>[] {
  const props = getDataProps(type);
  const lines = clipboardText.split(/\r?\n/).filter((line) => line.trim() !== "");
  const rows: Record<string, unknown>[] = [];

  for (const line of lines) {
    const cells = line.split("\t");
    const row: Record<string, unknown> = {};

    // Map each cell to its corresponding column prop
    for (let i = 0; i < props.length; i++) {
      row[props[i]] = cells[i]?.trim() ?? "";
    }

    // Initialize internal fields
    row.__errors = new Map<string, string>();

    // Skip rows where all data fields are empty
    const hasData = props.some((p) => {
      const val = String(row[p] ?? "").trim();
      return val !== "";
    });

    if (hasData) {
      rows.push(row);
    }
  }

  return rows;
}

/**
 * Detect if the first row of pasted data looks like a header row.
 * Checks if the values match common header names.
 */
export function detectHeaderRow(
  firstRow: Record<string, unknown>,
  type: BulkInputType,
): boolean {
  const headerPatterns: Record<BulkInputType, RegExp[]> = {
    students: [/name|اسم/i, /email|بريد/i, /phone|هاتف/i],
    teachers: [/name|اسم/i, /email|بريد/i, /phone|هاتف/i],
    grades: [/email|بريد/i, /subject|مادة/i, /score|درجة/i],
  };

  const patterns = headerPatterns[type];
  const values = Object.values(firstRow).filter(
    (v) => typeof v === "string" && !String(v).startsWith("__"),
  );

  let matches = 0;
  for (const val of values) {
    for (const pattern of patterns) {
      if (pattern.test(String(val))) {
        matches++;
        break;
      }
    }
  }

  // If more than half of the patterns match, it's likely a header
  return matches >= Math.ceil(patterns.length / 2);
}
