/**
 * @module @orbis/mobile/lib/api/notifications
 *
 * API functions for the notification system.
 * Wraps the base HTTP client until the SDK generator includes notification endpoints.
 */

import { api } from "./client";
import type {
  ApiResponse,
  ListNotificationsParams,
  MarkAllReadDto,
  NotificationDto,
  NotificationPreferenceInput,
  NotificationPreferenceResponseDto,
  UnreadCountDto,
} from "./types";

function extractData<T>(response: unknown): T {
  let data = response;
  while (typeof data === "object" && data !== null && "data" in data) {
    data = (data as Record<string, unknown>).data;
  }
  return data as T;
}

export async function fetchNotifications(
  params?: ListNotificationsParams,
): Promise<NotificationDto[]> {
  const query = new URLSearchParams();
  if (params?.category) query.set("category", params.category);
  if (params?.is_read !== undefined)
    query.set("is_read", String(params.is_read));
  if (params?.limit) query.set("limit", String(params.limit));
  if (params?.offset) query.set("offset", String(params.offset));

  const qs = query.toString();
  const url = `/api/v1/notifications${qs ? `?${qs}` : ""}`;
  const res = await api.client.get<ApiResponse<NotificationDto[]>>({ url });
  return extractData<NotificationDto[]>(res);
}

export async function fetchUnreadCount(): Promise<number> {
  const res = await api.client.get<ApiResponse<UnreadCountDto>>({
    url: "/api/v1/notifications/unread-count",
  });
  return extractData<UnreadCountDto>(res).count;
}

export async function markNotificationRead(id: string): Promise<void> {
  await api.client.patch({ url: `/api/v1/notifications/${id}/read` });
}

export async function markAllNotificationsRead(): Promise<number> {
  const res = await api.client.post<ApiResponse<MarkAllReadDto>>({
    url: "/api/v1/notifications/read-all",
  });
  return extractData<MarkAllReadDto>(res).updated;
}

export async function dismissNotification(id: string): Promise<void> {
  await api.client.patch({ url: `/api/v1/notifications/${id}/dismiss` });
}

export async function fetchNotificationPreferences(): Promise<
  NotificationPreferenceResponseDto[]
> {
  const res = await api.client.get<
    ApiResponse<NotificationPreferenceResponseDto[]>
  >({
    url: "/api/v1/notifications/preferences",
  });
  return extractData<NotificationPreferenceResponseDto[]>(res);
}

export async function updateNotificationPreferences(
  preferences: NotificationPreferenceInput[],
): Promise<NotificationPreferenceResponseDto[]> {
  const res = await api.client.put<
    ApiResponse<NotificationPreferenceResponseDto[]>
  >({
    url: "/api/v1/notifications/preferences",
    body: { preferences },
    headers: { "Content-Type": "application/json" },
  });
  return extractData<NotificationPreferenceResponseDto[]>(res);
}
