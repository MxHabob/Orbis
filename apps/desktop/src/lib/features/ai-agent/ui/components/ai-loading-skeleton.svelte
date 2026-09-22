<script lang="ts">
import { onMount, onDestroy } from "svelte";
import { Logo } from "@orbis/ui/logo";
import { Skeleton } from "@orbis/ui/skeleton";

let {
  statusText = "",
  class: className = "",
}: {
  statusText?: string;
  class?: string;
} = $props();

const defaultPhases = [
  "جاري التفكير وتحليل البيانات...",
  "جاري استرجاع وفحص السجلات...",
  "جاري صياغة الاستجابة الذكية...",
];

let currentPhaseIndex = $state(0);
let timer: ReturnType<typeof setInterval> | null = null;

onMount(() => {
  timer = setInterval(() => {
    currentPhaseIndex = (currentPhaseIndex + 1) % defaultPhases.length;
  }, 2600);
});

onDestroy(() => {
  if (timer) clearInterval(timer);
});

const currentStatus = $derived(
  statusText || defaultPhases[currentPhaseIndex]
);
</script>

<div class="flex flex-col gap-3 w-full text-start py-1 select-none {className}">
  <!-- Minimalist AI Status with Logo & Dynamic Shimmering Text -->
  <div class="flex items-center gap-2">
    <div class="relative flex items-center justify-center">
      <Logo class="h-4 w-auto" />
    </div>
    {#key currentStatus}
      <span class="ai-shimmer-text text-[12px] font-medium tracking-wide animate-fade-in">
        {currentStatus}
      </span>
    {/key}
  </div>

  <!-- Skeletons using existing Skeleton component -->
  <div class="flex flex-col gap-2 w-full max-w-[85%] ps-0.5">
    <Skeleton class="h-3 w-[75%] rounded-md" />
    <Skeleton class="h-3 w-[90%] rounded-md" />
    <Skeleton class="h-3 w-[50%] rounded-md" />
  </div>
</div>

<style>
  .ai-shimmer-text {
    background: linear-gradient(
      90deg,
      var(--muted-foreground) 0%,
      var(--primary) 50%,
      var(--muted-foreground) 100%
    );
    background-size: 200% 100%;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    animation: text-shimmer 2.4s ease-in-out infinite;
  }

  @keyframes text-shimmer {
    0% {
      background-position: 100% 0;
    }
    100% {
      background-position: -100% 0;
    }
  }

  @keyframes fade-in {
    from {
      opacity: 0.4;
      transform: translateY(2px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .animate-fade-in {
    animation: fade-in 0.35s ease-out;
  }
</style>
