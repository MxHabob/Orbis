<!--
@component ChatHistoryPopover
Displays a popover of local agent conversation sessions with search and management capabilities.
-->
<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import { ClockRotateRight, Plus, Trash } from "@orbis/icons";
import SearchField from "$lib/shared/components/search-field.svelte";
import { formatDate } from "$lib/core/i18n";
import { Badge } from "@orbis/ui/badge";
import type { AiChatSession } from "$lib/features/ai/session-store";
import * as m from "$lib/paraglide/messages";

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
        title={m.ai_ui_chat_history()}
      >
        <ClockRotateRight size={13} />
      </Button>
    {/snippet}
  </PopoverTrigger>

  <PopoverContent
    align="end"
    sideOffset={6}
    class="flex max-h-[min(32rem,calc(100vh-4rem))] w-[min(360px,calc(100vw-1rem))] flex-col gap-0 overflow-hidden rounded-md border border-border bg-card p-0 text-start font-sans shadow-md"
  >
    <div class="flex items-center justify-between gap-3 border-b border-border px-4 py-3">
      <div class="flex min-w-0 items-center gap-2">
        <h4 class="truncate text-sm font-semibold text-foreground">
          {m.ai_ui_chat_history()}
        </h4>
      </div>
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={handleNewChat}
        title={m.ai_ui_new_chat()}
        aria-label={m.ai_ui_new_chat()}
        class="shrink-0"
      >
        <Plus size={14} />
      </Button>
    </div>

    <div class="px-3 py-3">
      <SearchField
        bind:value={searchQuery}
        placeholder={m.ai_ui_search_chats()}
        class="w-full max-w-full text-xs"
      />
    </div>

    <div class="flex max-h-[min(18rem,calc(100vh-14rem))] flex-col gap-1.5 overflow-y-auto px-3 pb-3 text-start">
      {#if filteredSessions.length === 0}
        <div class="flex flex-col items-center gap-1.5 py-6 text-center text-[11px] text-muted-foreground">
          <ClockRotateRight size={20} class="text-muted-foreground/40" />
          <span>{m.ai_ui_no_previous_chats()}</span>
        </div>
      {:else}
        {#each filteredSessions as session (session.id)}
          <div
            role="button"
            tabindex="0"
            onclick={() => handleSelect(session.id)}
            onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') handleSelect(session.id); }}
            class="group flex w-full cursor-pointer flex-col gap-1 rounded-md border border-transparent p-2 text-start transition-colors {session.id === currentSessionId ? 'border-primary/20 bg-primary/10 text-primary' : 'text-foreground hover:bg-muted/40'}"
          >
            <div class="flex justify-between items-center w-full">
              <span class="min-w-0 flex-1 truncate pe-1 text-[12px] font-semibold">
                {session.title || m.ai_ui_untitled_chat()}
              </span>
              <div class="flex items-center gap-1 shrink-0">
                <Badge variant="secondary" class="text-[8px] h-3.5 px-1 font-mono">
                  {m.ai_ui_message_count({ count: session.messages.length })}
                </Badge>
                {#if onDeleteSession}
                  <button
                    type="button"
                    onclick={(e) => handleDelete(e, session.id)}
                    class="opacity-0 group-hover:opacity-100 hover:text-destructive p-0.5 rounded transition-opacity cursor-pointer"
                    title={m.ai_ui_delete_chat()}
                  >
                    <Trash size={11} />
                  </button>
                {/if}
              </div>
            </div>
            <div class="text-[9px] text-muted-foreground leading-normal flex justify-between items-center w-full">
              <span>{formatDate(session.updatedAt || session.createdAt)}</span>
              {#if session.id === currentSessionId}
                <span class="text-[8px] font-bold text-primary">{m.ai_ui_current_chat()}</span>
              {/if}
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </PopoverContent>
</Popover>
