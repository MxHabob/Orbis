import { authClient } from "@/lib/auth-client";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useCallback, useEffect, useMemo } from "react";
import { onNotificationCreated } from "@orbis/desktop-client/events";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { getUserId } from "@/lib/session";
import { useTRPC } from "@/ipc/query-adapter";

export interface Activity {
  id: number | string;
  type: string;
  title: string;
  message: string;
  status: "unread" | "read" | "archived";
  isRead: boolean;
  isArchived: boolean;
  isHighPriority: boolean;
  createdAt: Date | string;
  metadata?: Record<string, unknown>;
}

export interface NotificationResponse {
  data: Activity[];
  meta: {
    cursor: string | null;
    hasNextPage: boolean;
    hasPreviousPage: boolean;
  };
}

export function getMetadata(activity: Activity): Record<string, unknown> {
  return activity.metadata ?? {};
}

export function getMetadataProperty(activity: Activity, key: string): unknown {
  return getMetadata(activity)[key];
}

export function useNotifications() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { data: session } = authClient.useSession();
  const userId = session?.user?.id ?? getUserId();

  const {
    data: activitiesData,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.notifications.all(userId, { status: "inbox" }),
    queryFn: async () => {
      const rows = await commands.notifications.getAll(userId);
      return {
        data: rows as unknown as Activity[],
        meta: { cursor: null, hasNextPage: false, hasPreviousPage: false },
      } satisfies NotificationResponse;
    },
    enabled: !!userId,
  });

  const { data: archivedActivitiesData, isLoading: archivedIsLoading } =
    useQuery({
      queryKey: queryKeys.notifications.all(userId, { status: "archived" }),
      queryFn: async () => {
        const rows = await commands.notifications.getAll(userId, {
          status: "archived",
        });
        return {
          data: rows as unknown as Activity[],
          meta: { cursor: null, hasNextPage: false, hasPreviousPage: false },
        } satisfies NotificationResponse;
      },
      enabled: !!userId,
    });

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    onNotificationCreated(() => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.notifications.all(userId),
      });
    }).then((fn) => {
      unlisten = fn;
    });
    return () => unlisten?.();
  }, [queryClient, userId]);

  const updateStatusMutation = useMutation(
    trpc.notifications.updateStatus.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: queryKeys.notifications.all(userId),
        });
      },
    }),
  );

  const updateAllStatusMutation = useMutation(
    trpc.notifications.updateAllStatus.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: queryKeys.notifications.all(userId),
        });
      },
    }),
  );

  const notifications = activitiesData?.data ?? [];
  const archivedNotifications = archivedActivitiesData?.data ?? [];

  const markMessageAsRead = useCallback(
    (messageId: number | string) => {
      updateStatusMutation.mutate({
        activityId: messageId,
        status: "archived",
      });
    },
    [updateStatusMutation],
  );

  const markAllMessagesAsRead = useCallback(() => {
    updateAllStatusMutation.mutate({ status: "archived" });
  }, [updateAllStatusMutation]);

  const markAllMessagesAsSeen = useCallback(() => {
    updateAllStatusMutation.mutate({ status: "read" });
  }, [updateAllStatusMutation]);

  const hasUnseenNotifications = useMemo(
    () => notifications.some((n) => n.status === "unread"),
    [notifications],
  );

  return {
    isLoading: isLoading || archivedIsLoading,
    error,
    notifications,
    archived: archivedNotifications,
    hasUnseenNotifications,
    markMessageAsRead,
    markAllMessagesAsRead,
    markAllMessagesAsSeen,
  };
}
