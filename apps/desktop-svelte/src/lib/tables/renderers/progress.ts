/**
 * @orbis/data-grid — Progress Bar Cell Renderer
 * Draws a bar chart representing percentages/progress in cells.
 */

import type { CellRenderer, Threshold } from "@orbis/data-grid";
import { resolveThreshold } from "@orbis/data-grid";

export interface ProgressRendererConfig {
  /** Maximum progress value (default: 100) */
  max?: number;
  /** Whether to show a numeric text label next to the bar */
  showLabel?: boolean;
  /** Value thresholds to map to styling variants (e.g. success, warning, error) */
  thresholds?: Threshold[];
}

export function progressRenderer(config?: ProgressRendererConfig): CellRenderer {
  const { max = 100, showLabel = true, thresholds } = config ?? {};

  return ({ h, value }) => {
    const num = Number(value ?? 0);
    const pct = Math.min(Math.max((num / max) * 100, 0), 100);
    const variant = resolveThreshold(pct, thresholds, "default");

    const trackFill = h("div", {
      class: `orbis-r-progress__fill orbis-r-progress__fill--${variant}`,
      style: `width: ${pct}%`,
    });

    const track = h("div", { class: "orbis-r-progress__track" }, [trackFill]);

    if (!showLabel) return h("div", { class: "orbis-r-progress" }, [track]);

    const label = h(
      "span",
      { class: "orbis-r-progress__label" },
      `${pct.toFixed(0)}%`,
    );

    return h("div", { class: "orbis-r-progress" }, [track, label]);
  };
}
