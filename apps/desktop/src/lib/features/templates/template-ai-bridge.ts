import {
  generateNodeId,
  validateDocument,
  type TemplateDocument,
  type TemplateNode,
} from "@docforge/core";
import * as m from "$lib/paraglide/messages";
import type { EditorStore } from "./stores/editor-store.svelte";

const NODE_TYPES = new Set([
  "Frame",
  "Grid",
  "Text",
  "Image",
  "Shape",
  "Table",
  "Barcode",
  "QRCode",
  "Separator",
  "Spacer",
  "Repeater",
  "Conditional",
  "ComponentRef",
  "PageBreak",
  "Header",
  "Footer",
  "Watermark",
]);
const CONTAINER_TYPES = new Set([
  "Frame",
  "Grid",
  "Repeater",
  "Conditional",
  "Header",
  "Footer",
]);
const MAX_NESTING_DEPTH = 12;
const MAX_TOOL_CONTENT_LENGTH = 2_000_000;

let activeEditor: EditorStore | null = null;

type JsonObject = Record<string, unknown>;

export interface TemplateAiEditorContext extends JsonObject {
  templateName: string;
  documentType: string;
  entityType: string;
  locale: "ar" | "en";
  pageSetup: TemplateDocument["meta"]["pageSetup"];
  variables: Array<{
    path: string;
    type: string;
    label: string;
    custom?: boolean;
  }>;
  assets: Array<{ name: string; type: string }>;
}

function isObject(value: unknown): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function flattenSchema(
  schema: unknown,
  prefix: string,
  result: Array<{ path: string; type: string; label: string }>,
): void {
  if (!isObject(schema)) return;
  const properties = isObject(schema.properties) ? schema.properties : schema;

  for (const [key, value] of Object.entries(properties)) {
    if (!isObject(value)) continue;
    const path = prefix ? `${prefix}.${key}` : key;
    if (value.type === "object" && isObject(value.properties)) {
      flattenSchema(value.properties, path, result);
    } else {
      result.push({
        path,
        type: typeof value.type === "string" ? value.type : "string",
        label: typeof value.label === "string" ? value.label : path,
      });
    }
  }
}

export function registerTemplateAiEditor(store: EditorStore): () => void {
  activeEditor = store;
  return () => {
    if (activeEditor === store) activeEditor = null;
  };
}

export function getTemplateAiEditorContext(): TemplateAiEditorContext | null {
  if (!activeEditor) return null;

  const document = activeEditor.document;
  const variables: TemplateAiEditorContext["variables"] = [];
  for (const [namespace, schema] of Object.entries(
    activeEditor.systemSchemas,
  )) {
    flattenSchema(schema, namespace, variables);
  }
  for (const [path, definition] of Object.entries(document.variables)) {
    variables.push({
      path,
      type: definition.type,
      label: definition.label ?? path,
      custom: true,
    });
  }

  return {
    templateName: document.meta.name,
    documentType: document.meta.documentType,
    entityType: document.meta.entityType,
    locale: document.meta.locale,
    pageSetup: document.meta.pageSetup,
    variables: variables.slice(0, 120),
    assets: Object.entries(document.assets).map(([name, asset]) => ({
      name,
      type: asset.type,
    })),
  };
}

function normalizeNode(value: unknown, depth: number): TemplateNode {
  if (
    !isObject(value) ||
    typeof value.type !== "string" ||
    !NODE_TYPES.has(value.type)
  ) {
    throw new Error(m.template_ai_invalid_design());
  }
  if (depth > MAX_NESTING_DEPTH) {
    throw new Error(m.template_ai_invalid_design());
  }

  if (
    (value.children !== undefined && !Array.isArray(value.children)) ||
    (value.elseChildren !== undefined &&
      (value.type !== "Conditional" || !Array.isArray(value.elseChildren)))
  ) {
    throw new Error(m.template_ai_invalid_design());
  }
  if (value.type !== "PageBreak" && !isObject(value.props)) {
    throw new Error(m.template_ai_invalid_design());
  }
  if (CONTAINER_TYPES.has(value.type) && !Array.isArray(value.children)) {
    throw new Error(m.template_ai_invalid_design());
  }

  const normalized: JsonObject = {
    ...value,
    id: generateNodeId(value.type.toLowerCase()),
  };

  if (Array.isArray(value.children)) {
    normalized.children = value.children.map((child) =>
      normalizeNode(child, depth + 1),
    );
  }
  if (Array.isArray(value.elseChildren)) {
    normalized.elseChildren = value.elseChildren.map((child) =>
      normalizeNode(child, depth + 1),
    );
  }
  return normalized as unknown as TemplateNode;
}

function assertAvailableAssets(
  node: TemplateNode,
  assets: TemplateDocument["assets"],
): void {
  if (node.type === "Image") {
    if (node.props.assetRef && !assets[node.props.assetRef]) {
      throw new Error(m.template_ai_invalid_design());
    }
    if (node.props.src) {
      throw new Error(m.template_ai_invalid_design());
    }
  }

  if ("children" in node) {
    for (const child of node.children) assertAvailableAssets(child, assets);
  }
  if (node.type === "Conditional" && node.elseChildren) {
    for (const child of node.elseChildren) assertAvailableAssets(child, assets);
  }
}

export function applyTemplateAiDesign(toolContent: string): string {
  const store = activeEditor;
  if (!store) {
    throw new Error(m.template_ai_no_active_editor());
  }
  if (toolContent.length > MAX_TOOL_CONTENT_LENGTH) {
    throw new Error(m.template_ai_invalid_design());
  }

  let result: unknown;
  try {
    result = JSON.parse(toolContent);
  } catch {
    throw new Error(m.template_ai_invalid_design());
  }

  if (!isObject(result) || !isObject(result.document)) {
    throw new Error(m.template_ai_invalid_design());
  }

  const generated = result.document;
  if (
    !isObject(generated.meta) ||
    !Array.isArray(generated.pages) ||
    generated.pages.length === 0 ||
    (generated.meta.pageSetup !== undefined &&
      !isObject(generated.meta.pageSetup)) ||
    (generated.variables !== undefined && !isObject(generated.variables))
  ) {
    throw new Error(m.template_ai_invalid_design());
  }
  for (const field of ["name", "documentType", "entityType"] as const) {
    if (
      generated.meta[field] !== undefined &&
      typeof generated.meta[field] !== "string"
    ) {
      throw new Error(m.template_ai_invalid_design());
    }
  }
  if (
    generated.meta.locale !== undefined &&
    generated.meta.locale !== "ar" &&
    generated.meta.locale !== "en"
  ) {
    throw new Error(m.template_ai_invalid_design());
  }

  const current = store.document;
  const assets = current.assets;
  const pages = generated.pages.map((page) => {
    if (
      !isObject(page) ||
      page.type !== "Page" ||
      !Array.isArray(page.children)
    ) {
      throw new Error(m.template_ai_invalid_design());
    }
    return {
      ...page,
      type: "Page" as const,
      id: generateNodeId("page"),
      children: page.children.map((node) => normalizeNode(node, 0)),
    };
  });
  for (const page of pages) {
    for (const node of page.children) assertAvailableAssets(node, assets);
  }

  const generatedPageSetup = isObject(generated.meta.pageSetup)
    ? generated.meta.pageSetup
    : {};
  const generatedMargins = isObject(generatedPageSetup.margins)
    ? generatedPageSetup.margins
    : {};
  const pageSetup = {
    ...current.meta.pageSetup,
    ...generatedPageSetup,
    margins: {
      ...current.meta.pageSetup.margins,
      ...generatedMargins,
    },
  };
  const pageSizes = new Set([
    "A4",
    "A3",
    "A5",
    "Letter",
    "Legal",
    "ID_CARD",
    "BADGE",
    "Custom",
  ]);
  if (
    typeof pageSetup.size !== "string" ||
    !pageSizes.has(pageSetup.size) ||
    (pageSetup.orientation !== "portrait" &&
      pageSetup.orientation !== "landscape") ||
    pageSetup.unit !== "mm" ||
    !["top", "bottom", "left", "right"].every(
      (side) =>
        typeof pageSetup.margins[side as keyof typeof pageSetup.margins] ===
          "number" &&
        Number.isFinite(
          pageSetup.margins[side as keyof typeof pageSetup.margins],
        ) &&
        pageSetup.margins[side as keyof typeof pageSetup.margins] >= 0,
    ) ||
    (pageSetup.width !== undefined &&
      (typeof pageSetup.width !== "number" ||
        !Number.isFinite(pageSetup.width) ||
        pageSetup.width <= 0)) ||
    (pageSetup.height !== undefined &&
      (typeof pageSetup.height !== "number" ||
        !Number.isFinite(pageSetup.height) ||
        pageSetup.height <= 0)) ||
    (pageSetup.size === "Custom" &&
      (typeof pageSetup.width !== "number" ||
        typeof pageSetup.height !== "number"))
  ) {
    throw new Error(m.template_ai_invalid_design());
  }

  const generatedVariables = isObject(generated.variables)
    ? generated.variables
    : {};
  const variableTypes = new Set([
    "string",
    "number",
    "date",
    "boolean",
    "image",
    "array",
  ]);
  for (const definition of Object.values(generatedVariables)) {
    if (
      !isObject(definition) ||
      typeof definition.source !== "string" ||
      typeof definition.type !== "string" ||
      !variableTypes.has(definition.type) ||
      !definition.source.trim() ||
      (definition.label !== undefined &&
        typeof definition.label !== "string") ||
      (definition.labelAr !== undefined &&
        typeof definition.labelAr !== "string") ||
      (definition.format !== undefined &&
        typeof definition.format !== "string") ||
      (definition.expression !== undefined &&
        typeof definition.expression !== "string") ||
      (definition.defaultValue !== undefined &&
        typeof definition.defaultValue !== "string")
    ) {
      throw new Error(m.template_ai_invalid_design());
    }
  }

  const meta = {
    ...current.meta,
    ...generated.meta,
    id: current.meta.id,
    pageSetup,
  };
  const document = {
    ...current,
    ...generated,
    version: "2.0",
    meta,
    variables: {
      ...current.variables,
      ...generatedVariables,
    },
    assets,
    pages,
    components: current.components ?? {},
    styles: current.styles ?? {},
  } as TemplateDocument;

  const validation = validateDocument(document);
  if (!validation.isValid) {
    throw new Error(m.template_ai_invalid_design());
  }

  store.replaceDocument(document);
  return document.meta.name;
}
