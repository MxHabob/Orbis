import type { TimeSlotDto } from "$lib/api";
import { isAr } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import {
  ARABIC_PERIOD_NAMES,
  ENGLISH_PERIOD_NAMES,
} from "@orbis/ui/timetable";

export interface BreakRule {
  id: number;
  afterPeriod: number;
  duration: number;
  type: "recess" | "lunch" | "prayer" | "morning";
}

export interface PreviewSlot {
  key: string;
  slotIndex: number;
  nameAr: string;
  nameEn: string;
  startTime: string;
  endTime: string;
  isBreak: boolean;
  breakType: string | null;
}

export interface SlotGroup {
  key: string;
  slotIndex: number;
  nameAr: string;
  nameEn: string;
  startTime: string;
  endTime: string;
  isBreak: boolean;
  slots: TimeSlotDto[];
}

export function parseTimeToMinutes(hhmm: string): number {
  const [hours, minutes] = hhmm.split(":").map(Number);
  if (Number.isNaN(hours) || Number.isNaN(minutes)) return 0;
  return hours * 60 + minutes;
}

export function formatMinutesToTime(totalMinutes: number): string {
  const clamped = Math.max(0, Math.min(23 * 60 + 59, totalMinutes));
  const hours = Math.floor(clamped / 60);
  const minutes = clamped % 60;
  return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}`;
}

export function getPeriodNames(orderNumber: number): { ar: string; en: string } {
  return {
    ar:
      ARABIC_PERIOD_NAMES[orderNumber] ??
      m.academics_schedule_placeholder_period({ number: orderNumber }),
    en:
      ENGLISH_PERIOD_NAMES[orderNumber] ??
      m.academics_schedule_placeholder_period_en({ number: orderNumber }),
  };
}

export function getBreakLabel(type: string): string {
  switch (type) {
    case "lunch":
      return m.common_onboarding_break_lunch();
    case "prayer":
      return m.common_onboarding_break_prayer();
    case "morning":
      return m.academics_schedule_morning_assembly();
    default:
      return m.common_onboarding_break_recess();
  }
}

export function getBreakNames(type: string): { ar: string; en: string } {
  switch (type) {
    case "lunch":
      return {
        ar: m.academics_schedule_break_lunch({}, { locale: "ar" }),
        en: m.academics_schedule_break_lunch({}, { locale: "en" }),
      };
    case "prayer":
      return {
        ar: m.academics_schedule_break_prayer({}, { locale: "ar" }),
        en: m.academics_schedule_break_prayer({}, { locale: "en" }),
      };
    case "morning":
      return {
        ar: m.academics_schedule_break_morning_assembly({}, { locale: "ar" }),
        en: m.academics_schedule_break_morning_assembly({}, { locale: "en" }),
      };
    default:
      return {
        ar: m.academics_schedule_break_recess({}, { locale: "ar" }),
        en: m.academics_schedule_break_recess({}, { locale: "en" }),
      };
  }
}

export function getDefaultStudyDays(
  timeSlots: TimeSlotDto[],
  weekdayOrder: Map<number, number>,
): number[] {
  const existing = [
    ...new Set(timeSlots.map((slot) => Number(slot.day_of_week))),
  ].filter((day) => day >= 1 && day <= 7);
  if (existing.length > 0) {
    return existing.sort(
      (a, b) => (weekdayOrder.get(a) ?? 99) - (weekdayOrder.get(b) ?? 99),
    );
  }
  return isAr() ? [7, 1, 2, 3, 4] : [1, 2, 3, 4, 5];
}

export function buildGeneratedSlots(params: {
  startTime: string;
  periodsCount: number;
  periodDuration: number;
  gapMinutes: number;
  breaks: BreakRule[];
  overrides: Record<
    string,
    Partial<Pick<PreviewSlot, "nameAr" | "startTime" | "endTime">>
  >;
}): PreviewSlot[] {
  const count = Math.max(1, Math.min(12, Math.floor(params.periodsCount || 1)));
  const periodLen = Math.max(
    5,
    Math.min(240, Math.floor(params.periodDuration || 45)),
  );
  const gap = Math.max(0, Math.min(60, Math.floor(params.gapMinutes || 0)));
  let cursor = parseTimeToMinutes(params.startTime || "08:00");
  let slotCounter = 1;
  const rows: PreviewSlot[] = [];

  for (let periodNum = 1; periodNum <= count; periodNum += 1) {
    const names = getPeriodNames(periodNum);
    const key = `p-${periodNum}`;
    const override = params.overrides[key];
    const start = override?.startTime ?? formatMinutesToTime(cursor);
    const end =
      override?.endTime ??
      formatMinutesToTime(parseTimeToMinutes(start) + periodLen);
    rows.push({
      key,
      slotIndex: slotCounter++,
      nameAr: override?.nameAr ?? names.ar,
      nameEn: names.en,
      startTime: start,
      endTime: end,
      isBreak: false,
      breakType: null,
    });
    cursor = parseTimeToMinutes(end);

    const matchingBreaks = params.breaks.filter(
      (item) => Number(item.afterPeriod) === periodNum && item.duration > 0,
    );
    for (const br of matchingBreaks) {
      const brKey = `b-${br.id}-${periodNum}`;
      const brOverride = params.overrides[brKey];
      const brNames = getBreakNames(br.type);
      const brStart = brOverride?.startTime ?? formatMinutesToTime(cursor);
      const brEnd =
        brOverride?.endTime ??
        formatMinutesToTime(
          parseTimeToMinutes(brStart) + Math.max(5, Number(br.duration) || 15),
        );
      rows.push({
        key: brKey,
        slotIndex: slotCounter++,
        nameAr: brOverride?.nameAr ?? brNames.ar,
        nameEn: brNames.en,
        startTime: brStart,
        endTime: brEnd,
        isBreak: true,
        breakType: br.type,
      });
      cursor = parseTimeToMinutes(brEnd);
    }

    if (matchingBreaks.length === 0 && periodNum < count && gap > 0) {
      cursor += gap;
    }
  }
  return rows;
}

export function groupSlotsByPeriod(sortedSlots: TimeSlotDto[]): SlotGroup[] {
  const groups = new Map<string, SlotGroup>();
  for (const slot of sortedSlots) {
    const start = slot.start_time.slice(0, 5);
    const end = slot.end_time.slice(0, 5);
    const key = `${slot.slot_index}|${start}|${end}|${slot.is_break ? "1" : "0"}|${slot.name_ar ?? slot.name ?? ""}`;
    const existing = groups.get(key);
    if (existing) {
      existing.slots.push(slot);
    } else {
      groups.set(key, {
        key,
        slotIndex: slot.slot_index,
        nameAr: slot.name_ar ?? slot.name ?? "",
        nameEn: slot.name ?? slot.name_ar ?? "",
        startTime: start,
        endTime: end,
        isBreak: slot.is_break,
        slots: [slot],
      });
    }
  }
  return [...groups.values()].sort(
    (a, b) =>
      a.slotIndex - b.slotIndex || a.startTime.localeCompare(b.startTime),
  );
}
