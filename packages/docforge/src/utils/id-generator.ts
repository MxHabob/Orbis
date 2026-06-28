// ═══════════════════════════════════════════════════════════════
// @docforge/core — ID Generator
// Generates unique, human-readable node IDs
// ═══════════════════════════════════════════════════════════════

let counter = 0;

/**
 * Generate a unique node ID with a type-based prefix.
 * Format: "{type}_{shortId}" e.g., "text_a3f8b2"
 */
export function generateNodeId(nodeType: string): string {
  counter++;
  const timestamp = Date.now().toString(36);
  const random = Math.random().toString(36).substring(2, 6);
  const prefix = nodeType.toLowerCase().replace(/[^a-z]/g, "");
  return `${prefix}_${timestamp}${random}`;
}

/**
 * Generate a UUID v4 string.
 * Uses crypto.randomUUID if available, otherwise fallback.
 */
export function generateUUID(): string {
  if (typeof crypto !== "undefined" && crypto.randomUUID) {
    return crypto.randomUUID();
  }
  // Fallback for environments without crypto.randomUUID
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0;
    const v = c === "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

/** Reset the internal counter (useful for testing) */
export function resetIdCounter(): void {
  counter = 0;
}
