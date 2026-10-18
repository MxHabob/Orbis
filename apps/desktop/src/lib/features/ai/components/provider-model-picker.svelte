<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuSubContent,
  DropdownMenuPortal,
} from "@orbis/ui/dropdown-menu";
import { Sparkles, ChevronDown, Check, Search, SettingsAlt } from "@orbis/icons";
import { getAppsByCategory } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import { createConnectedAiProvidersQuery, useSwitchActiveAiProvider } from "$lib/features/ai/queries";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import { uiStore } from "$lib/stores";
import {
  ensureModelCatalog,
  getModelOptions,
} from "$lib/features/ai/model-catalog.svelte";

let {
  orgId = "",
  onOpenSettings,
}: {
  orgId?: string;
  onOpenSettings?: () => void;
} = $props();

const aiPreferences = $derived(uiStore.getAiProviderModel(orgId));
const savedProvider = $derived(aiPreferences.provider);
const savedModel = $derived(aiPreferences.model);
let isLoadingModelCatalog = $state(false);
let modelCatalogError = $state(false);
let modelCatalogLoadKey = "";
let modelSearch = $state("");

// Canonical AI apps from @orbis/app-store (single source of truth)
const aiApps = $derived(getAppsByCategory("ai-provider"));

// Query connected AI integrations for the organization
const providersQuery = createConnectedAiProvidersQuery(() => orgId);
const connectedIntegrations = $derived(providersQuery.data ?? []);
const switchProviderMutation = useSwitchActiveAiProvider();

// Helper to find matching OrbisApp from @orbis/app-store
function findAppForProvider(providerNameOrId: string): OrbisApp | undefined {
  const norm = normalizeProviderName(providerNameOrId);
  return aiApps.find(
    (app) => {
      const identifiers = [app.id, app.adapter_name, app.name]
        .filter((identifier): identifier is string => Boolean(identifier))
        .map(normalizeProviderName);
      return identifiers.some(
        (identifier) =>
          identifier === norm ||
          identifier.includes(norm) ||
          norm.includes(identifier),
      );
    },
  );
}

function normalizeProviderName(value: string): string {
  return value.toLowerCase().replace(/[^a-z0-9]/g, "");
}

// Extract model options dynamically from app settings
function getModelsForApp(app: OrbisApp): { label: string; value: string }[] {
  const modelSetting = app.settings?.find((s) => s.id === "model");
  if (modelSetting?.options && modelSetting.options.length > 0) {
    return getModelOptions(app.id, modelSetting.options);
  }
  if (modelSetting?.value) {
    return getModelOptions(app.id, [
      { label: String(modelSetting.value), value: String(modelSetting.value) },
    ]);
  }
  return [];
}

// FILTER: ONLY connected apps in the organization
const connectedAiApps = $derived.by(() => {
  return connectedIntegrations.map((integration) => {
    const app = findAppForProvider(integration.provider_name);
    const resolvedApp = app || {
      id: integration.provider_name,
      name: integration.provider_name,
      name_ar: integration.provider_name,
      category: "ai-provider" as const,
      active: true,
      icon: "Sparkles",
      short_description: "",
      settings: [],
      features: [],
    };
    const models = getModelsForApp(resolvedApp);
    const configuredModel = readConfiguredModel(integration.settings);
    if (configuredModel && !models.some((model) => model.value === configuredModel)) {
      models.push({ label: configuredModel, value: configuredModel });
    }
    return {
      integration,
      app: resolvedApp,
      models,
    };
  });
});

async function loadProviderModels(appIds: string[]) {
  const uniqueAppIds = [...new Set(appIds)];
  const loadKey = uniqueAppIds.sort().join("|");
  if (!loadKey) return;

  modelCatalogLoadKey = loadKey;
  isLoadingModelCatalog = true;
  modelCatalogError = false;
  try {
    await Promise.all(uniqueAppIds.map((appId) => ensureModelCatalog(appId)));
  } catch (error) {
    console.error("Failed to load AI provider models", error);
    modelCatalogError = true;
  } finally {
    isLoadingModelCatalog = false;
  }
}

$effect(() => {
  const appIds = connectedAiApps.map((item) => item.app.id);
  const loadKey = [...new Set(appIds)].sort().join("|");
  if (!loadKey || loadKey === modelCatalogLoadKey) return;
  void loadProviderModels(appIds);
});

function readConfiguredModel(settings: unknown): string {
  if (typeof settings !== "object" || settings === null || Array.isArray(settings)) {
    return "";
  }
  const values = settings as Record<string, unknown>;
  const value = values.default_model ?? values.model;
  if (typeof value !== "string") return "";
  const model = value.trim();
  return model && model.toLowerCase() !== "default" ? model : "";
}

function matchesModel(model: { label: string; value: string }, query: string) {
  const normalizedQuery = query.trim().toLowerCase();
  return (
    !normalizedQuery ||
    model.label.toLowerCase().includes(normalizedQuery) ||
    model.value.toLowerCase().includes(normalizedQuery)
  );
}

// Active connected integration
const activeConnected = $derived.by(() => {
  const active = connectedIntegrations.find((p) => p.is_enabled);
  if (active) return active;
  if (connectedIntegrations.length > 0) return connectedIntegrations[0];
  return null;
});

const selectedIntegration = $derived.by(() => {
  const saved = connectedIntegrations.find(
    (provider) =>
      provider.provider_name.toLowerCase() === savedProvider.toLowerCase(),
  );
  return saved ?? (providersQuery.isSuccess ? activeConnected : null);
});

const selectedProvider = $derived(
  selectedIntegration?.provider_name ??
    (providersQuery.isSuccess ? "" : savedProvider),
);

// Currently selected OrbisApp
const currentApp = $derived.by(() => {
  if (selectedProvider) {
    const found = findAppForProvider(selectedProvider);
    if (found) return found;
  }
  if (selectedIntegration) {
    const found = findAppForProvider(selectedIntegration.provider_name);
    if (found) return found;
  }
  if (connectedAiApps.length > 0) {
    return connectedAiApps[0].app;
  }
  return null;
});

const selectedModel = $derived.by(() => {
  const savedPreferenceMatchesProvider =
    savedProvider.toLowerCase() === selectedProvider.toLowerCase();
  const saved = savedPreferenceMatchesProvider ? savedModel.trim() : "";
  if (saved && saved.toLowerCase() !== "default") return saved;
  return readConfiguredModel(selectedIntegration?.settings);
});

// Clean short provider name for compact toolbars
const shortProviderName = $derived.by(() => {
  if (!currentApp) return m.ai_ui_not_connected();
  if (currentApp.id === "openrouter") return "OpenRouter";
  if (currentApp.id === "openai") return "OpenAI";
  if (currentApp.id === "anthropic") return "Claude";
  if (currentApp.id === "deepseek") return "DeepSeek";
  if (currentApp.id === "gemini-ai") return "Gemini";
  if (currentApp.id === "groq") return "Groq";
  if (currentApp.id === "mistral") return "Mistral";
  if (currentApp.id === "ollama") return "Ollama";
  return currentApp.name_ar?.split(" ")[0] || currentApp.name;
});

const shortModelName = $derived.by(() => {
  if (!selectedModel) return "";
  const clean = selectedModel.includes("/") ? selectedModel.split("/")[1] : selectedModel;
  return clean
    .replace("claude-3-7-", "3.7-")
    .replace("claude-3-5-", "3.5-")
    .replace("gpt-4o-mini", "4o-mini")
    .replace("gpt-4o", "4o");
});

async function handleSelectProviderAndModel(
  providerName: string,
  app: OrbisApp,
  modelValue: string,
) {
  const model = modelValue.trim();
  if (!model || model.toLowerCase() === "default") return;

  uiStore.setAiProviderModel(orgId, providerName, model);

  // Toggle active provider in backend integration table if not already enabled
  const matched = connectedIntegrations.find(
    (c) => c.provider_name.toLowerCase() === providerName.toLowerCase(),
  );
  if (matched && !matched.is_enabled) {
    try {
      await switchProviderMutation.mutateAsync({
        providerName: matched.provider_name,
        isEnabled: true,
      });
      toast.success(m.ai_ui_provider_activated({ name: app.name_ar || app.name }));
    } catch (e) {
      toast.error(extractErrorMessage(e));
    }
  }
}
</script>

<DropdownMenu>
  <DropdownMenuTrigger class="outline-none">
    <Button
      type="button"
      variant="outline"
      size="sm"
      class="h-7 px-2 rounded-md border-border/80 bg-background hover:bg-muted/70 flex items-center gap-1.5 cursor-pointer group shrink-0 max-w-[220px]"
      title={m.ai_ui_provider_model({ provider: shortProviderName, model: selectedModel || m.ai_ui_default_model() })}
    >
      <!-- Logo SVG Icon -->
      <div class="w-3.5 h-3.5 shrink-0 flex items-center justify-center">
        {#if currentApp?.logo}
          {@html currentApp.logo(currentApp.id)}
        {:else}
          <Sparkles size={12} class="text-primary" />
        {/if}
      </div>

      <!-- Provider Short Name -->
      <span class="font-semibold text-foreground text-[11px] truncate">
        {shortProviderName}
      </span>

      <!-- Dynamic Model Tag -->
      {#if shortModelName}
        <span class="text-[9.5px] font-mono text-muted-foreground font-medium px-1 py-0.2 rounded bg-muted truncate max-w-[85px]">
          {shortModelName}
        </span>
      {/if}

      <ChevronDown size={11} class="text-muted-foreground/80 shrink-0 transition-transform duration-200 group-hover:text-foreground" />
    </Button>
  </DropdownMenuTrigger>

  <DropdownMenuContent
    align="start"
    side="top"
    sideOffset={8}
    class="w-56 max-w-[calc(100vw-1rem)] max-h-[min(70dvh,32rem)] overflow-y-auto p-1 z-50"
  >
    <div class="flex items-center gap-3 px-3 py-2.5 border-b border-border/50">
      <div class="w-8 h-8 shrink-0 rounded-md border border-border/60 bg-muted/60 p-1 flex items-center justify-center">
        <Sparkles size={15} class="text-primary" />
      </div>
      <div class="flex min-w-0 flex-col gap-1 leading-tight">
        <span class="truncate text-xs font-bold uppercase tracking-wider text-foreground">
          {m.ai_ui_ai_providers()}
        </span>
        <span class="truncate text-[10px] font-semibold text-muted-foreground">
          {shortProviderName}
        </span>
      </div>
    </div>

    <div class="py-1">
      {#if providersQuery.isPending}
        <div class="flex items-center justify-center gap-2 px-3 py-3 text-xs text-muted-foreground" role="status">
          <Sparkles size={13} class="animate-pulse text-primary" />
          {m.ai_ui_loading_ai_center()}
        </div>
      {:else if connectedAiApps.length > 0}
        {#each connectedAiApps as item (item.integration.id)}
          {@const isSelectedProvider = selectedProvider.toLowerCase() === item.integration.provider_name.toLowerCase()}
          <DropdownMenuSub>
            <DropdownMenuSubTrigger
              title={item.app.name_ar || item.app.name}
              class="cursor-pointer px-3 py-1.5 text-xs rounded flex items-center gap-2"
            >
              <div class="size-6 shrink-0 rounded-md border border-border/60 bg-muted/50 p-1 flex items-center justify-center">
                {#if item.app.logo}
                  {@html item.app.logo(item.app.id)}
                {:else}
                  <Sparkles size={11} />
                {/if}
              </div>

              <span class="truncate font-medium flex-1 text-start">
                {item.app.name_ar || item.app.name}
              </span>

              {#if item.integration.is_enabled}
                <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 shrink-0" title={m.ai_ui_active()}></span>
              {/if}

              {#if isSelectedProvider}
                <Check size={12} class="text-primary shrink-0 ms-1" />
              {/if}
            </DropdownMenuSubTrigger>

            <DropdownMenuPortal>
              <DropdownMenuSubContent class="w-52 max-w-[calc(100vw-1rem)] max-h-[min(60dvh,26rem)] overflow-y-auto p-1 z-50 shadow-md rounded-md">
                <DropdownMenuLabel class="px-3 py-2 text-[10px] uppercase font-bold tracking-wider text-muted-foreground border-b border-border/50">
                  {m.ai_ui_available_models()}
                </DropdownMenuLabel>
                {#if isLoadingModelCatalog}
                  <div class="flex items-center gap-2 px-3 py-2 text-xs text-muted-foreground" role="status">
                    <Sparkles size={13} class="animate-pulse text-primary" />
                    {m.ai_ui_loading_models()}
                  </div>
                {:else if modelCatalogError}
                  <div class="flex items-center justify-between gap-2 px-3 py-2 text-xs text-destructive">
                    <span class="min-w-0">{m.ai_ui_model_catalog_load_failed()}</span>
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      class="h-7 shrink-0 px-2 text-xs"
                      onclick={() => loadProviderModels(connectedAiApps.map((provider) => provider.app.id))}
                    >
                      {m.ai_ui_retry()}
                    </Button>
                  </div>
                {/if}
                {#if item.models.length > 0}
                  <div class="relative px-2 pt-2">
                    <Search
                      size={13}
                      class="absolute inset-s-3 top-1/2 -translate-y-1/2 text-muted-foreground"
                    />
                    <Input
                      bind:value={modelSearch}
                      placeholder={m.shared_ui_search_placeholder()}
                      aria-label={m.ai_ui_available_models()}
                      class="h-8 ps-8 text-xs rounded-md"
                    />
                  </div>
                {/if}

                {#if item.models.length > 0}
                  {@const filteredModels = item.models.filter((model) =>
                    matchesModel(model, modelSearch),
                  )}
                  {#if filteredModels.length > 0}
                    {#each filteredModels as model (model.value)}
                    {@const isSelectedModel = isSelectedProvider && selectedModel === model.value}
                    <DropdownMenuItem
                      title={model.value}
                      class="cursor-pointer px-3 py-1.5 text-xs rounded flex items-center justify-between gap-2 font-mono {isSelectedModel ? 'bg-accent text-accent-foreground font-semibold' : ''}"
                      onclick={() => handleSelectProviderAndModel(item.integration.provider_name, item.app, model.value)}
                    >
                      <span class="truncate text-[11px]">{model.label}</span>
                      {#if isSelectedModel}
                        <Check size={12} class="text-primary shrink-0 ms-auto" />
                      {/if}
                    </DropdownMenuItem>
                    {/each}
                  {:else if !isLoadingModelCatalog && !modelCatalogError}
                    <p class="px-3 py-2 text-xs text-muted-foreground">
                      {m.shared_ui_select_no_results()}
                    </p>
                  {/if}
                {:else if !isLoadingModelCatalog && !modelCatalogError}
                  <p class="px-3 py-2 text-xs leading-relaxed text-muted-foreground">
                    {m.ai_ui_no_models_available()}
                  </p>
                {/if}
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
        {/each}
      {:else}
        <div class="px-3 py-4 text-center text-xs leading-relaxed text-muted-foreground">
          {m.ai_ui_no_ai_providers_linked()}
        </div>
      {/if}
    </div>

    {#if onOpenSettings}
      <DropdownMenuItem
        class="cursor-pointer px-3 py-1.5 text-xs rounded flex items-center gap-2 text-primary hover:text-primary font-medium"
        onclick={onOpenSettings}
      >
        <SettingsAlt size={13} />
        <span>{m.ai_ui_manage_providers_app_store()}</span>
      </DropdownMenuItem>
    {/if}
  </DropdownMenuContent>
</DropdownMenu>
