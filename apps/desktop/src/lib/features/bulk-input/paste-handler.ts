/**
 * @module paste-handler
 * Parses clipboard data (TSV from Excel/Google Sheets) into grid row objects.
 * Works with any tabular source that copies as tab-separated values.
 */

/**
 * Parse raw TSV clipboard text into a 2D string array.
 * Handles Windows (\r\n) and Unix (\n) line endings.
 */
export function parseTSV(text: string): string[][] {
  return text
    .split(/\r?\n/)
    .filter((line) => line.trim() !== "")
    .map((line) => line.split("\t"));
}

export function normalizeSex(val: string): string {
  const clean = val.trim().toLowerCase();
  if (!clean) return "";
  if (clean === "male" || clean === "m" || clean === "ذكر" || clean === "ولد") {
    return "male";
  }
  if (
    clean === "female" ||
    clean === "f" ||
    clean === "أنثى" ||
    clean === "انثى" ||
    clean === "بنت"
  ) {
    return "female";
  }
  return val;
}

export function normalizeDate(val: string): string {
  let clean = val.trim().replace(/\s+/g, "");
  if (!clean) return "";

  // Match DD/MM/YYYY or DD-MM-YYYY
  let match = clean.match(/^(\d{1,2})[/\-](\d{1,2})[/\-](\d{4})$/);
  if (match) {
    const day = match[1].padStart(2, "0");
    const month = match[2].padStart(2, "0");
    const year = match[3];
    return `${year}-${month}-${day}`;
  }

  // Match YYYY/MM/DD or YYYY-MM-DD
  match = clean.match(/^(\d{4})[/\-](\d{1,2})[/\-](\d{1,2})$/);
  if (match) {
    const year = match[1];
    const month = match[2].padStart(2, "0");
    const day = match[3].padStart(2, "0");
    return `${year}-${month}-${day}`;
  }

  return val;
}

/**
 * Convert a 2D TSV array into row objects using provided column props.
 * Each column prop maps to the corresponding TSV column index.
 *
 * @param data - 2D string array from parseTSV
 * @param props - Ordered array of property names to map columns to
 * @returns Array of row objects
 */
export function mapTSVToRows(
  data: string[][],
  props: string[],
): Record<string, string>[] {
  const rows: Record<string, string>[] = [];

  for (const cells of data) {
    const row: Record<string, string> = {};
    let hasData = false;

    for (let i = 0; i < props.length; i++) {
      let val = cells[i]?.trim() ?? "";
      const prop = props[i];

      if (prop === "sex") {
        val = normalizeSex(val);
      } else if (prop === "date_of_birth") {
        val = normalizeDate(val);
      }

      row[prop] = val;
      if (val) hasData = true;
    }

    if (hasData) {
      rows.push(row);
    }
  }

  return rows;
}

/**
 * Detect if the first row looks like a header row.
 * Uses common header patterns in English and Arabic.
 */
export function isHeaderRow(
  firstRow: string[],
  headerPatterns: RegExp[],
): boolean {
  if (firstRow.length === 0) return false;

  let matches = 0;
  for (const cell of firstRow) {
    for (const pattern of headerPatterns) {
      if (pattern.test(cell)) {
        matches++;
        break;
      }
    }
  }

  // If ≥50% of patterns match, it's a header
  return matches >= Math.ceil(headerPatterns.length / 2);
}

/** Common header patterns for student data */
export const STUDENT_HEADER_PATTERNS: RegExp[] = [
  /name|اسم|الاسم/i,
  /email|بريد|إيميل/i,
  /phone|هاتف|جوال/i,
  /national|وطني|هوية/i,
];

/** Common header patterns for grade data */
export const GRADE_HEADER_PATTERNS: RegExp[] = [
  /student|طالب|المعرف/i,
  /subject|مادة/i,
  /score|درجة|علامة/i,
  /semester|فصل/i,
];
