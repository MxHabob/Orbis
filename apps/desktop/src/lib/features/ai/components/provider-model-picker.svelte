<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuSubContent,
  DropdownMenuPortal,
} from "@orbis/ui/dropdown-menu";
import { Sparkles, ChevronDown, Check, SettingsAlt } from "@orbis/icons";
import { getAppsByCategory } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import { createConnectedAiProvidersQuery, useSwitchActiveAiProvider } from "$lib/features/ai/queries";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";

let {
  selectedModel = $bindable<string>(""),
  selectedProvider = $bindable<string>(""),
  orgId = "",
  onOpenSettings,
}: {
  selectedModel?: string;
  selectedProvider?: string;
  orgId?: string;
  onOpenSettings?: () => void;
} = $props();

// Canonical AI apps from @orbis/app-store (single source of truth)
const aiApps = $derived(getAppsByCategory("ai-provider"));

// Query connected AI integrations for the organization
const providersQuery = createConnectedAiProvidersQuery(() => orgId);
const connectedIntegrations = $derived(providersQuery.data ?? []);
const switchProviderMutation = useSwitchActiveAiProvider();

// Helper to find matching OrbisApp from @orbis/app-store
function findAppForProvider(providerNameOrId: string): OrbisApp | undefined {
  const norm = providerNameOrId.toLowerCase().trim().replace(/_/g, "-");
  return aiApps.find(
    (app) =>
      app.id.toLowerCase() === norm ||
      app.adapter_name?.toLowerCase() === norm ||
      app.name.toLowerCase().includes(norm) ||
      norm.includes(app.id.toLowerCase()),
  );
}

// Extract model options dynamically from app settings
function getModelsForApp(app: OrbisApp): { label: string; value: string }[] {
  const modelSetting = app.settings?.find((s) => s.id === "model");
  if (modelSetting?.options && modelSetting.options.length > 0) {
    return modelSetting.options;
  }
  if (modelSetting?.value) {
    return [{ label: String(modelSetting.value), value: String(modelSetting.value) }];
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
    return {
      integration,
      app: resolvedApp,
      models: getModelsForApp(resolvedApp),
    };
  });
});

// Active connected integration
const activeConnected = $derived.by(() => {
  const active = connectedIntegrations.find((p) => p.is_enabled);
  if (active) return active;
  if (connectedIntegrations.length > 0) return connectedIntegrations[0];
  return null;
});

// Currently selected OrbisApp
const currentApp = $derived.by(() => {
  if (selectedProvider) {
    const found = findAppForProvider(selectedProvider);
    if (found) return found;
  }
  if (activeConnected) {
    const found = findAppForProvider(activeConnected.provider_name);
    if (found) return found;
  }
  if (connectedAiApps.length > 0) {
    return connectedAiApps[0].app;
  }
  return null;
});

// Auto-sync initial provider and model from active integration
$effect(() => {
  if (!selectedProvider && activeConnected) {
    selectedProvider = activeConnected.provider_name;
  }
  if (!selectedModel && currentApp) {
    const defaultModelInConfig =
      (activeConnected?.settings as any)?.default_model ||
      (activeConnected?.settings as any)?.model;

    const models = getModelsForApp(currentApp);
    if (defaultModelInConfig) {
      selectedModel = defaultModelInConfig;
    } else if (models.length > 0) {
      selectedModel = models[0].value;
    }
  }
});

// Clean short provider name for compact toolbars
const shortProviderName = $derived.by(() => {
  if (!currentApp) return "غير متصل";
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
  selectedProvider = providerName;
  selectedModel = modelValue;

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
      toast.success(`تم تفعيل ${app.name_ar || app.name}`);
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
      class="h-7 px-2 rounded-lg border-border/80 bg-background/80 hover:bg-muted/80 hover:border-primary/40 active:scale-[0.98] transition-all flex items-center gap-1.5 cursor-pointer shadow-2xs group shrink-0 max-w-[220px]"
      title={`${shortProviderName} (${selectedModel || 'الافتراضي'})`}
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
        <span class="text-[9.5px] font-mono text-primary font-medium px-1 py-0.2 rounded bg-primary/10 border border-primary/20 truncate max-w-[85px]">
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
    class="w-60 p-1 z-50 shadow-xl rounded-xl border border-border bg-popover text-popover-foreground backdrop-blur-xl"
  >
    <!-- Menu Header -->
    <div class="flex items-center justify-between px-2.5 py-1.5 border-b border-border/50">
      <span class="text-xs font-bold text-foreground flex items-center gap-1.5">
        <Sparkles size={12} class="text-primary" />
        مزودات الذكاء الاصطناعي
      </span>
      {#if connectedAiApps.length > 0}
        <Badge variant="secondary" class="text-[9px] font-mono px-1 py-0 h-4">
          {connectedAiApps.length}
        </Badge>
      {/if}
    </div>

    <!-- Nested Providers & Models List -->
    <div class="py-1">
      {#if connectedAiApps.length > 0}
        {#each connectedAiApps as item (item.integration.id)}
          {@const isSelectedProvider = selectedProvider.toLowerCase() === item.integration.provider_name.toLowerCase()}
          <DropdownMenuSub>
            <DropdownMenuSubTrigger class="cursor-pointer px-2.5 py-1.5 text-xs rounded-lg flex items-center gap-2">
              <div class="w-4.5 h-4.5 shrink-0 rounded-md border border-border/60 bg-muted/40 p-0.5 flex items-center justify-center">
                {#if item.app.logo}
                  {@html item.app.logo(item.app.id)}
                {:else}
                  <Sparkles size={11} />
                {/if}
              </div>

              <span class="truncate font-semibold flex-1 text-start">
                {item.app.name_ar || item.app.name}
              </span>

              {#if item.integration.is_enabled}
                <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 ring-2 ring-emerald-500/20 shrink-0" title="نشط"></span>
              {/if}

              {#if isSelectedProvider}
                <Check size={12} class="text-primary shrink-0 ms-1" />
              {/if}
            </DropdownMenuSubTrigger>

            <DropdownMenuPortal>
              <DropdownMenuSubContent class="w-52 p-1 z-50 shadow-xl rounded-xl border border-border bg-popover text-popover-foreground backdrop-blur-xl">
                <DropdownMenuLabel class="text-[10px] uppercase font-bold text-muted-foreground px-2 py-1">
                  النماذج المتاحة
                </DropdownMenuLabel>
                <DropdownMenuSeparator class="bg-border/50" />

                {#if item.models.length > 0}
                  {#each item.models as model (model.value)}
                    {@const isSelectedModel = isSelectedProvider && selectedModel === model.value}
                    <DropdownMenuItem
                      class="cursor-pointer px-2.5 py-1.5 text-xs rounded-lg flex items-center justify-between font-mono {isSelectedModel ? 'bg-primary/10 text-primary font-bold' : ''}"
                      onclick={() => handleSelectProviderAndModel(item.integration.provider_name, item.app, model.value)}
                    >
                      <span class="truncate text-[11px]">{model.label}</span>
                      {#if isSelectedModel}
                        <Check size={12} class="text-primary shrink-0 ms-auto" />
                      {/if}
                    </DropdownMenuItem>
                  {/each}
                {:else}
                  <DropdownMenuItem
                    class="cursor-pointer px-2.5 py-1.5 text-xs rounded-lg flex items-center justify-between"
                    onclick={() => handleSelectProviderAndModel(item.integration.provider_name, item.app, "default")}
                  >
                    <span class="truncate text-[11px]">النموذج الافتراضي</span>
                    {#if isSelectedProvider}
                      <Check size={12} class="text-primary shrink-0 ms-auto" />
                    {/if}
                  </DropdownMenuItem>
                {/if}
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
        {/each}
      {:else}
        <div class="px-3 py-3 text-center text-xs text-muted-foreground">
          لا يوجد أي مزود ذكاء اصطناعي مرتبط حالياً
        </div>
      {/if}
    </div>

    {#if onOpenSettings}
      <DropdownMenuSeparator class="bg-border/50" />
      <DropdownMenuItem
        class="cursor-pointer px-2.5 py-1.5 text-xs rounded-lg flex items-center gap-1.5 text-primary hover:text-primary font-medium"
        onclick={onOpenSettings}
      >
        <SettingsAlt size={13} />
        <span>إدارة المزودات في App Store</span>
      </DropdownMenuItem>
    {/if}
  </DropdownMenuContent>
</DropdownMenu>
