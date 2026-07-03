/**
 * @orbis/data-grid — Avatar Cell Renderer
 * Draws a user avatar image or initials fallback using hyperscript.
 */

import type { CellRenderer } from "@orbis/data-grid";
import { createInitials } from "@orbis/data-grid";

export interface AvatarRendererConfig {
  /** Row property for the image URL (default: 'avatar_url') */
  srcKey?: string;
  /** Row property for the display name (default: same as column prop) */
  nameKey?: string;
  /** Avatar size: 'sm' (24px) | 'md' (28px) | 'lg' (36px) */
  size?: "sm" | "md" | "lg";
  /** Show the name text next to the avatar */
  showName?: boolean;
}

export function avatarRenderer(config?: AvatarRendererConfig): CellRenderer {
  const {
    srcKey = "avatar_url",
    nameKey,
    size = "md",
    showName = true,
  } = config ?? {};

  return ({ h, value, row, prop }) => {
    const name = String(row[nameKey ?? prop] ?? value ?? "");
    const src = row[srcKey] as string | undefined;
    const initials = createInitials(name);
    const sizeClass = `orbis-r-avatar--${size}`;

    const innerEl = src
      ? h("img", {
          class: "orbis-r-avatar__img",
          src,
          alt: name,
          loading: "lazy",
        })
      : h("span", { class: "orbis-r-avatar__fallback-text" }, initials);

    const avatarEl = h(
      "span",
      {
        class: `orbis-r-avatar__root ${sizeClass} ${
          src
            ? "orbis-r-avatar__root--has-img"
            : "orbis-r-avatar__root--has-fallback"
        }`,
      },
      [innerEl],
    );

    if (!showName) return avatarEl;

    return h("div", { class: "orbis-r-avatar" }, [
      avatarEl,
      h("span", { class: "orbis-r-avatar__name" }, name),
    ]);
  };
}
