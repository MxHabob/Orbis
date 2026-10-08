<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Button } from "@orbis/ui/button";
  import { XMark } from "@orbis/icons";

  let {
    contextLabel,
    stepLabel,
    stepPosition,
    progressValue,
    ariaLabel,
    closeLabel,
    onClose,
  }: {
    contextLabel: string;
    stepLabel: string;
    stepPosition: string;
    progressValue: number;
    ariaLabel: string;
    closeLabel?: string;
    onClose?: () => void;
  } = $props();

  const boundedProgress = $derived(
    Math.min(100, Math.max(0, Number.isFinite(progressValue) ? progressValue : 0)),
  );
</script>

<header class="shrink-0 border-b border-border/70 bg-background/95 px-4 py-3 backdrop-blur sm:px-6">
  <div class="mx-auto flex w-full max-w-5xl items-center justify-between gap-4">
    <div class="min-w-0">
      <p class="truncate text-xs text-muted-foreground">{contextLabel}</p>
      <p class="truncate text-sm font-semibold">{stepLabel}</p>
    </div>
    <div class="flex shrink-0 items-center gap-3">
      <span class="text-xs font-semibold tabular-nums text-muted-foreground">
        {stepPosition}
      </span>
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
</header>
