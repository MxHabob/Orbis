<!--
@component AskOrbisSidebar
A resizable right-hand sidebar overlay for Ask Orbis agent, mirroring the Seer design.
Allows users to ask questions, select prompt suggestions, and collapse/resize.
-->
<script lang="ts">
import { uiStore, authStore } from "$lib/stores";
import Resizer from "./resizer.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "$lib/features/onboarding/state.svelte";
import {
  X,
  MoreHorizontal,
  Columns,
  Plus,
  ArrowRight,
  ArrowLeft,
} from "lucide-svelte";
import { fly } from "svelte/transition";
import { getLocale } from "$lib/paraglide/runtime";
import { isDesktopApp } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import ChatHistoryPopover from "./chat-history-popover.svelte";

let { isStandalone = false }: { isStandalone?: boolean } = $props();

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

const userName = $derived(authStore.user?.full_name || (isRtl ? "مستخدم" : "User"));
const orgName = $derived(
  (authStore.user as any)?.org_name ||
  (authStore.user as any)?.organization_name ||
  (authStore.user as any)?.org?.name ||
  null
);

let inputValue = $state("");
const suggestions = $derived([
  m.agent_suggest_1(),
  m.agent_suggest_2(),
  m.agent_suggest_3(),
]);

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
}

let messages = $state<Message[]>([]);
let isThinking = $state(false);
let chatContainer = $state<HTMLDivElement>();

function scrollToBottom() {
  if (chatContainer) {
    setTimeout(() => {
      chatContainer.scrollTo({
        top: chatContainer.scrollHeight,
        behavior: "smooth",
      });
    }, 50);
  }
}

function handleSuggestionClick(suggestion: string) {
  submitMessage(suggestion);
}

function handleClose() {
  uiStore.setAskOrbis(false);
}

async function openStandaloneWindow() {
  if (!isDesktopApp()) return;
  try {
    const { commands } = await import("$lib/core/ipc");

    // Close the current sidebar overlay
    uiStore.setAskOrbis(false);

    // Create standalone window securely via Rust backend
    await commands.window.openAgent();
  } catch (e: any) {
    console.error("Failed to open standalone window:", e);
    toast.error(`Failed to open window: ${e?.message || e}`);
  }
}

async function submitMessage(query: string) {
  if (!query.trim()) return;

  inputValue = "";
  const isNewChat = messages.length === 0;

  messages.push({
    id: Math.random().toString(36).substring(7),
    role: "user",
    content: query,
  });

  scrollToBottom();
  isThinking = true;

  // Simulate a realistic agent thinking time
  await new Promise((resolve) => setTimeout(resolve, 800));

  isThinking = false;

  messages.push({
    id: Math.random().toString(36).substring(7),
    role: "assistant",
    content: "",
  });



  scrollToBottom();
}

function handleSubmit(e: SubmitEvent | KeyboardEvent) {
  if (e) e.preventDefault();
  submitMessage(inputValue);
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSubmit(e);
  }
}

function handleNewChat() {
  messages = [];
}

function handleSelectHistoryItem(messagesToLoad: Message[]) {
  messages = [...messagesToLoad];
  scrollToBottom();
}
</script>

<div
  bind:this={sidebarDiv}
  class={isStandalone 
    ? "relative flex w-full h-full flex-col bg-card text-card-foreground overflow-hidden select-none"
    : "absolute " + (isRtl ? 'left-3.5 border-r' : 'right-3.5 border-l') + " top-3.5 bottom-3.5 flex shrink-0 flex-col bg-card text-card-foreground rounded-xl overflow-hidden border border-border shadow-2xl z-50 select-none"}
  style:width={isStandalone ? "100%" : `${preferredWidth}rem`}
  style:min-width={isStandalone ? "100%" : `${preferredWidth}rem`}
  style:max-width={isStandalone ? "100%" : `${preferredWidth}rem`}
  transition:fly={isStandalone ? undefined : { x: isRtl ? -400 : 400, duration: 250 }}
>
  <!-- Resize Handle on the left/right edge -->
  {#if !isStandalone}
    <Resizer
      viewport={sidebarDiv}
      direction={isRtl ? "left" : "right"}
      {minWidth}
      {maxWidth}
      defaultValue={pxToRem(425)}
      persistId="ask-orbis-sidebar-width"
      onWidth={(w) => (preferredWidth = w)}
    />
  {/if}

  <!-- Header Section -->
  <div class="flex items-center justify-between px-3.5 h-11 shrink-0 border-b border-border select-none bg-card">
    <div class="flex items-center gap-2">
      <!-- Close Button -->
      {#if !isStandalone}
        <Button
          variant="ghost"
          size="icon-xs"
          onclick={handleClose}
          class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
          title="Close sidebar"
        >
          <X size={14} strokeWidth={2} />
        </Button>
      {/if}
      
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
      
      {#if !isStandalone}
        <Button
          variant="ghost"
          size="icon-xs"
          onclick={openStandaloneWindow}
          class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
          title="Split layout"
        >
          <Columns size={13} />
        </Button>
      {/if}

      <ChatHistoryPopover
        onSelect={handleSelectHistoryItem}
      />

      <Button
        variant="ghost"
        size="icon-xs"
        onclick={handleNewChat}
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="New Chat"
      >
        <Plus size={14} />
      </Button>
    </div>
  </div>

  <!-- Content Section -->
  <div 
    bind:this={chatContainer}
    class="flex-1 flex flex-col p-4 overflow-y-auto scrollbar-hide select-text"
  >
    {#if messages.length === 0}
      <div class="flex-1 flex flex-col items-center justify-center px-6 pb-8 select-none">
        <!-- Eye-Triangle Agent Logo -->
        <div class="text-muted-foreground mb-6 drop-shadow-[0_0_15px_rgba(0,0,0,0.02)]">
          <svg width="68" height="68" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 3L2 20h20L12 3z" stroke-linejoin="round" />
            <path d="M7.2 14.2c2.4-2.2 7.2-2.2 9.6 0" />
            <path d="M7.2 14.2c2.4 2.2 7.2 2.2 9.6 0" />
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
    {:else}
      <div class="flex flex-col gap-5 w-full">
        {#each messages as msg}
          {#if msg.role === 'user'}
            <!-- User message bubble on the right -->
            <div class="flex justify-end w-full">
              <div class="max-w-[85%] bg-muted text-foreground px-4 py-2.5 rounded-2xl text-[12.5px] leading-relaxed break-words font-medium shadow-sm">
                {msg.content}
              </div>
            </div>
          {:else}
            <!-- Agent response on the left -->
            <div class="flex flex-col gap-3 w-full text-start text-[13px] text-foreground/90 leading-relaxed font-normal">
              <p>
                {#if orgName}
                  {m.agent_response_greeting_with_org({
                    user: userName,
                    org: orgName,
                  })}
                {:else}
                  {m.agent_response_greeting_without_org({
                    user: userName,
                  })}
                {/if}
              </p>
              <p class="mt-1">
                {m.agent_response_question()}
              </p>
              <ul class="list-disc ps-5 flex flex-col gap-2 mt-1">
                <li>{m.agent_response_bullet_1()}</li>
                <li>{m.agent_response_bullet_2()}</li>
                <li>{m.agent_response_bullet_3()}</li>
              </ul>
            </div>
          {/if}
        {/each}

        <!-- Thinking/loading skeleton -->
        {#if isThinking}
          <div class="flex flex-col gap-3 w-full text-start mt-2">
            <Skeleton class="h-4 w-3/4 rounded-md" />
            <Skeleton class="h-4 w-5/6 rounded-md" />
            <Skeleton class="h-4 w-1/2 rounded-md" />
            <div class="flex flex-col gap-2 ps-5 mt-2">
              <Skeleton class="h-3.5 w-2/3 rounded-md" />
              <Skeleton class="h-3.5 w-3/4 rounded-md" />
              <Skeleton class="h-3.5 w-3/5 rounded-md" />
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Footer Input Area -->
  <div class="p-3.5 shrink-0 select-none">
    <form 
      onsubmit={handleSubmit} 
      class="flex items-center gap-2.5 w-full"
    >
        <Input
          type="text"
          bind:value={inputValue}
          onkeydown={handleKeyDown}
          placeholder={m.agent_input_placeholder()}
          class="flex-1 outline-none text-[11.5px] px-1 h-8 shadow-none"
        />
      <Button
        type="submit"
        disabled={!inputValue.trim() && !isThinking}
        class="w-8 h-8 flex items-center justify-center rounded-lg bg-muted/40 hover:bg-muted/80 text-muted-foreground hover:text-foreground disabled:opacity-40 disabled:hover:bg-muted/40 disabled:hover:text-muted-foreground cursor-pointer border border-border/50 transition-colors shrink-0"
      >
        {#if isRtl}
          <ArrowLeft size={14} strokeWidth={2.2} />
        {:else}
          <ArrowRight size={14} strokeWidth={2.2} />
        {/if}
      </Button>
    </form>
  </div>
</div>
