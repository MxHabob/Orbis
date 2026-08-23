<!--
@component AiEmptyState
Ultra-fast, responsive & highly flexible AI Assistant Empty State Component.
Features:
- Supports `compact` mode for tight containers/table cells/modals and standard mode for full pages
- Dynamic random hat selection per view (`hat="random"`) featuring 8 premium vector hats (Crown, Graduation, Firefighter, Wizard, Detective, Chef, Astronaut, Headset)
- Fixed top-aligned stationary Avatar (self-start) that never shifts during text expansion
- Center-aligned container block (justify-center)
- Fast clean character streaming
- Midday/Notion minimal design system
-->
<script lang="ts">
import { onDestroy } from "svelte";
import { Avatar, AvatarController, type AvatarHat } from "@orbis/avatar-engine";
import { Button } from "@orbis/ui-svelte/button";
import { Sparkles, ArrowRight } from "@lucide/svelte";

let {
  text = "أهلاً بك! لا توجد بيانات مسجلة هنا حالياً. يمكنك بدء العمل عبر إضافة أول عنصر أو تشغيل الأتمتة التلقائية.",
  actionLabel = "",
  onaction = undefined,
  hat = "random" as AvatarHat, // automatically picks a random hat per view for delight!
  speed = 6, // ultra-fast streaming (6ms per char)
  compact = false, // flexible mode for tight containers / modals / table cells
  class: className = "",
}: {
  text?: string;
  actionLabel?: string;
  onaction?: () => void;
  hat?: AvatarHat;
  speed?: number;
  compact?: boolean;
  class?: string;
} = $props();

const avatarController = new AvatarController();

let displayedText = $state("");

let animFrameId: number | null = null;
let stateTimeout: ReturnType<typeof setTimeout> | null = null;

function stopStreaming() {
  if (animFrameId !== null) {
    cancelAnimationFrame(animFrameId);
    animFrameId = null;
  }
  if (stateTimeout !== null) {
    clearTimeout(stateTimeout);
    stateTimeout = null;
  }
}

function startStreaming(fullText: string) {
  stopStreaming();
  displayedText = "";
  avatarController.think();

  let lastTime = performance.now();
  let index = 0;

  function step(now: number) {
    const delta = now - lastTime;
    if (delta >= speed) {
      const charsToAdd = Math.max(1, Math.floor(delta / speed));
      index = Math.min(fullText.length, index + charsToAdd);
      displayedText = fullText.slice(0, index);
      lastTime = now;
    }

    if (index < fullText.length) {
      animFrameId = requestAnimationFrame(step);
    } else {
      animFrameId = null;
      avatarController.happy();
      stateTimeout = setTimeout(() => {
        avatarController.idle();
      }, 1000);
    }
  }

  animFrameId = requestAnimationFrame(step);
}

$effect(() => {
  if (text) {
    startStreaming(text);
  }
});

onDestroy(() => {
  stopStreaming();
  avatarController.destroy();
});

const avatarSize = $derived(compact ? "60px" : "78px");
</script>

<div class="w-full flex items-center justify-center select-none {compact ? 'py-4 px-2' : 'py-8 px-4'} {className}">
  <div class="flex flex-col sm:flex-row items-start justify-center gap-3.5 w-full text-start {compact ? 'max-w-sm' : 'max-w-lg'}">
    
    <!-- 1. AI Avatar Pinned at Top (self-start) with Dynamic Random Hat -->
    <div class="shrink-0 flex items-start justify-center self-start {compact ? 'pt-0.5' : 'pt-1'}">
      <Avatar controller={avatarController} {hat} width={avatarSize} height={avatarSize} />
    </div>

    <!-- 2. Speech Bubble & Content -->
    <div class="flex flex-col gap-2.5 flex-1 min-w-0 w-full">
      <div class="relative bg-card border border-border/60 shadow-xs w-full font-sans {compact ? 'p-3 rounded-xl' : 'p-4 rounded-2xl'}">
        <!-- Pointer Arrow pointing towards Avatar -->
        <div class="absolute -inset-s-2 bg-card border-b border-l border-border/60 rotate-45 hidden sm:block {compact ? 'top-3.5 size-2.5' : 'top-4 size-3'}"></div>

        <div class="flex items-start gap-2">
          <Sparkles size={compact ? 13 : 15} class="text-primary shrink-0 mt-0.5" />
          <p class="text-foreground/90 leading-relaxed font-medium {compact ? 'text-[11px] min-h-[32px]' : 'text-xs min-h-[44px]'}">
            {displayedText}
          </p>
        </div>
      </div>

      <!-- 3. Action CTA Button -->
      {#if actionLabel && onaction}
        <div class="flex justify-start">
          <Button
            type="button"
            variant="default"
            size="sm"
            class="font-semibold rounded-lg shadow-xs cursor-pointer hover:opacity-90 active:scale-[0.98] transition-all {compact ? 'px-3 py-1 text-[11px] h-7' : 'px-4 py-1.5 text-xs h-8'}"
            onclick={onaction}
          >
            <span>{actionLabel}</span>
            <ArrowRight size={compact ? 11 : 13} class="ms-1.5 rtl:rotate-180" />
          </Button>
        </div>
      {/if}
    </div>

  </div>
</div>
