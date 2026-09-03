// ── createStickyColumns ─────────────────────────────────────────────
// Computes CSS positions for sticky (pinned) columns.
// Uses CSS logical properties (inset-inline-start) for RTL/LTR support.
// Ported from the React hook: packages/excel-grid/src/hooks/use-sticky-columns.ts

import type { StickyColumnConfig } from "./table-types.js";

export interface StickyColumnsOptions {
  /** Which columns are sticky and their widths */
  stickyColumns: StickyColumnConfig[];
  /** Current column visibility state */
  columnVisibility?: Record<string, boolean>;
}

export interface StickyColumnsState {
  /** Get inline style for a column (contains --stick-start CSS variable) */
  getStickyStyle: (columnId: string) => Record<string, string>;
  /** Get CSS class string for a column */
  getStickyClass: (columnId: string, baseClass?: string) => string;
  /** Check if a column is configured as sticky */
  isSticky: (columnId: string) => boolean;
}

/**
 * Creates reactive sticky column positioning.
 *
 * Uses CSS custom property `--stick-start` with `inset-inline-start`
 * so sticky columns work seamlessly in both LTR and RTL layouts.
 *
 * @example
 * ```svelte
 * <script>
 * const sticky = createStickyColumns({
 *   stickyColumns: [{ id: 'name', width: 250 }],
 *   columnVisibility,
 * });
 * </script>
 *
 * <th
 *   class={sticky.getStickyClass('name', 'px-4')}
 *   style={Object.entries(sticky.getStickyStyle('name')).map(([k,v]) => `${k}:${v}`).join(';')}
 * >
 * ```
 */
export function createStickyColumns(
  options: StickyColumnsOptions,
): StickyColumnsState {
  const stickyIds = new Set(options.stickyColumns.map((c) => c.id));

  // ── Derived: Cumulative Positions ────────────────────────────────

  const positions = $derived.by(() => {
    const vis = options.columnVisibility ?? {};
    const result: Record<string, number> = {};
    let offset = 0;

    for (const col of options.stickyColumns) {
      // Only count visible columns
      if (vis[col.id] === false) continue;
      result[col.id] = offset;
      offset += col.width;
    }

    return result;
  });

  // ── Public API ──────────────────────────────────────────────────

  function getStickyStyle(columnId: string): Record<string, string> {
    const pos = positions[columnId];
    if (pos === undefined) return {};
    return { "--stick-start": `${pos}px` };
  }

  function getStickyClass(columnId: string, baseClass?: string): string {
    const parts: string[] = [];
    if (baseClass) parts.push(baseClass);

    if (stickyIds.has(columnId)) {
      parts.push(
        "sticky",
        "inset-inline-start-[var(--stick-start)]",
        "z-10",
        "bg-background",
      );
    }

    return parts.join(" ");
  }

  function isSticky(columnId: string): boolean {
    return stickyIds.has(columnId);
  }

  return {
    getStickyStyle,
    getStickyClass,
    isSticky,
  };
}
