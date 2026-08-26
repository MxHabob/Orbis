<!--
@component WorkspaceCard
Generic, modular workspace card container with header actions and body slot.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    actions = undefined,
    children,
    class: className = "",
  }: {
    title: string;
    actions?: Snippet;
    children: Snippet;
    class?: string;
  } = $props();
</script>

<div class="flex flex-col bg-card border border-border/60 rounded-xl overflow-hidden shadow-2xs {className}">
  <div class="flex items-center justify-between p-4 border-b border-border/40 bg-muted/10">
    <h3 class="font-bold text-sm font-sans tracking-tight">{title}</h3>
    {#if actions}
      <div class="flex items-center gap-2">
        {@render actions()}
      </div>
    {/if}
  </div>
  <div class="flex-1 flex flex-col p-4 overflow-hidden">
    {@render children()}
  </div>
</div>
