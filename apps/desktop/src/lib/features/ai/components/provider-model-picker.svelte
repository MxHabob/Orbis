<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Separator } from "@orbis/ui/separator";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import { Sparkles, ChevronDown, Check, SettingsAlt, Plus } from "@orbis/icons";
import { getAppsByCategory } from "@orbis/app-store";
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

// Canonical AI apps from @orbis/app-store (single source of truth)
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
  return connectedIntegrations.map((integration) => {
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

// Clean short provider name for compact toolbars
const shortProviderName = $derived.by(() => {
  if (!currentApp) return isRtl ? "غير متصل" : "No AI";
  if (currentApp.id === "openrouter") return "OpenRouter";
  if (currentApp.id === "openai") return "OpenAI";
  if (currentApp.id === "anthropic") return "Claude";
  if (currentApp.id === "deepseek") return "DeepSeek";
  if (currentApp.id === "gemini-ai") return "Gemini";
  if (currentApp.id === "groq") return "Groq";
  if (currentApp.id === "mistral") return "Mistral";
  if (currentApp.id === "ollama") return "Ollama";
  return isRtl ? currentApp.name_ar?.split(" ")[0] || currentApp.name : currentApp.name;
});

const shortModelName = $derived.by(() => {
  if (!selectedModel) return "";
  // If matched in options, use the option label if cleaner
  const matchedOption = availableModels.find((m) => m.value === selectedModel);
  if (matchedOption) {
    return matchedOption.label.replace("Claude ", "").replace("GPT-", "");
  }
  // Strip provider prefixes like 'openai/' or 'anthropic/' for compact display
  const clean = selectedModel.includes("/") ? selectedModel.split("/")[1] : selectedModel;
  return clean.replace("claude-3-7-", "3.7-").replace("claude-3-5-", "3.5-");
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

<Popover bind:open={isOpen}>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      <Button
        {...props}
        type="button"
        variant="outline"
        size="sm"
        class="h-7 px-2 rounded-lg border-border/80 bg-background/80 hover:bg-muted/80 hover:border-primary/40 active:scale-[0.98] transition-all flex items-center gap-1.5 cursor-pointer shadow-2xs group shrink-0 max-w-[220px]"
        title={isRtl ? `${shortProviderName} (${selectedModel || 'الافتراضي'}) - انقر للتغيير` : `${shortProviderName} (${selectedModel || 'Default'}) - Click to switch`}
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

        <ChevronDown size={11} class="text-muted-foreground/80 shrink-0 transition-transform duration-200 {isOpen ? 'rotate-180 text-primary' : 'group-hover:text-foreground'}" />
      </Button>
    {/snippet}
  </PopoverTrigger>

  <PopoverContent
    side="top"
    align="start"
    sideOffset={8}
    class="w-76 p-2.5 space-y-2.5 bg-popover/98 text-popover-foreground border border-border shadow-2xl rounded-2xl z-50 backdrop-blur-2xl ring-1 ring-black/5 dark:ring-white/10 select-none"
  >
    <!-- Header with App Store management link -->
    <div class="px-1 flex items-center justify-between">
      <span class="text-[11px] font-bold text-foreground flex items-center gap-1.5">
        <Sparkles size={12} class="text-primary" />
        {isRtl ? "مزودات الذكاء الاصطناعي" : "AI Providers"}
      </span>
      {#if onOpenSettings}
        <Button
          variant="ghost"
          size="xs"
          class="h-6 px-1.5 text-[10px] text-primary hover:text-primary/90 flex items-center gap-1 cursor-pointer font-medium p-0"
          onclick={() => { isOpen = false; onOpenSettings(); }}
        >
          <SettingsAlt size={11} />
          {isRtl ? "إدارة في App Store" : "Manage"}
        </Button>
      {/if}
    </div>

    <Separator />

    <!-- Connected Providers List -->
    <div class="flex flex-col gap-1 max-h-40 overflow-y-auto scrollbar-thin">
      {#if connectedAiApps.length > 0}
        {#each connectedAiApps as item (item.integration.id)}
          {@const isSelected = selectedProvider.toLowerCase() === item.integration.provider_name.toLowerCase()}
          <button
            type="button"
            class="w-full text-start px-2.5 py-2 rounded-xl flex items-center justify-between hover:bg-muted/80 transition-all cursor-pointer border {isSelected ? 'bg-primary/10 border-primary/30 shadow-2xs' : 'border-transparent'}"
            onclick={() => handleSelectProvider(item.integration.provider_name, item.app)}
          >
            <div class="flex items-center gap-2 min-w-0">
              <div class="w-5 h-5 shrink-0 rounded-md border border-border/60 bg-muted/40 p-0.5 flex items-center justify-center">
                {#if item.app.logo}
                  {@html item.app.logo(item.app.id)}
                {:else}
                  <Sparkles size={12} />
                {/if}
              </div>
              <div class="flex flex-col min-w-0">
                <span class="text-[11.5px] font-bold text-foreground truncate">
                  {isRtl ? item.app.name_ar || item.app.name : item.app.name}
                </span>
                <span class="text-[9.5px] text-muted-foreground truncate">
                  {item.integration.env}
                </span>
              </div>
            </div>
            
            <div class="flex items-center gap-1.5 shrink-0">
              {#if item.integration.is_enabled}
                <Badge variant="outline" class="text-[8.5px] font-semibold bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border-emerald-500/20 px-1.5 py-0">
                  {isRtl ? "نشط" : "Active"}
                </Badge>
              {/if}
              {#if isSelected}
                <Check size={13} class="text-primary shrink-0" />
              {/if}
            </div>
          </button>
        {/each}
      {:else}
        <!-- Empty State: Prompt to connect in App Store -->
        <div class="px-3 py-3 text-center flex flex-col items-center gap-2 bg-muted/30 rounded-xl border border-dashed border-border">
          <span class="text-[11px] text-muted-foreground font-medium">
            {isRtl ? "لا يوجد أي مزود ذكاء اصطناعي مرتبط حالياً." : "No AI providers connected yet."}
          </span>
          {#if onOpenSettings}
            <Button
              variant="outline"
              size="xs"
              class="text-[11px] h-7 flex items-center gap-1 text-primary cursor-pointer rounded-lg"
              onclick={() => { isOpen = false; onOpenSettings(); }}
            >
              <Plus size={11} />
              {isRtl ? "ربط مزود في App Store" : "Connect Provider"}
            </Button>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Models List Section for Currently Selected App -->
    {#if currentApp && availableModels.length > 0}
      <Separator />

      <div class="flex flex-col gap-1.5">
        <div class="flex items-center justify-between px-1">
          <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-1">
            {isRtl ? "النماذج المتاحة" : "Available Models"}
          </span>
          <Badge variant="secondary" class="text-[9px] font-mono px-1 py-0 h-4">
            {availableModels.length} {isRtl ? "نموذج" : "models"}
          </Badge>
        </div>

        <div class="grid grid-cols-1 gap-1 max-h-36 overflow-y-auto scrollbar-thin">
          {#each availableModels as model (model.value)}
            {@const isSelectedModel = selectedModel === model.value}
            <button
              type="button"
              class="w-full text-start px-2.5 py-1.5 rounded-lg text-[11px] flex items-center justify-between hover:bg-muted/70 transition-colors cursor-pointer border {isSelectedModel ? 'bg-primary/15 font-bold text-primary border-primary/30' : 'border-transparent text-foreground'}"
              onclick={() => handleSelectModel(model.value)}
            >
              <span class="font-mono text-[10.5px] truncate">{model.label}</span>
              {#if isSelectedModel}
                <Check size={12} class="text-primary shrink-0" />
              {/if}
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </PopoverContent>
</Popover>
