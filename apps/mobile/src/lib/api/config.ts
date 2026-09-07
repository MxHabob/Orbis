/**
 * @module @orbis/mobile-api/config
 *
 * Environment and connection configuration for the Orbis Mobile API.
 * Automatically resolves localhost, Android Emulator loopback, and LAN host.
 */

import Constants from 'expo-constants';
import { Platform } from 'react-native';

let customBaseUrl: string | null = null;

export function resolveDefaultApiUrl(): string {
  // 1. Explicit public env override
  if (process.env.EXPO_PUBLIC_API_URL) {
    return process.env.EXPO_PUBLIC_API_URL;
  }

  // 2. Physical device connected via Expo dev server LAN IP
  const hostUri = Constants.expoConfig?.hostUri;
  if (hostUri) {
    const ip = hostUri.split(':')[0];
    if (ip && ip !== 'localhost' && ip !== '127.0.0.1') {
      return `http://${ip}:3000`;
    }
  }

  // 3. Android Emulator loopback
  if (Platform.OS === 'android') {
    return 'http://10.0.2.2:3000';
  }

  // 4. iOS Simulator and Web fallback
  return 'http://localhost:3000';
}

export function getApiBaseUrl(): string {
  if (customBaseUrl) {
    return customBaseUrl;
  }
  return resolveDefaultApiUrl();
}

export function setApiBaseUrl(url: string | null): void {
  if (url) {
    customBaseUrl = url.replace(/\/$/, '');
  } else {
    customBaseUrl = null;
  }
}
