<script lang="ts">
import { Button } from "@orbis/ui/button";
import { AdjustmentsHorizontal, ArrowLeft, ArrowRight, Plus, Sparkles } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
import { Textarea } from "@orbis/ui/textarea";
import { getLocale } from "$lib/paraglide/runtime";

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

const isRtl = $derived(getLocale() === "ar");
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
    label: "البحث عن الطلاب ومعلوماتهم",
    desc: "البحث بالاسم أو الرقم القومي وعرض تفاصيل الطالب وحالته الأكاديمية والمالية",
    prompt: "ابحث عن الطلاب في النظام وقدم ملخصاً عن حالتهم الأكاديمية",
  },
  {
    id: "finance",
    label: "الملخص المالي والرسوم",
    desc: "عرض إجمالي الرسوم المحصلة وغير المدفوعة والمطالبات المتأخرة",
    prompt: "قدم ملخصاً شاملاً عن الوضع المالي والرسوم المستحقة وغير المدفوعة",
  },
  {
    id: "academic",
    label: "الفصول والجدول الدراسي",
    desc: "عرض قائمة الفصول الدراسية والمواد والجدول الأسبوعي",
    prompt: "ما هي الفصول والمواد الدراسية النشطة في الفصل الحالي؟",
  },
  {
    id: "report",
    label: "إنشاء تقرير إداري شامل",
    desc: "توليد تقرير عن الطلاب أو الأداء أو الحضور أو العمليات",
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

<div class="relative shrink-0 select-none px-3 py-3 border-t border-border/60 bg-card/60 backdrop-blur-sm">
  <!-- Quick Commands Menu -->
  {#if showCommands}
    <div class="absolute bottom-[calc(100%-8px)] inset-s-3 inset-e-3 p-2 mb-2 bg-popover border border-border/80 shadow-2xl rounded-xl z-30 flex flex-col gap-1 overflow-hidden animate-in fade-in slide-in-from-bottom-2 duration-150">
      <div class="text-[10px] text-muted-foreground px-2 py-1 uppercase font-bold tracking-wider flex items-center gap-1">
        <Sparkles size={11} class="text-primary" />
        أوامر سريعة مقترحة
      </div>
      {#each commandOptions as cmd}
        <button 
          type="button" 
          class="w-full text-start px-3 py-2 flex flex-col gap-0.5 hover:bg-muted/80 rounded-lg transition-colors cursor-pointer group"
          onclick={() => handleCommandSelect(cmd.prompt)}
        >
          <span class="text-[12.5px] font-semibold text-foreground group-hover:text-primary transition-colors">{cmd.label}</span>
          <span class="text-[10px] text-muted-foreground">{cmd.desc}</span>
        </button>
      {/each}
    </div>
  {/if}

  <form onsubmit={handleSubmit} class="flex flex-col gap-2">
    <Textarea
      bind:this={inputEl}
      value={value}
      disabled={isThinking}
      onkeydown={handleKeyDown}
      oninput={handleInput}
      placeholder="اسأل Orbis عن أي شيء في النظام أو اكتب / للأوامر..."
      rows="2"
      class="w-full min-h-12 max-h-36 resize-none overflow-y-auto text-[13px] leading-5 rounded-xl px-3 py-2.5 outline-none shadow-xs border-border/80 focus:border-primary/60 bg-background"
    ></Textarea>

    <div class="flex items-center justify-between px-1">
      <div class="flex items-center gap-1.5">
        <Button
          type="button"
          variant="ghost"
          size="icon-xs"
          class="h-7 w-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
          title="محادثة جديدة"
          onclick={() => handleNewChat()}
        >
          <Plus class="h-4 w-4" />
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <span class="text-[10px] text-muted-foreground/60 hidden sm:inline-block">
          اضغط Enter للإرسال
        </span>

        <Button
          type="submit"
          aria-label="إرسال"
          disabled={!value.trim() || isThinking}
          class="h-7.5 w-7.5 rounded-lg shadow-sm bg-primary hover:bg-primary/90 text-primary-foreground disabled:opacity-40 disabled:bg-muted transition-all cursor-pointer flex items-center justify-center p-0"
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