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

function toLocalDateInputValue(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

export function getDateRangeForPreset(
  preset: ResolvedDateRangePreset,
  now = new Date(),
): DateRangeFilterValue {
  const today = toLocalDateInputValue(now);

  if (preset === "today") {
    return { preset, fromDate: today, toDate: today };
  }

  if (preset === "this_month") {
    const firstOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
    return {
      preset,
      fromDate: toLocalDateInputValue(firstOfMonth),
      toDate: today,
    };
  }

  const firstOfYear = new Date(now.getFullYear(), 0, 1);
  return {
    preset,
    fromDate: toLocalDateInputValue(firstOfYear),
    toDate: today,
  };
}

export function createDefaultDateRange(now = new Date()): DateRangeFilterValue {
  return getDateRangeForPreset("today", now);
}
