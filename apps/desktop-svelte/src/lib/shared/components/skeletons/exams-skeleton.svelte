<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte/skeleton";
</script>

<div class="flex flex-col gap-6 h-full p-6 w-full">
  <!-- Header -->
  <div class="flex items-center justify-between mb-4">
    <div class="flex flex-col gap-2">
      <Skeleton class="h-8 w-48 rounded" />
      <Skeleton class="h-4 w-64 rounded" />
    </div>
    <Skeleton class="h-10 w-32 rounded" />
  </div>

  <!-- Sessions Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    {#each Array(6) as _}
      <div class="flex flex-col gap-4 border border-border/50 rounded-xl p-4">
        <div class="flex justify-between items-start">
          <Skeleton class="h-6 w-32 rounded" />
          <Skeleton class="h-6 w-16 rounded-full" />
        </div>
        <Skeleton class="h-4 w-48 rounded" />
        <div class="flex items-center justify-between mt-4">
          <Skeleton class="h-4 w-24 rounded" />
          <Skeleton class="h-8 w-24 rounded" />
        </div>
      </div>
    {/each}
  </div>
</div>
