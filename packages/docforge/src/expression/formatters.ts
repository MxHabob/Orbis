// ═══════════════════════════════════════════════════════════════
// @docforge/core — Built-in Formatters
// 15 built-in formatters for template expressions.
// Consumers can register custom formatters via the registry.
// ═══════════════════════════════════════════════════════════════

import {
  formatArabicNumber,
  toArabicNumerals,
  toArabicOrdinal,
} from "../utils/arabic";

// ── Formatter Type ───────────────────────────────────────────

export type FormatterFn = (value: unknown, ...args: string[]) => unknown;

// ── Registry ─────────────────────────────────────────────────

const formatterRegistry = new Map<string, FormatterFn>();

/** Register a custom formatter */
export function registerFormatter(name: string, fn: FormatterFn): void {
  formatterRegistry.set(name, fn);
}

/** Get a formatter by name (built-in or custom) */
export function getFormatter(name: string): FormatterFn | undefined {
  return formatterRegistry.get(name) ?? BUILTIN_FORMATTERS[name];
}

/** List all available formatter names */
export function getAvailableFormatters(): string[] {
  return [...Object.keys(BUILTIN_FORMATTERS), ...formatterRegistry.keys()];
}

// ── Built-in Formatters ──────────────────────────────────────

const BUILTIN_FORMATTERS: Record<string, FormatterFn> = {
  /**
   * Format a date string.
   * Usage: {{date | date:'yyyy/MM/dd'}}
   */
  date: (value: unknown, format?: string) => {
    if (!value) return "";
    const d = new Date(String(value));
    if (Number.isNaN(d.getTime())) return String(value);

    const fmt = format ?? "yyyy-MM-dd";
    return formatDate(d, fmt);
  },

  /**
   * Convert a Gregorian date to Hijri.
   * Usage: {{date | hijri}}
   */
  hijri: (value: unknown) => {
    if (!value) return "";
    const d = new Date(String(value));
    if (Number.isNaN(d.getTime())) return String(value);

    try {
      return new Intl.DateTimeFormat("ar-SA-u-ca-islamic", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
      }).format(d);
    } catch {
      return String(value);
    }
  },

  /**
   * Format a number with specified decimal places.
   * Usage: {{amount | number:'0.00'}}
   */
  number: (value: unknown, format?: string) => {
    const n = Number(value);
    if (Number.isNaN(n)) return String(value ?? "");

    const decimals = format ? (format.split(".")[1]?.length ?? 0) : 0;
    return n.toLocaleString("en-US", {
      minimumFractionDigits: decimals,
      maximumFractionDigits: decimals,
    });
  },

  /**
   * Convert number to Arabic-Indic numerals.
   * Usage: {{num | arabic_number}}
   */
  arabic_number: (value: unknown) => {
    const n = Number(value);
    if (Number.isNaN(n)) return toArabicNumerals(String(value ?? ""));
    return formatArabicNumber(n);
  },

  /**
   * Format as percentage.
   * Usage: {{0.95 | percent}} → "95%"
   */
  percent: (value: unknown) => {
    const n = Number(value);
    if (Number.isNaN(n)) return String(value ?? "");
    // If value is <= 1, treat as fraction (0.95 → 95%)
    const pct = n <= 1 && n >= 0 ? n * 100 : n;
    return `${Math.round(pct)}%`;
  },

  /**
   * Convert to uppercase.
   * Usage: {{name | uppercase}}
   */
  uppercase: (value: unknown) => String(value ?? "").toUpperCase(),

  /**
   * Convert to lowercase.
   * Usage: {{name | lowercase}}
   */
  lowercase: (value: unknown) => String(value ?? "").toLowerCase(),

  /**
   * Get Arabic ordinal (الأول, الثاني...).
   * Usage: {{rank | ordinal}}
   */
  ordinal: (value: unknown) => {
    const n = Number(value);
    if (Number.isNaN(n)) return String(value ?? "");
    return toArabicOrdinal(n);
  },

  /**
   * Convert a numeric score to a grade letter.
   * Usage: {{95 | grade_letter}}
   */
  grade_letter: (value: unknown) => {
    const n = Number(value);
    if (Number.isNaN(n)) return String(value ?? "");
    if (n >= 97) return "أ+";
    if (n >= 93) return "أ";
    if (n >= 90) return "أ-";
    if (n >= 87) return "ب+";
    if (n >= 83) return "ب";
    if (n >= 80) return "ب-";
    if (n >= 77) return "ج+";
    if (n >= 73) return "ج";
    if (n >= 70) return "ج-";
    if (n >= 67) return "د+";
    if (n >= 63) return "د";
    if (n >= 60) return "د-";
    return "هـ";
  },

  /**
   * Truncate a string to N characters.
   * Usage: {{text | truncate:50}}
   */
  truncate: (value: unknown, maxLength?: string) => {
    const str = String(value ?? "");
    const max = Number.parseInt(maxLength ?? "100", 10);
    if (str.length <= max) return str;
    return `${str.substring(0, max)}...`;
  },

  /**
   * Provide a default value when the input is null/undefined/empty.
   * Usage: {{name | default:'غير محدد'}}
   */
  default: (value: unknown, defaultValue?: string) => {
    if (value == null || String(value).trim() === "") {
      return defaultValue ?? "";
    }
    return value;
  },

  /**
   * Pad string from the start.
   * Usage: {{num | pad_start:6:'0'}} → "000042"
   */
  pad_start: (value: unknown, length?: string, char?: string) => {
    const str = String(value ?? "");
    const len = Number.parseInt(length ?? "0", 10);
    return str.padStart(len, char ?? " ");
  },

  /**
   * Pad string from the end.
   * Usage: {{num | pad_end:6:'0'}}
   */
  pad_end: (value: unknown, length?: string, char?: string) => {
    const str = String(value ?? "");
    const len = Number.parseInt(length ?? "0", 10);
    return str.padEnd(len, char ?? " ");
  },

  /**
   * Join an array with a separator.
   * Usage: {{subjects | join:', '}}
   */
  join: (value: unknown, separator?: string) => {
    if (!Array.isArray(value)) return String(value ?? "");
    return value.join(separator ?? ", ");
  },

  /**
   * Get the count/length of an array.
   * Usage: {{students | count}}
   */
  count: (value: unknown) => {
    if (Array.isArray(value)) return value.length;
    if (typeof value === "string") return value.length;
    return 0;
  },
};

// ── Date Formatting Helper ───────────────────────────────────

function formatDate(date: Date, format: string): string {
  const pad = (n: number, len = 2) => String(n).padStart(len, "0");

  const replacements: Record<string, string> = {
    yyyy: String(date.getFullYear()),
    yy: String(date.getFullYear()).substring(2),
    MM: pad(date.getMonth() + 1),
    M: String(date.getMonth() + 1),
    dd: pad(date.getDate()),
    d: String(date.getDate()),
    HH: pad(date.getHours()),
    H: String(date.getHours()),
    mm: pad(date.getMinutes()),
    m: String(date.getMinutes()),
    ss: pad(date.getSeconds()),
    s: String(date.getSeconds()),
  };

  let result = format;
  // Replace longest tokens first to avoid partial matches
  for (const [token, value] of Object.entries(replacements).sort(
    (a, b) => b[0].length - a[0].length,
  )) {
    result = result.replace(new RegExp(token, "g"), value);
  }

  return result;
}
