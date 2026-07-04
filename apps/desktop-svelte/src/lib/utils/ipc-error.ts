/**
 * Extract a human-readable error message from Tauri IPC CommandError objects.
 *
 * Tauri `invoke()` throws **plain objects** matching `{ message: string; code?: string }`.
 * They are NOT `Error` instances — using `String(e)` produces `[object Object]`.
 *
 * @example
 * ```ts
 * catch (e: unknown) {
 *   toast.error(extractErrorMessage(e));
 * }
 * ```
 */
export function extractErrorMessage(e: unknown): string {
  // Tauri CommandError — plain object with `message` field
  if (typeof e === "object" && e !== null && "message" in e) {
    return String((e as { message: unknown }).message);
  }
  // Standard JS Error
  if (e instanceof Error) return e.message;
  // Fallback
  if (typeof e === "string") return e;
  return "Unknown error";
}
