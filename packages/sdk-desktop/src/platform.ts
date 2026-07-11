/**
 * Platform utilities for Tauri desktop environment.
 */
import { invoke } from "@tauri-apps/api/core";

/** Check if running inside a Tauri desktop app. */
export function isDesktopApp(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/** Listen for OS deep link events. */
export async function listenForDeepLinks(
  callback: (url: string) => void
): Promise<() => void> {
  const { listen } = await import("@tauri-apps/api/event");
  const unlisten = await listen<string>("deep-link://new-url", (event) => {
    callback(event.payload);
  });
  return unlisten;
}

/** Create a deep link URL for the app. */
export function createDeepLink(path: string): string {
  return `orbis://${path}`;
}
