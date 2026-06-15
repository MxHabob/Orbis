/**
 * Typed Tauri event helpers for backend → frontend push.
 */
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const DesktopEvents = {
  notificationCreated: "notification-created",
  syncStatusChanged: "sync-status-changed",
  searchWindowOpen: "search-window-open",
  deepLinkNavigate: "deep-link-navigate",
  desktopNavigate: "desktop-navigate",
  desktopNavigateWithParams: "desktop-navigate-with-params",
} as const;

export function onNotificationCreated<T = unknown>(
  handler: (payload: T) => void,
): Promise<UnlistenFn> {
  return listen<T>(DesktopEvents.notificationCreated, (event) =>
    handler(event.payload),
  );
}

export function onSyncStatusChanged<T = unknown>(
  handler: (payload: T) => void,
): Promise<UnlistenFn> {
  return listen<T>(DesktopEvents.syncStatusChanged, (event) =>
    handler(event.payload),
  );
}
