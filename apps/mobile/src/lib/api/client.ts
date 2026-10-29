/**
 * @module @orbis/mobile/api/client
 *
 * Unified API Client for Orbis Mobile.
 * Configures the @orbis/sdk client with dynamic baseUrl,
 * automatic Bearer token injection from SecureStore, and 401 Unauthorized handling.
 */

import { createOrbisClient } from "@orbis/sdk";
import { Platform } from "react-native";
import { authStorage } from "./storage";
import { isAllowedInstitutionEndpoint } from "./institution-discovery";
import { notifySessionInvalidated } from "./session-events";
import { OrbisError } from "./types";
import { onlineManager } from "@/lib/network/online-manager";

export const API_BASE_URL =
  process.env.EXPO_PUBLIC_API_URL ||
  Platform.select({
    android: "http://10.0.2.2:3000",
    ios: "http://localhost:3000",
    default: "http://localhost:3000",
  });

export function configureApiBaseUrl(baseUrl: string): void {
  const parsedUrl = new URL(baseUrl);
  const isDevelopmentLoopback =
    __DEV__ &&
    parsedUrl.protocol === "http:" &&
    ["localhost", "127.0.0.1", "10.0.2.2"].includes(parsedUrl.hostname);
  if (
    !isAllowedInstitutionEndpoint(parsedUrl.toString()) &&
    !isDevelopmentLoopback
  ) {
    throw new Error(
      "The Orbis server must use HTTPS or a private IPv4 address over HTTP.",
    );
  }
  api.client.setConfig({ baseUrl: parsedUrl.toString().replace(/\/+$/, "") });
}

export const api = createOrbisClient({
  baseUrl: API_BASE_URL,
  headers: async (): Promise<Record<string, string>> => {
    const token = await authStorage.getAccessToken();
    if (token) {
      return {
        Authorization: `Bearer ${token}`,
      };
    }
    return {};
  },
  onTokenRefreshed: async (token: string) => {
    await authStorage.setAccessToken(token);
  },
  onUnauthorized: notifySessionInvalidated,
});

const SAFE_OFFLINE_METHODS = new Set(["GET", "HEAD", "OPTIONS"]);

api.client.interceptors.request.use((request) => {
  if (
    !onlineManager.isOnline() &&
    !SAFE_OFFLINE_METHODS.has(request.method.toUpperCase())
  ) {
    throw new OrbisError(
      "This action requires an internet connection and was not saved.",
      "OFFLINE_WRITE_BLOCKED",
    );
  }

  return request;
});

export default api;
