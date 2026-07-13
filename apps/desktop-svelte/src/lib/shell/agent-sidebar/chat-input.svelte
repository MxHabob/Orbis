<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
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

let inputEl = $state<HTMLDivElement | null>(null);

function getPlaintextFromHtml(html: string): string {
  if (typeof document === "undefined") return html;
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;
  return tempDiv.textContent || "";
}

function handleInput(e: Event) {
  const target = e.currentTarget as HTMLDivElement;
  value = getPlaintextFromHtml(target.innerHTML);
}

function handleSubmit(e: SubmitEvent | KeyboardEvent | Event) {
  if (e) e.preventDefault();
  const trimmed = value.trim();
  if (trimmed && !isThinking) {
    onsubmit(trimmed);
    value = "";
    if (inputEl) {
      inputEl.innerHTML = "";
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
  if (inputEl) {
    inputEl.innerHTML = "";
  }
  onnewchat();
}

$effect(() => {
  if (value === "" && inputEl && inputEl.innerHTML !== "") {
    inputEl.innerHTML = "";
  }
});
</script>

<div class="relative p-3.5 shrink-0 select-none bg-card">
  <form onsubmit={handleSubmit} class="flex flex-col gap-2">
    <div
      bind:this={inputEl}
      contenteditable={!isThinking}
      onkeydown={handleKeyDown}
      oninput={handleInput}
      role="textbox"
      tabindex="0"
      class="w-full min-h-[48px] max-h-[160px] overflow-y-auto bg-muted/30 focus:bg-muted/50 border border-border/50 text-foreground text-[13px] rounded-xl px-4 py-3 outline-none transition-all resize-none shadow-sm placeholder:text-muted-foreground/60 empty:before:content-[attr(data-placeholder)] empty:before:text-muted-foreground/60"
      data-placeholder={m.agent_chat_placeholder()}
      dir={isRtl ? "rtl" : "ltr"}
    ></div>

    <div class="flex items-center justify-between px-1">
      <div class="flex items-center gap-1.5">
        <Button
          type="button"
          variant="ghost"
          size="icon"
          class="h-8 w-8 rounded-full text-muted-foreground hover:text-foreground hover:bg-muted/60"
          title={m.agent_chat_new()}
          onclick={handleNewChat}
        >
          <Plus class="h-4 w-4" />
        </Button>
      </div>

      <div class="flex items-center gap-2">
        {#if messagesLength > 0}
          <span class="text-[10px] text-muted-foreground/60 hidden sm:inline-block">
            {m.agent_chat_shortcut_enter()}
          </span>
        {/if}

        <Button
          type="submit"
          disabled={!value.trim() || isThinking}
          class="h-8 w-8 rounded-full shadow-sm bg-primary hover:bg-primary/90 text-primary-foreground disabled:opacity-50 disabled:bg-muted transition-all duration-200"
          size="icon"
        >
          {#if isRtl}
            <ArrowLeft class="h-4 w-4" />
          {:else}
            <ArrowRight class="h-4 w-4" />
          {/if}
        </Button>
      </div>
    </div>
  </form>
</div>