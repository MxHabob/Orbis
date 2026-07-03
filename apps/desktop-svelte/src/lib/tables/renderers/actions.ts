/**
 * @orbis/data-grid — Action Buttons Cell Renderer
 * Draws inline action buttons or an overflow trigger button.
 */

import type { CellRenderer, ActionItem } from "@orbis/data-grid";
import { iconSvg } from "@orbis/data-grid";

export interface ActionsRendererConfig {
  /** Array of actions definition */
  items: ActionItem[];
  /** Layout: 'inline' or 'menu' */
  layout?: "inline" | "menu";
  /** Maximum inline buttons before collapsing to overflow menu */
  maxInline?: number;
}

export function actionsRenderer(config: ActionsRendererConfig): CellRenderer {
  const { items, layout = "inline", maxInline = 3 } = config;

  return ({ h, row, rowIndex, prop }) => {
    const visible = items.filter((a) => !a.visible || a.visible(row));

    if (layout === "menu" || visible.length > maxInline) {
      return h(
        "button",
        {
          class: "orbis-r-action-trigger",
          "data-grid-action": "__menu__",
          "data-row-index": String(rowIndex),
          "data-prop": prop,
          type: "button",
        },
        "⋯",
      );
    }

    return h(
      "div",
      { class: "orbis-r-actions" },
      visible.map((action) => {
        const disabled = action.disabled?.(row) ?? false;
        const iconHtml = action.icon ? iconSvg(action.icon, 14) : "";

        return h(
          "button",
          {
            class: `orbis-r-action-btn orbis-r-action-btn--${action.variant ?? "default"}`,
            "data-grid-action": action.id,
            "data-row-index": String(rowIndex),
            "data-prop": prop,
            disabled: disabled ? "" : undefined,
            title: action.label ?? "",
            type: "button",
            innerHTML: iconHtml || undefined,
          },
          action.icon ? undefined : (action.label ?? action.id),
        );
      }),
    );
  };
}
