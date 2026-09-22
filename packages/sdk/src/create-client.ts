/**
 * Orbis Unified API Client SDK (Enterprise Edition).
 * High-performance, type-safe communication layer with integrated
 * authentication, header injection, and response handling.
 */

import { client } from "./client.gen.js";
import * as sdk from "./sdk.gen.js";

export { client as heyApiClient } from "./client.gen.js";

/**
 * SDK Configuration Interfaces
 */
export interface ClientOptions {
  baseUrl: string;
  headers?: () => Promise<Record<string, string>> | Record<string, string>;
  onUnauthorized?: () => void;
  onTokenRefreshed?: (token: string) => void | Promise<void>;
}

const DEFAULT_API_URL = "http://localhost:8017";

const resolveInitialBaseUrl = (): string => {
  if (typeof process !== "undefined" && process.env?.EXPO_PUBLIC_API_URL) {
    return process.env.EXPO_PUBLIC_API_URL;
  }
  if (typeof process !== "undefined" && process.env?.NEXT_PUBLIC_API_URL) {
    return process.env.NEXT_PUBLIC_API_URL;
  }
  return DEFAULT_API_URL;
};

// Initial default configuration
client.setConfig({
  baseUrl: resolveInitialBaseUrl(),
});

/**
 * MAIN CLIENT FACTORY
 * Configures the global @orbis/sdk client with interceptors for dynamic headers,
 * URL resolution, and authentication recovery.
 */
export const createOrbisClient = (options: ClientOptions) => {
  const { baseUrl } = options;

  client.setConfig({ baseUrl });

  /**
   * REQUEST INTERCEPTOR: Header Injection & URL Normalization
   */
  client.interceptors.request.use(async (request, reqOptions: any) => {
    let finalUrl = request.url;
    const configBaseUrl = client.getConfig().baseUrl;

    // Ensure URL is absolute
    if (
      configBaseUrl &&
      !finalUrl.startsWith("http") &&
      !finalUrl.startsWith("//")
    ) {
      finalUrl = `${configBaseUrl.replace(/\/$/, "")}/${finalUrl.replace(/^\//, "")}`;
    } else if (
      configBaseUrl &&
      typeof window !== "undefined" &&
      window.location?.origin &&
      finalUrl.startsWith(window.location.origin)
    ) {
      const relativePath = finalUrl.slice(window.location.origin.length);
      finalUrl = `${configBaseUrl.replace(/\/$/, "")}/${relativePath.replace(/^\//, "")}`;
    }

    const headers = new Headers(request.headers);

    // Inject custom dynamic headers (e.g. Bearer token from secure storage)
    if (options?.headers) {
      const extraHeaders =
        typeof options.headers === "function"
          ? await options.headers()
          : options.headers;
      Object.entries(extraHeaders).forEach(([k, v]) => {
        if (v) {
          headers.set(k, v as string);
        }
      });
    }

    // Inject per-request headers if present
    if (reqOptions?.headers) {
      const extraHeaders =
        typeof reqOptions.headers === "function"
          ? await reqOptions.headers()
          : reqOptions.headers;
      Object.entries(extraHeaders).forEach(([k, v]) => {
        if (v) {
          headers.set(k, v as string);
        }
      });
    }

    // Body handling for React Native fetch polyfill
    const requestBody =
      reqOptions?.serializedBody ?? reqOptions?.body ?? request.body;

    return new Request(finalUrl, {
      method: request.method,
      headers: headers,
      body: requestBody,
      credentials: "omit",
      // @ts-expect-error - Required for streaming requests in fetch environments
      duplex: "half",
    });
  });

  /**
   * RESPONSE INTERCEPTOR: Unauthorized (401) handling & error interception
   */
  client.interceptors.response.use(async (response, request) => {
    if (
      response.status === 401 &&
      !request.url.includes("/api/v1/auth/login")
    ) {
      options.onUnauthorized?.();
    }
    return response;
  });

  return {
    client,
    ...sdk,
  };
};
