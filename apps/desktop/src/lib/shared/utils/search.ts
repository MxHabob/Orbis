/**
 * Normalizes Arabic text to make search insensitive to diacritics, different forms of Alef, Teh Marbuta, and Yeh.
 */
export function normalizeArabic(text: string): string {
  if (!text) return "";
  return text
    .replace(/[\u064B-\u0652\u0640]/g, "") // remove diacritics (harakat) and tatweel
    .replace(/[أإآ]/g, "ا")
    .replace(/ة/g, "ه")
    .replace(/ى/g, "ي")
    .trim();
}

/**
 * Converts Arabic Indic digits (e.g. ٠, ١, ٢) to standard Latin digits (0-9).
 */
export function normalizeDigits(text: string): string {
  if (!text) return "";
  const map: Record<string, string> = {
    "٠": "0",
    "١": "1",
    "٢": "2",
    "٣": "3",
    "٤": "4",
    "٥": "5",
    "٦": "6",
    "٧": "7",
    "٨": "8",
    "٩": "9",
  };
  return text.replace(/[٠-٩]/g, (char) => map[char] || char);
}

/**
 * Checks if a student matches the search query using smart multilingual search.
 * Matches by Arabic name (with normalization), English name, email, or student number (academic number).
 */
export function matchStudent(student: any, query: string): boolean {
  if (!query) return true;

  // Clean and normalize the query
  let cleanQuery = query.toLowerCase().trim();
  cleanQuery = normalizeDigits(cleanQuery);
  const normalizedQuery = normalizeArabic(cleanQuery);
  const queryWords = normalizedQuery.split(/\s+/).filter(Boolean);

  if (queryWords.length === 0) return true;

  // Gather and normalize student fields
  const fullName = normalizeArabic(
    String(student.full_name || student.student_name || "").toLowerCase(),
  );
  const nameAr = normalizeArabic(String(student.name_ar || "").toLowerCase());
  const nameEn = String(student.name_en || "").toLowerCase();
  const email = String(student.email || "").toLowerCase();

  // Student numbers might contain dashes/dots/slashes, we normalize digits and also keep clean version
  const rawStudentNumber = String(student.student_number || "").toLowerCase();
  const normalizedStudentNumber = normalizeDigits(rawStudentNumber);
  const cleanStudentNumber = normalizedStudentNumber.replace(/[^a-z0-9]/g, "");

  const rawId = String(student.id || "").toLowerCase();
  const cleanId = rawId.replace(/[^a-z0-9]/g, "");

  // Construct search pool for word-based matching (general names/emails)
  const searchPool = `${fullName} ${nameAr} ${nameEn} ${email}`;

  // Check if all query words are present in the search pool
  const nameMatch = queryWords.every((word) => searchPool.includes(word));

  // For academic number, we also support searching by matching the normalized number directly.
  // We clean the query from any special characters for number matching to support "2026001" matching "2026-001"
  const cleanQueryForNumber = normalizedQuery.replace(/[^a-z0-9]/g, "");
  const numberMatch =
    cleanQueryForNumber.length > 0 &&
    (cleanStudentNumber.includes(cleanQueryForNumber) ||
      rawStudentNumber.includes(cleanQuery) ||
      cleanId.includes(cleanQueryForNumber));

  return nameMatch || numberMatch;
}
