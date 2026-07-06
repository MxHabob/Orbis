<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Textarea } from "@orbis/ui-svelte/textarea";
import { Plus, ArrowRight, ArrowLeft } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";

let {
  value = $bindable(""),
  isThinking = false,
  messagesLength = 0,
  isRtl = false,
  onsubmit,
  onnewchat,
}: {
  value?: string;
  isThinking?: boolean;
  messagesLength?: number;
  isRtl?: boolean;
  onsubmit: (query: string) => void;
  onnewchat: () => void;
} = $props();

let textareaEl = $state<HTMLTextAreaElement | null>(null);

function handleInput(e: Event) {
  const target = e.target as HTMLTextAreaElement;
  target.style.height = "auto";
  target.style.height = `${Math.min(target.scrollHeight, 120)}px`;
}

function handleSubmit(e: SubmitEvent | KeyboardEvent) {
  if (e) e.preventDefault();
  const trimmed = value.trim();
  if (trimmed && !isThinking) {
    onsubmit(trimmed);
    value = "";
    if (textareaEl) {
      textareaEl.style.height = "auto";
    }
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSubmit(e);
  }
}

function handleNewChat() {
  value = "";
  if (textareaEl) {
    textareaEl.style.height = "auto";
  }
  onnewchat();
}
</script>

<div class="p-3.5 shrink-0 select-none bg-card">
  <form onsubmit={handleSubmit} class="flex flex-col gap-2">
    <Textarea
      bind:ref={textareaEl}
      bind:value={value}
      onkeydown={handleKeyDown}
      oninput={handleInput}
      placeholder={m.agent_input_placeholder()}
      class="w-full resize-none scrollbar-hide text-xs"
      disabled={isThinking}
    />
    
    <div class="flex items-center justify-between px-1">
      <!-- Input Tools (Left-side/Right-side depending on RTL) -->
      <div class="flex items-center gap-1.5">
        <!-- Clear/New Chat button -->
        <Button
          variant="ghost"
          size="icon-xs"
          onclick={handleNewChat}
          type="button"
          class="h-7 w-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/50 cursor-pointer"
          title={m.agent_input_new_chat()}
          disabled={messagesLength === 0}
        >
          <Plus size={14} />
        </Button>
        
        <!-- Tip/Helper text or indicator -->
        {#if value.trim().length > 0}
          <span class="text-[9px] text-muted-foreground/60 font-medium select-none">
            {value.length} ch
          </span>
        {:else}
          <span class="text-[9px] text-muted-foreground/50 font-medium select-none">
            {m.agent_input_newline_tip()}
          </span>
        {/if}
      </div>

      <!-- Submit Button -->
      <Button
        type="submit"
        disabled={!value.trim() || isThinking}
        class="h-7 w-7 rounded-lg bg-primary hover:bg-primary/95 text-primary-foreground disabled:bg-muted/40 disabled:text-muted-foreground/60 disabled:opacity-50 cursor-pointer flex items-center justify-center transition-colors shrink-0"
      >
        {#if isRtl}
          <ArrowLeft size={13} strokeWidth={2.5} />
        {:else}
          <ArrowRight size={13} strokeWidth={2.5} />
        {/if}
      </Button>
    </div>
  </form>
</div>
