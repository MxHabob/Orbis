import { format, isValid, parse } from "date-fns";

export * from "./csv";
export * from "./excel";
export * from "./parser";
export * from "./schema";
export * from "./students";
export * from "./utils";

export function formatDate(dateStr: string): string | undefined {
  if (!dateStr) return undefined;

  // Try common formats
  const formats = [
    "yyyy-MM-dd",
    "dd/MM/yyyy",
    "MM/dd/yyyy",
    "yyyy/MM/dd",
    "dd-MM-yyyy",
    "MM-dd-yyyy",
    "d/M/yyyy",
    "M/d/yyyy",
  ];

  for (const fmt of formats) {
    const parsedDate = parse(dateStr, fmt, new Date());
    if (isValid(parsedDate)) {
      return format(parsedDate, "yyyy-MM-dd");
    }
  }

  // Fallback to native Date
  const nativeDate = new Date(dateStr);
  if (isValid(nativeDate)) {
    return format(nativeDate, "yyyy-MM-dd");
  }

  return undefined;
}

export function formatAmountValue({
  amount,
  inverted = false,
}: {
  amount: string;
  inverted?: boolean;
}): number {
  if (!amount) return 0;

  // Remove currency symbols and commas
  const cleanedAmount = amount.replace(/[^\d.-]/g, "");
  const parsedAmount = parseFloat(cleanedAmount);

  if (isNaN(parsedAmount)) return 0;

  return inverted ? -parsedAmount : parsedAmount;
}
