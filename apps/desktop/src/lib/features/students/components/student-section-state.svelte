<script lang="ts">
import type { Component, Snippet } from "svelte";

let {
  icon,
  title,
  description,
  action,
}: {
  icon: Component<{ class?: string; size?: number }>;
  title: string;
  description?: string;
  action?: Snippet;
} = $props();

const Icon = $derived(icon);
</script>

<div
  role="status"
  class="flex min-h-32 flex-col items-center justify-center gap-2 px-4 py-6 text-center"
>
  <div class="flex size-9 items-center justify-center rounded-lg bg-muted text-muted-foreground">
    <Icon size={17} />
  </div>
  <p class="text-sm font-medium text-foreground">{title}</p>
  {#if description}
    <p class="max-w-md text-xs text-muted-foreground">{description}</p>
  {/if}
  {#if action}
    <div class="mt-1">{@render action()}</div>
  {/if}
</div>
