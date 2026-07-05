<!--
@component ArchivedBanner
Displays a prominent banner when viewing an archived (read-only) year.
Place at the top of the main content area.
-->
<script lang="ts">
import { academicContext } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { Archive, Lock } from "lucide-svelte";
</script>

{#if academicContext.isReadOnly}
  <div
    class="flex items-center gap-3 px-4 py-2.5 bg-muted/60 border-b border-border/60 text-sm text-muted-foreground"
  >
    <div class="flex items-center gap-2 shrink-0">
      <div class="flex items-center justify-center size-6 rounded-full bg-accent">
        <Lock class="size-3.5 text-muted-foreground" />
      </div>
      <Archive class="size-4" />
    </div>
    <span>{m.year_archived_banner()}</span>
    <span class="ms-auto text-xs font-mono opacity-60">
      {academicContext.yearLabel}
    </span>
  </div>
{/if}

