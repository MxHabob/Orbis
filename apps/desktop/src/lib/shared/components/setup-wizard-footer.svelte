<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Button } from "@orbis/ui/button";
  import { ChevronLeft, ChevronRight } from "@orbis/icons";
  import { LoadingIndicator } from "@orbis/ui";

  let {
    backLabel,
    nextLabel,
    secondaryLabel,
    canGoBack = true,
    canGoNext = true,
    isSubmitting = false,
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
    onBack?: () => void;
    onNext?: () => void;
    onSecondary?: () => void;
  } = $props();
</script>

<footer
  class="z-20 mx-4 mb-3 w-[calc(100%-2rem)] max-w-5xl shrink-0 self-center rounded-lg border border-border/70 bg-background/90 px-3 py-3 shadow-md backdrop-blur sm:mb-4 sm:px-6"
>
  <nav
    class="mx-auto flex w-full max-w-5xl items-center justify-between gap-2"
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
          <ChevronLeft class="size-4 rtl:rotate-180" />
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
          aria-busy={isSubmitting}
        >
          {#if isSubmitting}
            <LoadingIndicator size={16} class="size-4" label={m.common_control_loading()} />
          {/if}
          <span>{nextLabel}</span>
          <ChevronRight class="size-4 rtl:rotate-180" />
        </Button>
      {/if}
    </div>
  </nav>
</footer>
