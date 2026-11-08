export function formatLicensePlan(
  plan: string | null | undefined,
): string | null {
  const value = plan?.trim();
  if (!value) return null;

  return value
    .split(/[_\s-]+/)
    .filter(Boolean)
    .map((part) =>
      part.length <= 3
        ? part.toUpperCase()
        : part.charAt(0).toUpperCase() + part.slice(1).toLowerCase(),
    )
    .join(" ");
}

export function normalizeLicenseProductKey(value: string): string | null {
  let clean = value.replace(/[^A-Za-z0-9]/g, "").toUpperCase();
  if (clean.startsWith("ORBIS")) {
    clean = clean.slice(5);
  }
  if (clean.length !== 20) return null;

  const segments = clean.match(/.{5}/g);
  if (!segments || segments.length !== 4) return null;
  return `ORBIS-${segments.join("-")}`;
}
