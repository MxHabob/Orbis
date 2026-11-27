<!--
  @component StatusStage
  Full-viewport OOBE-style status stage.
-->
<script lang="ts">
import type { Snippet } from "svelte";
import { fade } from "svelte/transition";

export type StatusStageMode = "preparing" | "complete" | "error";

let {
  status,
  phases = [],
  phaseIntervalMs = 3200,
  title = "",
  dir = "auto",
  class: className = "",
  children,
}: {
  status: StatusStageMode;
  phases?: string[];
  phaseIntervalMs?: number;
  title?: string;
  dir?: "rtl" | "ltr" | "auto";
  class?: string;
  children?: Snippet;
} = $props();

let phaseIndex = $state(0);

$effect(() => {
  if (status !== "preparing" || phases.length <= 1) {
    phaseIndex = 0;
    return;
  }

  const interval = setInterval(() => {
    phaseIndex = (phaseIndex + 1) % phases.length;
  }, phaseIntervalMs);

  return () => clearInterval(interval);
});

const activeHeading = $derived(
  status === "preparing" ? (phases[phaseIndex] ?? title) : title,
);
</script>

<section
  class="relative mx-auto flex min-h-[64vh] max-w-4xl flex-col items-center justify-center px-4 py-10 text-center {className}"
  role="status"
  aria-live="polite"
>
  {#if status === "preparing"}
    <div class="flex min-h-24 flex-col items-center justify-center">
      {#key phaseIndex}
        <h1
          in:fade={{ duration: 400 }}
          class="max-w-3xl text-4xl font-bold leading-snug tracking-tight text-foreground sm:text-5xl md:text-6xl"
          {dir}
        >
          {activeHeading}
        </h1>
      {/key}
    </div>
  {:else}
    {#if activeHeading}
      <h1
        class="max-w-4xl text-4xl font-bold leading-snug tracking-tight text-foreground sm:text-5xl md:text-6xl"
        {dir}
      >
        {activeHeading}
      </h1>
    {/if}
    {#if children}
      {@render children()}
    {/if}
  {/if}
</section>
