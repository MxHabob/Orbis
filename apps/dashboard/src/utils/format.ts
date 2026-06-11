import { TZDate } from "@date-fns/tz";
import {
  differenceInDays,
  differenceInMonths,
  format,
  startOfDay,
} from "date-fns";
import { normalizeCurrencyCode } from "./currency";

const SIZE_UNITS = [
  "byte",
  "kilobyte",
  "megabyte",
  "gigabyte",
  "terabyte",
  "petabyte",
] as const;

type SizeUnit = (typeof SIZE_UNITS)[number];

const formatterCache = new Map<string, Intl.NumberFormat>();

function getFormatter(unit: SizeUnit, locale = "en-US"): Intl.NumberFormat {
  const key = `${locale}-${unit}`;
  if (!formatterCache.has(key)) {
    formatterCache.set(
      key,
      new Intl.NumberFormat(locale, {
        style: "unit",
        unit,
        unitDisplay: "short",
        maximumFractionDigits: 1,
      }),
    );
  }
  return formatterCache.get(key)!;
}

/**
 * Formats a number of bytes into a human-readable string using Intl.NumberFormat.
 * Follows 2026 performance and reliability standards.
 */
export function formatSize(bytes: number | bigint | undefined | null): string {
  // Handle falsy or non-numeric values early
  if (!bytes || Number.isNaN(Number(bytes)) || bytes < 0) {
    return getFormatter("byte").format(0);
  }

  const b = Number(bytes);
  const exponent = Math.floor(Math.log(b) / Math.log(1024));
  const unitIndex = Math.max(0, Math.min(exponent, SIZE_UNITS.length - 1));
  const unit: SizeUnit = SIZE_UNITS[unitIndex] ?? "byte";
  const value = b / 1024 ** unitIndex;

  try {
    return getFormatter(unit).format(value);
  } catch (error) {
    // Robust fallback for environments with partial Intl support
    const fallbackUnits = ["B", "KB", "MB", "GB", "TB", "PB"];
    return `${value.toFixed(1)} ${fallbackUnits[unitIndex]}`;
  }
}

type FormatAmountParams = {
  currency: string;
  amount: number;
  locale?: string | null;
  maximumFractionDigits?: number;
  minimumFractionDigits?: number;
};

export function formatAmount({
  currency,
  amount,
  locale = "en-US",
  minimumFractionDigits,
  maximumFractionDigits,
}: FormatAmountParams) {
  if (!currency) {
    return;
  }

  // Normalize currency code to ISO 4217 format
  const normalizedCurrency = normalizeCurrencyCode(currency);

  // Fix: locale can be null, but Intl.NumberFormat expects string | string[] | undefined
  // So, if locale is null, pass undefined instead
  const safeLocale = locale ?? undefined;

  try {
    return Intl.NumberFormat(safeLocale, {
      style: "currency",
      currency: normalizedCurrency,
      minimumFractionDigits,
      maximumFractionDigits,
    }).format(amount);
  } catch (error) {
    // Fallback to USD if currency is invalid
    console.warn(
      `Invalid currency code: ${currency} (normalized to ${normalizedCurrency}), falling back to USD`,
      error,
    );
    return Intl.NumberFormat(safeLocale, {
      style: "currency",
      currency: "USD",
      minimumFractionDigits,
      maximumFractionDigits,
    }).format(amount);
  }
}

export function secondsToHoursAndMinutes(seconds: number) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);

  if (hours && minutes) {
    return `${hours}h ${minutes}m`;
  }

  if (hours) {
    return `${hours}h`;
  }

  if (minutes) {
    return `${minutes}m`;
  }

  return "0m";
}

type BurnRateData = {
  value: number;
  date: string;
};

export function calculateAvgBurnRate(data: BurnRateData[] | null) {
  if (!data) {
    return 0;
  }

  return data?.reduce((acc, curr) => acc + curr.value, 0) / data?.length;
}

export function formatAccountName({
  name = "",
  currency,
}: {
  name?: string;
  currency?: string | null;
}) {
  if (currency) {
    return `${name} (${currency})`;
  }

  return name;
}

export function formatDateRange(dates: TZDate[]): string {
  if (!dates.length) return "";

  const formatFullDate = (date: TZDate) => format(date, "MMM d");
  const formatDay = (date: TZDate) => format(date, "d");

  const startDate = dates[0];
  const endDate = dates[1];

  if (!startDate) return "";

  if (
    dates.length === 1 ||
    !endDate ||
    startDate.getTime() === endDate.getTime()
  ) {
    return formatFullDate(startDate);
  }

  if (startDate.getMonth() === endDate.getMonth()) {
    // Same month
    return `${format(startDate, "MMM")} ${formatDay(startDate)} - ${formatDay(endDate)}`;
  }
  // Different months
  return `${formatFullDate(startDate)} - ${formatFullDate(endDate)}`;
}

export function getDueDateStatus(dueDate: string): string {
  // Parse due date as UTC (it's stored as UTC midnight)
  const due = new TZDate(dueDate, "UTC");

  // Get current date in UTC for consistent comparison
  const now = new Date();
  const nowUTC = new TZDate(now.toISOString(), "UTC");

  // Compare at the day level in UTC
  const nowDay = startOfDay(nowUTC);
  const dueDay = startOfDay(due);

  const diffDays = differenceInDays(dueDay, nowDay);
  const diffMonths = differenceInMonths(dueDay, nowDay);

  if (diffDays === 0) return "Today";
  if (diffDays === 1) return "Tomorrow";
  if (diffDays === -1) return "Yesterday";

  if (diffDays > 0) {
    if (diffMonths < 1) return `in ${diffDays} days`;
    return `in ${diffMonths} month${diffMonths === 1 ? "" : "s"}`;
  }

  if (diffMonths < 1)
    return `${Math.abs(diffDays)} day${Math.abs(diffDays) === 1 ? "" : "s"} ago`;
  return `${diffMonths} month${diffMonths === 1 ? "" : "s"} ago`;
}

export function formatRelativeTime(date: Date): string {
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diffInSeconds < 60) {
    return "just now";
  }

  const intervals = [
    { label: "y", seconds: 31536000 },
    { label: "mo", seconds: 2592000 },
    { label: "d", seconds: 86400 },
    { label: "h", seconds: 3600 },
    { label: "m", seconds: 60 },
  ] as const;

  for (const interval of intervals) {
    const count = Math.floor(diffInSeconds / interval.seconds);
    if (count > 0) {
      return `${count}${interval.label} ago`;
    }
  }

  return "just now";
}

export function formatCompactAmount(
  amount: number,
  locale?: string | null,
): string {
  const absAmount = Math.abs(amount);
  const safeLocale = locale ?? "en-US";

  if (absAmount >= 1000000) {
    const formatted = (absAmount / 1000000).toLocaleString(safeLocale, {
      minimumFractionDigits: 1,
      maximumFractionDigits: 1,
    });
    return `${formatted}m`;
  }
  // Always show in thousands notation
  const formatted = (absAmount / 1000).toLocaleString(safeLocale, {
    minimumFractionDigits: 1,
    maximumFractionDigits: 1,
  });
  return `${formatted}k`;
}
