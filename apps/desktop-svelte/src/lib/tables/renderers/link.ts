/**
 * @orbis/data-grid — Link Cell Renderer
 * Draws clickable hyperlinks inside cell.
 */

import type { CellRenderer } from "@orbis/data-grid";

export interface LinkRendererConfig {
  /** Row property for href (default: cell value) */
  hrefKey?: string;
  /** Open in a new tab */
  external?: boolean;
  /** Truncate text to N characters */
  truncate?: number;
}

export function linkRenderer(config?: LinkRendererConfig): CellRenderer {
  const { hrefKey, external = true, truncate } = config ?? {};

  return ({ h, value, row, prop }) => {
    const text = String(value ?? "");
    const href = hrefKey ? String(row[hrefKey] ?? "") : text;

    if (!href) return text;

    const displayText =
      truncate && text.length > truncate
        ? `${text.slice(0, truncate)}...`
        : text;

    const attrs: Record<string, string> = {
      class: "orbis-r-link text-primary hover:underline",
      href,
    };

    if (external) {
      attrs.target = "_blank";
      attrs.rel = "noopener noreferrer";
    }

    return h("a", attrs, displayText);
  };
}
