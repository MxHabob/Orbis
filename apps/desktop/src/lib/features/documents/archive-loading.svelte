<script lang="ts">
  import { Skeleton } from "@orbis/ui/skeleton";

  let { count = 12 }: { count?: number } = $props();
</script>

<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6">
  {#each Array(count) as _}
    <div class="rounded-xl border border-border/40 bg-card p-2.5 shadow-2xs">
      <div class="archive-media">
        <Skeleton class="h-full w-full rounded-lg" />
      </div>
      <div class="space-y-2 px-0.5 pt-2">
        <Skeleton class="h-3.5 w-4/5 rounded" />
        <Skeleton class="h-2.5 w-2/5 rounded" />
      </div>
    </div>
  {/each}
</div>

<style>
  .archive-media {
    aspect-ratio: 4 / 3;
  }
</style>
