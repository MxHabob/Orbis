/**
 * @module @orbis/mobile-api/storage
 *
 * Secure persistent storage wrapper for mobile.
 * Uses hardware-backed Keychain/KeyStore via expo-secure-store on iOS/Android,
 * with graceful fallback to localStorage / in-memory on Web and unsupported platforms.
 */

import * as SecureStore from "expo-secure-store";
import { Platform } from "react-native";

export const STORAGE_KEYS = {
  TOKEN: "orbis_token",
  USER: "orbis_user",
  EFFECTIVE_CONTEXT: "orbis_effective_context",
  DEVICE_ID: "orbis_device_id",
  SETUP_COMPLETE: "orbis_is_setup_complete",
  API_BASE_URL: "orbis_api_base_url",
  INSTITUTIONS: "orbis_institutions",
  ACTIVE_INSTITUTION_ID: "orbis_active_institution_id",
  INSTITUTION_SESSIONS: "orbis_institution_sessions",
} as const;

export interface InstitutionSessionSnapshot {
  token: string;
  user: string;
  effectiveContext: string | null;
  setupComplete: string | null;
}

const memoryFallback = new Map<string, string>();

async function isSecureStoreSupported(): Promise<boolean> {
  if (Platform.OS === "web") return false;
  try {
    return await SecureStore.isAvailableAsync();
  } catch {
    return false;
  }
}

export async function getSecureItem(key: string): Promise<string | null> {
  try {
    const supported = await isSecureStoreSupported();
    if (supported) {
      return await SecureStore.getItemAsync(key);
    }
  } catch (err) {
    console.warn(`[SecureStore] Error reading key "${key}":`, err);
  }

  // Fallback to localStorage (Web) or in-memory
  if (typeof window !== "undefined" && window.localStorage) {
    return window.localStorage.getItem(key);
  }
  return memoryFallback.get(key) ?? null;
}

export async function setSecureItem(key: string, value: string): Promise<void> {
  try {
    const supported = await isSecureStoreSupported();
    if (supported) {
      await SecureStore.setItemAsync(key, value, {
        keychainAccessible: SecureStore.AFTER_FIRST_UNLOCK,
      });
      return;
    }
  } catch (err) {
    console.warn(`[SecureStore] Error writing key "${key}":`, err);
  }

  if (typeof window !== "undefined" && window.localStorage) {
    window.localStorage.setItem(key, value);
    return;
  }
  memoryFallback.set(key, value);
}

export async function deleteSecureItem(key: string): Promise<void> {
  try {
    const supported = await isSecureStoreSupported();
    if (supported) {
      await SecureStore.deleteItemAsync(key);
      return;
    }
  } catch (err) {
    console.warn(`[SecureStore] Error deleting key "${key}":`, err);
  }

  if (typeof window !== "undefined" && window.localStorage) {
    window.localStorage.removeItem(key);
    return;
  }
  memoryFallback.delete(key);
}

export async function getInstitutionSession(
  institutionId: string,
): Promise<InstitutionSessionSnapshot | null> {
  const stored = await getSecureItem(STORAGE_KEYS.INSTITUTION_SESSIONS);
  if (!stored) return null;

  const sessions: unknown = JSON.parse(stored);
  if (
    typeof sessions !== "object" ||
    sessions === null ||
    Array.isArray(sessions)
  ) {
    throw new Error("Saved institution sessions have an invalid format.");
  }

  const session = (sessions as Record<string, unknown>)[institutionId];
  if (
    typeof session !== "object" ||
    session === null ||
    !("token" in session) ||
    typeof session.token !== "string" ||
    !("user" in session) ||
    typeof session.user !== "string" ||
    !("effectiveContext" in session) ||
    (typeof session.effectiveContext !== "string" &&
      session.effectiveContext !== null) ||
    !("setupComplete" in session) ||
    (typeof session.setupComplete !== "string" &&
      session.setupComplete !== null)
  ) {
    return null;
  }

  return {
    token: session.token,
    user: session.user,
    effectiveContext: session.effectiveContext,
    setupComplete: session.setupComplete,
  };
}

export async function setInstitutionSession(
  institutionId: string,
  snapshot: InstitutionSessionSnapshot,
): Promise<void> {
  const stored = await getSecureItem(STORAGE_KEYS.INSTITUTION_SESSIONS);
  const current: Record<string, InstitutionSessionSnapshot> = stored
    ? JSON.parse(stored)
    : {};
  await setSecureItem(
    STORAGE_KEYS.INSTITUTION_SESSIONS,
    JSON.stringify({ ...current, [institutionId]: snapshot }),
  );
}

export async function deleteInstitutionSession(
  institutionId: string,
): Promise<void> {
  const stored = await getSecureItem(STORAGE_KEYS.INSTITUTION_SESSIONS);
  if (!stored) return;

  const current: Record<string, InstitutionSessionSnapshot> =
    JSON.parse(stored);
  delete current[institutionId];
  await setSecureItem(
    STORAGE_KEYS.INSTITUTION_SESSIONS,
    JSON.stringify(current),
  );
}

export const authStorage = {
  getAccessToken: () => getSecureItem(STORAGE_KEYS.TOKEN),
  setAccessToken: (token: string) => setSecureItem(STORAGE_KEYS.TOKEN, token),
  clear: async () => {
    await Promise.all([
      deleteSecureItem(STORAGE_KEYS.TOKEN),
      deleteSecureItem(STORAGE_KEYS.USER),
      deleteSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT),
    ]);
  },
};
