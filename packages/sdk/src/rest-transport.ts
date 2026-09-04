import { OrbisError } from "./errors.js";
import type { SessionTokens, TokenStore } from "./token-store.js";
import type { Transport, TransportRequest } from "./transport.js";
import { buildUrl } from "./transport.js";

export interface RestTransportOptions {
  baseUrl: string;
  tokenStore: TokenStore;
  fetchImpl?: typeof fetch;
  onUnauthorized?: () => void | Promise<void>;
  refreshSession?: () => Promise<SessionTokens | null>;
}

export interface ApiEnvelope<T> {
  success: boolean;
  data?: T;
  error?: {
    code?: string;
    message?: string;
    field?: string;
    requestId?: string;
  };
}

export class RestTransport implements Transport {
  private refreshPromise: Promise<SessionTokens | null> | null = null;

  constructor(private readonly options: RestTransportOptions) {}

  async request<T = unknown>(request: TransportRequest): Promise<T> {
    const token = await this.options.tokenStore.get();
    const headers: Record<string, string> = {
      Accept: "application/json",
      ...(request.headers ?? {}),
    };

    if (request.method !== "GET" && request.body !== undefined && !(request.body instanceof FormData)) {
      headers["Content-Type"] = "application/json";
    }

    if (token?.accessToken) {
      headers.Authorization = `Bearer ${token.accessToken}`;
    }

    const url = buildUrl(this.options.baseUrl, request.path, request.query);
    const response = await (this.options.fetchImpl ?? fetch)(url, {
      method: request.method,
      headers,
      body: request.body !== undefined
        ? request.body instanceof FormData
          ? request.body
          : JSON.stringify(request.body)
        : undefined,
      signal: request.signal,
    });

    if (response.status === 401) {
      await this.handleUnauthorized();
      throw new OrbisError("Unauthorized", "AUTH_EXPIRED", { status: 401 });
    }

    const text = await response.text();
    let payload: ApiEnvelope<T> | null = null;

    if (text) {
      try {
        payload = JSON.parse(text) as ApiEnvelope<T>;
      } catch {
        throw new OrbisError("Invalid JSON response", "NETWORK_ERROR", { status: response.status });
      }
    }

    if (!response.ok) {
      const message = payload?.error?.message ?? response.statusText ?? "Request failed";
      const code = (payload?.error?.code as ErrorCode) ?? "UNKNOWN_ERROR";
      throw new OrbisError(message, code, {
        status: response.status,
        field: payload?.error?.field,
        requestId: payload?.error?.requestId,
      });
    }

    if (payload?.success === false) {
      const message = payload.error?.message ?? "Request failed";
      const code = (payload.error?.code as ErrorCode) ?? "UNKNOWN_ERROR";
      throw new OrbisError(message, code, {
        status: response.status,
        field: payload.error?.field,
        requestId: payload.error?.requestId,
      });
    }

    return (payload?.data as T) ?? ((payload ?? null) as T);
  }

  private async handleUnauthorized(): Promise<void> {
    if (this.options.onUnauthorized) {
      await this.options.onUnauthorized();
      return;
    }

    if (!this.options.refreshSession) {
      return;
    }

    if (!this.refreshPromise) {
      this.refreshPromise = this.options.refreshSession();
    }

    try {
      const next = await this.refreshPromise;
      if (next) {
        await this.options.tokenStore.set(next);
      } else {
        await this.options.tokenStore.clear();
      }
    } finally {
      this.refreshPromise = null;
    }
  }
}

export function createRestTransport(options: RestTransportOptions): Transport {
  return new RestTransport(options);
}

type ErrorCode = "AUTH_EXPIRED" | "AUTH_FAILED" | "UNAUTHORIZED" | "FORBIDDEN" | "NOT_FOUND" | "VALIDATION_ERROR" | "CONFLICT" | "RATE_LIMITED" | "NETWORK_ERROR" | "INTERNAL_ERROR" | "UNKNOWN_ERROR";
