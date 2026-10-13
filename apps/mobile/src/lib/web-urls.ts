export function normalizeWebUrl(value: unknown): string | null {
  if (typeof value !== "string" || !value.trim()) return null;

  const candidate = value.trim();
  const withProtocol = /^[a-z][a-z\d+.-]*:/i.test(candidate)
    ? candidate
    : `https://${candidate}`;

  try {
    const url = new URL(withProtocol);
    if (
      (url.protocol !== "https:" && url.protocol !== "http:") ||
      !url.hostname ||
      url.username ||
      url.password
    ) {
      return null;
    }
    return url.toString();
  } catch {
    return null;
  }
}

export function getOrganizationWebsite(organization: unknown): string | null {
  if (typeof organization !== "object" || organization === null) return null;

  const record = organization as Record<string, unknown>;
  const metadata =
    typeof record.metadata === "object" && record.metadata !== null
      ? (record.metadata as Record<string, unknown>)
      : null;

  return normalizeWebUrl(record.website ?? metadata?.website);
}
