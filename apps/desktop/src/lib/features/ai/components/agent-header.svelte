<script lang="ts">
  import type { AiChatSession } from "$lib/features/ai/session-store";
  import * as m from "$lib/paraglide/messages";
  import type { OrbisApp } from "@orbis/app-store";
  import { getAppsByCategory } from "@orbis/app-store";
  import { ChartBar, Expand, Plus, Sparkles, XMark } from "@orbis/icons";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";
  import ChatHistoryPopover from "./chat-history-popover.svelte";

  let {
    isStandalone = false,
    headerTitle = "",
    messagesLength = 0,
    currentOrgId = "",
    viewMode = "chat",
    sessions = [],
    currentSessionId = null,
    pendingActionsCount = 0,
    selectedModel = $bindable(""),
    selectedProvider = $bindable(""),
    onClose,
    onOpenStandalone,
    onSelectSession,
    onDeleteSession,
    onNewChat,
    onToggleView,
    onOpenSettings,
  }: {
    isStandalone?: boolean;
    headerTitle?: string;
    messagesLength?: number;
    currentOrgId?: string;
    viewMode?: "chat" | "cockpit";
    sessions?: AiChatSession[];
    currentSessionId?: string | null;
    pendingActionsCount?: number;
    selectedModel?: string;
    selectedProvider?: string;
    onToggleView?: () => void;
    onClose?: () => void;
    onOpenStandalone?: () => void;
    onSelectSession?: (id: string) => void;
    onDeleteSession?: (id: string) => void;
    onNewChat?: () => void;
    onOpenSettings?: () => void;
  } = $props();

  // Canonical AI apps from @orbis/app-store
  const aiApps = $derived(getAppsByCategory("ai-provider"));

  // Resolve active app from selectedProvider
  const currentApp = $derived.by<OrbisApp | undefined>(() => {
    if (!selectedProvider) return aiApps[0];
    const norm = selectedProvider.toLowerCase().trim().replace(/_/g, "-");
    return aiApps.find(
      (app) =>
        app.id.toLowerCase() === norm ||
        app.adapter_name?.toLowerCase() === norm ||
        app.name.toLowerCase().includes(norm) ||
        norm.includes(app.id.toLowerCase()),
    );
  });

  // Dynamic Display Name
  const displayTitle = $derived.by(() => {
    if (
      headerTitle &&
      headerTitle !== "Orbis" &&
      headerTitle !== m.ai_ui_new_chat() &&
      headerTitle !== "New Chat"
    ) {
      return headerTitle;
    }
    if (!currentApp) return "Orbis AI";
    if (currentApp.id === "openrouter") return "OpenRouter";
    if (currentApp.id === "openai") return "OpenAI";
    if (currentApp.id === "anthropic") return "Claude";
    if (currentApp.id === "deepseek") return "DeepSeek";
    if (currentApp.id === "gemini-ai") return "Gemini";
    return currentApp.name_ar?.split(" ")[0] || currentApp.name;
  });
</script>

<div
  class="flex items-center justify-between px-3 h-12 shrink-0 border-b border-border/80 select-none bg-card/95 backdrop-blur-xl"
>
  <!-- Left/Start Side: Provider Avatar + Title + Status -->
  <div class="flex items-center gap-2 min-w-0 flex-1 overflow-hidden">
    <!-- Close Button (if docked sidebar) -->
    {#if !isStandalone}
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={onClose}
        class="w-7 h-7 shrink-0 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/80 active:scale-95 transition-all cursor-pointer"
        title={m.ai_ui_close()}
      >
        <XMark size={14} strokeWidth={2.2} />
      </Button>
    {/if}

    <!-- Dynamic SVG Logo Avatar -->
    <div class="w-6.5 h-6.5 rounded-lg flex items-center justify-center shrink-0">
      {#if currentApp?.logo}
        {@html currentApp.logo(currentApp.id)}
      {:else}
        <Sparkles size={13} class="text-primary" />
      {/if}
    </div>

    <!-- Title, Model Badge & Status -->
    <div class="flex items-center gap-1.5 min-w-0 truncate">
      <span class="text-[12px] font-bold text-foreground truncate">
        {displayTitle}
      </span>

      {#if selectedModel}
        <Badge
          variant="outline"
          class="text-[9px] font-mono px-1.5 py-0 h-4 border-border/80 text-muted-foreground bg-muted/40 truncate max-w-[90px] hidden sm:inline-flex"
        >
          {selectedModel.includes("/")
            ? selectedModel.split("/")[1]
            : selectedModel}
        </Badge>
      {/if}

      <!-- Active Connection Pulse Indicator -->
      <span
        class="w-1.5 h-1.5 rounded-full bg-emerald-500 ring-2 ring-emerald-500/20 shrink-0"
        title={m.ai_ui_connected()}
      ></span>
    </div>
  </div>

  <!-- Right/End Side: Header Action Buttons -->
  <div class="flex items-center gap-1 shrink-0">
    <!-- View mode toggle (Chat <-> Cockpit Tools) -->
    <Button
      variant="ghost"
      size="icon-xs"
      onclick={onToggleView}
      class="w-7 h-7 relative flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/80 active:scale-95 transition-all cursor-pointer {viewMode ===
      'cockpit'
        ? 'bg-primary/15 text-primary'
        : ''}"
      title={viewMode === "cockpit" ? m.ai_ui_return_to_chat() : m.ai_ui_operations_center()}
    >
      <ChartBar size={13} />
      {#if pendingActionsCount > 0}
        <span
          class="absolute -top-0.5 -end-0.5 w-2 h-2 rounded-full bg-amber-500 ring-2 ring-card animate-pulse"
        ></span>
      {/if}
    </Button>

    <!-- Standalone Window Expand (if docked) -->
    {#if !isStandalone}
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={onOpenStandalone}
        class="w-7 h-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/80 active:scale-95 transition-all cursor-pointer"
        title={m.ai_ui_open_standalone_window()}
      >
        <Expand size={13} />
      </Button>
    {/if}

    <!-- Chat History Popover -->
    <ChatHistoryPopover
      {sessions}
      {currentSessionId}
      {onSelectSession}
      {onDeleteSession}
      {onNewChat}
    />

    <!-- New Chat Button -->
    <Button
      variant="ghost"
      size="icon-xs"
      onclick={onNewChat}
      class="w-7 h-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/80 active:scale-95 transition-all cursor-pointer"
      title={m.ai_ui_new_chat()}
    >
      <Plus size={14} />
    </Button>
  </div>
</div>
