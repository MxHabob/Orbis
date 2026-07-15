<!--
@component LivePage (Placeholder)
AI Center live — camera grid, live feeds, events feed.
-->
<script lang="ts">
import { Video } from "lucide-svelte";
</script>

<div class="h-full overflow-y-auto">
  <div class="px-8 py-6 max-w-5xl">
    <h1 class="text-xl font-semibold text-foreground mb-1">Live</h1>
    <p class="text-xs text-muted-foreground mb-6">
      Monitor live camera feeds and real-time events from connected surveillance devices.
    </p>

    <div class="border border-border/60 rounded-lg p-8">
      <div class="flex flex-col items-center justify-center py-8">
        <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center mb-4">
          <Video size={20} class="text-muted-foreground" />
        </div>
        <h3 class="text-sm font-medium text-foreground mb-1">No cameras connected</h3>
        <p class="text-xs text-muted-foreground text-center max-w-xs">
          Connect a Hikvision camera in the Integrations page to view live feeds here.
        </p>
      </div>
    </div>
  </div>
</div>
