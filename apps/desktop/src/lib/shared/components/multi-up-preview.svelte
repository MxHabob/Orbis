<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * MultiUpPreview — CSS-based WYSIWYG preview of Multi-Up sheet layout.
 * Renders a scaled representation of the physical sheet with template slots.
 * Matches the Rust engine's compute_layout() output for pixel-perfect accuracy.
 */
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { ChevronLeft, ChevronRight } from "@lucide/svelte";

import { getLocale } from "$lib/paraglide/runtime";

import type { MultiUpLayout, SlotPosition } from "@docforge/core";

let {
  layout,
  totalRecords = 0,
  currentPage = $bindable(1),
  entityLabels = [],
}: {
  layout: MultiUpLayout;
  totalRecords: number;
  currentPage: number;
  entityLabels?: string[];
} = $props();

// Derived values
const totalPages = $derived(
  layout.itemsPerSheet > 0 ? Math.ceil(totalRecords / layout.itemsPerSheet) : 1,
);

const startIndex = $derived((currentPage - 1) * layout.itemsPerSheet);

// Scaling: fit the sheet preview into the container
const CONTAINER_MAX_W = 380;
const CONTAINER_MAX_H = 460;
const scale = $derived(
  Math.min(
    CONTAINER_MAX_W / layout.sheetWidthMm,
    CONTAINER_MAX_H / layout.sheetHeightMm,
  ),
);

function prevPage() {
  if (currentPage > 1) currentPage--;
}
function nextPage() {
  if (currentPage < totalPages) currentPage++;
}
</script>

<div class="flex flex-col gap-3 items-center w-full">
  <!-- Sheet Preview Container -->
  <div
    class="relative bg-white border border-border shadow-md rounded-sm"
    style="
      width: {layout.sheetWidthMm * scale}px;
      height: {layout.sheetHeightMm * scale}px;
    "
  >
    <!-- Template Slots -->
    {#each layout.slots as slot, i}
      {@const recordIdx = startIndex + i}
      {@const hasRecord = recordIdx < totalRecords}
      <div
        class="absolute border transition-all duration-150 {hasRecord ? 'border-primary bg-primary/5' : 'border-dashed border-border/40 bg-muted/10'}"
        style="
          left: {slot.xMm * scale}px;
          top: {slot.yMm * scale}px;
          width: {slot.widthMm * scale}px;
          height: {slot.heightMm * scale}px;
        "
      >
        {#if hasRecord}
          <div class="absolute inset-0 flex flex-col items-center justify-center gap-0.5 overflow-hidden p-0.5">
            <span class="text-[9px] font-bold text-primary/80">
              #{recordIdx + 1}
            </span>
            {#if entityLabels[recordIdx]}
              <span class="text-[7px] text-muted-foreground truncate max-w-full px-1 text-center leading-tight">
                {entityLabels[recordIdx]}
              </span>
            {/if}
          </div>
        {:else}
          <div class="absolute inset-0 flex items-center justify-center">
            <span class="text-[8px] text-muted-foreground/40">—</span>
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <!-- Page Navigation -->
  {#if totalPages > 1}
    <div class="flex items-center gap-2">
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={prevPage}
        disabled={currentPage <= 1}
      >
        <ChevronLeft size={14} class={getLocale() === "ar" ? "rtl:rotate-180" : ""} />
      </Button>
      <span class="text-[10px] text-muted-foreground font-medium min-w-[80px] text-center">
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

  <!-- Layout Stats -->
  <div class="flex items-center gap-2 flex-wrap justify-center">
    <Badge variant="secondary" class="text-[10px]">
      {layout.columns}×{layout.rows} = {layout.itemsPerSheet} / {m.common_print_multi_up_items_per_sheet()}
    </Badge>
    <Badge variant="outline" class="text-[10px]">
      {totalPages} {m.common_print_multi_up_sheets_needed()}
    </Badge>
  </div>
</div>
