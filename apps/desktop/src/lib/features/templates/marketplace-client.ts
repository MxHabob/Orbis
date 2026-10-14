import type { CreateTemplateRequest } from "$lib/api/types";

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
  document: Record<string, unknown>;
  preview_url?: string | null;
}

interface ReadyTemplateCatalog {
  version: 1;
  templates: ReadyTemplate[];
}

function getCatalogUrl(): string {
  const url = import.meta.env.VITE_ORBIS_TEMPLATE_CATALOG_URL?.trim();
  if (!url) {
    throw new Error("The ready-template catalog URL is not configured.");
  }

  const parsed = new URL(url);
  if (parsed.protocol !== "https:" && parsed.hostname !== "localhost") {
    throw new Error("The ready-template catalog must use HTTPS.");
  }
  return parsed.href;
}

function isReadyTemplate(value: unknown): value is ReadyTemplate {
  if (value === null || typeof value !== "object") return false;
  const template = value as Partial<ReadyTemplate>;
  return (
    typeof template.id === "string" &&
    typeof template.name === "string" &&
    typeof template.document_type === "string" &&
    typeof template.entity_type === "string" &&
    typeof template.page_size === "string" &&
    typeof template.orientation === "string" &&
    template.document !== null &&
    typeof template.document === "object" &&
    !Array.isArray(template.document)
  );
}

export async function listReadyTemplates(
  search = "",
): Promise<ReadyTemplate[]> {
  const response = await fetch(getCatalogUrl(), {
    headers: { Accept: "application/json" },
    cache: "no-cache",
  });
  if (!response.ok) {
    throw new Error(`Could not load the ready-template catalog (${response.status}).`);
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

  const query = search.trim().toLocaleLowerCase();
  return catalog.templates.filter((template) => {
    if (!query) return true;
    return (
      template.name.toLocaleLowerCase().includes(query) ||
      template.name_ar?.toLocaleLowerCase().includes(query) === true ||
      template.description?.toLocaleLowerCase().includes(query) === true ||
      template.description_ar?.toLocaleLowerCase().includes(query) === true
    );
  });
}

export function toCreateTemplateRequest(
  template: ReadyTemplate,
): CreateTemplateRequest {
  const documentPageSetup = template.document.meta as
    | { pageSetup?: { margins?: Record<string, number> } }
    | undefined;

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
      JSON.stringify(
        documentPageSetup?.pageSetup?.margins ?? {
          top: 10,
          bottom: 10,
          left: 10,
          right: 10,
        },
      ),
    document_json: JSON.stringify(template.document),
    locale: template.locale ?? "ar",
    tags: template.tags ?? JSON.stringify(["ready-template", template.id]),
  };
}
