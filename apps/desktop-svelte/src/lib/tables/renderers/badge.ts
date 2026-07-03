/**
 * @orbis/data-grid — Badge Cell Renderer
 * Draws a S.C.R.A.P.S.-styled status badge or dot indicator.
 */

import type { CellRenderer } from "@orbis/data-grid";

export interface BadgeRendererConfig {
  /** Map of cell value to visual variant */
  variantMap?: Record<string, string>;
  /** Map of cell value to translated display label */
  labelMap?: Record<string, string>;
  /** Draw as a dot indicator instead of a full badge pill */
  dot?: boolean;
}

export function badgeRenderer(config?: BadgeRendererConfig): CellRenderer {
  return ({ h, value }) => {
    const raw = String(value ?? "");
    const variant = config?.variantMap?.[raw] ?? "default";
    const label = config?.labelMap?.[raw] ?? raw;

    if (config?.dot) {
      return h("span", { class: "orbis-r-badge-dot" }, [
        h("span", { class: `orbis-r-dot orbis-r-dot--${variant}` }),
        h("span", {}, label),
      ]);
    }

    return h(
      "span",
      { class: `orbis-r-badge orbis-r-badge--${variant}` },
      label,
    );
  };
}
