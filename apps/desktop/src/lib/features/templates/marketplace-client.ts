import type { CreateTemplateRequest } from "$lib/api/types";
import { deserializeDocument, serializeDocument } from "@docforge/core";

export interface ReadyTemplate {
  id: string;
  name: string;
  name_ar?: string | null;
  description?: string | null;
  description_ar?: string | null;
  document_type: string;
  entity_type: string;
  page_size: string;
  page_width_mm?: number | null;
  page_height_mm?: number | null;
  orientation: string;
  margins_json?: string | null;
  locale?: string | null;
  tags?: string | null;
  document_url: string;
  preview_url?: string | null;
}

function parsePublicUrl(value: string): URL {
  const parsed = new URL(value);
  const isLocalHttp =
    parsed.protocol === "http:" &&
    ["localhost", "127.0.0.1", "[::1]"].includes(parsed.hostname);
  if (parsed.protocol !== "https:" && !isLocalHttp) {
    throw new Error("Ready-template files must use HTTPS.");
  }
  return parsed;
}

function getCatalogUrl(): string {
  const url = import.meta.env.VITE_ORBIS_TEMPLATE_CATALOG_URL?.trim();
  if (!url) {
    throw new Error("The ready-template catalog URL is not configured.");
  }

  return parsePublicUrl(url).href;
}

export function isNetworkUnavailable(error: unknown): boolean {
  if (typeof navigator !== "undefined" && !navigator.onLine) return true;
  if (!(error instanceof TypeError)) return false;

  return /failed to fetch|networkerror|load failed|fetch failed/i.test(
    error.message,
  );
}

function isOptionalString(value: unknown): boolean {
  return value === undefined || value === null || typeof value === "string";
}

function isOptionalNumber(value: unknown): boolean {
  return (
    value === undefined ||
    value === null ||
    (typeof value === "number" && Number.isFinite(value))
  );
}

function isReadyTemplate(value: unknown): value is ReadyTemplate {
  if (value === null || typeof value !== "object") return false;
  const template = value as Partial<ReadyTemplate>;
  return (
    typeof template.id === "string" &&
    template.id.length > 0 &&
    typeof template.name === "string" &&
    typeof template.document_type === "string" &&
    typeof template.entity_type === "string" &&
    typeof template.page_size === "string" &&
    typeof template.orientation === "string" &&
    typeof template.document_url === "string" &&
    isOptionalString(template.name_ar) &&
    isOptionalString(template.description) &&
    isOptionalString(template.description_ar) &&
    isOptionalString(template.margins_json) &&
    isOptionalString(template.locale) &&
    isOptionalString(template.tags) &&
    isOptionalString(template.preview_url) &&
    isOptionalNumber(template.page_width_mm) &&
    isOptionalNumber(template.page_height_mm)
  );
}

function isDocument(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export async function listReadyTemplates(): Promise<ReadyTemplate[]> {
  const response = await fetch(getCatalogUrl(), {
    headers: { Accept: "application/json" },
    cache: "no-cache",
  });
  if (!response.ok) {
    throw new Error(
      `Could not load the ready-template catalog (${response.status}).`,
    );
  }

  const catalog: unknown = await response.json();
  if (
    catalog === null ||
    typeof catalog !== "object" ||
    !("version" in catalog) ||
    catalog.version !== 1 ||
    !("templates" in catalog) ||
    !Array.isArray(catalog.templates) ||
    !catalog.templates.every(isReadyTemplate)
  ) {
    throw new Error("The ready-template catalog has an invalid format.");
  }

  return catalog.templates;
}

export async function createTemplateFromCatalog(
  template: ReadyTemplate,
): Promise<CreateTemplateRequest> {
  const documentUrl = parsePublicUrl(template.document_url).href;
  const response = await fetch(documentUrl, {
    headers: { Accept: "application/json" },
    cache: "force-cache",
  });
  if (!response.ok) {
    throw new Error(
      `Could not download the selected template (${response.status}).`,
    );
  }

  const document: unknown = await response.json();
  if (!isDocument(document)) {
    throw new Error("The selected template file has an invalid format.");
  }

  const normalizedDocument = deserializeDocument(JSON.stringify(document));

  return {
    code: `ready:${template.id}`,
    name: template.name,
    name_ar: template.name_ar,
    description: template.description,
    description_ar: template.description_ar,
    document_type: template.document_type,
    entity_type: template.entity_type,
    page_size: template.page_size,
    page_width_mm: template.page_width_mm,
    page_height_mm: template.page_height_mm,
    orientation: template.orientation,
    margins_json:
      template.margins_json ??
      JSON.stringify(normalizedDocument.meta.pageSetup.margins),
    document_json: serializeDocument(normalizedDocument),
    locale: template.locale ?? "ar",
    tags: template.tags ?? JSON.stringify(["ready-template", template.id]),
  };
}
