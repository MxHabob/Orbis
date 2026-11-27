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
  if (
    typeof response === "object" &&
    response !== null &&
    "error" in response &&
    response.error !== undefined &&
    response.error !== null
  ) {
    const err = response.error;
    const message =
      err instanceof Error
        ? err.message
        : typeof err === "object" &&
            "message" in err &&
            typeof err.message === "string"
          ? err.message
          : typeof err === "string"
            ? err
            : "Notification request failed.";
    throw new Error(message);
  }

  let data = response;
  while (typeof data === "object" && data !== null && "data" in data) {
    data = (data as Record<string, unknown>).data;
  }
  if (data === undefined || data === null) {
    throw new Error("Notification response did not include data.");
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
  const payload = extractData<UnreadCountDto>(res);
  if (typeof payload.count !== "number") {
    throw new Error("Invalid unread notification count response.");
  }
  return payload.count;
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
