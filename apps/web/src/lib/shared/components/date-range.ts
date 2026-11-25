import { formatDateKey } from "$lib/core/i18n/format";

export type DateRangePreset =
  | "today"
  | "this_month"
  | "year_to_date"
  | "custom";

export interface DateRangeFilterValue {
  preset: DateRangePreset;
  fromDate: string;
  toDate: string;
}

export type ResolvedDateRangePreset = Exclude<DateRangePreset, "custom">;

export function getDateRangeForPreset(
  preset: ResolvedDateRangePreset,
  now = new Date(),
): DateRangeFilterValue {
  const today = formatDateKey(now);

  if (preset === "today") {
    return { preset, fromDate: today, toDate: today };
  }

  if (preset === "this_month") {
    return {
      preset,
      fromDate: `${today.slice(0, 7)}-01`,
      toDate: today,
    };
  }

  return {
    preset,
    fromDate: `${today.slice(0, 4)}-01-01`,
    toDate: today,
  };
}

export function getDateRangeForMonth(
  month: string,
  now = new Date(),
): DateRangeFilterValue {
  const match = /^(\d{4})-(\d{2})$/.exec(month);
  if (!match) return getDateRangeForPreset("this_month", now);

  const year = Number(match[1]);
  const monthIndex = Number(match[2]) - 1;
  if (monthIndex < 0 || monthIndex > 11) {
    return getDateRangeForPreset("this_month", now);
  }

  const today = formatDateKey(now);
  const lastDay = new Date(Date.UTC(year, monthIndex + 1, 0)).getUTCDate();

  return {
    preset: "this_month",
    fromDate: `${month}-01`,
    toDate: today.startsWith(`${month}-`) ? today : `${month}-${lastDay}`,
  };
}

export function createDefaultDateRange(now = new Date()): DateRangeFilterValue {
  return getDateRangeForPreset("today", now);
}
