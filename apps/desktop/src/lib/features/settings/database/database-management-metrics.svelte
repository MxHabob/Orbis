<script lang="ts">
import type { DatabaseManagementMetric } from "./database-management-types";

let { stats }: { stats: DatabaseManagementMetric[] } = $props();
</script>

<dl class="grid grid-cols-2 divide-x divide-y divide-border border-y border-border sm:grid-cols-4 sm:divide-y-0 rtl:divide-x-reverse">
  {#each stats as stat (stat.label)}
    <div class="min-w-0 px-3 py-3 text-start sm:px-4">
      <dt class="truncate text-[10px] text-muted-foreground">{stat.label}</dt>
      <dd
        class="mt-1 truncate font-mono text-sm font-semibold text-foreground"
        title={stat.fullTitle || stat.value}
      >
        {stat.value}
      </dd>
      {#if stat.sub}
        <dd class="mt-1 truncate text-[10px] text-muted-foreground">{stat.sub}</dd>
      {/if}
    </div>
  {/each}
</dl>
