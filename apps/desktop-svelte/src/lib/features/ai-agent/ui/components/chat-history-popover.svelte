<!--
@component ChatHistoryPopover
Displays a popover of recent agent sessions with search capability.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@orbis/ui-svelte/popover";
import { History } from "@lucide/svelte";
import SearchField from "$lib/shared/components/search-field.svelte";
import { createAgentSessionsQuery } from "$lib/features/ai/queries";
import { formatDate } from "$lib/core/i18n";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";

let {
  orgId = "",
  onSelectSession,
}: {
  orgId?: string;
  onSelectSession?: (sessionId: string) => void;
} = $props();

const sessionsQuery = createAgentSessionsQuery(() => orgId);
const sessions = $derived($sessionsQuery.data ?? []);

let searchQuery = $state("");

const filteredSessions = $derived(
  sessions.filter(
    (session) =>
      session.id.toLowerCase().includes(searchQuery.toLowerCase()) ||
      session.agent_type.toLowerCase().includes(searchQuery.toLowerCase()),
  ),
);
</script>

<Popover>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      <Button
        {...props}
        variant="ghost"
        size="icon-xs"
        class="w-6 h-6 flex items-center justify-center rounded-md cursor-pointer"
        title="History"
      >
        <History size={13} />
      </Button>
    {/snippet}
  </PopoverTrigger>
  
  <PopoverContent 
    align="end"
    class="w-64 space-y-2 z-50 select-none bg-popover text-popover-foreground border border-border/60"
  >
    <div class="text-[10px] font-semibold text-foreground px-0.5 text-start tracking-wide">
      Agent history
    </div>
    
    <SearchField
      bind:value={searchQuery}
      placeholder="Search sessions..."
      class="w-full max-w-full md:max-w-full"
    />
    

    <div class="flex flex-col gap-2 max-h-56 overflow-y-auto scrollbar-hide text-start px-0.5">
      {#if $sessionsQuery.isLoading}
        <div class="space-y-2 py-2">
          <Skeleton class="h-10 w-full rounded" />
          <Skeleton class="h-10 w-full rounded" />
        </div>
      {:else if $sessionsQuery.isError}
        <div class="text-center text-[10px] text-destructive/80 py-4 px-1 leading-relaxed">
          {orgId ? "خدمة الوكيل غير متوفرة حالياً" : "Agent service offline"}
        </div>
      {:else if filteredSessions.length === 0}
        <div class="text-center text-[11.5px] text-muted-foreground py-6">
          No sessions found
        </div>
      {:else}
        {#each filteredSessions as session}
          <button
            onclick={() => onSelectSession?.(session.id)}
            class="w-full text-start bg-transparent hover:bg-muted/40 p-2 rounded-md transition-all cursor-pointer group flex flex-col gap-1 focus:outline-none"
          >
            <div class="text-[12.5px] font-bold font-mono text-foreground group-hover:text-primary transition-colors flex justify-between items-center">
              <span>#{session.id.substring(0, 8)}</span>
              <Badge variant={session.status === 'active' ? 'lime' : 'secondary'} class="text-[7px] h-3.5 px-1 shrink-0">
                {session.status}
              </Badge>
            </div>
            <div class="text-[9px] text-muted-foreground leading-normal flex justify-between items-center w-full mt-0.5">
              <span>{session.agent_type}</span>
              <span>{formatDate(session.created_at)}</span>
            </div>
          </button>
        {/each}
      {/if}
    </div>
  </PopoverContent>
</Popover>
