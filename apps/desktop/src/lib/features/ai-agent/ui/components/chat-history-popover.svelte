<!--
@component ChatHistoryPopover
Displays a popover of local agent conversation sessions with search and management capabilities.
-->
<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import { History, Plus, Trash2, MessageSquare } from "@lucide/svelte";
import SearchField from "$lib/shared/components/search-field.svelte";
import { formatDate } from "$lib/core/i18n";
import { Badge } from "@orbis/ui/badge";
import type { AiChatSession } from "$lib/features/ai/session-store";

let {
  sessions = [],
  currentSessionId = null,
  onSelectSession,
  onDeleteSession,
  onNewChat,
}: {
  sessions?: AiChatSession[];
  currentSessionId?: string | null;
  onSelectSession?: (sessionId: string) => void;
  onDeleteSession?: (sessionId: string) => void;
  onNewChat?: () => void;
} = $props();

let searchQuery = $state("");
let isOpen = $state(false);

const filteredSessions = $derived(
  sessions.filter((s) => {
    if (!searchQuery.trim()) return true;
    const query = searchQuery.toLowerCase();
    const matchesTitle = s.title.toLowerCase().includes(query);
    const matchesContent = s.messages.some((m) =>
      m.content.toLowerCase().includes(query),
    );
    return matchesTitle || matchesContent;
  }),
);

function handleSelect(sessionId: string) {
  onSelectSession?.(sessionId);
  isOpen = false;
}

function handleNewChat() {
  onNewChat?.();
  isOpen = false;
}

function handleDelete(e: MouseEvent, sessionId: string) {
  e.stopPropagation();
  onDeleteSession?.(sessionId);
}
</script>

<Popover bind:open={isOpen}>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      <Button
        {...props}
        variant="ghost"
        size="icon-xs"
        class="w-6 h-6 flex items-center justify-center rounded-md cursor-pointer text-muted-foreground hover:text-foreground hover:bg-muted"
        title="سجل المحادثات"
      >
        <History size={13} />
      </Button>
    {/snippet}
  </PopoverTrigger>
  
  <PopoverContent 
    align="end"
    class="w-72 space-y-2 z-50 select-none bg-popover text-popover-foreground border border-border/80 shadow-xl p-3"
  >
    <div class="flex items-center justify-between px-0.5">
      <span class="text-[11px] font-bold text-foreground tracking-wide flex items-center gap-1.5">
        <MessageSquare size={13} class="text-primary" />
        سجل المحادثات السابقة
      </span>
      <Button
        variant="ghost"
        size="xs"
        class="h-6 text-[10px] px-1.5 font-semibold text-primary hover:bg-primary/10"
        onclick={handleNewChat}
      >
        <Plus size={11} class="me-1" />
        محادثة جديدة
      </Button>
    </div>
    
    <SearchField
      bind:value={searchQuery}
      placeholder="بحث في المحادثات..."
      class="w-full max-w-full md:max-w-full text-xs"
    />

    <div class="flex flex-col gap-1.5 max-h-60 overflow-y-auto scrollbar-hide text-start px-0.5 pt-1">
      {#if filteredSessions.length === 0}
        <div class="text-center text-[11px] text-muted-foreground py-6 flex flex-col items-center gap-1.5">
          <History size={20} class="text-muted-foreground/40" />
          <span>لا توجد محادثات سابقة</span>
        </div>
      {:else}
        {#each filteredSessions as session (session.id)}
          <div
            role="button"
            tabindex="0"
            onclick={() => handleSelect(session.id)}
            onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') handleSelect(session.id); }}
            class="w-full text-start p-2 rounded-lg transition-all cursor-pointer group flex flex-col gap-1 border border-transparent {session.id === currentSessionId ? 'bg-primary/10 border-primary/20 text-primary' : 'hover:bg-muted/60 text-foreground'}"
          >
            <div class="flex justify-between items-center w-full">
              <span class="text-[12px] font-semibold truncate flex-1 pe-1">
                {session.title || "محادثة بدون عنوان"}
              </span>
              <div class="flex items-center gap-1 shrink-0">
                <Badge variant="secondary" class="text-[8px] h-3.5 px-1 font-mono">
                  {session.messages.length} رسالة
                </Badge>
                {#if onDeleteSession}
                  <button
                    type="button"
                    onclick={(e) => handleDelete(e, session.id)}
                    class="opacity-0 group-hover:opacity-100 hover:text-destructive p-0.5 rounded transition-opacity cursor-pointer"
                    title="حذف المحادثة"
                  >
                    <Trash2 size={11} />
                  </button>
                {/if}
              </div>
            </div>
            <div class="text-[9px] text-muted-foreground leading-normal flex justify-between items-center w-full">
              <span>{formatDate(session.updatedAt || session.createdAt)}</span>
              {#if session.id === currentSessionId}
                <span class="text-[8px] font-bold text-primary">المحادثة الحالية</span>
              {/if}
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </PopoverContent>
</Popover>
