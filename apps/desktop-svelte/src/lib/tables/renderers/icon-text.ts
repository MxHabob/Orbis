/**
 * @orbis/data-grid — Icon + Text Cell Renderer
 * Combines an inline SVG icon with a text label.
 */

import type { CellRenderer } from "@orbis/data-grid";
import { iconSvg } from "@orbis/data-grid";

export interface IconTextRendererConfig {
  /** Lucide icon name (fixed) */
  icon?: string;
  /** Row property for dynamic icon name mapping */
  iconKey?: string;
  /** Map of cell value to icon name */
  iconMap?: Record<string, string>;
  /** Icon layout position relative to text */
  iconPosition?: "start" | "end";
}

export function iconTextRenderer(config?: IconTextRendererConfig): CellRenderer {
  const { icon, iconKey, iconMap, iconPosition = "start" } = config ?? {};

  return ({ h, value, row }) => {
    const text = String(value ?? "");
    let iconName = icon;

    if (iconKey && row[iconKey]) {
      iconName = String(row[iconKey]);
    } else if (iconMap && iconMap[text]) {
      iconName = iconMap[text];
    }

    if (!iconName) return text;

    const iconHtml = iconSvg(iconName, 14);
    if (!iconHtml) return text;

    const iconEl = h("span", {
      class: "orbis-r-icon-text__icon",
      innerHTML: iconHtml,
    });
    const textEl = h("span", { class: "orbis-r-icon-text__label" }, text);

    const children =
      iconPosition === "start" ? [iconEl, textEl] : [textEl, iconEl];

    return h("div", { class: "orbis-r-icon-text" }, children);
  };
}
