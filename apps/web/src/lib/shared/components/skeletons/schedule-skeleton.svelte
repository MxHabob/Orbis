<script lang="ts">
import { Skeleton } from "@orbis/ui/skeleton";
</script>

<div class="flex h-full w-full">
  <!-- Sidebar -->
  <div class="w-64 border-l border-border/50 p-4 flex flex-col gap-4">
    <Skeleton class="h-10 w-full rounded" />
    <Skeleton class="h-10 w-full rounded" />
    <Skeleton class="h-64 w-full rounded" />
  </div>

  <!-- Main Timetable -->
  <div class="flex-1 p-6 flex flex-col gap-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        {#each Array(5) as _}
          <Skeleton class="h-10 w-24 rounded" />
        {/each}
      </div>
      <Skeleton class="h-10 w-32 rounded" />
    </div>

    <div class="flex-1 border border-border/50 rounded-xl overflow-hidden">
      <Skeleton class="h-full w-full" />
    </div>
  </div>
</div>
