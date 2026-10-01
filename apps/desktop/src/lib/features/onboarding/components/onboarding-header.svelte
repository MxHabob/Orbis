<!--
  @component OnboardingHeader
  Floating minimal header with high-end segmented progress indicators (without noisy text),
  brand identity, language switch (EN/AR), and theme toggle.
-->
<script lang="ts">
import { Cube, Sun, Moon } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { themeStore } from "$lib/stores";
import { setLocale, getLocale } from "$lib/paraglide/runtime";
import type { OnboardingStep } from "../types";

let {
  currentStep,
  steps,
  isRTL = false,
}: {
  currentStep: OnboardingStep;
  steps: Array<{ id: OnboardingStep }>;
  isRTL?: boolean;
} = $props();

const currentStepIndex = $derived(steps.findIndex((s) => s.id === currentStep));
const isDark = $derived(themeStore.theme === "dark");

function toggleLanguage() {
  const current = getLocale();
  const next = current.startsWith("ar") ? "en" : "ar";
  setLocale(next);
}

function toggleTheme() {
  themeStore.toggle();
}
</script>

<header class="fixed top-4 left-1/2 -translate-x-1/2 z-40 w-[95%] max-w-5xl rounded-full bg-background/80 backdrop-blur-xl border border-border/70 shadow-lg shadow-black/5 px-4 sm:px-6 py-2.5 flex items-center justify-between transition-all">
  <!-- Brand Identity -->
  <div class="flex items-center gap-2.5 select-none shrink-0">
    <div class="size-7 rounded-full bg-primary/10 border border-primary/30 flex items-center justify-center text-primary shadow-xs">
      <Cube size={15} />
    </div>
    <span class="text-sm font-bold tracking-tight text-foreground font-sans">Orbis</span>
  </div>

  <!-- Minimalist Visual Progress (Pure Progress Segments Without Text) -->
  <nav aria-label="Progress" class="flex items-center gap-1.5 sm:gap-2 px-2 select-none">
    {#each steps as s, idx}
      {@const isDone = idx < currentStepIndex}
      {@const isCurrent = idx === currentStepIndex}

      <div
        class="h-1.5 rounded-full transition-all duration-300 {isCurrent
          ? 'w-7 sm:w-10 bg-primary shadow-xs shadow-primary/30 ring-2 ring-primary/20'
          : isDone
            ? 'w-3.5 sm:w-5 bg-foreground/60'
            : 'w-2 sm:w-3 bg-muted-foreground/20'}"
        title={`Step ${idx + 1} of ${steps.length}`}
      ></div>
    {/each}
  </nav>

  <!-- Quick Utility Controls (Language & Theme) -->
  <div class="flex items-center gap-1.5 shrink-0">
    <!-- Language Switcher -->
    <Button
      variant="ghost"
      size="sm"
      onclick={toggleLanguage}
      class="h-7 px-2.5 text-xs font-mono rounded-full text-muted-foreground hover:text-foreground hover:bg-muted/50"
      title={isRTL ? "Switch to English" : "التحويل إلى العربية"}
    >
      <span>{isRTL ? "EN" : "عربي"}</span>
    </Button>

    <!-- Theme Toggle -->
    <Button
      variant="ghost"
      size="sm"
      onclick={toggleTheme}
      class="h-7 w-7 p-0 rounded-full text-muted-foreground hover:text-foreground hover:bg-muted/50"
      title={isDark ? (isRTL ? "الوضع المضيء" : "Light Mode") : (isRTL ? "الوضع الليلي" : "Dark Mode")}
    >
      {#if isDark}
        <Sun size={14} />
      {:else}
        <Moon size={14} />
      {/if}
    </Button>
  </div>
</header>
