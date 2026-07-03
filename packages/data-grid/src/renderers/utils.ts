/**
 * @orbis/data-grid — Renderer Utilities
 *
 * Lightweight helper functions for building cell renderers.
 * No UI, no framework dependencies — just pure functions.
 */

// ── Initials ─────────────────────────────────────────────────────────────────

/**
 * Extract initials from a name string.
 * Handles Latin and Arabic names.
 *
 * @example
 * createInitials("Ali Hassan")  // → "AH"
 * createInitials("سارة أحمد")   // → "سأ"
 * createInitials("A")           // → "A"
 * createInitials("")            // → "?"
 */
export function createInitials(name: string, maxChars = 2): string {
  if (!name || !name.trim()) return "?";
  const words = name.trim().split(/\s+/);
  if (words.length === 1) {
    return words[0]!.slice(0, maxChars).toUpperCase();
  }
  return words
    .slice(0, maxChars)
    .map((w) => w[0])
    .join("")
    .toUpperCase();
}

// ── Threshold Resolution ─────────────────────────────────────────────────────

export interface Threshold {
  /** Minimum value for this variant (inclusive) */
  min: number;
  /** Variant name (e.g. 'success', 'warning', 'error') */
  variant: string;
}

/**
 * Resolve a numeric value to a variant string based on thresholds.
 * Thresholds are checked from highest `min` to lowest.
 *
 * @example
 * resolveThreshold(85, [
 *   { min: 90, variant: 'success' },
 *   { min: 70, variant: 'warning' },
 *   { min: 0, variant: 'error' },
 * ]) // → 'warning'
 */
export function resolveThreshold(
  value: number,
  thresholds?: Threshold[],
  fallback = "default",
): string {
  if (!thresholds || thresholds.length === 0) return fallback;

  // Sort descending by min value
  const sorted = [...thresholds].sort((a, b) => b.min - a.min);
  for (const t of sorted) {
    if (value >= t.min) return t.variant;
  }
  return fallback;
}

// ── Icon SVG ─────────────────────────────────────────────────────────────────

/**
 * Minimal SVG icon paths for common grid actions.
 * Uses Lucide icon paths. Consumer can add more via registerIcon().
 */
const _icons: Record<string, string> = {
  // Edit / Write
  Pencil: "M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z",
  // Delete
  Trash2:
    "M3 6h18M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2M10 11v6M14 11v6",
  // View / Eye
  Eye: "M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0M12 12a3 3 0 1 0 0-.001",
  // More / Ellipsis
  MoreHorizontal: "M12 12h.01M19 12h.01M5 12h.01",
  // Check
  Check: "M20 6 9 17l-5-5",
  // X / Close
  X: "M18 6 6 18M6 6l12 12",
  // Download
  Download: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3",
  // Copy
  Copy: "M20 8H10a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V10a2 2 0 0 0-2-2ZM4 16a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2",
  // External link
  ExternalLink:
    "M15 3h6v6M10 14 21 3M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6",
  // User
  User: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8Z",
  // Mail
  Mail: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2Zm16 2-8 5-8-5",
  // Phone
  Phone:
    "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.127.96.361 1.903.7 2.81a2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0 1 22 16.92Z",
  // Lock
  Lock: "M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2ZM7 11V7a5 5 0 0 1 10 0v4",
  // Unlock
  Unlock:
    "M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2ZM17 11V7a5 5 0 0 0-9.584-1.995",
  // Star
  Star: "m12 2 3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2Z",
  // Settings
  Settings:
    "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2ZM12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z",
};

/**
 * Register a custom icon SVG path for use in renderers.
 *
 * @example
 * registerIcon('CustomIcon', 'M12 2L2 22h20Z');
 */
export function registerIcon(name: string, svgPath: string): void {
  _icons[name] = svgPath;
}

/**
 * Generate a minimal inline SVG string for a Lucide icon.
 * Designed for use inside h() renderers via innerHTML.
 *
 * @example
 * // In a renderer:
 * h('span', { class: 'icon', innerHTML: iconSvg('Pencil', 14) })
 */
export function iconSvg(name: string, size = 16): string {
  const path = _icons[name];
  if (!path) return "";
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="${path}"/></svg>`;
}
