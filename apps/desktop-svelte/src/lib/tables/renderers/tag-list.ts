/**
 * @orbis/data-grid — Tag List Cell Renderer
 * Draws comma-separated tags or array values as a row of S.C.R.A.P.S. tags.
 */

import type { CellRenderer } from "@orbis/data-grid";

export interface TagListRendererConfig {
  /** Separator for string values (default: ',') */
  separator?: string;
  /** Max tags to render before collapsing into "+N more" badge */
  maxVisible?: number;
  /** Badge theme variant */
  variant?: "default" | "outline" | "brand" | "secondary";
}

export function tagListRenderer(config?: TagListRendererConfig): CellRenderer {
  const {
    separator = ",",
    maxVisible = 2,
    variant = "secondary",
  } = config ?? {};

  return ({ h, value }) => {
    let items: string[] = [];
    if (Array.isArray(value)) {
      items = value.map(String);
    } else if (value) {
      items = String(value)
        .split(separator)
        .map((s) => s.trim())
        .filter(Boolean);
    }

    if (items.length === 0) return "";

    const visibleItems = items.slice(0, maxVisible);
    const hiddenCount = items.length - maxVisible;

    const badges = visibleItems.map((item) =>
      h("span", { class: `orbis-r-tag orbis-r-tag--${variant}` }, item),
    );

    if (hiddenCount > 0) {
      badges.push(
        h(
          "span",
          { class: "orbis-r-tag orbis-r-tag--more" },
          `+${hiddenCount}`,
        ),
      );
    }

    return h("div", { class: "orbis-r-tag-list" }, badges);
  };
}
