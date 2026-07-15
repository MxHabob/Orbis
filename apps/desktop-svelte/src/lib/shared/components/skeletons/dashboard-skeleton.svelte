<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte/skeleton";
</script>

<div class="flex flex-col gap-6 p-6 h-full w-full">
  <!-- Welcome Area -->
  <div class="flex items-center gap-4 mb-2">
    <Skeleton class="h-16 w-16 rounded-full" />
    <div class="flex flex-col gap-2">
      <Skeleton class="h-8 w-64 rounded" />
      <Skeleton class="h-4 w-48 rounded" />
    </div>
  </div>

  <!-- Top Stat Cards -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
    {#each Array(4) as _}
      <Skeleton class="h-32 w-full rounded-xl" />
    {/each}
  </div>

  <!-- Main Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <div class="lg:col-span-2 flex flex-col gap-6">
      <Skeleton class="h-64 w-full rounded-xl" />
      <Skeleton class="h-64 w-full rounded-xl" />
    </div>
    <div class="flex flex-col gap-6">
      <Skeleton class="h-96 w-full rounded-xl" />
    </div>
  </div>
</div>
