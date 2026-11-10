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

  const firstDay = new Date(year, monthIndex, 1);
  const lastDay = new Date(year, monthIndex + 1, 0);
  const isCurrentMonth =
    year === now.getFullYear() && monthIndex === now.getMonth();

  return {
    preset: "this_month",
    fromDate: toLocalDateInputValue(firstDay),
    toDate: isCurrentMonth
      ? toLocalDateInputValue(now)
      : toLocalDateInputValue(lastDay),
  };
}

export function createDefaultDateRange(now = new Date()): DateRangeFilterValue {
  return getDateRangeForPreset("today", now);
}
