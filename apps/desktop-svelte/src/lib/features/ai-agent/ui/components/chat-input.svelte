<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Plus, ArrowRight, ArrowLeft } from "@lucide/svelte";
import * as m from "$lib/paraglide/messages";
    import { Textarea } from "@orbis/ui-svelte/textarea";

let {
  value = $bindable(""),
  isThinking = false,
  messagesLength = 0,
  onsubmit,
  onnewchat,
}: {
  value?: string;
  isThinking?: boolean;
  messagesLength?: number;
  onsubmit: (query: string) => void;
  onnewchat: (agentType?: string) => void;
} = $props();

let inputEl = $state<HTMLTextAreaElement | null>(null);

function handleInput(e: Event) {
  value = (e.currentTarget as HTMLTextAreaElement).value;
}

function handleSubmit(e: SubmitEvent | KeyboardEvent | Event) {
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

let attachments = $state<File[]>([]);
let isDragging = $state(false);

function handleDragOver(e: DragEvent) {
  e.preventDefault();
  isDragging = true;
}

function handleDragLeave(e: DragEvent) {
  e.preventDefault();
  isDragging = false;
}

function handleDrop(e: DragEvent) {
  e.preventDefault();
  isDragging = false;
  if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
    attachments = [...attachments, ...Array.from(e.dataTransfer.files)];
  }
}

function removeAttachment(index: number) {
  attachments = attachments.filter((_, i) => i !== index);
}

$effect(() => {
  if (inputEl && inputEl.value !== value) inputEl.value = value;
});

const showCommands = $derived(value === "/");

const commandOptions = [
  {
    id: "academic",
    label: () => m.agent_command_academic_label(),
    desc: () => m.agent_command_academic_desc(),
  },
  {
    id: "finance",
    label: () => m.agent_command_finance_label(),
    desc: () => m.agent_command_finance_desc(),
  },
  { id: "exam", label: () => m.agent_command_exam_label(), desc: () => m.agent_command_exam_desc() },
  { id: "default", label: () => m.agent_command_default_label(), desc: () => m.agent_command_default_desc() },
];

function handleCommandSelect(agentType: string) {
  value = "";
  if (inputEl) inputEl.value = "";
  handleNewChat(agentType);
}
</script>

<div 
  class="relative shrink-0 select-none px-3 py-3 {isDragging ? 'ring-2 ring-primary/40 bg-primary/5' : ''}"
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  role="presentation"
>
  <!-- Drop overlay -->
  {#if isDragging}
    <div class="absolute inset-2 z-20 flex items-center justify-center rounded-lg border border-dashed border-primary bg-background/95 backdrop-blur-sm">
      <div class="flex flex-col items-center gap-2 text-primary">
        <Plus class="w-8 h-8 animate-bounce" />
        <span class="text-sm font-bold">{m.agent_chat_drop_files()}</span>
      </div>
    </div>
  {/if}

  <!-- Quick Commands Menu -->
  {#if showCommands}
    <div class="absolute bottom-[calc(100%-8px)] inset-s-3 inset-e-3 p-2 mb-2 bg-popover border border-border shadow-lg rounded-lg z-30 flex flex-col gap-1 overflow-hidden animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div class="text-[10px] text-muted-foreground px-2 py-1 uppercase font-bold tracking-wider">{m.agent_chat_select_agent()}</div>
      {#each commandOptions as cmd}
        <button 
          type="button" 
          class="w-full text-start px-3 py-2 flex flex-col gap-0.5 hover:bg-muted/80 rounded-md transition-colors cursor-pointer group"
          onclick={() => handleCommandSelect(cmd.id)}
        >
          <span class="text-[13px] font-semibold text-foreground group-hover:text-primary transition-colors">{cmd.label()}</span>
          <span class="text-[10px] text-muted-foreground">{cmd.desc()}</span>
        </button>
      {/each}
    </div>
  {/if}

  <form onsubmit={handleSubmit} class="flex flex-col gap-2">
    <!-- Attachments Display -->
    {#if attachments.length > 0}
      <div class="flex gap-2 px-1 flex-wrap">
        {#each attachments as file, i}
          <div class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-[11px] font-medium">
            <span class="truncate max-w-30">{file.name}</span>
            <button type="button" aria-label={m.agent_chat_remove_attachment()} class="text-muted-foreground hover:text-destructive cursor-pointer" onclick={() => removeAttachment(i)}>
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
          </div>
        {/each}
      </div>
    {/if}

    <Textarea
      bind:this={inputEl}
      value={value}
      disabled={isThinking}
      onkeydown={handleKeyDown}
      oninput={handleInput}
      placeholder={m.agent_chat_placeholder()}
      rows="2"
      class="w-full min-h-11 max-h-40 resize-y overflow-y-auto text-[13px] leading-5 rounded-lg  px-3 py-2.5 outline-none shadow-xs"
    ></Textarea>

    <div class="flex items-center justify-between px-1">
      <div class="flex items-center gap-1.5">
        <Button
          type="button"
          variant="ghost"
          size="icon"
          class="h-8 w-8 rounded-full text-muted-foreground hover:text-foreground hover:bg-muted/60"
          title={m.agent_chat_new()}
          onclick={() => handleNewChat()}
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
          aria-label={m.agent_chat_send()}
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