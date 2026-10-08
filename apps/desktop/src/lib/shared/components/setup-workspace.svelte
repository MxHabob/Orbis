<script lang="ts">
import type { Snippet } from "svelte";

let {
  viewportId = "setup-workspace-viewport",
  header,
  children,
  footer,
}: {
  viewportId?: string;
  header?: Snippet;
  children: Snippet;
  footer?: Snippet;
} = $props();
</script>

<div class="relative flex h-full min-h-0 w-full flex-col overflow-hidden bg-background font-sans text-foreground selection:bg-primary/20">
  {#if header}
    {@render header()}
  {/if}

  <main
    id={viewportId}
    class="min-h-0 flex-1 overflow-y-auto overscroll-contain scrollbar-hide"
  >
    <div class="mx-auto w-full max-w-5xl px-4 py-6 sm:px-6 sm:py-8 lg:px-10 lg:py-10">
      {@render children()}
    </div>
  </main>

  {#if footer}
    {@render footer()}
  {/if}
</div>
