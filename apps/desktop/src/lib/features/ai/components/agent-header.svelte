<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { ChartBar, Expand, Plus, Sparkles, XMark } from "@orbis/icons";
import ChatHistoryPopover from "./chat-history-popover.svelte";
import { getAppsByCategory } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import type { AiChatSession } from "$lib/features/ai/session-store";
import { getLocale } from "$lib/paraglide/runtime";

let {
  isStandalone = false,
  headerTitle = "",
  messagesLength = 0,
  currentOrgId = "",
  viewMode = "chat",
  sessions = [],
  currentSessionId = null,
  pendingActionsCount = 0,
  selectedModel = "",
  selectedProvider = "",
  onClose,
  onOpenStandalone,
  onSelectSession,
  onDeleteSession,
  onNewChat,
  onToggleView,
}: {
  isStandalone: boolean;
  headerTitle: string;
  messagesLength: number;
  currentOrgId: string;
  viewMode?: "chat" | "cockpit";
  sessions?: AiChatSession[];
  currentSessionId?: string | null;
  pendingActionsCount?: number;
  selectedModel?: string;
  selectedProvider?: string;
  onToggleView?: () => void;
  onClose: () => void;
  onOpenStandalone: () => void;
  onSelectSession: (id: string) => void;
  onDeleteSession?: (id: string) => void;
  onNewChat: () => void;
} = $props();

const isRtl = $derived(getLocale() === "ar");

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
      norm.includes(app.id.toLowerCase())
  );
});

// Dynamic Title & Subtitle Badge
const providerName = $derived.by(() => {
  if (!currentApp) return "Orbis AI";
  return isRtl ? currentApp.name_ar || currentApp.name : currentApp.name;
});

const displayTitle = $derived.by(() => {
  if (headerTitle && headerTitle !== "Orbis" && headerTitle !== "محادثة جديدة") {
    return headerTitle;
  }
  return providerName;
});
</script>

<div class="flex items-center justify-between px-3.5 h-11 shrink-0 border-b border-border select-none bg-card">
  <!-- Title & Provider-Specific Avatar -->
  <div class="flex items-center gap-2 min-w-0">
    <!-- Close Button -->
    {#if !isStandalone}
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={onClose}
        class="w-6 h-6 shrink-0 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="إغلاق الشريط الجانبي"
      >
        <XMark size={14} strokeWidth={2} />
      </Button>
    {/if}
    
    <!-- Dynamic Provider Avatar & Name -->
    <div class="flex items-center gap-2 min-w-0">
      <!-- Dynamic SVG Logo Squircle Avatar -->
      <div class="w-6.5 h-6.5 rounded-lg border border-border/80 bg-muted/40 flex items-center justify-center p-1 shrink-0 shadow-2xs">
        {#if currentApp?.logo}
          {@html currentApp.logo(currentApp.id)}
        {:else}
          <Sparkles size={13} class="text-primary" />
        {/if}
      </div>

      <!-- Title & Model Chip -->
      <div class="flex items-center gap-1.5 min-w-0 truncate">
        <span class="text-[12px] font-bold text-foreground truncate">
          {displayTitle}
        </span>

        {#if selectedModel}
          <Badge variant="outline" class="text-[9.5px] px-1.5 py-0 font-mono text-muted-foreground/80 hidden sm:inline-flex shrink-0">
            {selectedModel}
          </Badge>
        {/if}
      </div>
    </div>
  </div>

  <!-- Header Actions -->
  <div class="flex items-center gap-1 shrink-0">
    <!-- View mode toggle (Chat <-> Tools/Cockpit) -->
    <Button
      variant="ghost"
      size="icon-xs"
      onclick={onToggleView}
      class="w-6 h-6 relative flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer {viewMode === 'cockpit' ? 'bg-primary/10 text-primary' : ''}"
      title={viewMode === 'cockpit' ? 'العودة للمحادثة' : 'مركز الأدوات والعمليات'}
    >
      <ChartBar size={13} />
      {#if pendingActionsCount > 0}
        <span class="absolute -top-0.5 -right-0.5 w-2 h-2 rounded-full bg-amber-500 ring-2 ring-card animate-pulse"></span>
      {/if}
    </Button>
    
    <!-- Standalone Window / Split Grid -->
    {#if !isStandalone}
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={onOpenStandalone}
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="فتح في نافذة منفصلة"
      >
        <Expand size={13} />
      </Button>
    {/if}

    <!-- Chat History Sessions Popover -->
    <ChatHistoryPopover
      sessions={sessions}
      currentSessionId={currentSessionId}
      onSelectSession={onSelectSession}
      onDeleteSession={onDeleteSession}
      onNewChat={onNewChat}
    />

    <!-- New Chat Button -->
    <Button
      variant="ghost"
      size="icon-xs"
      onclick={onNewChat}
      class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
      title="محادثة جديدة"
    >
      <Plus size={14} />
    </Button>
  </div>
</div>
