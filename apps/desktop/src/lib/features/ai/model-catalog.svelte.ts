import { appDataDir, join } from "@tauri-apps/api/path";
import { exists, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

const CATALOG_URL = "https://models.dev/api.json";
const CACHE_FILE = "model-catalog.json";
const PROVIDER_SOURCES: Record<string, string> = {
  anthropic: "anthropic",
  deepseek: "deepseek",
  "gemini-ai": "google",
  groq: "groq",
  mistral: "mistral",
  openai: "openai",
  openrouter: "openrouter",
};

export type ModelOption = { label: string; value: string };

interface ModelCatalog {
  source: string;
  updatedAt: string;
  providers: Record<string, ModelOption[]>;
}

interface UpstreamModel {
  id?: unknown;
  name?: unknown;
  tool_call?: unknown;
  status?: unknown;
  modalities?: {
    output?: unknown;
  };
}

export const modelCatalog = $state<ModelCatalog>({
  source: CATALOG_URL,
  updatedAt: "",
  providers: {},
});

let loadPromise: Promise<void> | undefined;

function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function parseCache(value: unknown): ModelCatalog {
  if (!isRecord(value) || value.source !== CATALOG_URL) {
    throw new Error("The saved model catalog has an unsupported format.");
  }
  if (typeof value.updatedAt !== "string" || !isRecord(value.providers)) {
    throw new Error("The saved model catalog is incomplete.");
  }

  const providers: Record<string, ModelOption[]> = {};
  for (const appId of Object.keys(PROVIDER_SOURCES)) {
    const options = value.providers[appId];
    if (options === undefined) continue;
    if (!Array.isArray(options)) {
      throw new Error(`The saved model catalog is invalid for ${appId}.`);
    }
    const parsedOptions: ModelOption[] = [];
    for (const option of options) {
      if (
        !isRecord(option) ||
        typeof option.label !== "string" ||
        typeof option.value !== "string"
      ) {
        throw new Error(`The saved model catalog is invalid for ${appId}.`);
      }
      parsedOptions.push({ label: option.label, value: option.value });
    }
    providers[appId] = parsedOptions;
  }

  return { source: CATALOG_URL, updatedAt: value.updatedAt, providers };
}

function buildCatalog(payload: unknown): ModelCatalog {
  if (!isRecord(payload)) {
    throw new Error("The model catalog response is not an object.");
  }

  const providers: Record<string, ModelOption[]> = {};
  for (const [appId, sourceId] of Object.entries(PROVIDER_SOURCES)) {
    const sourceProvider = payload[sourceId];
    if (!isRecord(sourceProvider) || !isRecord(sourceProvider.models)) {
      continue;
    }

    const options: ModelOption[] = [];
    for (const [modelKey, rawModel] of Object.entries(sourceProvider.models)) {
      if (!isRecord(rawModel)) continue;
      const model = rawModel as UpstreamModel;
      const modelId = typeof model.id === "string" ? model.id : modelKey;
      const modelName = typeof model.name === "string" ? model.name.trim() : "";
      const outputModalities = Array.isArray(model.modalities?.output)
        ? model.modalities.output
        : [];

      if (
        !modelId.trim() ||
        model.tool_call !== true ||
        model.status === "deprecated" ||
        !outputModalities.includes("text")
      ) {
        continue;
      }

      options.push({
        label: modelName ? `${modelName} (${modelId})` : modelId,
        value: modelId,
      });
    }

    if (options.length > 0) {
      options.sort((a, b) => a.label.localeCompare(b.label));
      providers[appId] = options;
    }
  }

  if (Object.keys(providers).length === 0) {
    throw new Error("The model catalog did not contain any supported models.");
  }

  return {
    source: CATALOG_URL,
    updatedAt: new Date().toISOString(),
    providers,
  };
}

async function cachePath(): Promise<string> {
  return join(await appDataDir(), CACHE_FILE);
}

async function readCachedCatalog(): Promise<string | null> {
  if (isTauri()) {
    const path = await cachePath();
    return (await exists(path)) ? readTextFile(path) : null;
  }
  if (typeof localStorage !== "undefined") {
    return localStorage.getItem(CACHE_FILE);
  }
  return null;
}

async function writeCachedCatalog(contents: string): Promise<void> {
  if (isTauri()) {
    await writeTextFile(await cachePath(), contents);
    return;
  }
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(CACHE_FILE, contents);
  }
}

function applyCatalog(catalog: ModelCatalog): void {
  modelCatalog.source = catalog.source;
  modelCatalog.updatedAt = catalog.updatedAt;
  modelCatalog.providers = catalog.providers;
}

export async function loadModelCatalog(): Promise<void> {
  if (!loadPromise) {
    loadPromise = (async () => {
      const cached = await readCachedCatalog();
      if (cached === null) return;
      applyCatalog(parseCache(JSON.parse(cached)));
    })().catch((error: unknown) => {
      loadPromise = undefined;
      throw error;
    });
  }
  await loadPromise;
}

export function getModelOptions(
  appId: string,
  fallback: ModelOption[] = [],
): ModelOption[] {
  const catalogOptions = modelCatalog.providers[appId] ?? [];
  if (catalogOptions.length === 0) return fallback;

  const options = new Map(catalogOptions.map((option) => [option.value, option]));
  for (const option of fallback) {
    if (!options.has(option.value)) options.set(option.value, option);
  }
  return [...options.values()];
}

export async function refreshModelCatalog(): Promise<{
  providerCount: number;
  modelCount: number;
  updatedAt: string;
}> {
  const response = await fetch(CATALOG_URL, {
    headers: { Accept: "application/json" },
    signal: AbortSignal.timeout(20_000),
  });
  if (!response.ok) {
    throw new Error(`Model catalog request failed with HTTP ${response.status}.`);
  }

  const catalog = buildCatalog(await response.json());
  const contents = `${JSON.stringify(catalog, null, 2)}\n`;
  await writeCachedCatalog(contents);
  applyCatalog(catalog);

  return {
    providerCount: Object.keys(catalog.providers).length,
    modelCount: Object.values(catalog.providers).reduce(
      (count, options) => count + options.length,
      0,
    ),
    updatedAt: catalog.updatedAt,
  };
}
