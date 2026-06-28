// ═══════════════════════════════════════════════════════════════
// @docforge/core — Arabic Utilities
// Number conversion, BiDi helpers — zero dependencies
// ═══════════════════════════════════════════════════════════════

const WESTERN_DIGITS = "0123456789";
const ARABIC_DIGITS = "٠١٢٣٤٥٦٧٨٩";

/** Convert Western numerals (0-9) to Arabic-Indic numerals (٠-٩) */
export function toArabicNumerals(input: number | string): string {
  const str = String(input);
  let result = "";
  for (const ch of str) {
    const idx = WESTERN_DIGITS.indexOf(ch);
    result += idx >= 0 ? ARABIC_DIGITS[idx]! : ch;
  }
  return result;
}

/** Convert Arabic-Indic numerals (٠-٩) to Western numerals (0-9) */
export function toWesternNumerals(input: string): string {
  let result = "";
  for (const ch of input) {
    const idx = ARABIC_DIGITS.indexOf(ch);
    result += idx >= 0 ? WESTERN_DIGITS[idx]! : ch;
  }
  return result;
}

/** Format a number with Arabic-Indic digits and proper grouping */
export function formatArabicNumber(
  n: number,
  options?: { decimals?: number; groupSeparator?: string; decimalSeparator?: string },
): string {
  const { decimals = 0, groupSeparator = "٬", decimalSeparator = "٫" } = options ?? {};

  const fixed = n.toFixed(decimals);
  const [intPart, decPart] = fixed.split(".");

  // Add grouping
  const grouped = (intPart ?? "").replace(/\B(?=(\d{3})+(?!\d))/g, groupSeparator);
  const arabicInt = toArabicNumerals(grouped);

  if (decPart) {
    return arabicInt + decimalSeparator + toArabicNumerals(decPart);
  }
  return arabicInt;
}

/** Arabic ordinal names for common rankings */
const ARABIC_ORDINALS: Record<number, string> = {
  1: "الأول",
  2: "الثاني",
  3: "الثالث",
  4: "الرابع",
  5: "الخامس",
  6: "السادس",
  7: "السابع",
  8: "الثامن",
  9: "التاسع",
  10: "العاشر",
};

/** Get Arabic ordinal name (الأول، الثاني...) for numbers 1-10 */
export function toArabicOrdinal(n: number): string {
  return ARABIC_ORDINALS[n] ?? toArabicNumerals(n);
}

/** Detect if a string contains Arabic characters */
export function containsArabic(text: string): boolean {
  return /[\u0600-\u06FF\u0750-\u077F\u08A0-\u08FF\uFB50-\uFDFF\uFE70-\uFEFF]/.test(text);
}

/** Determine the dominant text direction for a string */
export function detectDirection(text: string): "rtl" | "ltr" {
  // Count RTL vs LTR characters
  const rtlCount = (text.match(/[\u0600-\u06FF\u0750-\u077F\u08A0-\u08FF\uFB50-\uFDFF\uFE70-\uFEFF\u0590-\u05FF]/g) ?? []).length;
  const ltrCount = (text.match(/[a-zA-Z]/g) ?? []).length;
  return rtlCount >= ltrCount ? "rtl" : "ltr";
}
