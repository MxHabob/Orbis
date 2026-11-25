/**
 * Orbis Unified API Client SDK (Enterprise Edition).
 * High-performance, type-safe communication layer with integrated
 * authentication, header injection, and response handling.
 */

import { client } from "./gen/client.gen.js";
import { createClient, createConfig } from "./gen/client/index.js";
import * as sdk from "./gen/sdk.gen.js";

export { client as heyApiClient } from "./gen/client.gen.js";

/**
 * SDK Configuration Interfaces
 */
export interface OrbisClientOptions {
  baseUrl: string;
  headers?: () => Promise<Record<string, string>> | Record<string, string>;
  onUnauthorized?: () => void;
  onTokenRefreshed?: (token: string) => void | Promise<void>;
}

export type ClientOptions = OrbisClientOptions;

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
 * Creates an independent client with interceptors for dynamic headers,
 * URL resolution, and authentication recovery.
 */
export const createOrbisClient = (options: OrbisClientOptions) => {
  const { baseUrl } = options;

  const client = createClient(createConfig({ baseUrl }));

  client.interceptors.request.use(async (request, reqOptions) => {
    const headers = new Headers(request.headers);

    // Inject custom dynamic headers (e.g. Bearer token from secure storage)
    if (options?.headers) {
      const extraHeaders =
        typeof options.headers === "function"
          ? await options.headers()
          : options.headers;
      Object.entries(extraHeaders).forEach(([k, v]) => {
        if (v && !headers.has(k)) {
          headers.set(k, v);
        }
      });
    }

    if (reqOptions.baseUrl !== client.getConfig().baseUrl) {
      throw new Error("The Orbis API endpoint changed before this request was sent");
    }

    // Body handling for React Native fetch polyfill
    const requestBody =
      reqOptions.serializedBody ?? request.body;

    return new Request(request.url, {
      method: request.method,
      headers: headers,
      body: requestBody,
      credentials: "omit",
      signal: request.signal,
      redirect: request.redirect,
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

  // The generated module exports endpoints only. Bind each export while
  // preserving its generated request and response types for callers.
  const endpoints = Object.fromEntries(
    Object.entries(sdk).map(([name, endpoint]) => [
      name,
      (requestOptions: { client?: typeof client } = {}) =>
        (endpoint as (options: unknown) => unknown)({
          ...requestOptions,
          client: requestOptions.client ?? client,
        }),
    ]),
  ) as unknown as typeof sdk;
  return { ...endpoints, client };
};

export type OrbisClient = ReturnType<typeof createOrbisClient>;
