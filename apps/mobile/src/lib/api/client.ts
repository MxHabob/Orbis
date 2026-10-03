/**
 * @module @orbis/mobile/api/client
 *
 * Unified API Client for Orbis Mobile.
 * Configures the @orbis/sdk client with dynamic baseUrl,
 * automatic Bearer token injection from SecureStore, and 401 Unauthorized handling.
 */

import { createOrbisClient } from "@orbis/sdk";
import { router } from "expo-router";
import { Platform } from "react-native";
import { authStorage } from "./storage";

export const API_BASE_URL =
  process.env.EXPO_PUBLIC_API_URL ||
  Platform.select({
    android: "http://10.0.2.2:3000",
    ios: "http://localhost:3000",
    default: "http://localhost:3000",
  });

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
  onUnauthorized: () => {
    authStorage.clear().then(() => {
      try {
        router.replace("/(auth)/sign-in" as any);
      } catch (_e) {
        console.warn("[Orbis Mobile Auth] Redirect failed, routing not fully mounted yet.");
      }
    });
  },
});

export default api;
