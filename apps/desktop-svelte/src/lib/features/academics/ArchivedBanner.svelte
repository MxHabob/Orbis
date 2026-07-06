<!--
@component ArchivedBanner
Displays a prominent banner when viewing an archived (read-only) year.
Place at the top of the main content area.
-->
<script lang="ts">
import { academicContext } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { Lock } from "lucide-svelte";
</script>

{#if academicContext.isReadOnly}
  <div
    class="relative w-full border-b border-border bg-background overflow-hidden py-3 px-4 flex items-center justify-between gap-4 shrink-0 select-none"
  >
    <!-- Light mode dot pattern grid -->
    <div
      class="absolute inset-0 dark:hidden pointer-events-none"
      style="background-image: radial-gradient(circle, rgba(0,0,0,0.08) 1px, transparent 1px); background-size: 12px 12px;"
    ></div>
    <!-- Dark mode dot pattern grid -->
    <div
      class="absolute inset-0 hidden dark:block pointer-events-none"
      style="background-image: radial-gradient(circle, rgba(255,255,255,0.07) 1px, transparent 1px); background-size: 12px 12px;"
    ></div>

    <div class="relative z-10 flex items-center gap-3">
      <div class="flex items-center justify-center size-9 rounded-lg border border-border bg-card text-muted-foreground shadow-sm">
        <Lock class="size-4.5 text-primary" />
      </div>
      <div class="flex flex-col text-start">
        <h3 class="text-xs font-semibold tracking-wide text-foreground">
          {m.year_status_archived()}
        </h3>
        <p class="text-[10px] text-muted-foreground leading-normal">
          {m.year_archived_banner()}
        </p>
      </div>
    </div>

    <div class="relative z-10 flex items-center gap-2">
      <span class="text-xs font-mono opacity-80 bg-muted/60 border border-border/60 px-2 py-0.5 rounded-md text-foreground">
        {academicContext.yearLabel}
      </span>
    </div>
  </div>
{/if}

