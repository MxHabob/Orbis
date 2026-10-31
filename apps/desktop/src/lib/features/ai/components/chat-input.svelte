<script lang="ts">
import { Button } from "@orbis/ui/button";
import { ArrowRight, Plus } from "@orbis/icons";
import ProviderModelPicker from "./provider-model-picker.svelte";
import * as m from "$lib/paraglide/messages";

let {
  value = $bindable(""),
  isThinking = false,
  messagesLength = 0,
  orgId = "",
  onsubmit,
  onnewchat,
  onOpenSettings,
}: {
  value?: string;
  isThinking?: boolean;
  messagesLength?: number;
  orgId?: string;
  onsubmit: (query: string) => void;
  onnewchat: (agentType?: string) => void;
  onOpenSettings?: () => void;
} = $props();

let inputEl = $state<HTMLTextAreaElement | null>(null);

function handleInput(e: Event) {
  value = (e.currentTarget as HTMLTextAreaElement).value;
}

function handleSubmit(e?: SubmitEvent | KeyboardEvent | Event) {
  if (e) e.preventDefault();
  const trimmed = value.trim();
  if (trimmed && !isThinking) {
    onsubmit(trimmed);
    value = "";
    if (inputEl) inputEl.value = "";
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSubmit(e);
  }
}

function handleNewChat(type: string = "default") {
  value = "";
  if (inputEl) inputEl.value = "";
  onnewchat(type);
}

$effect(() => {
  if (inputEl && inputEl.value !== value) inputEl.value = value;
});

const showCommands = $derived(value === "/");

const commandOptions = [
  {
    id: "students",
    label: m.ai_ui_student_stats_label(),
    desc: m.ai_ui_student_stats_description(),
    prompt: m.ai_ui_student_stats_prompt(),
  },
  {
    id: "finance",
    label: m.ai_ui_financial_summary_label(),
    desc: m.ai_ui_financial_summary_description(),
    prompt: m.ai_ui_financial_summary_prompt(),
  },
  {
    id: "academic",
    label: m.ai_ui_classes_schedule_label(),
    desc: m.ai_ui_classes_schedule_description(),
    prompt: m.ai_ui_classes_schedule_prompt(),
  },
  {
    id: "report",
    label: m.ai_ui_admin_report_label(),
    desc: m.ai_ui_admin_report_description(),
    prompt: m.ai_ui_admin_report_prompt(),
  },
];

function handleCommandSelect(prompt: string) {
  value = prompt;
  if (inputEl) {
    inputEl.value = prompt;
    inputEl.focus();
  }
}
</script>

<div class="relative shrink-0 select-none px-3.5 pb-3.5 pt-1.5 bg-gradient-to-t from-card via-card/90 to-transparent">
  <!-- Quick Commands Popup Menu -->
  {#if showCommands}
    <div class="absolute bottom-[calc(100%+4px)] inset-x-3.5 z-30 flex max-h-[min(20rem,calc(100vh-8rem))] flex-col gap-0 overflow-y-auto rounded-md border border-border bg-card p-1 shadow-md animate-in fade-in slide-in-from-bottom-2 duration-150">
      <div class="border-b border-border px-2 py-2 text-[10px] font-semibold text-muted-foreground">
        {m.ai_ui_suggested_smart_commands()}
      </div>
      {#each commandOptions as cmd (cmd.id)}
        <button 
          type="button" 
          class="group flex w-full cursor-pointer flex-col gap-0.5 rounded-md px-2.5 py-2 text-start transition-colors hover:bg-muted/50"
          onclick={() => handleCommandSelect(cmd.prompt)}
        >
          <span class="text-xs font-semibold text-foreground group-hover:text-primary">{cmd.label}</span>
          <span class="text-[10px] leading-tight text-muted-foreground [overflow-wrap:anywhere]">{cmd.desc}</span>
        </button>
      {/each}
    </div>
  {/if}

  <!-- Unified Floating Input Capsule -->
  <form 
    onsubmit={handleSubmit} 
    class="flex flex-col bg-background/95 border border-border/80 focus-within:border-primary/60 focus-within:ring-2 focus-within:ring-primary/15 rounded-2xl p-2 shadow-xs transition-all"
  >
    <!-- Textarea Input Area -->
    <textarea
      bind:this={inputEl}
      dir="auto"
      value={value}
      disabled={isThinking}
      onkeydown={handleKeyDown}
      oninput={handleInput}
      placeholder={m.ai_ui_chat_input_placeholder()}
      rows="2"
      class="w-full min-h-[44px] max-h-32 resize-none overflow-y-auto text-[13px] leading-relaxed px-2 py-1 outline-none border-0 bg-transparent text-foreground placeholder:text-muted-foreground/60 scrollbar-thin"
    ></textarea>

    <!-- Bottom Actions Toolbar -->
    <div class="flex items-center justify-between gap-1.5 pt-1.5 border-t border-border/40 min-w-0">
      <!-- Left/Start side: New chat + Provider/Model Picker -->
      <div class="flex items-center gap-1 min-w-0 flex-1 overflow-hidden">
        <Button
          type="button"
          variant="ghost"
          size="icon-xs"
          class="h-7 w-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted active:scale-95 cursor-pointer shrink-0"
          title={m.ai_ui_new_chat()}
          onclick={() => handleNewChat()}
        >
          <Plus class="h-4 w-4" />
        </Button>

        <!-- Responsive Provider & Model Selector -->
        <ProviderModelPicker
          orgId={orgId}
          onOpenSettings={onOpenSettings}
        />
      </div>

      <!-- Right/End side: Send Button with RTL CSS auto-rotation -->
      <div class="flex items-center gap-1.5 shrink-0">
        <Button
          type="submit"
          aria-label={m.ai_ui_send()}
          disabled={!value.trim() || isThinking}
          class="h-7.5 w-7.5 rounded-xl shadow-sm bg-primary hover:bg-primary/90 active:scale-95 text-primary-foreground disabled:opacity-30 disabled:bg-muted disabled:text-muted-foreground transition-all cursor-pointer flex items-center justify-center p-0 shrink-0"
          size="icon"
        >
          <ArrowRight class="h-3.5 w-3.5 rtl:rotate-180" />
        </Button>
      </div>
    </div>
  </form>
</div>