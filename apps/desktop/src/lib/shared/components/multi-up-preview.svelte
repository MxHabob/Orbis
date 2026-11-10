<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * MultiUpPreview — CSS-based WYSIWYG preview of Multi-Up sheet layout.
 * Renders a scaled representation of the physical sheet with template slots.
 * Matches the Rust engine's compute_layout() output for pixel-perfect accuracy.
 */
import { Button } from "@orbis/ui/button";
import { ChevronLeft, ChevronRight } from "@orbis/icons";

import { getLocale } from "$lib/paraglide/runtime";

import type { MultiUpLayout } from "@docforge/core";

let {
  layout,
  totalRecords = 0,
  currentPage = $bindable(1),
}: {
  layout: MultiUpLayout;
  totalRecords: number;
  currentPage: number;
} = $props();

// Derived values
const totalPages = $derived(
  layout.itemsPerSheet > 0 ? Math.ceil(totalRecords / layout.itemsPerSheet) : 1,
);

const startIndex = $derived((currentPage - 1) * layout.itemsPerSheet);

const sheetAspectRatio = $derived(
  layout.sheetWidthMm / layout.sheetHeightMm,
);

function prevPage() {
  if (currentPage > 1) currentPage--;
}
function nextPage() {
  if (currentPage < totalPages) currentPage++;
}
</script>

<div class="flex h-full min-h-0 w-full flex-col items-center gap-2">
  <!-- Sheet Preview Container -->
  <div
    class="flex min-h-0 min-w-0 flex-1 items-center justify-center"
  >
    <div
      class="relative shrink-0 rounded-sm border border-border bg-white shadow-md"
      style="
        width: {sheetAspectRatio > 1 ? '100%' : 'auto'};
        height: {sheetAspectRatio <= 1 ? '100%' : 'auto'};
        max-width: 100%;
        max-height: 100%;
        aspect-ratio: {layout.sheetWidthMm} / {layout.sheetHeightMm};
      "
    >
      <!-- Template Slots -->
      {#each layout.slots as slot, i (i)}
        {@const recordIdx = startIndex + i}
        {@const hasRecord = recordIdx < totalRecords}
        <div
          class="absolute border transition-all duration-150 {hasRecord ? 'border-primary bg-primary/5' : 'border-dashed border-border/40 bg-muted/10'}"
          style="
            left: {slot.xMm / layout.sheetWidthMm * 100}%;
            top: {slot.yMm / layout.sheetHeightMm * 100}%;
            width: {slot.widthMm / layout.sheetWidthMm * 100}%;
            height: {slot.heightMm / layout.sheetHeightMm * 100}%;
          "
        >
          {#if hasRecord}
            <div class="absolute inset-0 flex items-center justify-center overflow-hidden p-1">
              <span class="text-sm font-bold text-primary">
                #{recordIdx + 1}
              </span>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>

  <!-- Page Navigation -->
  {#if totalPages > 1}
    <div class="flex shrink-0 items-center gap-2">
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={prevPage}
        disabled={currentPage <= 1}
      >
        <ChevronLeft size={14} class={getLocale() === "ar" ? "rtl:rotate-180" : ""} />
      </Button>
      <span class="text-xs text-muted-foreground font-medium min-w-[80px] text-center">
        {m.common_print_multi_up_page_of({ current: String(currentPage), total: String(totalPages) })}
      </span>
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={nextPage}
        disabled={currentPage >= totalPages}
      >
        <ChevronRight size={14} class={getLocale() === "ar" ? "rtl:rotate-180" : ""} />
      </Button>
    </div>
  {/if}

</div>
