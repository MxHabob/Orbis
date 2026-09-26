<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Sparkles, ChevronDown, Check, SettingsAlt, Plus } from "@orbis/icons";
import { getAppsByCategory, getAppById } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import { createConnectedAiProvidersQuery, useSwitchActiveAiProvider } from "$lib/features/ai/queries";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import { getLocale } from "$lib/paraglide/runtime";

let {
  selectedModel = $bindable<string>(""),
  selectedProvider = $bindable<string>(""),
  orgId = "",
  onOpenSettings,
}: {
  selectedModel: string;
  selectedProvider: string;
  orgId?: string;
  onOpenSettings?: () => void;
} = $props();

const isRtl = $derived(getLocale() === "ar");

// Canonical AI apps from @orbis/app-store
const aiApps = $derived(getAppsByCategory("ai-provider"));

// Query connected AI integrations for the organization
const providersQuery = createConnectedAiProvidersQuery(() => orgId);
const connectedIntegrations = $derived(providersQuery.data ?? []);
const switchProviderMutation = useSwitchActiveAiProvider();

let isOpen = $state(false);

// Helper to find matching OrbisApp from @orbis/app-store
function findAppForProvider(providerNameOrId: string): OrbisApp | undefined {
  const norm = providerNameOrId.toLowerCase().trim().replace(/_/g, "-");
  return aiApps.find(
    (app) =>
      app.id.toLowerCase() === norm ||
      app.adapter_name?.toLowerCase() === norm ||
      app.name.toLowerCase().includes(norm) ||
      norm.includes(app.id.toLowerCase())
  );
}

// FILTER: ONLY connected apps in the organization
const connectedAiApps = $derived.by(() => {
  return connectedIntegrations
    .map((integration) => {
      const app = findAppForProvider(integration.provider_name);
      return {
        integration,
        app: app || {
          id: integration.provider_name,
          name: integration.provider_name,
          name_ar: integration.provider_name,
          category: "ai-provider" as const,
          active: true,
          icon: "Sparkles",
          short_description: "",
          settings: [],
          features: [],
        },
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

// Extract model options dynamically from app settings
const availableModels = $derived.by(() => {
  if (!currentApp) return [];
  const modelSetting = currentApp.settings?.find((s) => s.id === "model");
  if (modelSetting?.options && modelSetting.options.length > 0) {
    return modelSetting.options;
  }
  if (modelSetting?.value) {
    return [{ label: String(modelSetting.value), value: String(modelSetting.value) }];
  }
  return [];
});

// Auto-sync initial provider and model from active integration
$effect(() => {
  if (!selectedProvider && activeConnected) {
    selectedProvider = activeConnected.provider_name;
  }
  if (!selectedModel) {
    const defaultModelInConfig =
      (activeConnected?.settings as any)?.default_model ||
      (activeConnected?.settings as any)?.model;

    if (defaultModelInConfig) {
      selectedModel = defaultModelInConfig;
    } else if (availableModels.length > 0) {
      selectedModel = availableModels[0].value;
    }
  }
});

const displayProviderName = $derived.by(() => {
  if (!currentApp) return isRtl ? "غير متصل" : "No AI Connected";
  return isRtl ? currentApp.name_ar || currentApp.name : currentApp.name;
});

const displayModelName = $derived.by(() => {
  if (!selectedModel) return "Default";
  const found = availableModels.find((m) => m.value === selectedModel);
  return found ? found.label : selectedModel;
});

async function handleSelectProvider(providerName: string, app: OrbisApp) {
  selectedProvider = providerName;
  const modelSetting = app.settings?.find((s) => s.id === "model");
  if (modelSetting?.options?.[0]) {
    selectedModel = modelSetting.options[0].value;
  } else if (modelSetting?.value) {
    selectedModel = String(modelSetting.value);
  }

  // Toggle active provider in backend integration table
  const matched = connectedIntegrations.find((c) => c.provider_name.toLowerCase() === providerName.toLowerCase());
  if (matched && !matched.is_enabled) {
    try {
      await switchProviderMutation.mutateAsync({
        providerName: matched.provider_name,
        isEnabled: true,
      });
      toast.success(
        isRtl
          ? `تم تفعيل ${app.name_ar || app.name}`
          : `Active AI switched to ${app.name}`
      );
    } catch (e) {
      toast.error(extractErrorMessage(e));
    }
  }

  isOpen = false;
}

function handleSelectModel(modelValue: string) {
  selectedModel = modelValue;
  isOpen = false;
}
</script>

<div class="relative inline-flex items-center select-none font-sans text-xs">
  <!-- Pill Trigger Button at the Toolbar -->
  <button
    type="button"
    class="h-7 px-2.5 rounded-lg border border-border/70 bg-card hover:bg-muted/80 hover:border-primary/40 transition-all flex items-center gap-1.5 cursor-pointer shadow-2xs group"
    onclick={() => (isOpen = !isOpen)}
    title={isRtl ? "تغيير المزود والنموذج" : "Switch AI Provider & Model"}
  >
    <div class="w-3.5 h-3.5 shrink-0 flex items-center justify-center">
      {#if currentApp?.logo}
        {@html currentApp.logo(currentApp.id)}
      {:else}
        <Sparkles size={12} class="text-primary" />
      {/if}
    </div>
    
    <span class="font-bold text-foreground text-[11px] max-w-24 truncate">
      {displayProviderName}
    </span>

    <span class="text-muted-foreground/70 font-mono text-[10px] max-w-24 truncate hidden sm:inline-block">
      ({displayModelName})
    </span>

    <ChevronDown size={11} class="text-muted-foreground transition-transform {isOpen ? 'rotate-180' : ''}" />
  </button>

  <!-- Dropdown Popover (Showing ONLY Connected Providers) -->
  {#if isOpen}
    <button
      type="button"
      class="fixed inset-0 z-40 bg-transparent cursor-default"
      onclick={() => (isOpen = false)}
      aria-label="Close"
    ></button>

    <div
      class="absolute bottom-full {isRtl ? 'right-0' : 'left-0'} mb-2 w-72 p-2 bg-popover border border-border/80 shadow-2xl rounded-xl z-50 flex flex-col gap-2 animate-in fade-in slide-in-from-bottom-2 duration-150"
    >
      <!-- Header with App Store link -->
      <div class="px-2 py-1 flex items-center justify-between border-b border-border/50 pb-1.5">
        <span class="text-[11px] font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
          <Sparkles size={12} class="text-primary" />
          {isRtl ? "المزودات المرتبطة" : "Connected Providers"}
        </span>
        {#if onOpenSettings}
          <button
            type="button"
            class="text-[10.5px] text-primary hover:underline flex items-center gap-1 cursor-pointer font-medium"
            onclick={() => { isOpen = false; onOpenSettings(); }}
          >
            <SettingsAlt size={11} />
            {isRtl ? "App Store" : "App Store"}
          </button>
        {/if}
      </div>

      <!-- Connected Providers List -->
      <div class="flex flex-col gap-1 max-h-48 overflow-y-auto scrollbar-thin">
        {#if connectedAiApps.length > 0}
          {#each connectedAiApps as item (item.integration.id)}
            {@const isSelected = selectedProvider.toLowerCase() === item.integration.provider_name.toLowerCase()}
            <button
              type="button"
              class="w-full text-start px-2.5 py-1.5 rounded-lg flex items-center justify-between hover:bg-muted/80 transition-colors cursor-pointer {isSelected ? 'bg-primary/10 border border-primary/20' : ''}"
              onclick={() => handleSelectProvider(item.integration.provider_name, item.app)}
            >
              <div class="flex items-center gap-2">
                <div class="w-4 h-4 shrink-0 flex items-center justify-center">
                  {#if item.app.logo}
                    {@html item.app.logo(item.app.id)}
                  {:else}
                    <Sparkles size={12} />
                  {/if}
                </div>
                <div class="flex flex-col">
                  <span class="text-[12px] font-bold text-foreground flex items-center gap-1.5">
                    {isRtl ? item.app.name_ar || item.app.name : item.app.name}
                    {#if item.integration.is_enabled}
                      <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 ring-2 ring-emerald-500/20" title={isRtl ? "نشط" : "Active"}></span>
                    {/if}
                  </span>
                </div>
              </div>
              
              <div class="flex items-center gap-1 shrink-0">
                <Badge variant={item.integration.is_enabled ? "default" : "secondary"} class="text-[9px] px-1.5 py-0 font-medium">
                  {item.integration.is_enabled ? (isRtl ? "نشط" : "Active") : (isRtl ? "مربوط" : "Connected")}
                </Badge>
                {#if isSelected}
                  <Check size={13} class="text-primary shrink-0" />
                {/if}
              </div>
            </button>
          {/each}
        {:else}
          <!-- Empty State: Prompt to connect in App Store -->
          <div class="px-3 py-3 text-center flex flex-col items-center gap-2 bg-muted/30 rounded-lg">
            <span class="text-[11.5px] text-muted-foreground font-medium">
              {isRtl ? "لا يوجد أي مزود ذكاء اصطناعي مرتبط بالمنظمة حالياً." : "No AI providers connected yet for this organization."}
            </span>
            {#if onOpenSettings}
              <Button
                variant="outline"
                size="xs"
                class="text-[11px] h-6 flex items-center gap-1 text-primary cursor-pointer"
                onclick={() => { isOpen = false; onOpenSettings(); }}
              >
                <Plus size={11} />
                {isRtl ? "ربط مزود من App Store" : "Connect in App Store"}
              </Button>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Models List Section for Currently Selected App -->
      {#if currentApp && availableModels.length > 0}
        <div class="pt-1.5 border-t border-border/50 flex flex-col gap-1">
          <span class="text-[10px] font-semibold text-muted-foreground px-2">
            {isRtl ? `النماذج المتاحة (${displayProviderName}):` : `Models for (${displayProviderName}):`}
          </span>

          <div class="grid grid-cols-1 gap-0.5 px-1 max-h-36 overflow-y-auto scrollbar-thin">
            {#each availableModels as model (model.value)}
              {@const isSelectedModel = selectedModel === model.value}
              <button
                type="button"
                class="w-full text-start px-2.5 py-1.5 rounded-md text-[11px] flex items-center justify-between hover:bg-muted/70 transition-colors cursor-pointer {isSelectedModel ? 'bg-primary/15 font-bold text-primary' : 'text-foreground'}"
                onclick={() => handleSelectModel(model.value)}
              >
                <span class="font-mono truncate">{model.label}</span>
                {#if isSelectedModel}
                  <Check size={12} class="text-primary shrink-0" />
                {/if}
              </button>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>
