<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Button } from "@orbis/ui/button";
  import { ChevronLeft, ChevronRight } from "@orbis/icons";

  let {
    backLabel,
    nextLabel,
    secondaryLabel,
    canGoBack = true,
    canGoNext = true,
    isSubmitting = false,
    isRTL = false,
    onBack,
    onNext,
    onSecondary,
  }: {
    backLabel: string;
    nextLabel: string;
    secondaryLabel?: string;
    canGoBack?: boolean;
    canGoNext?: boolean;
    isSubmitting?: boolean;
    isRTL?: boolean;
    onBack?: () => void;
    onNext?: () => void;
    onSecondary?: () => void;
  } = $props();
</script>

<footer class="shrink-0 border-t border-border/70 bg-background/95 px-3 py-3 backdrop-blur sm:px-6">
  <nav
    class="mx-auto flex w-full max-w-5xl items-center justify-between gap-2"
    dir={isRTL ? "rtl" : "ltr"}
    aria-label={m.onboarding_stages_title()}
  >
    <div class="flex min-w-0 items-center gap-2">
      {#if onBack}
        <Button
          variant="ghost"
          size="sm"
          onclick={onBack}
          disabled={!canGoBack || isSubmitting}
          class="gap-1 px-2 sm:gap-2 sm:px-3"
        >
          {#if isRTL}
            <ChevronRight class="size-4" />
          {:else}
            <ChevronLeft class="size-4" />
          {/if}
          <span>{backLabel}</span>
        </Button>
      {/if}
    </div>

    <div class="flex min-w-0 items-center gap-1 sm:gap-2">
      {#if secondaryLabel && onSecondary}
        <Button
          variant="ghost"
          size="sm"
          onclick={onSecondary}
          disabled={isSubmitting}
          class="px-2 sm:px-3"
        >
          <span>{secondaryLabel}</span>
        </Button>
      {/if}
      {#if onNext}
        <Button
          variant="default"
          size="sm"
          onclick={onNext}
          disabled={!canGoNext || isSubmitting}
          class="gap-1 px-3 sm:gap-2 sm:px-4"
        >
          <span>{nextLabel}</span>
          {#if isRTL}
            <ChevronLeft class="size-4" />
          {:else}
            <ChevronRight class="size-4" />
          {/if}
        </Button>
      {/if}
    </div>
  </nav>
</footer>
