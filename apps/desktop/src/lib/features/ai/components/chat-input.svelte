<script lang="ts">
import { Button } from "@orbis/ui/button";
import { ArrowRight, Plus, Sparkles } from "@orbis/icons";
import ProviderModelPicker from "./provider-model-picker.svelte";

let {
  value = $bindable(""),
  isThinking = false,
  messagesLength = 0,
  selectedModel = $bindable(""),
  selectedProvider = $bindable(""),
  orgId = "",
  onsubmit,
  onnewchat,
  onOpenSettings,
}: {
  value?: string;
  isThinking?: boolean;
  messagesLength?: number;
  selectedModel?: string;
  selectedProvider?: string;
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
    label: "إحصائيات وحالة الطلاب",
    desc: "البحث عن الطلاب ومعلوماتهم الأكاديمية والمالية",
    prompt: "ما هي إحصائيات الطلاب وحالتهم الأكاديمية والمالية الحالية؟",
  },
  {
    id: "finance",
    label: "الملخص المالي والرسوم",
    desc: "عرض الرسوم المحصلة وغير المدفوعة والمطالبات المتأخرة",
    prompt: "قدم تقريراً مالياً شاملاً عن الرسوم المحصلة والمتبقية",
  },
  {
    id: "academic",
    label: "الفصول والجدول الدراسي",
    desc: "عرض الفصول والمواد الدراسية النشطة والجدول الأسبوعي",
    prompt: "ما هي الفصول والمواد الدراسية النشطة في هذا الفصل؟",
  },
  {
    id: "report",
    label: "توليد تقرير إداري شامل",
    desc: "تحليل مؤشرات الأداء، الحضور والغياب، والتقييمات",
    prompt: "أنشئ تقريراً إحصائياً شاملاً عن أداء المدرسة هذا الفصل",
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
    <div class="absolute bottom-[calc(100%+4px)] inset-x-3.5 p-2 bg-card/95 border border-border/80 shadow-2xl rounded-2xl z-30 flex flex-col gap-1 backdrop-blur-2xl animate-in fade-in slide-in-from-bottom-2 duration-150 ring-1 ring-black/5 dark:ring-white/10">
      <div class="text-[10px] text-muted-foreground px-2 py-1 uppercase font-bold tracking-wider flex items-center gap-1.5 border-b border-border/40 pb-1.5">
        <Sparkles size={11} class="text-primary" />
        أوامر ذكية مقترحة
      </div>
      {#each commandOptions as cmd}
        <button 
          type="button" 
          class="w-full text-start px-2.5 py-2 flex flex-col gap-0.5 hover:bg-muted/80 rounded-xl transition-colors cursor-pointer group"
          onclick={() => handleCommandSelect(cmd.prompt)}
        >
          <span class="text-[12px] font-bold text-foreground group-hover:text-primary transition-colors">{cmd.label}</span>
          <span class="text-[10px] text-muted-foreground leading-tight">{cmd.desc}</span>
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
      value={value}
      disabled={isThinking}
      onkeydown={handleKeyDown}
      oninput={handleInput}
      placeholder="اسأل Orbis عن أي شيء... أو اكتب / للأوامر"
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
          title="محادثة جديدة"
          onclick={() => handleNewChat()}
        >
          <Plus class="h-4 w-4" />
        </Button>

        <!-- Responsive Provider & Model Selector -->
        <ProviderModelPicker
          bind:selectedModel={selectedModel}
          bind:selectedProvider={selectedProvider}
          orgId={orgId}
          onOpenSettings={onOpenSettings}
        />
      </div>

      <!-- Right/End side: Send Button with RTL CSS auto-rotation -->
      <div class="flex items-center gap-1.5 shrink-0">
        <Button
          type="submit"
          aria-label="إرسال"
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