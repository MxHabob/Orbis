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

  // Handle both StudentDetailDto ({ student, person, enrollment }) and direct models
  const s = student?.student || student;
  const p = student?.person || student;
  const e = student?.enrollment || student;

  // Gather and normalize student fields
  const fullName = normalizeArabic(
    String(
      (p.first_name && p.last_name ? `${p.first_name} ${p.last_name}` : "") ||
      p.full_name ||
      p.name_ar ||
      p.name_en ||
      s.student_name ||
      ""
    ).toLowerCase(),
  );
  const nameAr = normalizeArabic(String(p.name_ar || "").toLowerCase());
  const nameEn = String(p.name_en || "").toLowerCase();
  const email = String(p.email || "").toLowerCase();
  const phone = String(p.phone || "").toLowerCase();
  const nationalId = String(p.national_id || "").toLowerCase();

  // Student numbers might contain dashes/dots/slashes, we normalize digits and also keep clean version
  const rawStudentNumber = String(s.student_number || student.student_number || "").toLowerCase();
  const normalizedStudentNumber = normalizeDigits(rawStudentNumber);
  const cleanStudentNumber = normalizedStudentNumber.replace(/[^a-z0-9]/g, "");

  const rawId = String(s.id || student.id || "").toLowerCase();
  const cleanId = rawId.replace(/[^a-z0-9]/g, "");

  // Construct search pool for word-based matching
  const searchPool = `${fullName} ${nameAr} ${nameEn} ${email} ${phone} ${nationalId}`;

  // Check if all query words are present in the search pool
  const nameMatch = queryWords.every((word) => searchPool.includes(word));

  // For academic number, ID, or national ID matching
  const cleanQueryForNumber = normalizedQuery.replace(/[^a-z0-9]/g, "");
  const numberMatch =
    cleanQueryForNumber.length > 0 &&
    (cleanStudentNumber.includes(cleanQueryForNumber) ||
      rawStudentNumber.includes(cleanQuery) ||
      cleanId.includes(cleanQueryForNumber) ||
      nationalId.includes(cleanQueryForNumber));

  return nameMatch || numberMatch;
}
