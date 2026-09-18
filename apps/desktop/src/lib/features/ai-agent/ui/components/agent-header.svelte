<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Avatar } from "@orbis/avatar-engine";
import { X, Activity, MessageSquare, Columns, Plus } from "@lucide/svelte";
import ChatHistoryPopover from "./chat-history-popover.svelte";
import type { AiChatSession } from "$lib/features/ai/session-store";

let {
  isStandalone = false,
  userHat = "none",
  headerTitle = "",
  messagesLength = 0,
  currentOrgId = "",
  viewMode = "chat",
  sessions = [],
  currentSessionId = null,
  pendingActionsCount = 0,
  onClose,
  onOpenStandalone,
  onSelectSession,
  onDeleteSession,
  onNewChat,
  onToggleView,
}: {
  isStandalone: boolean;
  userHat: string;
  headerTitle: string;
  messagesLength: number;
  currentOrgId: string;
  viewMode?: "chat" | "cockpit";
  sessions?: AiChatSession[];
  currentSessionId?: string | null;
  pendingActionsCount?: number;
  onToggleView?: () => void;
  onClose: () => void;
  onOpenStandalone: () => void;
  onSelectSession: (id: string) => void;
  onDeleteSession?: (id: string) => void;
  onNewChat: () => void;
} = $props();
</script>

<div class="flex items-center justify-between px-3.5 h-11 shrink-0 border-b border-border select-none bg-card">
  <div class="flex items-center gap-2">
    <!-- Close Button -->
    {#if !isStandalone}
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={onClose}
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="إغلاق الشريط الجانبي"
      >
        <X size={14} strokeWidth={2} />
      </Button>
    {/if}
    
    <!-- Title & Avatar -->
    <span class="text-[12px] font-semibold tracking-wide text-foreground flex items-center gap-1.5 select-none">
      <Avatar hat={userHat} width="28px" height="28px" />
      <span class="flex items-center gap-1 font-bold">
        {headerTitle}
      </span>
    </span>
  </div>

  <!-- Actions -->
  <div class="flex items-center gap-1">
    <!-- View mode toggle (Chat <-> Tools/Cockpit) -->
    <Button
      variant="ghost"
      size="icon-xs"
      onclick={onToggleView}
      class="w-6 h-6 relative flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer {viewMode === 'cockpit' ? 'bg-primary/10 text-primary' : ''}"
      title={viewMode === 'cockpit' ? 'العودة للمحادثة' : 'مركز الأدوات والعمليات'}
    >
      <Activity size={13} />
      {#if pendingActionsCount > 0}
        <span class="absolute -top-0.5 -right-0.5 w-2 h-2 rounded-full bg-amber-500 ring-2 ring-card animate-pulse"></span>
      {/if}
    </Button>
    
    <!-- Standalone Window / Split Layout -->
    {#if !isStandalone}
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={onOpenStandalone}
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="فتح في نافذة منفصلة"
      >
        <Columns size={13} />
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
