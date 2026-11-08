/**
 * Hijri <-> Gregorian Date Utilities using standard Intl API.
 */

export interface HijriDate {
  hy: number; // Hijri Year
  hm: number; // Hijri Month (1-12)
  hd: number; // Hijri Day (1-30)
}

export interface HijriSchoolCalendar {
  hijriYear: number;
  monthLengths: number[];
  startDate: Date;
  endDate: Date;
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

export function getHijriSchoolMonthLengths(
  startDate: Date | string,
): { hijriYear: number; monthLengths: number[] } | null {
  const date =
    typeof startDate === "string"
      ? new Date(`${startDate.split("T")[0]}T12:00:00`)
      : startDate;
  const hijriStart = gregorianToHijri(date);
  if (!hijriStart || hijriStart.hm !== 1 || hijriStart.hd !== 1) return null;

  const starts = Array.from({ length: 9 }, (_, index) =>
    hijriToGregorian(hijriStart.hy, index + 1, 1),
  );
  for (let index = 0; index < starts.length; index += 1) {
    const converted = gregorianToHijri(starts[index]);
    if (
      !converted ||
      converted.hy !== hijriStart.hy ||
      converted.hm !== index + 1 ||
      converted.hd !== 1
    ) {
      return null;
    }
  }

  const utcDay = (value: Date) =>
    Date.UTC(value.getFullYear(), value.getMonth(), value.getDate()) /
    86_400_000;
  const monthLengths = starts
    .slice(0, 8)
    .map((value, index) => utcDay(starts[index + 1]) - utcDay(value));
  return monthLengths.every((days) => days === 29 || days === 30)
    ? { hijriYear: hijriStart.hy, monthLengths }
    : null;
}

export function getNextHijriSchoolCalendar(
  referenceDate: Date = new Date(),
): HijriSchoolCalendar | null {
  const currentHijri = gregorianToHijri(referenceDate);
  if (!currentHijri) return null;
  const hijriYear =
    currentHijri.hm === 1 && currentHijri.hd === 1
      ? currentHijri.hy
      : currentHijri.hy + 1;
  const startDate = hijriToGregorian(hijriYear, 1, 1);
  const lengths = getHijriSchoolMonthLengths(startDate);
  if (!lengths || lengths.hijriYear !== hijriYear) return null;

  const endDate = hijriToGregorian(hijriYear, 9, 1);
  endDate.setDate(endDate.getDate() - 1);
  return {
    hijriYear,
    monthLengths: lengths.monthLengths,
    startDate,
    endDate,
  };
}

import * as m from "$lib/paraglide/messages";

export const getHijriMonths = () => [
  m.common_hijri_month_1(),
  m.common_hijri_month_2(),
  m.common_hijri_month_3(),
  m.common_hijri_month_4(),
  m.common_hijri_month_5(),
  m.common_hijri_month_6(),
  m.common_hijri_month_7(),
  m.common_hijri_month_8(),
  m.common_hijri_month_9(),
  m.common_hijri_month_10(),
  m.common_hijri_month_11(),
  m.common_hijri_month_12(),
];
