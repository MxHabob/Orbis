export const ARABIC_PERIOD_NAMES: Record<number, string> = {
  1: "الحصة الأولى",
  2: "الحصة الثانية",
  3: "الحصة الثالثة",
  4: "الحصة الرابعة",
  5: "الحصة الخامسة",
  6: "الحصة السادسة",
  7: "الحصة السابعة",
  8: "الحصة الثامنة",
  9: "الحصة التاسعة",
  10: "الحصة العاشرة",
  11: "الحصة الحادية عشرة",
  12: "الحصة الثانية عشرة",
};

export const ENGLISH_PERIOD_NAMES: Record<number, string> = {
  1: "1st Period",
  2: "2nd Period",
  3: "3rd Period",
  4: "4th Period",
  5: "5th Period",
  6: "6th Period",
  7: "7th Period",
  8: "8th Period",
  9: "9th Period",
  10: "10th Period",
  11: "11th Period",
  12: "12th Period",
};

export function getDurationMinutes(
  startTime?: string,
  endTime?: string,
): number {
  if (!startTime || !endTime) return 0;
  const [h1 = NaN, m1 = NaN] = startTime.split(":").map((v) => parseInt(v, 10));
  const [h2 = NaN, m2 = NaN] = endTime.split(":").map((v) => parseInt(v, 10));
  if (isNaN(h1) || isNaN(m1) || isNaN(h2) || isNaN(m2)) return 0;
  return h2 * 60 + m2 - (h1 * 60 + m1);
}

export function formatTimeSlotTime(val?: string, isArabic = false): string {
  if (!val) return "";
  const clean = val.slice(0, 5);
  const [hStr = "", mStr = "00"] = clean.split(":");
  let h = parseInt(hStr, 10);
  const m = mStr || "00";
  if (isNaN(h)) return val;
  const period = h >= 12 ? (isArabic ? "م" : "PM") : isArabic ? "ص" : "AM";
  h = h % 12 || 12;
  return `${h}:${m} ${period}`;
}

export function formatTimeRange(
  start?: string,
  end?: string,
  isArabic = false,
): string {
  if (!start || !end) return "";
  return `${formatTimeSlotTime(start, isArabic)} - ${formatTimeSlotTime(end, isArabic)}`;
}

const SUBJECT_PALETTE = [
  "#6366f1", // Indigo
  "#0d9488", // Teal
  "#db2777", // Pink / Magenta
  "#d97706", // Amber
  "#059669", // Emerald
  "#0284c7", // Sky
  "#9333ea", // Violet
  "#ea580c", // Orange
] as const;

/**
 * Returns an explicit subject color when configured, or a deterministic
 * high-contrast palette color derived from the subject ID.
 */
export function resolveSubjectColor(
  subjectId?: string | number | null,
  explicitColor?: string | null,
): string {
  const raw = explicitColor?.trim();
  if (raw && raw !== "var(--primary)") return raw;
  const key = String(subjectId ?? "");
  if (!key) return SUBJECT_PALETTE[0];
  let hash = 0;
  for (let i = 0; i < key.length; i++) {
    hash = (hash * 31 + key.charCodeAt(i)) >>> 0;
  }
  return SUBJECT_PALETTE[hash % SUBJECT_PALETTE.length]!;
}

export interface EnrichedTimeSlot {
  id: string;
  period_id?: string;
  day_of_week?: number;
  start_time: string;
  end_time: string;
  slot_index?: number;
  slot_number?: number;
  period_number?: number;
  name?: string;
  name_ar?: string;
  is_break: boolean;
  break_type?: string;
  duration_minutes: number;
  display_name: string;
  display_name_ar: string;
  display_name_en: string;
  [key: string]: any;
}

export function enrichTimeSlots(
  rawSlots: any[] = [],
  locale = "ar",
): EnrichedTimeSlot[] {
  const isArabic = locale === "ar";
  if (!Array.isArray(rawSlots) || rawSlots.length === 0) return [];

  const sorted = [...rawSlots].sort((a, b) => {
    if (a.start_time && b.start_time) {
      return a.start_time.localeCompare(b.start_time);
    }
    return (
      (Number(a.slot_index ?? a.slot_number) || 0) -
      (Number(b.slot_index ?? b.slot_number) || 0)
    );
  });

  let periodCounter = 0;

  return sorted.map((slot) => {
    const duration =
      slot.duration_minutes ??
      getDurationMinutes(slot.start_time, slot.end_time);

    const isExplicitBreak =
      slot.is_break === true ||
      slot.isBreak === true ||
      slot.is_break === 1 ||
      slot.slot_type === "break" ||
      Boolean(slot.break_type);

    const isNameBreak = Boolean(
      (slot.name_ar &&
        (slot.name_ar.includes("استراحة") || slot.name_ar.includes("فسحة"))) ||
        (slot.name &&
          (slot.name.toLowerCase().includes("break") ||
            slot.name.toLowerCase().includes("recess"))),
    );

    const isBreak = isExplicitBreak || isNameBreak;

    let periodNumber = 0;
    if (!isBreak) {
      periodCounter += 1;
      periodNumber = periodCounter;
    }

    const nameAr = isBreak
      ? slot.name_ar ||
        (slot.break_type === "lunch"
          ? "استراحة الغداء"
          : slot.break_type === "prayer"
            ? "استراحة الصلاة"
            : "الفسحة / استراحة")
      : slot.name_ar ||
        ARABIC_PERIOD_NAMES[periodNumber] ||
        `الحصة ${periodNumber}`;

    const nameEn = isBreak
      ? slot.name ||
        (slot.break_type === "lunch"
          ? "Lunch Break"
          : slot.break_type === "prayer"
            ? "Prayer Break"
            : "Recess / Break")
      : slot.name ||
        ENGLISH_PERIOD_NAMES[periodNumber] ||
        `Period ${periodNumber}`;

    return {
      ...slot,
      id: String(slot.id),
      is_break: isBreak,
      duration_minutes: duration,
      period_number: periodNumber,
      name_ar: slot.name_ar || (isBreak ? nameAr : undefined),
      name: slot.name || (isBreak ? nameEn : undefined),
      display_name_ar: nameAr,
      display_name_en: nameEn,
      display_name: isArabic ? nameAr : nameEn,
    };
  });
}
