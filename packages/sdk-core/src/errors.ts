/**
 * OrbisError — unified error handling across all platforms.
 *
 * Replaces the fragmented error handling where Desktop used
 * extractErrorMessage() and Web used different patterns.
 */

export class OrbisError extends Error {
  public readonly code: string;
  public readonly statusCode: number;
  public readonly details?: unknown;

  constructor(message: string, code: string = "UNKNOWN", statusCode: number = 500, details?: unknown) {
    super(message);
    this.name = "OrbisError";
    this.code = code;
    this.statusCode = statusCode;
    this.details = details;
  }

  static fromUnknown(e: unknown): OrbisError {
    if (e instanceof OrbisError) return e;
    if (e instanceof Error) return new OrbisError(e.message, "ERROR");
    return new OrbisError(extractErrorMessage(e), "UNKNOWN");
  }

  static notFound(entity: string, id: string): OrbisError {
    return new OrbisError(`${entity} '${id}' not found`, "NOT_FOUND", 404);
  }

  static unauthorized(message: string = "Unauthorized"): OrbisError {
    return new OrbisError(message, "UNAUTHORIZED", 401);
  }

  static forbidden(message: string = "Forbidden"): OrbisError {
    return new OrbisError(message, "FORBIDDEN", 403);
  }

  static validation(message: string): OrbisError {
    return new OrbisError(message, "VALIDATION", 422);
  }

  static conflict(message: string): OrbisError {
    return new OrbisError(message, "CONFLICT", 409);
  }
}

/**
 * Extract a human-readable error message from any error type.
 *
 * Handles:
 * - OrbisError → message
 * - Tauri IPC errors → string payload
 * - Standard Error → message
 * - String → itself
 * - Objects with message field → message
 * - Anything else → JSON stringified
 */
export function extractErrorMessage(e: unknown): string {
  if (e instanceof OrbisError) return e.message;
  if (e instanceof Error) return e.message;
  if (typeof e === "string") return e;

  // Tauri IPC errors come as plain objects
  if (e && typeof e === "object") {
    const obj = e as Record<string, unknown>;

    // { message: "..." }
    if (typeof obj.message === "string") return obj.message;

    // { error: "..." }
    if (typeof obj.error === "string") return obj.error;

    // { msg: "..." }
    if (typeof obj.msg === "string") return obj.msg;

    // Nested: { error: { message: "..." } }
    if (obj.error && typeof obj.error === "object") {
      const inner = obj.error as Record<string, unknown>;
      if (typeof inner.message === "string") return inner.message;
    }

    // Last resort — JSON stringify
    try {
      return JSON.stringify(e);
    } catch {
      return "[object Object]";
    }
  }

  return String(e);
}
