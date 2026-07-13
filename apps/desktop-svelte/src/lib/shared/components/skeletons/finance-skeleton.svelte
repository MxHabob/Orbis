<script lang="ts">
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
</script>

<div class="flex flex-col gap-6 p-6 h-full w-full">
  <!-- Top Stat Cards -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
    {#each Array(4) as _}
      <Skeleton class="h-32 w-full rounded-xl" />
    {/each}
  </div>

  <!-- Main Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <div class="lg:col-span-2">
      <Skeleton class="h-96 w-full rounded-xl" />
    </div>
    <div class="flex flex-col gap-6">
      <Skeleton class="h-48 w-full rounded-xl" />
      <Skeleton class="h-48 w-full rounded-xl" />
    </div>
  </div>
</div>
