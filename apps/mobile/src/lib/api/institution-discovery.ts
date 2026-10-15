import { setSecureItem, STORAGE_KEYS } from "./storage";

export interface DirectoryInstitution {
  organization_id: string;
  name: string;
  name_ar: string | null;
  slug: string;
  deployment_id: string;
  deployment_name: string;
  endpoint_url: string;
}

const DIRECTORY_BASE_URL =
  process.env.EXPO_PUBLIC_MULVERSE_API_URL || "https://api.mulverse.com";

export async function searchInstitutionDirectory(
  search: string,
): Promise<DirectoryInstitution[]> {
  const query = search.trim();
  if (query.length < 2 || query.length > 100) {
    throw new Error("Search must contain 2 to 100 characters.");
  }

  const baseUrl = new URL(DIRECTORY_BASE_URL);
  const isDevelopmentLoopback =
    __DEV__ &&
    baseUrl.protocol === "http:" &&
    ["localhost", "127.0.0.1", "10.0.2.2"].includes(baseUrl.hostname);
  if (
    (baseUrl.protocol !== "https:" && !isDevelopmentLoopback) ||
    baseUrl.username ||
    baseUrl.password ||
    baseUrl.search ||
    baseUrl.hash
  ) {
    throw new Error("Mulverse directory must use a valid HTTPS URL.");
  }

  baseUrl.pathname = "/api/v1/public/orbis-institutions";
  baseUrl.searchParams.set("search", query);
  const response = await fetch(baseUrl.toString());
  if (!response.ok) {
    throw new Error(`Institution search failed (${response.status}).`);
  }

  const payload: unknown = await response.json();
  if (!Array.isArray(payload)) {
    throw new Error("Mulverse returned an invalid institution list.");
  }

  if (!payload.every(isDirectoryInstitution)) {
    throw new Error("Mulverse returned invalid institution connection details.");
  }
  return payload;
}

export async function saveSelectedInstitution(
  institution: DirectoryInstitution,
): Promise<void> {
  if (!isDirectoryInstitution(institution)) {
    throw new Error("The selected institution has invalid connection details.");
  }
  await setSecureItem(STORAGE_KEYS.API_BASE_URL, institution.endpoint_url);
}

function isDirectoryInstitution(value: unknown): value is DirectoryInstitution {
  if (!value || typeof value !== "object") return false;
  const institution = value as Record<string, unknown>;
  return (
    typeof institution.organization_id === "string" &&
    typeof institution.name === "string" &&
    (typeof institution.name_ar === "string" || institution.name_ar === null) &&
    typeof institution.slug === "string" &&
    typeof institution.deployment_id === "string" &&
    typeof institution.deployment_name === "string" &&
    typeof institution.endpoint_url === "string" &&
    isPublicEndpoint(institution.endpoint_url)
  );
}

function isPublicEndpoint(value: string): boolean {
  try {
    const url = new URL(value);
    return (
      url.protocol === "https:" &&
      !!url.hostname &&
      !url.username &&
      !url.password &&
      !url.search &&
      !url.hash
    );
  } catch {
    return false;
  }
}
