export function formatLicensePlan(plan: string | null | undefined): string | null {
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
