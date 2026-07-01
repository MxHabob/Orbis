// ═══════════════════════════════════════════════════════════════
// @docforge/core — Color Utilities
// Parse, convert, and manipulate colors — zero dependencies
// ═══════════════════════════════════════════════════════════════

/** Parse a hex color (#rgb or #rrggbb) to RGB components */
export function hexToRgb(
  hex: string,
): { r: number; g: number; b: number } | null {
  const clean = hex.replace(/^#/, "");
  let r: number, g: number, b: number;

  if (clean.length === 3) {
    r = Number.parseInt(clean[0]! + clean[0]!, 16);
    g = Number.parseInt(clean[1]! + clean[1]!, 16);
    b = Number.parseInt(clean[2]! + clean[2]!, 16);
  } else if (clean.length === 6) {
    r = Number.parseInt(clean.substring(0, 2), 16);
    g = Number.parseInt(clean.substring(2, 4), 16);
    b = Number.parseInt(clean.substring(4, 6), 16);
  } else {
    return null;
  }

  return { r, g, b };
}

/** Convert RGB to hex color string */
export function rgbToHex(r: number, g: number, b: number): string {
  const toHex = (c: number) =>
    Math.max(0, Math.min(255, Math.round(c)))
      .toString(16)
      .padStart(2, "0");
  return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

/** Adjust color opacity — returns rgba() string */
export function withOpacity(hex: string, opacity: number): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;
  return `rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, ${Math.max(0, Math.min(1, opacity))})`;
}

/** Lighten a hex color by a percentage (0-100) */
export function lighten(hex: string, percent: number): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;
  const factor = percent / 100;
  return rgbToHex(
    rgb.r + (255 - rgb.r) * factor,
    rgb.g + (255 - rgb.g) * factor,
    rgb.b + (255 - rgb.b) * factor,
  );
}

/** Darken a hex color by a percentage (0-100) */
export function darken(hex: string, percent: number): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;
  const factor = 1 - percent / 100;
  return rgbToHex(rgb.r * factor, rgb.g * factor, rgb.b * factor);
}
