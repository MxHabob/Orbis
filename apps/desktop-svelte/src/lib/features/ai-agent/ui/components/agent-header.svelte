<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Avatar } from "@orbis/avatar-engine";
import { X, MoreHorizontal, Columns, Plus } from "lucide-svelte";
import ChatHistoryPopover from "./chat-history-popover.svelte";

let {
  isStandalone = false,
  userHat = "none",
  headerTitle = "",
  messagesLength = 0,
  currentOrgId = "",
  onClose,
  onOpenStandalone,
  onSelectSession,
  onNewChat,
  viewMode = "chat",
  onToggleView,
}: {
  isStandalone: boolean;
  userHat: string;
  headerTitle: string;
  messagesLength: number;
  currentOrgId: string;
  viewMode?: "chat" | "cockpit";
  onToggleView?: () => void;
  onClose: () => void;
  onOpenStandalone: () => void;
  onSelectSession: (id: string) => void;
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
        title="Close sidebar"
      >
        <X size={14} strokeWidth={2} />
      </Button>
    {/if}
    
    <!-- Title & Avatar -->
    <span class="text-[12px] font-semibold tracking-wide text-foreground flex items-center gap-1.5 select-none">
      {#if messagesLength > 0}
        <Avatar hat={userHat} width="32px" height="32px" />
      {/if}
      <span class="flex items-center gap-0.5">
        {headerTitle}
      </span>
    </span>
  </div>

  <!-- Actions -->
  <div class="flex items-center gap-1">
    <Button
      variant="ghost"
      size="icon-xs"
      onclick={onToggleView}
      class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer {viewMode === 'cockpit' ? 'bg-muted text-foreground' : ''}"
      title="Activity Logs"
    >
      <MoreHorizontal size={14} />
    </Button>
    
    {#if !isStandalone}
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={onOpenStandalone}
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="Split layout"
      >
        <Columns size={13} />
      </Button>
    {/if}

    <ChatHistoryPopover
      orgId={currentOrgId}
      onSelectSession={onSelectSession}
    />

    <Button
      variant="ghost"
      size="icon-xs"
      onclick={onNewChat}
      class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
      title="New Chat"
    >
      <Plus size={14} />
    </Button>
  </div>
</div>
