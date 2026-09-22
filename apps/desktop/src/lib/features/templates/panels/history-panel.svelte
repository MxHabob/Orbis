<script lang="ts">
import * as m from "$lib/paraglide/messages";
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
import { ArrowForward, CheckCircle, ClockRotateRight, Trash, UndoAction } from "@orbis/icons";
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
  <div class="px-3 py-2 border-b border-border/40 flex items-center justify-between">
    <div class="flex items-center gap-2">
      <ClockRotateRight size={13} class="text-primary" />
      <h3 class="text-xs font-bold text-foreground">{m.template_panel_history()}</h3>
    </div>
    <div class="flex items-center gap-1">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5" onclick={() => store.undo()} disabled={!canUndo}>
            <Undo2 size={11} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.template_history_undo()}</TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5" onclick={() => store.redo()} disabled={!canRedo}>
            <Redo2 size={11} />
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
        <p class="text-[10px] text-muted-foreground text-center">
          {m.template_history_empty()}
        </p>
      </div>
    {:else}
      <div class="py-1">
        <!-- Initial state -->
        <button
          type="button"
          class="w-full flex items-center gap-2 px-3 py-1.5 text-start transition-all cursor-pointer
            {currentIndex === -1 ? 'bg-primary/10 text-primary' : 'text-muted-foreground hover:bg-accent/30'}"
          onclick={() => jumpToState(-1)}
        >
          <div class="w-3 shrink-0 flex justify-center">
            {#if currentIndex === -1}
              <CheckCircle size={10} class="text-primary" />
            {:else}
              <Circle size={6} class="text-muted-foreground/30" />
            {/if}
          </div>
          <span class="text-[10px] font-medium">
            {isAr ? m.template_history_initial() : "Initial State"}
          </span>
        </button>

        {#each history as cmd, i (i)}
          {@const isCurrent = currentIndex === i}
          {@const isFuture = i > currentIndex}
          <button
            type="button"
            class="w-full flex items-center gap-2 px-3 py-1.5 text-start transition-all cursor-pointer
              {isCurrent ? 'bg-primary/10 text-primary font-medium' : ''}
              {isFuture ? 'text-muted-foreground/40' : 'text-foreground'}
              {!isCurrent && !isFuture ? 'hover:bg-accent/30' : ''}"
            onclick={() => jumpToState(i)}
          >
            <div class="w-3 shrink-0 flex justify-center">
              {#if isCurrent}
                <CheckCircle size={10} class="text-primary" />
              {:else if isFuture}
                <Circle size={5} class="text-muted-foreground/20" />
              {:else}
                <Circle size={6} class="text-muted-foreground/40" />
              {/if}
            </div>

            <!-- Timeline line -->
            <div class="flex flex-col min-w-0 flex-grow">
              <span class="text-[10px] truncate">
                {isAr ? (cmd.descriptionAr || cmd.description) : cmd.description}
              </span>
              {#if cmd.timestamp}
                <span class="text-[7px] text-muted-foreground/50">
                  {new Date(cmd.timestamp).toLocaleTimeString(isAr ? "ar" : "en", { hour: "2-digit", minute: "2-digit", second: "2-digit" })}
                </span>
              {/if}
            </div>

            <!-- Step number -->
            <Badge variant={isCurrent ? "success" : "ghost"} class="text-[7px] h-3 px-1 shrink-0">
              {i + 1}
            </Badge>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Footer stats -->
  <div class="px-3 py-1.5 border-t border-border/30 bg-muted/10 flex items-center justify-between">
    <span class="text-[8px] text-muted-foreground">
      {history.length} {isAr ? m.template_history_command() : "commands"}
    </span>
    <span class="text-[8px] text-muted-foreground">
      {currentIndex + 1}/{history.length}
    </span>
  </div>
</div>
</TooltipProvider>
