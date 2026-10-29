const arabicMarks = /[\u064B-\u065F\u0670\u06D6-\u06ED\u0640]/gu;
const alefVariants = /[أإآٱ]/gu;

export function normalizeArabic(value: string): string {
  return value
    .normalize("NFKC")
    .replace(arabicMarks, "")
    .replace(alefVariants, "ا")
    .replace(/ة/gu, "ه")
    .replace(/ى/gu, "ي")
    .replace(/\s+/gu, " ")
    .trim();
}

export function normalizeEnglish(value: string): string {
  return value
    .normalize("NFKC")
    .toLocaleLowerCase("en")
    .replace(/\s+/gu, " ")
    .trim();
}

export function normalizeTerm(value: string, language: "ar" | "en"): string {
  return language === "ar" ? normalizeArabic(value) : normalizeEnglish(value);
}
