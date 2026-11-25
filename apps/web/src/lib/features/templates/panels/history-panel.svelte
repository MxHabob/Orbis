<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { formatDate } from "$lib/core/i18n";
/**
 * HistoryPanel — Scrollable undo/redo history.
 * Allows clicking to jump to any history state.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui/tooltip";
import {
  ArrowForward,
  CheckCircle,
  ClockRotateRight,
  Trash,
  UndoAction,
} from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";

let {
  store,
}: {
  store: EditorStore;
} = $props();

const isAr = $derived(getLocale() === "ar");

// Derive the history from the store
const history = $derived(store.commandHistory);
const currentIndex = $derived(store.historyIndex);
const canUndo = $derived(store.canUndo);
const canRedo = $derived(store.canRedo);

function jumpToState(index: number) {
  store.jumpToHistoryIndex(index);
}
</script>

<TooltipProvider delayDuration={300}>
<div class="flex flex-col h-full text-start">
  <!-- Header -->
  <div class="px-3 py-2.5 border-b border-border flex items-center justify-between">
    <div class="flex items-center gap-2">
      <ClockRotateRight size={15} class="text-muted-foreground" />
      <h3 class="text-sm font-semibold text-foreground">{m.template_panel_history()}</h3>
    </div>
    <div class="flex items-center gap-1">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm" class="cursor-pointer" onclick={() => store.undo()} disabled={!canUndo}>
            <UndoAction size={14} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.template_history_undo()}</TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm" class="cursor-pointer" onclick={() => store.redo()} disabled={!canRedo}>
            <ArrowForward size={14} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.template_history_redo()}</TooltipContent>
      </Tooltip>
    </div>
  </div>

  <!-- ClockRotateRight List -->
  <div class="flex-grow overflow-y-auto scrollbar-hide">
    {#if history.length === 0}
      <div class="flex flex-col items-center justify-center py-8 px-4 gap-2">
        <ClockRotateRight size={20} class="text-muted-foreground/30" />
        <p class="text-xs text-muted-foreground text-center">
          {m.template_history_empty()}
        </p>
      </div>
    {:else}
      <div class="py-1">
        <!-- Initial state -->
        <button
          type="button"
          class="w-full flex items-center gap-2 px-3 py-2 text-start transition-colors cursor-pointer
            {currentIndex === -1 ? 'bg-primary/10 text-primary' : 'text-muted-foreground hover:bg-accent/30'}"
          onclick={() => jumpToState(-1)}
        >
          <div class="w-3 shrink-0 flex justify-center">
            {#if currentIndex === -1}
              <CheckCircle size={10} class="text-primary" />
            {:else}
              <CheckCircle size={6} class="text-muted-foreground/30" />
            {/if}
          </div>
          <span class="text-xs font-medium">
            {m.template_history_initial()}
          </span>
        </button>

        {#each history as cmd, i (i)}
          {@const isCurrent = currentIndex === i}
          {@const isFuture = i > currentIndex}
          <button
            type="button"
            class="w-full flex items-center gap-2 px-3 py-2 text-start transition-colors cursor-pointer
              {isCurrent ? 'bg-primary/10 text-primary font-medium' : ''}
              {isFuture ? 'text-muted-foreground/40' : 'text-foreground'}
              {!isCurrent && !isFuture ? 'hover:bg-accent/30' : ''}"
            onclick={() => jumpToState(i)}
          >
            <div class="w-3 shrink-0 flex justify-center">
              {#if isCurrent}
                <CheckCircle size={10} class="text-primary" />
              {:else if isFuture}
                <CheckCircle size={5} class="text-muted-foreground/20" />
              {:else}
                <CheckCircle size={6} class="text-muted-foreground/40" />
              {/if}
            </div>

            <!-- Timeline line -->
            <div class="flex flex-col min-w-0 flex-grow">
              <span class="text-xs truncate">
                {isAr ? (cmd.descriptionAr || cmd.description) : cmd.description}
              </span>
              {#if cmd.timestamp}
                <span class="text-[10px] text-muted-foreground">
                  {formatDate(cmd.timestamp, "time", { hour: "2-digit", minute: "2-digit", second: "2-digit" })}
                </span>
              {/if}
            </div>

            <!-- Step number -->
            <Badge variant={isCurrent ? "success" : "ghost"} class="text-[9px] h-5 px-1.5 shrink-0">
              {i + 1}
            </Badge>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Footer stats -->
  <div class="px-3 py-2 border-t border-border bg-card flex items-center justify-between">
    <span class="text-[10px] text-muted-foreground">
      {history.length} {m.template_history_commands()}
    </span>
    <span class="text-[10px] text-muted-foreground">
      {currentIndex + 1}/{history.length}
    </span>
  </div>
</div>
</TooltipProvider>
