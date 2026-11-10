import { describe, expect, it } from "bun:test";
import {
  createDefaultDateRange,
  getDateRangeForPreset,
} from "../src/lib/shared/components/date-range";

describe("date range presets", () => {
  const now = new Date(2026, 10, 10, 12);

  it("uses the local date for the today preset", () => {
    expect(createDefaultDateRange(now)).toEqual({
      preset: "today",
      fromDate: "2026-11-10",
      toDate: "2026-11-10",
    });
  });

  it("starts the month preset on the first day of this month", () => {
    expect(getDateRangeForPreset("this_month", now)).toEqual({
      preset: "this_month",
      fromDate: "2026-11-01",
      toDate: "2026-11-10",
    });
  });

  it("starts the year-to-date preset on the first day of this year", () => {
    expect(getDateRangeForPreset("year_to_date", now)).toEqual({
      preset: "year_to_date",
      fromDate: "2026-01-01",
      toDate: "2026-11-10",
    });
  });
});
