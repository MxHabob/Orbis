/**
 * @module stores/notification
 *
 * Svelte 5 Rune-based Notification Store for Orbis Desktop.
 * Manages notification state, unread badge count, real-time Tauri event listening,
 * live SSE stream subscriptions, read/dismiss mutations, and toast triggers.
 */

import { toast } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
import { isDesktopApp } from "$lib/api/runtime";
import type { NotificationDto } from "$lib/api/types";

type NotificationListFilter = {
  category?: string;
  is_dismissed?: boolean;
};

class NotificationStore {
  items = $state<NotificationDto[]>([]);
  unreadCount = $state(0);
  isLoading = $state(false);
  isLoaded = $state(false);
  loadError = $state(false);
  activeFilter = $state<NotificationListFilter>({ is_dismissed: false });

  hasUnread = $derived(this.unreadCount > 0);

  private unlistenFn: (() => void) | null = null;
  private sseSource: EventSource | null = null;
  private loadRequestId = 0;

  async init(): Promise<void> {
    if (this.isLoaded) return;
    await Promise.all([this.loadUnreadCount(), this.loadNotifications()]);
    void this.setupLiveUpdates();
  }

  async loadNotifications(
    filter: NotificationListFilter = this.activeFilter,
  ): Promise<void> {
    this.activeFilter = { ...filter };
    const requestId = ++this.loadRequestId;
    this.isLoading = true;
    this.loadError = false;
    try {
      const res = await api.listNotifications(this.activeFilter);
      if (requestId !== this.loadRequestId) return;
      this.items = Array.isArray(res) ? res : [];
      this.isLoaded = true;
    } catch (e: unknown) {
      if (requestId !== this.loadRequestId) return;
      console.warn("Failed to load notifications:", extractErrorMessage(e));
      this.items = [];
      this.loadError = true;
    } finally {
      if (requestId === this.loadRequestId) this.isLoading = false;
    }
  }

  async loadUnreadCount(): Promise<void> {
    try {
      const res = await api.getUnreadNotificationCount();
      this.unreadCount = res?.count ?? 0;
    } catch (e: unknown) {
      console.warn("Failed to load unread count:", extractErrorMessage(e));
    }
  }

  async markAsRead(id: string): Promise<void> {
    // Optimistic update
    const item = this.items.find((n) => n.id === id);
    if (item && !item.is_read) {
      item.is_read = true;
      item.read_at = new Date().toISOString();
      if (this.unreadCount > 0) this.unreadCount--;
    }

    try {
      await api.markNotificationRead(id);
    } catch (e: unknown) {
      console.warn("Failed to mark notification read:", extractErrorMessage(e));
      await this.loadUnreadCount();
    }
  }

  async markAllAsRead(): Promise<void> {
    // Optimistic update
    for (const item of this.items) {
      item.is_read = true;
    }
    this.unreadCount = 0;

    try {
      await api.markAllNotificationsRead();
    } catch (e: unknown) {
      console.warn("Failed to mark all read:", extractErrorMessage(e));
      await this.loadUnreadCount();
      await this.loadNotifications();
    }
  }

  async dismiss(id: string): Promise<void> {
    // Optimistic update
    const prev = this.items;
    const item = prev.find((n) => n.id === id);
    if (item && !item.is_read && this.unreadCount > 0) {
      this.unreadCount--;
    }
    this.items = this.items.filter((n) => n.id !== id);

    try {
      await api.dismissNotification(id);
    } catch (e: unknown) {
      console.warn("Failed to dismiss notification:", extractErrorMessage(e));
      this.items = prev;
    }
  }

  async archiveAll(): Promise<void> {
    const notifications: NotificationDto[] = [];
    const pageSize = 50;
    let offset = 0;

    try {
      while (true) {
        const page = await api.listNotifications({
          is_dismissed: false,
          limit: pageSize,
          offset,
        });
        notifications.push(...page);
        if (page.length < pageSize) break;
        offset += page.length;
      }

      const results = await Promise.allSettled(
        notifications.map((item) => api.dismissNotification(item.id)),
      );
      const failures = results.filter(
        (result): result is PromiseRejectedResult =>
          result.status === "rejected",
      );

      await Promise.all([this.loadNotifications(), this.loadUnreadCount()]);
      if (failures.length > 0) {
        console.error("Failed to archive some notifications:", failures);
        toast.error(extractErrorMessage(failures[0].reason));
      }
    } catch (error) {
      console.error("Failed to archive all notifications:", error);
      toast.error(extractErrorMessage(error));
    }
  }

  private async setupEventListener(): Promise<void> {
    if (!isDesktopApp) return;

    try {
      const { listen } = await import("@tauri-apps/api/event");
      const unlisten = await listen<{ id: string; title: string }>(
        "notification-created",
        (event) => {
          this.unreadCount++;
          void this.loadNotifications();
          toast.info(event.payload.title);
        },
      );
      this.unlistenFn = unlisten;
    } catch (e: unknown) {
      console.debug("Tauri event listener setup skipped:", e);
    }
  }

  private async setupLiveUpdates(): Promise<void> {
    if (typeof window === "undefined") return;
    try {
      let baseUrl = import.meta.env?.VITE_API_URL ?? "http://localhost:3000";
      if (isDesktopApp) {
        const connection = await api.connectionGetConfig();
        if (connection.mode !== "api") {
          await this.setupEventListener();
          return;
        }
        if (!connection.api?.server_url) {
          console.warn(
            "Cannot start notification stream: API server URL is missing.",
          );
          return;
        }
        baseUrl = connection.api.server_url;
      }

      this.setupSse(baseUrl);
    } catch (e: unknown) {
      console.warn(
        "Failed to set up notification updates:",
        extractErrorMessage(e),
      );
    }
  }

  private setupSse(baseUrl: string): void {
    if (typeof EventSource === "undefined") return;

    try {
      const token = localStorage.getItem("orbis_token");
      if (!token) return;

      const url = `${baseUrl.replace(/\/$/, "")}/api/v1/notifications/stream?token=${encodeURIComponent(token)}`;
      const es = new EventSource(url);
      es.addEventListener("notification", (e) => {
        try {
          const notif = JSON.parse(e.data) as NotificationDto;
          this.unreadCount++;
          if (
            !this.activeFilter.is_dismissed &&
            (!this.activeFilter.category ||
              notif.category === this.activeFilter.category)
          ) {
            this.items = [
              notif,
              ...this.items.filter((i) => i.id !== notif.id),
            ];
          }
          toast.info(notif.title);
        } catch (err: unknown) {
          console.debug("Error parsing SSE notification payload:", err);
        }
      });
      this.sseSource = es;
    } catch (e: unknown) {
      console.debug("SSE setup skipped:", e);
    }
  }

  destroy(): void {
    if (this.unlistenFn) {
      this.unlistenFn();
      this.unlistenFn = null;
    }
    if (this.sseSource) {
      this.sseSource.close();
      this.sseSource = null;
    }
  }
}

export const notificationStore = new NotificationStore();
