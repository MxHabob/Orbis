<!--
  @component AcademicsError
  Route error boundary for all Academics routes.
  Catches route-level data loading or rendering failures.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { invalidateAll } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { ArrowLeft, InfoCircle, Refresh } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";

const orgId = $derived(page.params.orgId || "");
const status = $derived(page.status || 500);
const errorMessage = $derived(
  page.error?.message ||
    (m.common_unexpected_error_message
      ? m.common_unexpected_error_message()
      : "An unexpected error occurred in Academics."),
);
</script>

<svelte:head>
  <title>{m.academics_layout_panel_title ? m.academics_layout_panel_title() : "Academics"} | Orbis</title>
</svelte:head>

<div class="h-full w-full flex flex-col items-center justify-center p-6 text-start font-sans">
  <div class="max-w-md w-full border border-border/70 rounded-xl shadow-xs bg-card p-6 flex flex-col gap-5">
    <div class="flex items-center gap-3">
      <div class="size-10 rounded-lg bg-destructive/10 text-destructive flex items-center justify-center shrink-0">
        <InfoCircle size={22} />
      </div>
      <div class="min-w-0">
        <h2 class="text-sm font-bold text-foreground">
          {status} — {m.common_error_occurred ? m.common_error_occurred() : "Error Occurred"}
        </h2>
        <p class="text-xs text-muted-foreground mt-0.5 truncate">
          {m.common_error_occurred ? m.common_error_occurred() : "Unable to load the requested academics view"}
        </p>
      </div>
    </div>

    <div class="p-3 rounded-md bg-muted/40 border border-border/40 text-xs font-mono text-muted-foreground break-words">
      {errorMessage}
    </div>

    <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
      {#if orgId}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 text-xs font-semibold"
          href={localizeHref(`/${orgId}/academics`)}
        >
          <ArrowLeft size={13} class="rtl:rotate-180" />
          <span>{m.common_nav_overview ? m.common_nav_overview() : "Academics Overview"}</span>
        </Button>
      {/if}

      <Button
        variant="default"
        size="sm"
        class="h-8 gap-1.5 text-xs font-semibold cursor-pointer"
        onclick={() => invalidateAll()}
      >
        <Refresh size={13} />
        <span>{m.common_retry ? m.common_retry() : "Retry"}</span>
      </Button>
    </div>
  </div>
</div>
