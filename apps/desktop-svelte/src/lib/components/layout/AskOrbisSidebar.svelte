<!--
@component AskOrbisSidebar
A resizable right-hand sidebar overlay for Ask Orbis agent, mirroring the Seer design.
Allows users to ask questions, select prompt suggestions, and collapse/resize.
-->
<script lang="ts">
import { uiStore } from "$lib/stores";
import Resizer from "./Resizer.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "$lib/components/onboarding/onboarding-state.svelte";
import {
  X,
  MoreHorizontal,
  Columns,
  History,
  Plus,
  ArrowRight,
  ArrowLeft,
} from "lucide-svelte";
import { onMount } from "svelte";
import { fly } from "svelte/transition";
import { getLocale } from "$lib/paraglide/runtime";

function pxToRem(px: number): number {
  if (typeof document === "undefined") return px / 16;
  const base = parseFloat(getComputedStyle(document.documentElement).fontSize);
  return px / base;
}

let sidebarDiv = $state<HTMLDivElement>();
let preferredWidth = $state(pxToRem(380));
const minWidth = $derived(pxToRem(320));
const maxWidth = $derived(pxToRem(650));

const isRtl = $derived(getLocale() === "ar");

let inputValue = $state("");
const suggestions = $derived([
  m.agent_suggest_1(),
  m.agent_suggest_2(),
  m.agent_suggest_3(),
]);

function handleSuggestionClick(suggestion: string) {
  inputValue = suggestion;
}

function handleClose() {
  uiStore.setAskOrbis(false);
}

function handleSubmit(e: SubmitEvent | KeyboardEvent) {
  if (e) e.preventDefault();
  if (!inputValue.trim()) return;

  // Here we would typically submit the query to an LLM or service
  inputValue = "";
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSubmit(e);
  }
}
</script>

<div
  bind:this={sidebarDiv}
  class="absolute {isRtl ? 'left-3.5 border-r' : 'right-3.5 border-l'} top-3.5 bottom-3.5 flex shrink-0 flex-col bg-card text-card-foreground rounded-xl overflow-hidden border border-border shadow-2xl z-50 select-none"
  style:width="{preferredWidth}rem"
  style:min-width="{preferredWidth}rem"
  transition:fly={{ x: isRtl ? -400 : 400, duration: 250 }}
>
  <!-- Resize Handle on the left/right edge -->
  <Resizer
    viewport={sidebarDiv}
    direction={isRtl ? "right" : "left"}
    {minWidth}
    {maxWidth}
    defaultValue={pxToRem(380)}
    persistId="ask-orbis-sidebar-width"
    onWidth={(w) => (preferredWidth = w)}
  />

  <!-- Header Section -->
  <div class="flex items-center justify-between px-3.5 h-11 shrink-0 border-b border-border select-none bg-card">
    <div class="flex items-center gap-2">
      <!-- Close Button -->
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={handleClose}
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="Close sidebar"
      >
        <X size={14} strokeWidth={2} />
      </Button>
      
      <!-- Title -->
      <span class="text-[12px] font-semibold tracking-wide text-foreground flex items-center gap-1.5">
        {m.agent_title()} 🧙
      </span>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-1">
      <Button
        variant="ghost"
        size="icon-xs"
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="More options"
      >
        <MoreHorizontal size={14} />
      </Button>
      
      <Button
        variant="ghost"
        size="icon-xs"
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="Split layout"
      >
        <Columns size={13} />
      </Button>

      <Button
        variant="ghost"
        size="icon-xs"
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="History"
      >
        <History size={13} />
      </Button>

      <Button
        variant="ghost"
        size="icon-xs"
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="New Chat"
      >
        <Plus size={14} />
      </Button>
    </div>
  </div>

  <!-- Content Section -->
  <div class="flex-1 flex flex-col items-center justify-center px-6 overflow-y-auto pb-8 select-none bg-card">
    <!-- Eye-Triangle Agent Logo -->
    <div class="text-muted-foreground mb-6 drop-shadow-[0_0_15px_rgba(0,0,0,0.02)]">
      <svg width="68" height="68" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <!-- Outer Triangle -->
        <path d="M12 3L2 20h20L12 3z" stroke-linejoin="round" />
        <!-- Eye top arch -->
        <path d="M7.2 14.2c2.4-2.2 7.2-2.2 9.6 0" />
        <!-- Eye bottom arch -->
        <path d="M7.2 14.2c2.4 2.2 7.2 2.2 9.6 0" />
        <!-- Pupil -->
        <circle cx="12" cy="14.2" r="2.2" fill="currentColor" />
      </svg>
    </div>

    <!-- Subtitle -->
    <p class="text-[13px] text-foreground/90 text-center font-medium max-w-[280px] leading-relaxed mb-6 select-none">
      {m.agent_subtitle()}
    </p>

    <!-- Suggestion Buttons -->
    <div class="flex flex-col gap-2 w-full max-w-[340px]">
      {#each suggestions as sug}
        <Button
          variant="outline"
          onclick={() => handleSuggestionClick(sug)}
          class="w-full text-center py-3 px-4 text-[11.5px] font-medium text-foreground/80 bg-card border border-border/80 hover:border-zinc-400 dark:hover:border-zinc-600 hover:bg-muted/40 hover:text-foreground rounded-lg cursor-pointer transition-all duration-150 active:scale-[0.99] leading-relaxed whitespace-normal h-auto wrap-break-word shadow-sm"
        >
          {sug}
        </Button>
      {/each}
    </div>
  </div>

  <!-- Footer Input Area -->
  <div class="p-3.5 shrink-0 bg-card select-none">
    <form 
      onsubmit={handleSubmit} 
      class="flex items-center gap-1.5 bg-muted/20 border border-border rounded-lg focus-within:border-zinc-400 dark:focus-within:border-zinc-600 focus-within:ring-1 focus-within:ring-zinc-450/10 dark:focus-within:ring-zinc-550/10 transition-all px-2.5 py-1.5"
    >
      <Input
        type="text"
        bind:value={inputValue}
        onkeydown={handleKeyDown}
        placeholder={m.agent_input_placeholder()}
        class="flex-1 bg-transparent dark:bg-transparent border-none outline-none text-foreground placeholder-muted-foreground text-[11.5px] px-1 h-7 shadow-none focus-visible:ring-0 focus-visible:ring-offset-0 focus:ring-0"
      />
      <Button
        type="submit"
        disabled={!inputValue.trim()}
        class="w-7 h-7 flex items-center justify-center rounded-md bg-muted/40 hover:bg-muted/80 text-muted-foreground hover:text-foreground disabled:opacity-40 disabled:hover:bg-muted/40 disabled:hover:text-muted-foreground cursor-pointer border border-border/50 transition-colors"
      >
        {#if onboardingState.locale === 'ar'}
          <ArrowLeft size={13} strokeWidth={2.2} />
        {:else}
          <ArrowRight size={13} strokeWidth={2.2} />
        {/if}
      </Button>
    </form>
  </div>
</div>
