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

import type { MultiUpConfig, MultiUpLayout } from "@docforge/core";

let {
  layout,
  totalRecords = 0,
  cutLineWidthMm,
  cutLineColor,
  sheetMargins,
  currentPage = $bindable(1),
}: {
  layout: MultiUpLayout;
  totalRecords: number;
  cutLineWidthMm: number;
  cutLineColor: string;
  sheetMargins: MultiUpConfig["sheetMargins"];
  currentPage: number;
} = $props();

// Derived values
const totalPages = $derived(
  layout.itemsPerSheet > 0 ? Math.ceil(totalRecords / layout.itemsPerSheet) : 1,
);

const visiblePage = $derived(
  Math.max(1, Math.min(currentPage, totalPages || 1)),
);

const startIndex = $derived((visiblePage - 1) * layout.itemsPerSheet);

const sheetAspectRatio = $derived(layout.sheetWidthMm / layout.sheetHeightMm);

const hasOverflow = $derived(
  layout.slots.some(
    (slot) =>
      slot.xMm < -0.5 ||
      slot.yMm < -0.5 ||
      slot.xMm + slot.widthMm > layout.sheetWidthMm + 0.5 ||
      slot.yMm + slot.heightMm > layout.sheetHeightMm + 0.5,
  ),
);

const cutLines = $derived.by(() => {
  if (cutLineWidthMm <= 0) return [];

  const lines: {
    id: string;
    xMm: number;
    yMm: number;
    widthMm: number;
    heightMm: number;
  }[] = [];

  for (let col = 1; col < layout.columns; col++) {
    const leftSlot = layout.slots.find(
      (slot) => slot.col === col - 1 && slot.row === 0,
    );
    const rightSlot = layout.slots.find(
      (slot) => slot.col === col && slot.row === 0,
    );
    if (!leftSlot || !rightSlot) continue;

    const centerMm = (leftSlot.xMm + leftSlot.widthMm + rightSlot.xMm) / 2;
    lines.push({
      id: `vertical-${col}`,
      xMm: centerMm - cutLineWidthMm / 2,
      yMm: sheetMargins.top / 2,
      widthMm: cutLineWidthMm,
      heightMm:
        layout.sheetHeightMm - (sheetMargins.top + sheetMargins.bottom) / 2,
    });
  }

  for (let row = 1; row < layout.rows; row++) {
    const topSlot =
      layout.slots.find((slot) => slot.row === row - 1 && slot.col === 0) ??
      layout.slots.find((slot) => slot.row === row - 1);
    const bottomSlot = layout.slots.find((slot) => slot.row === row);
    if (!topSlot || !bottomSlot) continue;

    const centerMm = (topSlot.yMm + topSlot.heightMm + bottomSlot.yMm) / 2;
    lines.push({
      id: `horizontal-${row}`,
      xMm: sheetMargins.left / 2,
      yMm: centerMm - cutLineWidthMm / 2,
      widthMm:
        layout.sheetWidthMm - (sheetMargins.left + sheetMargins.right) / 2,
      heightMm: cutLineWidthMm,
    });
  }

  return lines;
});

function prevPage() {
  if (visiblePage > 1) currentPage = visiblePage - 1;
}
function nextPage() {
  if (visiblePage < totalPages) currentPage = visiblePage + 1;
}
</script>

<div class="flex h-full min-h-0 w-full flex-col items-center gap-2">
  <!-- Sheet Preview Container -->
  <div
    class="flex min-h-0 min-w-0 w-full flex-1 items-center justify-center"
  >
    <div
      class="relative shrink-0 overflow-hidden rounded-sm border border-border bg-white shadow-md"
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
        {@const slotOverflows =
          slot.xMm < -0.5 ||
          slot.yMm < -0.5 ||
          slot.xMm + slot.widthMm > layout.sheetWidthMm + 0.5 ||
          slot.yMm + slot.heightMm > layout.sheetHeightMm + 0.5}
        <div
          class="absolute border transition-all duration-150 {slotOverflows ? 'border-amber-500 bg-amber-500/10' : hasRecord ? 'border-primary bg-primary/5' : 'border-dashed border-border/40 bg-muted/10'}"
          style="
            left: {slot.xMm / layout.sheetWidthMm * 100}%;
            top: {slot.yMm / layout.sheetHeightMm * 100}%;
            width: {slot.widthMm / layout.sheetWidthMm * 100}%;
            height: {slot.heightMm / layout.sheetHeightMm * 100}%;
          "
        >
          {#if hasRecord}
            <div class="absolute inset-0 flex items-center justify-center overflow-hidden p-1">
              <span class="text-sm font-bold {slotOverflows ? 'text-amber-600' : 'text-primary'}">
                #{recordIdx + 1}
              </span>
            </div>
          {/if}
        </div>
      {/each}

      {#each cutLines as line (line.id)}
        <div
          class="absolute pointer-events-none"
          style="
            left: {line.xMm / layout.sheetWidthMm * 100}%;
            top: {line.yMm / layout.sheetHeightMm * 100}%;
            width: {line.widthMm / layout.sheetWidthMm * 100}%;
            height: {line.heightMm / layout.sheetHeightMm * 100}%;
            background-color: {cutLineColor};
          "
        ></div>
      {/each}
    </div>
  </div>

  <!-- Page Navigation -->
  {#if hasOverflow}
    <p class="text-[11px] text-amber-600 dark:text-amber-400 text-center px-2">
      {m.common_print_multi_up_overflow_hint()}
    </p>
  {/if}
  {#if totalPages > 1}
    <div class="flex shrink-0 items-center gap-2">
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={prevPage}
        disabled={visiblePage <= 1}
      >
        <ChevronLeft size={14} class={getLocale() === "ar" ? "rtl:rotate-180" : ""} />
      </Button>
      <span class="text-xs text-muted-foreground font-medium min-w-[80px] text-center">
        {m.common_print_multi_up_page_of({ current: String(visiblePage), total: String(totalPages) })}
      </span>
      <Button
        variant="ghost"
        size="icon-xs"
        onclick={nextPage}
        disabled={visiblePage >= totalPages}
      >
        <ChevronRight size={14} class={getLocale() === "ar" ? "rtl:rotate-180" : ""} />
      </Button>
    </div>
  {/if}

</div>
