/**
 * Hijri <-> Gregorian Date Utilities using standard Intl API.
 */

export interface HijriDate {
  hy: number; // Hijri Year
  hm: number; // Hijri Month (1-12)
  hd: number; // Hijri Day (1-30)
}

/**
 * Converts a Gregorian Date to Hijri.
 */
export function gregorianToHijri(
  date: Date | string | null | undefined,
): HijriDate | null {
  if (!date) return null;
  const d = typeof date === "string" ? new Date(date) : date;
  if (isNaN(d.getTime())) return null;

  try {
    const parts = new Intl.DateTimeFormat("en-US-u-ca-islamic-umalqura", {
      day: "numeric",
      month: "numeric",
      year: "numeric",
    }).formatToParts(d);

    const hd = parseInt(parts.find((p) => p.type === "day")?.value || "1", 10);
    const hm = parseInt(
      parts.find((p) => p.type === "month")?.value || "1",
      10,
    );
    const hy = parseInt(
      parts.find((p) => p.type === "year")?.value || "1447",
      10,
    );

    return { hy, hm, hd };
  } catch (e) {
    console.error("Hijri conversion failed:", e);
    return null;
  }
}

/**
 * Converts a Hijri date (hy, hm, hd) to Gregorian Date.
 */
export function hijriToGregorian(hy: number, hm: number, hd: number): Date {
  // 1447-01-01 Hijri was approx 2025-06-26 Gregorian
  const baseGregorian = new Date(2025, 5, 26);
  const hijriYearDiff = hy - 1447;
  const approxDays =
    Math.floor(hijriYearDiff * 354.367) + (hm - 1) * 29.5 + (hd - 1);

  const targetDate = new Date(
    baseGregorian.getTime() + approxDays * 24 * 60 * 60 * 1000,
  );

  // Search within +/- 15 days to find the exact match
  for (let offset = -15; offset <= 15; offset++) {
    const checkDate = new Date(
      targetDate.getTime() + offset * 24 * 60 * 60 * 1000,
    );
    const result = gregorianToHijri(checkDate);
    if (result && result.hy === hy && result.hm === hm && result.hd === hd) {
      return checkDate;
    }
  }
  return targetDate; // Fallback if exact match not found
}

export const HIJRI_MONTHS_EN = [
  "Muharram",
  "Safar",
  "Rabi' al-Awwal",
  "Rabi' al-Thani",
  "Jumada al-Awwal",
  "Jumada al-Thani",
  "Rajab",
  "Sha'ban",
  "Ramadan",
  "Shawwal",
  "Dhu al-Qadah",
  "Dhu al-Hijjah",
];

export const HIJRI_MONTHS_AR = [
  "محرم",
  "صفر",
  "ربيع الأول",
  "ربيع الآخر",
  "جمادى الأولى",
  "جمادى الآخرة",
  "رجب",
  "شعبان",
  "رمضان",
  "شوال",
  "ذو القعدة",
  "ذو الحجة",
];
