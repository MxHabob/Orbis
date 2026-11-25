<script lang="ts">
import type { Snippet } from "svelte";

let {
  title,
  description = "",
  children,
}: {
  title: string;
  description?: string;
  children: Snippet;
} = $props();
</script>

<div class="flex flex-col gap-2 border-b border-border/40 pb-3 sm:flex-row sm:items-center sm:justify-between">
  <div class="min-w-0 space-y-0.5">
    <p class="text-xs font-semibold text-foreground">{title}</p>
    {#if description}
      <p class="text-[11px] leading-relaxed text-muted-foreground">{description}</p>
    {/if}
  </div>
  <div class="flex flex-wrap items-center gap-2 sm:justify-end">
    {@render children()}
  </div>
</div>
