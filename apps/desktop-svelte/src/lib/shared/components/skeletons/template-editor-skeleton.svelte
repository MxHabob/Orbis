<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte/skeleton";
</script>

<div class="flex flex-col h-full w-full">
  <!-- Toolbar -->
  <div class="h-14 border-b border-border/50 flex items-center px-4 justify-between shrink-0">
    <div class="flex items-center gap-4">
      <Skeleton class="h-8 w-32 rounded" />
      <Skeleton class="h-8 w-8 rounded" />
      <Skeleton class="h-8 w-8 rounded" />
    </div>
    <div class="flex items-center gap-4">
      <Skeleton class="h-8 w-24 rounded" />
      <Skeleton class="h-8 w-24 rounded" />
    </div>
  </div>

  <div class="flex flex-1 overflow-hidden">
    <!-- Left Sidebar (Tools) -->
    <div class="w-64 border-r border-border/50 p-4 flex flex-col gap-4">
      <Skeleton class="h-10 w-full rounded" />
      <div class="grid grid-cols-2 gap-2 mt-4">
        {#each Array(8) as _}
          <Skeleton class="h-16 w-full rounded" />
        {/each}
      </div>
    </div>

    <!-- Canvas -->
    <div class="flex-1 bg-muted/20 flex items-center justify-center p-8">
      <Skeleton class="w-[800px] h-[1000px] shadow-sm rounded-sm" />
    </div>

    <!-- Right Sidebar (Properties) -->
    <div class="w-72 border-l border-border/50 p-4 flex flex-col gap-4">
      <Skeleton class="h-6 w-32 rounded mb-4" />
      {#each Array(5) as _}
        <div class="flex flex-col gap-2">
          <Skeleton class="h-4 w-24 rounded" />
          <Skeleton class="h-10 w-full rounded" />
        </div>
      {/each}
    </div>
  </div>
</div>
