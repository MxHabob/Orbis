<!--
  @component StatusStage
  Full-viewport OOBE-style status stage powered by `@orbis/avatar-engine`.
  Keeps the Avatar mounted across status transitions (preparing -> complete/error)
  so state animations flow seamlessly without layout jumps.
-->
<script lang="ts">
import type { Snippet } from "svelte";
import { fade } from "svelte/transition";
import { Avatar, type AvatarHat, type AvatarState } from "@orbis/avatar-engine";

export type StatusStageMode = "preparing" | "complete" | "error";

const AVATAR_STATE_MAP: Record<StatusStageMode, AvatarState> = {
  preparing: "thinking",
  complete: "celebrating",
  error: "error",
};

let {
  status,
  hat = "graduation",
  avatarSize = "160px",
  avatarLabel = "Orbis",
  phases = [],
  phaseIntervalMs = 3200,
  title = "",
  subtitle = "",
  dir = "auto",
  class: className = "",
  children,
}: {
  status: StatusStageMode;
  hat?: AvatarHat;
  avatarSize?: string;
  avatarLabel?: string;
  phases?: string[];
  phaseIntervalMs?: number;
  title?: string;
  subtitle?: string;
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

const avatarState = $derived(AVATAR_STATE_MAP[status]);
const activeHeading = $derived(
  status === "preparing" ? (phases[phaseIndex] ?? title) : title,
);
</script>

<section
  class="relative mx-auto flex min-h-[64vh] max-w-4xl flex-col items-center justify-center px-4 py-10 text-center {className}"
  role="status"
  aria-live="polite"
>
  <div class="relative mb-6 flex items-center justify-center">
    <div
      class="pointer-events-none absolute size-44 rounded-full bg-primary/10 blur-3xl {status === 'preparing' ? 'animate-pulse motion-reduce:animate-none' : ''}"
      aria-hidden="true"
    ></div>
    <Avatar
      state={avatarState}
      autoBlink={status !== "preparing"}
      {hat}
      width={avatarSize}
      height={avatarSize}
      label={avatarLabel}
    />
  </div>

  {#if status === "preparing"}
    <div class="flex min-h-24 flex-col items-center justify-center gap-3">
      {#key phaseIndex}
        <h1
          in:fade={{ duration: 400 }}
          class="max-w-2xl text-2xl font-semibold leading-tight tracking-tight text-foreground sm:text-4xl"
          {dir}
        >
          {activeHeading}
        </h1>
      {/key}
      {#if subtitle}
        <p class="text-sm text-muted-foreground" {dir}>
          {subtitle}
        </p>
      {/if}
    </div>
  {:else}
    {#if activeHeading}
      <h1
        class="max-w-3xl text-3xl font-semibold leading-tight tracking-tight text-foreground sm:text-5xl"
        {dir}
      >
        {activeHeading}
      </h1>
    {/if}
    {#if subtitle}
      <p class="mt-3 max-w-xl text-sm text-muted-foreground" {dir}>
        {subtitle}
      </p>
    {/if}
    {#if children}
      {@render children()}
    {/if}
  {/if}
</section>
