<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Button } from "@orbis/ui/button";
  import { XMark } from "@orbis/icons";
  import { Skeleton } from "@orbis/ui/skeleton";

  let {
    contextLabel,
    stepLabel,
    stepPosition,
    progressValue,
    ariaLabel,
    isLoading = false,
    closeLabel,
    onClose,
  }: {
    contextLabel: string;
    stepLabel: string;
    stepPosition: string;
    progressValue: number;
    ariaLabel: string;
    isLoading?: boolean;
    closeLabel?: string;
    onClose?: () => void;
  } = $props();

  const boundedProgress = $derived(
    Math.min(100, Math.max(0, Number.isFinite(progressValue) ? progressValue : 0)),
  );
</script>

<header
  class="z-20 mx-4 mt-3 w-[calc(100%-2rem)] max-w-5xl shrink-0 self-center rounded-lg border border-border/70 bg-background/90 px-4 py-3 shadow-md backdrop-blur sm:mt-4 sm:px-6"
  aria-busy={isLoading}
>
  <div class="mx-auto flex w-full max-w-5xl items-center justify-between gap-4">
    <div class="min-w-0">
      {#if isLoading}
        <Skeleton class="mb-1 h-3 w-20" aria-hidden="true" />
        <Skeleton class="h-4 w-36" aria-hidden="true" />
      {:else}
        <p class="truncate text-xs text-muted-foreground">{contextLabel}</p>
        <p class="truncate text-sm font-semibold">{stepLabel}</p>
      {/if}
    </div>
    <div class="flex shrink-0 items-center gap-3">
      {#if isLoading}
        <Skeleton class="h-4 w-10" aria-hidden="true" />
      {:else}
        <span class="text-xs font-semibold tabular-nums text-muted-foreground">
          {stepPosition}
        </span>
      {/if}
      {#if onClose && closeLabel}
        <Button
          variant="ghost"
          size="icon"
          onclick={onClose}
          class="size-8"
          title={closeLabel}
          aria-label={closeLabel}
        >
          <XMark class="size-4" />
        </Button>
      {/if}
    </div>
  </div>
  {#if isLoading}
    <Skeleton class="mx-auto mt-3 h-1 max-w-5xl rounded-full" aria-hidden="true" />
  {:else}
    <div
      class="mx-auto mt-3 h-1 max-w-5xl overflow-hidden rounded-full bg-muted"
      role="progressbar"
      aria-label={ariaLabel || m.onboarding_stages_title()}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-valuenow={boundedProgress}
    >
      <div
        class="h-full rounded-full bg-primary transition-[width] duration-300 motion-reduce:transition-none"
        style:width="{boundedProgress}%"
      ></div>
    </div>
  {/if}
</header>
