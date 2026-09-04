export type ErrorCode =
  | "AUTH_EXPIRED"
  | "AUTH_FAILED"
  | "UNAUTHORIZED"
  | "FORBIDDEN"
  | "NOT_FOUND"
  | "VALIDATION_ERROR"
  | "CONFLICT"
  | "RATE_LIMITED"
  | "NETWORK_ERROR"
  | "INTERNAL_ERROR"
  | "UNKNOWN_ERROR";

export interface OrbisErrorPayload {
  code?: ErrorCode;
  message?: string;
  field?: string;
  status?: number;
  requestId?: string;
  cause?: unknown;
}

export class OrbisError extends Error {
  readonly code: ErrorCode;
  readonly status?: number;
  readonly field?: string;
  readonly requestId?: string;
  readonly cause?: unknown;

  constructor(message: string, code: ErrorCode = "UNKNOWN_ERROR", init?: Partial<OrbisErrorPayload>) {
    super(message);
    this.name = "OrbisError";
    this.code = code;
    this.status = init?.status;
    this.field = init?.field;
    this.requestId = init?.requestId;
    this.cause = init?.cause;
  }

  static fromUnknown(error: unknown, fallbackMessage = "Unexpected error"): OrbisError {
    if (error instanceof OrbisError) {
      return error;
    }

    if (error && typeof error === "object") {
      const maybeMessage = "message" in error ? String((error as { message?: unknown }).message ?? "") : "";
      const maybeCode = "code" in error ? String((error as { code?: unknown }).code ?? "UNKNOWN_ERROR") : "UNKNOWN_ERROR";
      return new OrbisError(maybeMessage || fallbackMessage, maybeCode as ErrorCode, {
        cause: error,
      });
    }

    return new OrbisError(String(error ?? fallbackMessage), "UNKNOWN_ERROR");
  }
}

export function extractErrorMessage(error: unknown): string {
  if (error instanceof OrbisError) {
    return error.message;
  }

  if (error && typeof error === "object" && "message" in error) {
    const message = (error as { message?: unknown }).message;
    return typeof message === "string" ? message : "Unknown error";
  }

  return String(error ?? "Unknown error");
}
