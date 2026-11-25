<script lang="ts">
import { Skeleton } from "@orbis/ui/skeleton";
</script>

<div class="flex w-full min-w-0 flex-col items-center gap-[clamp(0.75rem,2.7cqh,1.5rem)]" aria-busy="true">
  <div class="flex w-full items-center justify-center gap-3">
    <Skeleton class="size-12 shrink-0 rounded-full" />
    <Skeleton class="h-8 w-full max-w-56 rounded" />
  </div>

  <div class="flex w-full flex-wrap justify-center gap-3">
    {#each Array(4) as _}
      <Skeleton class="h-8 w-20 rounded-md" />
    {/each}
  </div>

  <div class="grid w-full grid-cols-1 gap-3 @min-[32rem]/workspace:grid-cols-2 @min-[48rem]/workspace:grid-cols-3">
    {#each Array(6) as _}
      <Skeleton class="h-[clamp(5rem,15cqh,7rem)] w-full rounded-lg" />
    {/each}
  </div>
</div>
