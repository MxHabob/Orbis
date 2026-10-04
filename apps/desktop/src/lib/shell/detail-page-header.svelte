<!--
@component DetailPageHeader
Shared header for entity detail pages (Student, User, Class, Subject).
Provides a back button, title slot, and actions slot in a consistent layout.
-->
<script lang="ts">
import { Button } from "@orbis/ui/button";
import { ArrowLeft, ArrowRight, Printer } from "@orbis/icons";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import type { Snippet } from "svelte";

let {
  backHref,
  backLabel = "",
  children,
  actions,
  printType,
  printIds,
}: {
  backHref?: string;
  backLabel?: string;
  children: Snippet;
  actions?: Snippet;
  printType?: string;
  printIds?: string[];
} = $props();

import { openPrintDialog } from "$lib/stores/print.svelte";

function handleBack(e: MouseEvent) {
  if (backHref) {
    e.preventDefault();
    goto(localizeHref(backHref));
  } else if (typeof window !== "undefined") {
    window.history.back();
  }
}
</script>

<div
  class="px-6 py-3 flex items-center justify-between gap-4 select-none shrink-0 border-b border-border/40 bg-card/5"
>
  <div class="flex items-center gap-3 text-start shrink min-w-0">
    <Button
      href={backHref ? localizeHref(backHref) : undefined}
      onclick={handleBack}
      variant="secondary"
      size="icon"
      class="h-8 w-8 hover:bg-accent/80 cursor-pointer shrink-0"
      title={backLabel}
    >
      {#if getLocale() === "ar"}
        <ArrowRight class="h-4.5 w-4.5" />
      {:else}
        <ArrowLeft class="h-4.5 w-4.5" />
      {/if}
    </Button>
    <div class="h-4 w-px bg-border/80 shrink-0"></div>
    {@render children()}
  </div>

  {#if actions || printType}
    <div class="flex items-center gap-2 flex-nowrap shrink-0 justify-end">
      {@render actions?.()}
      
      {#if printType && printIds && printIds.length > 0}
        <Button
          variant="outline"
          size="sm"
          class="gap-1.5"
          onclick={() => openPrintDialog(printType, printIds)}
        >
          <Printer class="h-4 w-4" />
          <span class="hidden sm:inline">{m.shared_ui_print()}</span>
        </Button>
      {/if}
    </div>
  {/if}
</div>
