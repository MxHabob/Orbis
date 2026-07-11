/**
 * @orbis/sdk-web — HTTP client for the Orbis API Gateway.
 *
 * Used by the Dashboard (Next.js) and any other web app.
 * Manages token, base URL, and standard error handling.
 */

import { OrbisError } from "@orbis/sdk-core";

export interface HttpClientConfig {
  baseUrl: string;
  getToken: () => string | null;
  onUnauthorized?: () => void;
}

let _config: HttpClientConfig = {
  baseUrl: "",
  getToken: () => null,
};

/** Configure the HTTP client — call once at app startup. */
export function configure(config: HttpClientConfig): void {
  _config = config;
}

/**
 * Low-level HTTP request wrapper.
 * All SDK methods go through here.
 */
export async function request<T>(
  method: "GET" | "POST" | "PUT" | "DELETE",
  path: string,
  body?: unknown,
): Promise<T> {
  const token = _config.getToken();
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    Accept: "application/json",
  };
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const url = `${_config.baseUrl}${path}`;
  const init: RequestInit = { method, headers };
  if (body !== undefined && method !== "GET") {
    init.body = JSON.stringify(body);
  }

  const res = await fetch(url, init);

  if (res.status === 401) {
    _config.onUnauthorized?.();
    throw new OrbisError("Unauthorized", "AUTH_EXPIRED");
  }

  const json = await res.json().catch(() => null);

  if (!res.ok) {
    const msg = json?.error ?? json?.message ?? res.statusText;
    throw new OrbisError(msg, `HTTP_${res.status}`);
  }

  // API gateway wraps in { success, data }
  return (json?.data ?? json) as T;
}

/** Convenience wrappers. */
export const http = {
  get: <T>(path: string) => request<T>("GET", path),
  post: <T>(path: string, body?: unknown) => request<T>("POST", path, body),
  put: <T>(path: string, body?: unknown) => request<T>("PUT", path, body),
  del: <T>(path: string) => request<T>("DELETE", path),
};
