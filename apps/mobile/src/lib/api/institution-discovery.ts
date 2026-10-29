import { getSecureItem, STORAGE_KEYS, setSecureItem } from "./storage";

export interface DirectoryInstitution {
  organization_id: string;
  name: string;
  name_ar: string | null;
  logo_url?: string | null;
  slug: string;
  deployment_id: string;
  deployment_name: string;
  endpoint_url: string;
  latitude?: number | null;
  longitude?: number | null;
  is_local?: boolean;
}

export function parseLocalInstitutionQr(data: string): DirectoryInstitution {
  if (data.length > 4096) {
    throw new Error("The institution QR code is too large.");
  }

  let payload: unknown;
  try {
    payload = JSON.parse(data);
  } catch {
    throw new Error(
      "The QR code does not contain a valid Orbis configuration.",
    );
  }

  if (
    typeof payload !== "object" ||
    payload === null ||
    !("type" in payload) ||
    payload.type !== "orbis-institution" ||
    !("version" in payload) ||
    payload.version !== 1 ||
    !("institution" in payload) ||
    !isDirectoryInstitution(payload.institution)
  ) {
    throw new Error(
      "The QR code does not contain a supported Orbis institution.",
    );
  }

  return { ...payload.institution, is_local: true };
}

export function createManualLocalInstitution(
  name: string,
  endpoint: string,
): DirectoryInstitution {
  const institutionName = name.trim();
  if (institutionName.length < 2 || institutionName.length > 100) {
    throw new Error("Institution name must contain 2 to 100 characters.");
  }

  let url: URL;
  try {
    url = new URL(endpoint.trim());
  } catch {
    throw new Error("Enter a valid local HTTP or HTTPS server address.");
  }

  if (!isAllowedInstitutionEndpoint(url.toString())) {
    throw new Error(
      "Local HTTP is allowed only for private IPv4 addresses; other servers must use HTTPS.",
    );
  }

  const endpointUrl = url.toString().replace(/\/+$/, "");
  const deploymentId = `local:${url.host}${url.pathname.replace(/\/+$/, "")}`;
  const slug = url.hostname.toLowerCase().replace(/[^a-z0-9-]/g, "-");

  return {
    organization_id: deploymentId,
    name: institutionName,
    name_ar: null,
    slug: `local-${slug}`,
    deployment_id: deploymentId,
    deployment_name: url.hostname,
    endpoint_url: endpointUrl,
    is_local: true,
  };
}

const DIRECTORY_BASE_URL =
  process.env.EXPO_PUBLIC_MULVERSE_API_URL || "https://api.mulverse.com";

export async function searchInstitutionDirectory(
  search?: string,
  signal?: AbortSignal,
): Promise<DirectoryInstitution[]> {
  const query = search?.trim() ?? "";
  if (query.length === 1 || query.length > 100) {
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
  if (query) baseUrl.searchParams.set("search", query);
  const response = await fetch(baseUrl.toString(), { signal });
  if (!response.ok) {
    throw new Error(`Institution search failed (${response.status}).`);
  }

  const payload: unknown = await response.json();
  if (!Array.isArray(payload)) {
    throw new Error("Mulverse returned an invalid institution list.");
  }

  if (!payload.every(isDirectoryInstitution)) {
    throw new Error(
      "Mulverse returned invalid institution connection details.",
    );
  }
  return payload;
}

export async function saveSelectedInstitution(
  institution: DirectoryInstitution,
): Promise<DirectoryInstitution[]> {
  if (!isDirectoryInstitution(institution)) {
    throw new Error("The selected institution has invalid connection details.");
  }
  const knownInstitutions = await getSavedInstitutions();
  const nextInstitutions = [
    institution,
    ...knownInstitutions.filter(
      (saved) => saved.deployment_id !== institution.deployment_id,
    ),
  ];
  await Promise.all([
    setSecureItem(STORAGE_KEYS.INSTITUTIONS, JSON.stringify(nextInstitutions)),
    setSecureItem(
      STORAGE_KEYS.ACTIVE_INSTITUTION_ID,
      institution.deployment_id,
    ),
    setSecureItem(STORAGE_KEYS.API_BASE_URL, institution.endpoint_url),
  ]);
  return nextInstitutions;
}

export async function getSavedInstitutions(): Promise<DirectoryInstitution[]> {
  const stored = await getSecureItem(STORAGE_KEYS.INSTITUTIONS);
  if (!stored) return [];

  const value: unknown = JSON.parse(stored);
  if (!Array.isArray(value) || !value.every(isDirectoryInstitution)) {
    throw new Error("Saved institution connections have an invalid format.");
  }
  return value;
}

export async function checkInstitutionConnection(
  institution: DirectoryInstitution,
): Promise<void> {
  if (!isDirectoryInstitution(institution)) {
    throw new Error("The selected institution has invalid connection details.");
  }
  const healthUrl = new URL(institution.endpoint_url);
  healthUrl.pathname = `${healthUrl.pathname.replace(/\/+$/, "")}/api/v1/health`;
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 8_000);

  try {
    const response = await fetch(healthUrl.toString(), {
      method: "GET",
      headers: { Accept: "application/json" },
      signal: controller.signal,
    });
    if (!response.ok) {
      throw new Error(`Institution server check failed (${response.status}).`);
    }

    const payload: unknown = await response.json();
    if (
      typeof payload !== "object" ||
      payload === null ||
      !("status" in payload) ||
      payload.status !== "ok" ||
      !("database_connected" in payload) ||
      payload.database_connected !== true
    ) {
      throw new Error("The selected institution is not ready for sign-in.");
    }
  } finally {
    clearTimeout(timeout);
  }
}

function isDirectoryInstitution(value: unknown): value is DirectoryInstitution {
  if (!value || typeof value !== "object") return false;
  const institution = value as Record<string, unknown>;
  const hasValidCoordinate =
    institution.latitude === undefined ||
    institution.latitude === null ||
    (typeof institution.latitude === "number" &&
      Number.isFinite(institution.latitude) &&
      institution.latitude >= -90 &&
      institution.latitude <= 90);
  const hasValidLongitude =
    institution.longitude === undefined ||
    institution.longitude === null ||
    (typeof institution.longitude === "number" &&
      Number.isFinite(institution.longitude) &&
      institution.longitude >= -180 &&
      institution.longitude <= 180);

  return (
    typeof institution.organization_id === "string" &&
    typeof institution.name === "string" &&
    (typeof institution.name_ar === "string" || institution.name_ar === null) &&
    (institution.logo_url === undefined ||
      institution.logo_url === null ||
      (typeof institution.logo_url === "string" &&
        isPublicImageUrl(institution.logo_url))) &&
    typeof institution.slug === "string" &&
    typeof institution.deployment_id === "string" &&
    typeof institution.deployment_name === "string" &&
    typeof institution.endpoint_url === "string" &&
    isAllowedInstitutionEndpoint(institution.endpoint_url) &&
    (institution.is_local === undefined ||
      typeof institution.is_local === "boolean") &&
    hasValidCoordinate &&
    hasValidLongitude &&
    !(
      institution.latitude !== undefined &&
      institution.latitude !== null &&
      (institution.longitude === undefined || institution.longitude === null)
    ) &&
    !(
      institution.longitude !== undefined &&
      institution.longitude !== null &&
      (institution.latitude === undefined || institution.latitude === null)
    )
  );
}

function isPublicImageUrl(value: string): boolean {
  try {
    const url = new URL(value);
    return (
      url.protocol === "https:" &&
      !!url.hostname &&
      !url.username &&
      !url.password
    );
  } catch {
    return false;
  }
}

export function isAllowedInstitutionEndpoint(value: string): boolean {
  try {
    const url = new URL(value);
    const hasSafeComponents =
      !!url.hostname &&
      !url.username &&
      !url.password &&
      !url.search &&
      !url.hash;

    return (
      hasSafeComponents &&
      (url.protocol === "https:" ||
        (url.protocol === "http:" && isPrivateIpv4Address(url.hostname)))
    );
  } catch {
    return false;
  }
}

function isPrivateIpv4Address(hostname: string): boolean {
  const octets = hostname.split(".");
  if (
    octets.length !== 4 ||
    !octets.every((octet) => /^\d{1,3}$/.test(octet))
  ) {
    return false;
  }

  const [first, second, third, fourth] = octets.map(Number);
  if ([first, second, third, fourth].some((octet) => octet > 255)) {
    return false;
  }

  return (
    first === 10 ||
    (first === 172 && second >= 16 && second <= 31) ||
    (first === 192 && second === 168)
  );
}
