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

export function getDurationMinutes(startTime?: string, endTime?: string): number {
  if (!startTime || !endTime) return 0;
  const [h1, m1] = startTime.split(":").map((v) => parseInt(v, 10));
  const [h2, m2] = endTime.split(":").map((v) => parseInt(v, 10));
  if (isNaN(h1) || isNaN(m1) || isNaN(h2) || isNaN(m2)) return 0;
  return (h2 * 60 + m2) - (h1 * 60 + m1);
}

export function formatTimeSlotTime(val?: string, isArabic = false): string {
  if (!val) return "";
  const clean = val.slice(0, 5);
  const [hStr, mStr] = clean.split(":");
  let h = parseInt(hStr, 10);
  const m = mStr || "00";
  if (isNaN(h)) return val;
  const period = h >= 12 ? (isArabic ? "م" : "PM") : isArabic ? "ص" : "AM";
  h = h % 12 || 12;
  return `${h}:${m} ${period}`;
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

export function enrichTimeSlots(rawSlots: any[] = [], locale = "ar"): EnrichedTimeSlot[] {
  const isArabic = locale === "ar";
  if (!Array.isArray(rawSlots) || rawSlots.length === 0) return [];

  const sorted = [...rawSlots].sort((a, b) => {
    if (a.start_time && b.start_time) {
      return a.start_time.localeCompare(b.start_time);
    }
    return (Number(a.slot_index ?? a.slot_number) || 0) - (Number(b.slot_index ?? b.slot_number) || 0);
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
      slot.slot_type === "break";

    const isNameBreak = Boolean(
      (slot.name_ar && (slot.name_ar.includes("استراحة") || slot.name_ar.includes("فسحة"))) ||
      (slot.name && (slot.name.toLowerCase().includes("break") || slot.name.toLowerCase().includes("recess")))
    );

    const isDurationBreak = duration > 0 && duration <= 25;

    const isBreak = isExplicitBreak || isNameBreak || isDurationBreak;

    let periodNumber = 0;
    if (!isBreak) {
      periodCounter += 1;
      periodNumber = periodCounter;
    }

    const nameAr = isBreak
      ? (slot.name_ar || (slot.break_type === "lunch" ? "استراحة الغداء" : slot.break_type === "prayer" ? "استراحة الصلاة" : "الفسحة / استراحة"))
      : (slot.name_ar || ARABIC_PERIOD_NAMES[periodNumber] || `الحصة ${periodNumber}`);

    const nameEn = isBreak
      ? (slot.name || (slot.break_type === "lunch" ? "Lunch Break" : slot.break_type === "prayer" ? "Prayer Break" : "Recess / Break"))
      : (slot.name || ENGLISH_PERIOD_NAMES[periodNumber] || `Period ${periodNumber}`);

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
