export function buildInstallmentDates(anchor: string, count: number, entered: readonly string[]) {
  const base = new Date(anchor + "T00:00:00Z");
  const day = base.getUTCDate();
  if (!Number.isFinite(day)) return [];

  return Array.from({ length: count }, (_, index) => {
    if (entered[index]) return entered[index];
    const month = new Date(base.getTime());
    month.setUTCDate(1);
    month.setUTCMonth(base.getUTCMonth() + index + 1);
    const lastDay = new Date(Date.UTC(month.getUTCFullYear(), month.getUTCMonth() + 1, 0)).getUTCDate();
    month.setUTCDate(Math.min(day, lastDay));
    return month.toISOString().slice(0, 10);
  });
}

export function hasOrderedInstallmentDates(dates: readonly string[], count: number) {
  return dates.length === count && dates.every((date, index) => {
    if (!/^\d{4}-\d{2}-\d{2}$/.test(date)) return false;
    const parsed = new Date(date + "T00:00:00Z");
    return Number.isFinite(parsed.getTime())
      && parsed.toISOString().slice(0, 10) === date
      && (index === 0 || date > dates[index - 1]);
  });
}
