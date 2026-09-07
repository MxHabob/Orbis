/**
 * @module @orbis/mobile-api/storage
 *
 * Secure persistent storage wrapper for mobile.
 * Uses hardware-backed Keychain/KeyStore via expo-secure-store on iOS/Android,
 * with graceful fallback to localStorage / in-memory on Web and unsupported platforms.
 */

import * as SecureStore from 'expo-secure-store';
import { Platform } from 'react-native';

export const STORAGE_KEYS = {
  TOKEN: 'orbis_token',
  USER: 'orbis_user',
  DEVICE_ID: 'orbis_device_id',
  SETUP_COMPLETE: 'orbis_is_setup_complete',
  API_BASE_URL: 'orbis_api_base_url',
} as const;

const memoryFallback = new Map<string, string>();

async function isSecureStoreSupported(): Promise<boolean> {
  if (Platform.OS === 'web') return false;
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
  if (typeof window !== 'undefined' && window.localStorage) {
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

  if (typeof window !== 'undefined' && window.localStorage) {
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

  if (typeof window !== 'undefined' && window.localStorage) {
    window.localStorage.removeItem(key);
    return;
  }
  memoryFallback.delete(key);
}
