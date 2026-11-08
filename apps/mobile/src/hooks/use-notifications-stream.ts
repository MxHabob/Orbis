/**
 * @module @orbis/mobile/hooks/use-notifications-stream
 *
 * Real-time SSE notification listener for Orbis Mobile.
 * Connects to `/api/v1/notifications/stream`, listens for live notifications,
 * invalidates React Query caches, updates unread count, and presents in-app toasts.
 */

import { useEffect, useRef } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useAuth } from "@/lib/api/auth-context";
import { API_BASE_URL } from "@/lib/api/client";
import { authStorage } from "@/lib/api/storage";
import { queryKeys } from "@/lib/api/query-keys";
import type { NotificationDto } from "@/lib/api/types";
import { useToast } from "@/components/ui/toast";
import { useI18n } from "@/lib/i18n";
import { subscribeToNetworkStatus } from "@/lib/network/online-manager";

export function useNotificationsStream(): void {
  const { isAuthenticated } = useAuth();
  const queryClient = useQueryClient();
  const toast = useToast();
  const { isArabic } = useI18n();

  const isArabicRef = useRef(isArabic);
  useEffect(() => {
    isArabicRef.current = isArabic;
  }, [isArabic]);

  useEffect(() => {
    if (!isAuthenticated) return;

    let isCancelled = false;
    let xhr: XMLHttpRequest | null = null;
    let reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
    let retryDelay = 2000;
    const MAX_RETRY_DELAY = 30000;

    const scheduleReconnect = () => {
      if (isCancelled) return;
      if (xhr) {
        xhr.abort();
        xhr = null;
      }
      if (reconnectTimeout) {
        clearTimeout(reconnectTimeout);
      }
      reconnectTimeout = setTimeout(() => {
        retryDelay = Math.min(retryDelay * 1.5, MAX_RETRY_DELAY);
        void connect();
      }, retryDelay);
    };

    const connect = async () => {
      if (isCancelled) return;

      const token = await authStorage.getAccessToken();
      if (!token || isCancelled) return;

      try {
        const streamUrl = `${API_BASE_URL}/api/v1/notifications/stream?token=${encodeURIComponent(token)}`;
        xhr = new XMLHttpRequest();
        xhr.open("GET", streamUrl, true);
        xhr.setRequestHeader("Accept", "text/event-stream");
        xhr.setRequestHeader("Authorization", `Bearer ${token}`);

        let lastIndex = 0;

        xhr.onprogress = () => {
          if (isCancelled || !xhr) return;
          retryDelay = 2000; // Reset retry delay on active progress

          const text = xhr.responseText;
          const chunk = text.slice(lastIndex);
          lastIndex = text.length;

          // SSE parsing: events are separated by double newline
          const messages = chunk.split("\n\n");
          for (const message of messages) {
            const lines = message.split("\n");
            let eventType = "message";
            let data = "";

            for (const line of lines) {
              if (line.startsWith("event:")) {
                eventType = line.slice(6).trim();
              } else if (line.startsWith("data:")) {
                data += line.slice(5).trim();
              }
            }

            if (eventType === "notification" && data) {
              try {
                const notif = JSON.parse(data) as NotificationDto;
                // Invalidate query caches
                void queryClient.invalidateQueries({
                  queryKey: queryKeys.notifications.all,
                });

                // Display toast
                const title =
                  isArabicRef.current && notif.title_ar
                    ? notif.title_ar
                    : notif.title;
                toast.info(title);
              } catch {
                // Ignore malformed chunk
              }
            }
          }
        };

        xhr.onerror = () => {
          scheduleReconnect();
        };

        xhr.onreadystatechange = () => {
          if (!xhr) return;
          if (xhr.readyState === 4 && !isCancelled) {
            // Connection closed by server or error, schedule reconnect
            scheduleReconnect();
          }
        };

        xhr.send();
      } catch {
        scheduleReconnect();
      }
    };

    const unsubscribeNetwork = subscribeToNetworkStatus((status) => {
      if (status.isOnline && !xhr) {
        retryDelay = 2000;
        void connect();
      }
    });

    void connect();

    return () => {
      isCancelled = true;
      if (reconnectTimeout) {
        clearTimeout(reconnectTimeout);
      }
      if (xhr) {
        xhr.abort();
        xhr = null;
      }
      unsubscribeNetwork();
    };
  }, [isAuthenticated, queryClient, toast]);
}
