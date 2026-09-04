import { AuthClient } from "./auth.js";
import { OrbisError } from "./errors.js";
import type { TokenStore } from "./token-store.js";
import { createRestTransport } from "./rest-transport.js";
import type { Transport } from "./transport.js";

export interface OrbisClientOptions {
  baseUrl?: string;
  transport?: Transport;
  tokenStore?: TokenStore;
  onUnauthorized?: () => void | Promise<void>;
  refreshSession?: () => Promise<{ accessToken: string; refreshToken: string; expiresAt?: string; refreshExpiresAt?: string } | null>;
}

export class OrbisClient {
  private readonly transport: Transport;
  private readonly tokenStore?: TokenStore;
  readonly auth: AuthClient;

  constructor(options: OrbisClientOptions = {}) {
    const tokenStore = options.tokenStore ?? {
      async get() { return null; },
      async set() {},
      async clear() {},
    };

    this.tokenStore = tokenStore;

    this.transport = options.transport ?? createRestTransport({
      baseUrl: options.baseUrl ?? "http://localhost:3002",
      tokenStore,
      onUnauthorized: options.onUnauthorized,
      refreshSession: options.refreshSession,
    });

    this.auth = new AuthClient(this, tokenStore);
  }

  get transportInstance(): Transport {
    return this.transport;
  }

  get tokenStoreInstance(): TokenStore | undefined {
    return this.tokenStore;
  }

  async request<T = unknown>(
    method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
    path: string,
    payload?: unknown,
    query?: Record<string, string | number | boolean | null | undefined>,
  ): Promise<T> {
    try {
      return await this.transport.request<T>({
        method,
        path,
        body: payload,
        query,
      });
    } catch (error) {
      throw OrbisError.fromUnknown(error, "Request failed");
    }
  }

  async get<T = unknown>(path: string, query?: Record<string, string | number | boolean | null | undefined>): Promise<T> {
    return this.request<T>("GET", path, undefined, query);
  }

  async post<T = unknown>(path: string, body?: unknown): Promise<T> {
    return this.request<T>("POST", path, body);
  }

  async put<T = unknown>(path: string, body?: unknown): Promise<T> {
    return this.request<T>("PUT", path, body);
  }

  async patch<T = unknown>(path: string, body?: unknown): Promise<T> {
    return this.request<T>("PATCH", path, body);
  }

  async del<T = unknown>(path: string): Promise<T> {
    return this.request<T>("DELETE", path);
  }
}

export function createOrbisClient(options: OrbisClientOptions = {}): OrbisClient {
  return new OrbisClient(options);
}
